use {
    crate::{
        DriverBox,
        ether::{self, Buffer, Ether, InterruptCause, RxFrame},
        state_markers::Opened,
    },
    core::{
        cell::{RefCell, RefMut},
        pin::Pin,
    },
    smoltcp::{
        phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken},
        time::Instant,
    },
};

pub struct Dev<const MTU: usize> {
    eth: RefCell<DriverBox<Ether<'static, MTU, Opened>>>,
    capabilities: DeviceCapabilities,
}

/// The `RefCell` borrow of the driver, owned by the RX token's guard for the
/// token's lifetime.
type EthRefMut<'a, const MTU: usize> = RefMut<'a, DriverBox<Ether<'static, MTU, Opened>>>;

/// Holds the frame as an [`RxFrame`] guard owning the `RefCell` borrow: the
/// loan is exclusive by construction, and the guard's `Drop` releases the
/// frame (`BufferRelease`), tolerating link-down instead of panicking.
pub struct EthernetRxToken<'a, const MTU: usize>(RxFrame<'static, EthRefMut<'a, MTU>, MTU>);

pub struct EthernetTxToken<'a, const MTU: usize>(
    Option<Pin<&'static mut Buffer<MTU>>>,
    &'a RefCell<DriverBox<Ether<'static, MTU, Opened>>>,
);

impl<const MTU: usize> Dev<MTU> {
    pub fn new(
        eth: DriverBox<Ether<'static, MTU, Opened>>,
        capabilities: DeviceCapabilities,
    ) -> Self {
        assert!(
            capabilities.medium == Medium::Ethernet,
            "Ethernet device must have Ethernet medium"
        );

        Self {
            eth: RefCell::new(eth),
            capabilities,
        }
    }

    pub fn eth(&mut self) -> Pin<&mut Ether<'static, MTU, Opened>> {
        self.eth.get_mut().as_mut()
    }

    /// Polls the PHY for link-state changes; see [`Ether::link_process`] for
    /// the full cost analysis.
    ///
    /// **MDIO cost:** with `ETHER_CFG_USE_LINKSTA == 0` (default) this
    /// performs a blocking, bit-banged MDIO read that stalls the CPU for tens
    /// of microseconds.  Call from a slow periodic task (100 ms – 1 s), not
    /// the hot smoltcp poll loop.  Setting `ETHER_CFG_USE_LINKSTA = 1` in
    /// FSP_CFG eliminates the cost entirely when the PHY LINKSTA pin is wired.
    pub fn poll_link(&mut self) {
        if self.eth().get_open() != 0 {
            _ = self.eth().link_process();
        }
    }

    pub fn is_up(&mut self) -> bool {
        self.eth().is_up()
    }

    pub fn populate_buffers(&mut self, cause: InterruptCause) {
        if cause.went_up {
            self.eth().as_mut().update_rx_buffers(cause);
        }
        // TX needs no work here: buffer reclamation is driven by the
        // descriptor TACT flag in `take_tx_buf`, not by the TC interrupt.
    }

    fn send_buffer(&mut self) -> Option<Pin<&'static mut Buffer<MTU>>> {
        // &mut self proves exclusivity here; get_mut avoids the RefCell runtime
        // borrow check (and its panic path) that borrow_mut would incur.
        self.eth.get_mut().as_mut().take_tx_buf()
    }
}

impl<const MTU: usize> Device for Dev<MTU> {
    type RxToken<'a> = EthernetRxToken<'a, MTU>;
    type TxToken<'a> = EthernetTxToken<'a, MTU>;

    // N.B.: Tokens are mutable borrowing `self`, so it is impossible to have
    //       multiple tokens.

    fn receive(&mut self, _: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Fast-path: one volatile SRAM read — no FFI, no MMIO. Avoids the TX
        // descriptor take/park roundtrip and the R_ETHER_Read call (which does a
        // link check + ETHERC MMIO read) on the common idle-poll case where there
        // is no pending RX frame. Can false-positive when the ring is unarmed, in
        // which case read_zerocopy below will return the real error.
        if !self.eth.get_mut().rx_pending() {
            return None;
        }

        let tx = self.send_buffer()?;
        // The RX guard owns the `RefCell` borrow for the token's lifetime.
        // smoltcp consumes the RX token (dropping the guard and its borrow)
        // before it uses the TX token, so the TX token's deferred
        // `borrow_mut` cannot collide with it; a hypothetical out-of-order
        // use would surface as a `RefCell` panic, not as aliasing.
        // (Two steps so the error path's `RefMut` is fully gone before the
        // driver is re-borrowed to park the TX buffer back.)
        let frame = match RxFrame::read(self.eth.borrow_mut()) {
            Ok(frame) => Some(frame),
            Err((eth, err)) => {
                drop(eth);
                if err != ether::FSP_ERR_ETHER_ERROR_NO_DATA {
                    log::error!("Ethernet read error: {err}");
                }
                return None;
            }
        };
        let Some(frame) = frame else {
            self.eth.borrow_mut().as_mut().tx_buffer_update(tx);
            return None;
        };

        Some((
            EthernetRxToken(frame),
            EthernetTxToken(Some(tx), &self.eth),
        ))
    }

    fn transmit(&mut self, _: Instant) -> Option<Self::TxToken<'_>> {
        Some(EthernetTxToken(Some(self.send_buffer()?), &self.eth))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        self.capabilities.clone()
    }
}

impl<const MTU: usize> RxToken for EthernetRxToken<'_, MTU> {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        // The guard derefs to the frame slice; dropping `self` afterwards
        // releases the descriptor (BufferRelease) and the RefCell borrow.
        f(&self.0)
    }
}

impl<const MTU: usize> TxToken for EthernetTxToken<'_, MTU> {
    fn consume<R, F>(mut self, length: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut pin_buf = self.0.take().unwrap();
        let buf = pin_buf.as_mut().as_mut_bytes();
        let result = f(&mut buf[..length]);

        if length < 60 {
            buf[length..60].fill(0);
        }

        // This token always carries a buffer taken from `take_tx_buf`, so the
        // descriptor slot is empty and a successful submit reclaims nothing
        // (`Ok(None)`). On failure the buffer is handed back un-submitted; park
        // it into its slot so it isn't lost. (Bind the result first so the
        // `RefCell` borrow is released before the re-borrow in the error arm.)
        let res = self
            .1
            .borrow_mut()
            .as_mut()
            .write_zerocopy(pin_buf, length.max(60));

        match res {
            Ok(_reclaimed) => result,
            Err((buf, err)) => {
                if err != ether::FSP_ERR_ETHER_ERROR_LINK {
                    log::error!("Failed to write to the network: {err}");
                }
                self.1.borrow_mut().as_mut().tx_buffer_update(buf);
                result
            }
        }
    }
}

impl<const MTU: usize> Drop for EthernetTxToken<'_, MTU> {
    fn drop(&mut self) {
        if let Some(buf) = self.0.take() {
            log::trace!("Dropping Unused TxToken");
            let back = self.1.borrow_mut().as_mut().tx_buffer_update(buf);

            assert!(back.is_none(), "Going to leak the transmit buffer");
        }
    }
}
