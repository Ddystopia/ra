use {
    crate::ether::{self, Buffer, EtherInstance},
    core::{cell::RefCell, pin::Pin},
    ra_fsp_sys::r_ether::InterruptCause,
    smoltcp::{
        phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken},
        time::Instant,
    },
};

pub struct Dev<const MTU: usize> {
    eth: RefCell<Pin<&'static mut EtherInstance<MTU>>>,
    capabilities: DeviceCapabilities,
}

pub struct EthernetRxToken<'a, const MTU: usize>(
    Option<Pin<&'static mut Buffer<MTU>>>,
    usize,
    &'a RefCell<Pin<&'static mut EtherInstance<MTU>>>,
);

pub struct EthernetTxToken<'a, const MTU: usize>(
    Option<Pin<&'static mut Buffer<MTU>>>,
    &'a RefCell<Pin<&'static mut EtherInstance<MTU>>>,
);

impl<const MTU: usize> Dev<MTU> {
    pub fn new(
        eth: Pin<&'static mut EtherInstance<MTU>>,
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

    pub fn eth(&mut self) -> Pin<&mut EtherInstance<MTU>> {
        self.eth.get_mut().as_mut()
    }

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
        } else if cause.transmits {
            self.eth().as_mut().update_tx_buffers(cause);
        }
    }

    fn send_buffer(&mut self) -> Option<Pin<&'static mut Buffer<MTU>>> {
        self.eth.borrow_mut().as_mut().take_tx_buf()
    }
}

impl<const MTU: usize> Device for Dev<MTU> {
    type RxToken<'a> = EthernetRxToken<'a, MTU>;
    type TxToken<'a> = EthernetTxToken<'a, MTU>;

    // N.B.: Tokens are mutable borrowing `self`, so it is impossible to have
    //       multiple tokens.

    fn receive(&mut self, _: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let tx = self.send_buffer()?;
        // smoltcp never drops rx token, so it is fine to read it right away
        match self.eth().read_zerocopy() {
            Ok((buf, len)) => Some((
                EthernetRxToken(Some(buf), len, &self.eth),
                EthernetTxToken(Some(tx), &self.eth),
            )),
            Err(ether::FSP_ERR_ETHER_ERROR_NO_DATA) => {
                self.eth().as_mut().tx_buffer_update(tx);
                None
            }
            Err(err) => {
                log::error!("Ethernet read error: {err}");
                self.eth().as_mut().tx_buffer_update(tx);
                None
            }
        }
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
        f(&self.0.as_ref().unwrap()[..self.1])
    }
}

impl<const MTU: usize> Drop for EthernetRxToken<'_, MTU> {
    fn drop(&mut self) {
        let buf = self.0.take().unwrap();

        self.2
            .borrow_mut()
            .as_mut()
            .rx_buffer_update(buf)
            .expect("Failed to update buffer")
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

        match self
            .1
            .borrow_mut()
            .as_mut()
            .write_zerocopy(pin_buf, length.max(60))
        {
            Ok(()) | Err(ether::FSP_ERR_ETHER_ERROR_LINK) => result,
            Err(err) => {
                log::error!("Failed to write to the network: {err}");
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
