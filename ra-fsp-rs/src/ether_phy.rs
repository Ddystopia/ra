#![allow(non_upper_case_globals)]

use core::{pin::Pin, ptr};

use crate::unsafe_pinned::UnsafePinned;

use ra_fsp_sys::generated::{
    fsp_err_t, st_ether_phy_instance_ctrl, R_ETHER_PHY_Close, R_ETHER_PHY_Open,
};

pub use ra_fsp_sys::generated::{
    e_ether_phy_flow_control,
    e_ether_phy_lsi_type,
    e_ether_phy_mii_type,
    ether_phy_api_t,
    ether_phy_cfg_t,
    ether_phy_instance_ctrl_t,
    ether_phy_instance_t,
    g_ether_phy_on_ether_phy,
};

pub struct EtherPhyInstance(UnsafePinned<ether_phy_instance_ctrl_t>);

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

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub type for_c_dyn_macro_Config = ether_phy_cfg_t;
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub type for_c_dyn_macro_Instance = EtherPhyInstance;

pub unsafe trait EtherPhy {
    fn open(self: Pin<&mut Self>, conf: &'static ether_phy_cfg_t) -> Result<(), fsp_err_t>;
    fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t>;
    fn c_dyn(self: Pin<&mut Self>, conf: &'static ether_phy_cfg_t) -> ether_phy_instance_t;
}

pub const fn c_dyn(
    this: Pin<&mut EtherPhyInstance>,
    conf: &'static ether_phy_cfg_t,
) -> ether_phy_instance_t {
    ether_phy_instance_t {
        p_ctrl: get_mut(this),
        p_cfg: conf,
        p_api: &raw const g_ether_phy_on_ether_phy,
    }
}

unsafe impl EtherPhy for EtherPhyInstance {
    fn open(self: Pin<&mut Self>, conf: &'static ether_phy_cfg_t) -> Result<(), fsp_err_t> {
        match unsafe { R_ETHER_PHY_Open(get_mut(self), conf) } {
            0 => Ok(()),
            err => Err(err),
        }
    }
    fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t> {
        match unsafe { R_ETHER_PHY_Close(get_mut(self)) } {
            0 => Ok(()),
            err => Err(err),
        }
    }
    fn c_dyn(self: Pin<&mut Self>, conf: &'static ether_phy_cfg_t) -> ether_phy_instance_t {
        c_dyn(self, conf)
    }
}

#[inline(always)]
const fn get_mut(this: Pin<&mut EtherPhyInstance>) -> *mut core::ffi::c_void {
    unsafe { this.get_unchecked_mut().ptr().cast() }
}

impl EtherPhyInstance {
    pub const fn new() -> Self {
        Self(UnsafePinned::new(st_ether_phy_instance_ctrl {
            open: 0,
            p_ether_phy_cfg: ptr::null(),
            p_reg_pir: ptr::null_mut(),
            local_advertise: 0,
        }))
    }

    #[inline(always)]
    pub const fn ptr(&self) -> *mut ether_phy_instance_ctrl_t {
        UnsafePinned::raw_get(&raw const self.0)
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
            flow_control: self.flow_control as u32,
            mii_type: self.mii_type,
            p_context: ptr::null(),
            p_extend: ptr::null(),
        }
    }
}

// fsp_err_t R_ETHER_PHY_Open(ether_phy_ctrl_t * const p_ctrl, ether_phy_cfg_t const * const p_cfg) __attribute__((optimize("O0")));
//
// fsp_err_t R_ETHER_PHY_Close(ether_phy_ctrl_t * const p_ctrl);
//
// fsp_err_t R_ETHER_PHY_ChipInit(ether_phy_ctrl_t * const p_ctrl, ether_phy_cfg_t const * const p_cfg);
//
// fsp_err_t R_ETHER_PHY_Read(ether_phy_ctrl_t * const p_ctrl, uint32_t reg_addr, uint32_t * const p_data);
//
// fsp_err_t R_ETHER_PHY_Write(ether_phy_ctrl_t * const p_ctrl, uint32_t reg_addr, uint32_t data);
//
// fsp_err_t R_ETHER_PHY_StartAutoNegotiate(ether_phy_ctrl_t * const p_ctrl);
//
// fsp_err_t R_ETHER_PHY_LinkPartnerAbilityGet(ether_phy_ctrl_t * const p_ctrl,
//                                             uint32_t * const         p_line_speed_duplex,
//                                             uint32_t * const         p_local_pause,
//                                             uint32_t * const         p_partner_pause);
//
// fsp_err_t R_ETHER_PHY_LinkStatusGet(ether_phy_ctrl_t * const p_ctrl);
