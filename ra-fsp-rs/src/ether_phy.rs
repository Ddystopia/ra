#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)] // pin-init pr

use core::{marker::PhantomData, pin::Pin, ptr};
use crate::pin_init::{PinInit, pin_data, pin_init_from_closure};
use ra_fsp_sys::generated::fsp_err_t;

use crate::{
    pac,
    state_markers::{Closed, Opened},
    unsafe_pinned::UnsafePinned,
};

pub use ra_fsp_sys::generated::{
    ETHER_PHY_CFG_PARAM_CHECKING_ENABLE, //
    e_ether_phy_lsi_type,
    e_ether_phy_mii_type,
    ether_phy_api_t,
    ether_phy_cfg_t,
    ether_phy_flow_control_t,
    ether_phy_instance_ctrl_t,
    ether_phy_instance_t,
    g_ether_phy_on_ether_phy,
};

const _: () = assert!(
    ETHER_PHY_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option ETHER_PHY_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

#[pin_data]
pub struct EtherPhy<S: 'static> {
    ctrl: UnsafePinned<ether_phy_instance_ctrl_t>,
    cfg: UnsafePinned<ether_phy_cfg_t>,
    inst: UnsafePinned<ether_phy_instance_t>,
    regs: pac::EDMAC0,
    _marker: PhantomData<S>,
}

#[derive(Debug, Copy, Clone)]
pub struct EtherPhyConfig {
    pub channel: u8,
    pub phy_lsi_address: u8,
    pub phy_reset_wait_time: u32,
    pub mii_bit_access_wait_time: i32,
    pub phy_lsi_type: e_ether_phy_lsi_type,
    pub flow_control: bool,
    pub mii_type: e_ether_phy_mii_type,
}

unsafe impl<S> crate::LifetimeDriver for EtherPhy<S> {
    type Target<'a> = EtherPhy<S>;
}
unsafe impl<S> crate::Block for EtherPhy<S> {
    type Config = ether_phy_cfg_t;
    type Instance = ether_phy_instance_t;
    type Api = ether_phy_api_t;
    type State = S;

    const API: &ether_phy_api_t = unsafe { &g_ether_phy_on_ether_phy };

    fn ctrl(&self) -> *mut core::ffi::c_void {
        UnsafePinned::raw_get(&raw const self.ctrl).cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}


impl EtherPhy<Closed> {
    pub const fn new_closed(edmac: pac::EDMAC0, conf: EtherPhyConfig) -> impl PinInit<Self> {
        unsafe {
            pin_init_from_closure(move |slot: *mut Self| {
                ptr::write(
                    slot,
                    Self {
                        ctrl: UnsafePinned::new(core::mem::zeroed()),
                        cfg: UnsafePinned::new(conf.c_conf()),
                        regs: edmac,
                        inst: UnsafePinned::new(ether_phy_instance_t {
                            p_ctrl: ptr::null_mut(),
                            p_cfg: ptr::null(),
                            p_api: <Self as crate::Block>::API,
                        }),
                        _marker: PhantomData,
                    },
                );
                (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast();
                (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const().cast();
                Ok(())
            })
        }
    }
}

impl EtherPhyConfig {
    pub const fn c_conf(self) -> ether_phy_cfg_t {
        ether_phy_cfg_t {
            channel: self.channel,
            phy_lsi_address: self.phy_lsi_address as u8,
            phy_reset_wait_time: self.phy_reset_wait_time,
            mii_bit_access_wait_time: self.mii_bit_access_wait_time,
            phy_lsi_type: self.phy_lsi_type,
            flow_control: self.flow_control as ether_phy_flow_control_t,
            mii_type: self.mii_type,
            p_context: ptr::null(),
            p_extend: ptr::null(),
        }
    }
}
