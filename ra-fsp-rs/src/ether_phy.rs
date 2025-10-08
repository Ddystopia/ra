#![allow(non_upper_case_globals)]
use {crate::unsafe_pinned::UnsafePinned, core::ptr};

pub use ra_fsp_sys::generated::{
    ETHER_PHY_CFG_PARAM_CHECKING_ENABLE, //
    e_ether_phy_lsi_type,
    e_ether_phy_mii_type,
    ether_phy_api_t,
    ether_phy_cfg_t,
    ether_phy_instance_ctrl_t,
    ether_phy_instance_t,
    g_ether_phy_on_ether_phy,
};

const _: () = assert!(
    ETHER_PHY_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option ETHER_PHY_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

pub struct EtherPhyInstance {
    ctrl: UnsafePinned<ether_phy_instance_ctrl_t>,
    cfg: UnsafePinned<ether_phy_cfg_t>,
    inst: ether_phy_instance_t,
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

unsafe impl crate::Block for EtherPhyInstance {
    type CConfig = ether_phy_cfg_t;
    type CInstance = ether_phy_instance_t;
    type CApi = ether_phy_api_t;

    const API: &ether_phy_api_t = unsafe { &g_ether_phy_on_ether_phy };

    fn instance(&mut self) -> &mut ether_phy_instance_t {
        self.inst.p_cfg = self.cfg.get().cast_const();
        self.inst.p_ctrl = self.ctrl.get().cast::<core::ffi::c_void>();
        &mut self.inst
    }
}

impl EtherPhyInstance {
    pub const fn new(edmac: crate::pac::EDMAC0, conf: EtherPhyConfig) -> Self {
        _ = edmac;

        unsafe {
            Self {
                ctrl: UnsafePinned::new(core::mem::zeroed()),
                cfg: UnsafePinned::new(conf.c_conf()),
                inst: ether_phy_instance_t {
                    p_ctrl: ptr::null_mut(),
                    p_cfg: ptr::null(),
                    p_api: <Self as crate::Block>::API,
                },
            }
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
            flow_control: self.flow_control as u32,
            mii_type: self.mii_type,
            p_context: ptr::null(),
            p_extend: ptr::null(),
        }
    }
}
