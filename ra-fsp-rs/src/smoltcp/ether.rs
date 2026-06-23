use {
    crate::{
        DriverBox,
        ether::{self, Buffer, Ether, EtherRx, EtherTx, RxFrame},
        state_markers::Opened,
    },
    core::pin::Pin,
    smoltcp::{
        phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken},
        time::Instant,
    },
};

/// smoltcp [`Device`] over the zero-copy [`Ether`] driver.
///
/// Tokens are split borrows of the driver (INV-TXRX-DISJOINT): the RX token
/// owns an [`RxFrame`] over the RX half, the TX token owns the TX half plus a
/// reclaimed buffer. Both can be alive at once and may be consumed or dropped
/// in any order; no runtime borrow tracking is involved.
pub struct Dev {
    eth: DriverBox<Ether<'static, Opened>>,
    capabilities: DeviceCapabilities,
}

pub struct EthernetRxToken<'a>(RxFrame<EtherRx<'a>>);

pub struct EthernetTxToken<'a> {
    buf: Option<Pin<&'static mut Buffer>>,
    tx: EtherTx<'a>,
}

impl Dev {
    pub fn new(
        eth: DriverBox<Ether<'static, Opened>>,
        capabilities: DeviceCapabilities,
    ) -> Self {
        assert!(
            capabilities.medium == Medium::Ethernet,
            "Ethernet device must have Ethernet medium"
        );

        Self { eth, capabilities }
    }

    pub fn eth(&mut self) -> Pin<&mut Ether<'static, Opened>> {
        self.eth.as_mut()
    }

    /// Polls the PHY for link-state changes; see [`Ether::link_process`] for
    /// the full cost analysis.
    ///
    /// **MDIO cost:** with `ETHER_CFG_USE_LINKSTA == 0` (default) this
    /// performs a blocking, bit-banged MDIO read that stalls the CPU for tens
    /// of microseconds.  Call from a slow periodic task (100 ms – 1 s). Setting
    /// `ETHER_CFG_USE_LINKSTA = 1` in FSP_CFG eliminates the cost entirely when
    /// the PHY LINKSTA pin is wired.
    pub fn poll_link(&mut self) {
        if self.eth.get_open() != 0 {
            _ = self.eth().link_process();
        }
    }

    pub fn is_up(&self) -> bool {
        self.eth.is_up()
    }

    /// Re-arms the RX ring if FSP has reset it on a link-up. Safe to call
    /// unconditionally and repeatedly — the decision is derived from the
    /// driver's ring state, not from any link event; see
    /// [`Ether::update_rx_buffers`].
    pub fn populate_buffers(&mut self) {
        self.eth().update_rx_buffers();
        // TX needs no work here: buffer reclamation is driven by the
        // descriptor TACT flag in `take_tx_buf`, not by the TC interrupt.
    }
}

impl Device for Dev {
    type RxToken<'a> = EthernetRxToken<'a>;
    type TxToken<'a> = EthernetTxToken<'a>;

    fn receive(&mut self, _: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Fast-path: one volatile SRAM read — no FFI, no MMIO. Avoids the TX
        // descriptor take/park roundtrip and the R_ETHER_Read call (which does a
        // link check + ETHERC MMIO read) on the common idle-poll case where there
        // is no pending RX frame. Can false-positive when the ring is unarmed, in
        // which case the read below will return the real error.
        if !self.eth.rx_pending() {
            return None;
        }

        let (rx, mut tx) = self.eth.as_mut().split();
        let buf = tx.take_tx_buf()?;
        match RxFrame::read(rx) {
            Ok(frame) => Some((
                EthernetRxToken(frame),
                EthernetTxToken { buf: Some(buf), tx },
            )),
            // The RX half comes back in the error and is dropped; the TX half
            // is independent (INV-TXRX-DISJOINT), so park the buffer back.
            Err((_, err)) => {
                if err != ether::FSP_ERR_ETHER_ERROR_NO_DATA {
                    log::error!("Ethernet read error: {err}");
                }
                tx.tx_buffer_update(buf);
                None
            }
        }
    }

    fn transmit(&mut self, _: Instant) -> Option<Self::TxToken<'_>> {
        let (_, mut tx) = self.eth.as_mut().split();
        let buf = tx.take_tx_buf()?;
        Some(EthernetTxToken { buf: Some(buf), tx })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        self.capabilities.clone()
    }
}

impl RxToken for EthernetRxToken<'_> {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        // The guard derefs to the frame slice; dropping `self` afterwards
        // releases the descriptor (BufferRelease).
        //
        // N.B.: this surfaces a single RX fragment. smoltcp expects whole
        // frames, so the buffers must be sized so no frame splits (every buffer
        // >= MTU; see `INV-WHOLEFRAME`). A mixed/undersized pool that splits is
        // not reassembled here — that is the caller's job via the lower-level
        // [`RxFrame::position`] API, not the smoltcp glue.
        f(&self.0)
    }
}

impl TxToken for EthernetTxToken<'_> {
    fn consume<R, F>(mut self, length: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut pin_buf = self.buf.take().unwrap();
        let buf = pin_buf.as_mut().as_mut_bytes();
        let result = f(&mut buf[..length]);

        if length < 60 {
            buf[length..60].fill(0);
        }

        // This token always carries a buffer taken from `take_tx_buf`, so the
        // descriptor slot is empty and a successful submit reclaims nothing.
        // On failure the buffer is handed back un-submitted; park it into its
        // slot so it isn't lost.
        if let Err((buf, err)) = self.tx.write_zerocopy(pin_buf, length.max(60)) {
            if err != ether::FSP_ERR_ETHER_ERROR_LINK {
                log::error!("Failed to write to the network: {err}");
            }
            self.tx.tx_buffer_update(buf);
        }

        result
    }
}

impl Drop for EthernetTxToken<'_> {
    fn drop(&mut self) {
        if let Some(buf) = self.buf.take() {
            log::trace!("Dropping Unused TxToken");
            let back = self.tx.tx_buffer_update(buf);

            assert!(back.is_none(), "Going to leak the transmit buffer");
        }
    }
}
