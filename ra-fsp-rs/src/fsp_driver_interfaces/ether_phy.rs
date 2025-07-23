use core::pin::Pin;

use ra_fsp_sys::generated::{ether_phy_api_t, ether_phy_cfg_t, fsp_err_t};

pub unsafe trait EtherPhy {
    fn open(self: Pin<&mut Self>, conf: &'static ether_phy_cfg_t) -> Result<(), fsp_err_t>;
    fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t>;
    fn c_api(&self) -> &'static ether_phy_api_t;
}
