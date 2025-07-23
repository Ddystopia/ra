use core::pin::Pin;

use ra_fsp_sys::generated::{fsp_err_t, ioport_api_t, ioport_cfg_t};

pub unsafe trait IoPort {
    fn open(self: Pin<&mut Self>, conf: &'static ioport_cfg_t) -> Result<(), fsp_err_t>;
    fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t>;
    fn c_api(&self) -> &'static ioport_api_t;
}
