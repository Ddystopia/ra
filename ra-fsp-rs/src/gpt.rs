use core::pin::Pin;

use ra_fsp_sys::generated::{
    GPT_CFG_PARAM_CHECKING_ENABLE,
    R_GPT_Close, //
    R_GPT_Open,
    R_GPT0_Type,
    g_timer_on_gpt,
    gpt_instance_ctrl_t,
    timer_api_t,
    timer_cfg_t,
    timer_instance_t,
};

use crate::{Result, fsp_try_unsafe, unsafe_pinned::UnsafePinned};

const _: () = assert!(
    GPT_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option GPT_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

pub struct GptInstance {
    ctrl: UnsafePinned<gpt_instance_ctrl_t>,
    cfg: UnsafePinned<timer_cfg_t>,
    inst: UnsafePinned<timer_instance_t>,
}

unsafe impl crate::Block for GptInstance {
    type CConfig = timer_cfg_t;
    type CInstance = timer_instance_t;
    type CApi = timer_api_t;

    const API: &Self::CApi = unsafe { &g_timer_on_gpt };

    fn instance(self: Pin<&mut Self>) -> &Self::CInstance {
        unsafe {
            let this = self.get_unchecked_mut();
            if (*this.inst.get()).p_cfg.is_null() {
                (*this.inst.get()).p_ctrl = this.ctrl.get().cast::<core::ffi::c_void>();
                (*this.inst.get()).p_cfg = this.cfg.get().cast_const();
            }
            &*this.inst.get().cast_const()
        }
    }
}

impl GptInstance {
    pub const fn new(cfg: timer_cfg_t) -> Self {
        Self {
            ctrl: UnsafePinned::new(unsafe { core::mem::zeroed() }),
            cfg: UnsafePinned::new(cfg),
            inst: UnsafePinned::new(timer_instance_t {
                p_ctrl: core::ptr::null_mut(),
                p_cfg: core::ptr::null(),
                p_api: <Self as crate::Block>::API,
            }),
        }
    }

    pub fn open(self: Pin<&mut Self>) -> Result<()> {
        let p_cfg = self.cfg.get().cast_const();

        fsp_try_unsafe!(R_GPT_Open(self.ctrl_void(), p_cfg))
    }

    #[inline(always)]
    fn ctrl_void(self: Pin<&mut GptInstance>) -> *mut core::ffi::c_void {
        self.ctrl().cast()
    }

    #[inline(always)]
    fn ctrl(self: Pin<&mut Self>) -> *mut gpt_instance_ctrl_t {
        UnsafePinned::raw_get(&raw const self.ctrl)
    }

    #[inline(always)]
    pub const fn regs(&self) -> *mut R_GPT0_Type {
        unsafe { *self.ctrl.get() }.p_reg
    }
}

unsafe impl Send for GptInstance {}
unsafe impl Sync for GptInstance {}

impl Drop for GptInstance {
    fn drop(&mut self) {
        let this = unsafe { Pin::new_unchecked(self) };
        fsp_try_unsafe!(R_GPT_Close(this.ctrl_void())).expect("Error closing GPT timer");
    }
}
