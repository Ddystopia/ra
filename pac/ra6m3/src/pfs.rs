#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved_0_p000: [u8; 0x04],
    _reserved_1_p001: [u8; 0x04],
    _reserved_2_p002: [u8; 0x04],
    _reserved_3_p003: [u8; 0x04],
    _reserved_4_p004: [u8; 0x04],
    _reserved_5_p005: [u8; 0x04],
    _reserved_6_p006: [u8; 0x04],
    _reserved_7_p007: [u8; 0x04],
    _reserved_8_p008: [u8; 0x04],
    _reserved_9_p009: [u8; 0x04],
    p010pfs: P010PFS,
    _reserved11: [u8; 0x04],
    _reserved_11_p010pfs: [u8; 0x02],
    _reserved12: [u8; 0x06],
    _reserved_12_p014: [u8; 0x04],
    _reserved_13_p015: [u8; 0x04],
    _reserved_14_p100: [u8; 0x04],
    _reserved_15_p101: [u8; 0x04],
    _reserved_16_p102: [u8; 0x04],
    _reserved_17_p103: [u8; 0x04],
    _reserved_18_p104: [u8; 0x04],
    _reserved_19_p105: [u8; 0x04],
    _reserved_20_p106: [u8; 0x04],
    _reserved_21_p107: [u8; 0x04],
    _reserved_22_p108: [u8; 0x04],
    _reserved_23_p109: [u8; 0x04],
    _reserved_24_p110: [u8; 0x04],
    _reserved_25_p111: [u8; 0x04],
    _reserved_26_p112: [u8; 0x04],
    _reserved_27_p113: [u8; 0x04],
    _reserved_28_p114: [u8; 0x04],
    _reserved_29_p115: [u8; 0x04],
    _reserved_30_p200: [u8; 0x04],
    _reserved_31_p201: [u8; 0x04],
    _reserved_32_p202: [u8; 0x04],
    _reserved_33_p203: [u8; 0x04],
    _reserved_34_p204: [u8; 0x04],
    _reserved_35_p205: [u8; 0x04],
    _reserved_36_p206: [u8; 0x04],
    _reserved_37_p207: [u8; 0x04],
    _reserved_38_p208: [u8; 0x04],
    _reserved_39_p209: [u8; 0x04],
    _reserved_40_p210: [u8; 0x04],
    _reserved_41_p211: [u8; 0x04],
    _reserved_42_p212: [u8; 0x04],
    _reserved_43_p213: [u8; 0x04],
    _reserved_44_p214: [u8; 0x04],
    _reserved45: [u8; 0x04],
    _reserved_45_p300: [u8; 0x04],
    _reserved_46_p301: [u8; 0x04],
    _reserved_47_p302: [u8; 0x04],
    _reserved_48_p303: [u8; 0x04],
    _reserved_49_p304: [u8; 0x04],
    _reserved_50_p305: [u8; 0x04],
    _reserved_51_p306: [u8; 0x04],
    _reserved_52_p307: [u8; 0x04],
    _reserved_53_p308: [u8; 0x04],
    _reserved_54_p309: [u8; 0x04],
    _reserved_55_p310: [u8; 0x04],
    _reserved_56_p311: [u8; 0x04],
    _reserved_57_p312: [u8; 0x04],
    _reserved_58_p313: [u8; 0x04],
    _reserved_59_p314: [u8; 0x04],
    _reserved_60_p315: [u8; 0x04],
    _reserved_61_p400: [u8; 0x04],
    _reserved_62_p401: [u8; 0x04],
    _reserved_63_p402: [u8; 0x04],
    _reserved_64_p403: [u8; 0x04],
    _reserved_65_p404: [u8; 0x04],
    _reserved_66_p405: [u8; 0x04],
    _reserved_67_p406: [u8; 0x04],
    _reserved_68_p407: [u8; 0x04],
    _reserved_69_p408: [u8; 0x04],
    _reserved_70_p409: [u8; 0x04],
    _reserved_71_p410: [u8; 0x04],
    _reserved_72_p411: [u8; 0x04],
    _reserved_73_p412: [u8; 0x04],
    _reserved_74_p413: [u8; 0x04],
    _reserved_75_p414: [u8; 0x04],
    _reserved_76_p415: [u8; 0x04],
    _reserved_77_p500: [u8; 0x04],
    _reserved_78_p501: [u8; 0x04],
    _reserved_79_p502: [u8; 0x04],
    _reserved_80_p503: [u8; 0x04],
    _reserved_81_p504: [u8; 0x04],
    _reserved_82_p505: [u8; 0x04],
    _reserved_83_p506: [u8; 0x04],
    _reserved_84_p507: [u8; 0x04],
    _reserved_85_p508: [u8; 0x04],
    _reserved86: [u8; 0x08],
    _reserved_86_p511: [u8; 0x04],
    _reserved_87_p512: [u8; 0x04],
    _reserved_88_p513: [u8; 0x04],
    _reserved_89_p514: [u8; 0x04],
    _reserved_90_p515: [u8; 0x04],
    _reserved_91_p600: [u8; 0x04],
    _reserved_92_p601: [u8; 0x04],
    _reserved_93_p602: [u8; 0x04],
    _reserved_94_p603: [u8; 0x04],
    _reserved_95_p604: [u8; 0x04],
    _reserved_96_p605: [u8; 0x04],
    _reserved_97_p606: [u8; 0x04],
    _reserved_98_p607: [u8; 0x04],
    _reserved_99_p608: [u8; 0x04],
    _reserved_100_p609: [u8; 0x04],
    _reserved_101_p610: [u8; 0x04],
    _reserved_102_p611: [u8; 0x04],
    _reserved_103_p612: [u8; 0x04],
    _reserved_104_p613: [u8; 0x04],
    _reserved_105_p614: [u8; 0x04],
    _reserved_106_p615: [u8; 0x04],
    _reserved_107_p700: [u8; 0x04],
    _reserved_108_p701: [u8; 0x04],
    _reserved_109_p702: [u8; 0x04],
    _reserved_110_p703: [u8; 0x04],
    _reserved_111_p704: [u8; 0x04],
    _reserved_112_p705: [u8; 0x04],
    _reserved_113_p706: [u8; 0x04],
    _reserved_114_p707: [u8; 0x04],
    _reserved_115_p708: [u8; 0x04],
    _reserved116: [u8; 0x1c],
    _reserved_116_p800: [u8; 0x04],
    _reserved_117_p801: [u8; 0x04],
    _reserved_118_p802: [u8; 0x04],
    _reserved_119_p803: [u8; 0x04],
    _reserved_120_p804: [u8; 0x04],
    _reserved_121_p805: [u8; 0x04],
    _reserved_122_p806: [u8; 0x04],
    _reserved123: [u8; 0x24],
    _reserved_123_p900: [u8; 0x04],
    _reserved_124_p901: [u8; 0x04],
    _reserved125: [u8; 0x0c],
    _reserved_125_p905: [u8; 0x04],
    _reserved_126_p906: [u8; 0x04],
    _reserved_127_p907: [u8; 0x04],
    _reserved_128_p908: [u8; 0x04],
    _reserved129: [u8; 0x1c],
    _reserved_129_pa00: [u8; 0x04],
    _reserved_130_pa01: [u8; 0x04],
    _reserved131: [u8; 0x18],
    _reserved_131_pa08: [u8; 0x04],
    _reserved_132_pa09: [u8; 0x04],
    _reserved_133_pa10: [u8; 0x04],
    _reserved134: [u8; 0x14],
    _reserved_134_pb00: [u8; 0x04],
    _reserved_135_pb01: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - P000 Pin Function Control Register
    #[inline(always)]
    pub const fn p000pfs(&self) -> &P000PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x02 - P000 Pin Function Control Register
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P000PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x03 - P000 Pin Function Control Register
    #[inline(always)]
    pub const fn p000pfs_by(&self) -> &P000PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(3).cast() }
    }
    ///0x04 - P001 Pin Function Control Register
    #[inline(always)]
    pub const fn p001pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P001 Pin Function Control Register
    #[inline(always)]
    pub const fn p001pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P001 Pin Function Control Register
    #[inline(always)]
    pub const fn p001pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x04 - P002 Pin Function Control Register
    #[inline(always)]
    pub const fn p002pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P002 Pin Function Control Register
    #[inline(always)]
    pub const fn p002pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P002 Pin Function Control Register
    #[inline(always)]
    pub const fn p002pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x04 - P003 Pin Function Control Register
    #[inline(always)]
    pub const fn p003pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P003 Pin Function Control Register
    #[inline(always)]
    pub const fn p003pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P003 Pin Function Control Register
    #[inline(always)]
    pub const fn p003pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x04 - P004 Pin Function Control Register
    #[inline(always)]
    pub const fn p004pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P004 Pin Function Control Register
    #[inline(always)]
    pub const fn p004pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P004 Pin Function Control Register
    #[inline(always)]
    pub const fn p004pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x04 - P005 Pin Function Control Register
    #[inline(always)]
    pub const fn p005pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P005 Pin Function Control Register
    #[inline(always)]
    pub const fn p005pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P005 Pin Function Control Register
    #[inline(always)]
    pub const fn p005pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x04 - P006 Pin Function Control Register
    #[inline(always)]
    pub const fn p006pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P006 Pin Function Control Register
    #[inline(always)]
    pub const fn p006pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P006 Pin Function Control Register
    #[inline(always)]
    pub const fn p006pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x04 - P007 Pin Function Control Register
    #[inline(always)]
    pub const fn p007pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P007 Pin Function Control Register
    #[inline(always)]
    pub const fn p007pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P007 Pin Function Control Register
    #[inline(always)]
    pub const fn p007pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x20 - P008 Pin Function Control Register
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P008PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x22 - P008 Pin Function Control Register
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P008PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(34).cast() }
    }
    ///0x23 - P008 Pin Function Control Register
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P008PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(35).cast() }
    }
    ///0x20 - P009 Pin Function Control Register
    #[inline(always)]
    pub const fn p009pfs(&self) -> &P008PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x22 - P009 Pin Function Control Register
    #[inline(always)]
    pub const fn p009pfs_ha(&self) -> &P008PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(34).cast() }
    }
    ///0x23 - P009 Pin Function Control Register
    #[inline(always)]
    pub const fn p009pfs_by(&self) -> &P008PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(35).cast() }
    }
    ///0x28 - P010 Pin Function Control Register
    #[inline(always)]
    pub const fn p010pfs(&self) -> &P010PFS {
        &self.p010pfs
    }
    ///0x30 - P010 Pin Function Control Register
    #[inline(always)]
    pub const fn p010pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x31 - P010 Pin Function Control Register
    #[inline(always)]
    pub const fn p010pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(49).cast() }
    }
    ///0x38 - P014 Pin Function Control Register
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P014PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x3a - P014 Pin Function Control Register
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P014PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(58).cast() }
    }
    ///0x3b - P014 Pin Function Control Register
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(59).cast() }
    }
    ///0x38 - P015 Pin Function Control Register
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P014PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x3a - P015 Pin Function Control Register
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P014PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(58).cast() }
    }
    ///0x3b - P015 Pin Function Control Register
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P014PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(59).cast() }
    }
    ///0x40 - P100 Pin Function Control Register
    #[inline(always)]
    pub const fn p100pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P100 Pin Function Control Register
    #[inline(always)]
    pub const fn p100pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P100 Pin Function Control Register
    #[inline(always)]
    pub const fn p100pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x44 - P101 Pin Function Control Register
    #[inline(always)]
    pub const fn p101pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P101 Pin Function Control Register
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P101 Pin Function Control Register
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x44 - P102 Pin Function Control Register
    #[inline(always)]
    pub const fn p102pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P102 Pin Function Control Register
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P102 Pin Function Control Register
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x44 - P103 Pin Function Control Register
    #[inline(always)]
    pub const fn p103pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P103 Pin Function Control Register
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P103 Pin Function Control Register
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x44 - P104 Pin Function Control Register
    #[inline(always)]
    pub const fn p104pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P104 Pin Function Control Register
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P104 Pin Function Control Register
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x44 - P105 Pin Function Control Register
    #[inline(always)]
    pub const fn p105pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P105 Pin Function Control Register
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P105 Pin Function Control Register
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x44 - P106 Pin Function Control Register
    #[inline(always)]
    pub const fn p106pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P106 Pin Function Control Register
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P106 Pin Function Control Register
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x44 - P107 Pin Function Control Register
    #[inline(always)]
    pub const fn p107pfs(&self) -> &P101PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x46 - P107 Pin Function Control Register
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P101PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(70).cast() }
    }
    ///0x47 - P107 Pin Function Control Register
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P101PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(71).cast() }
    }
    ///0x60 - P108 Pin Function Control Register
    #[inline(always)]
    pub const fn p108pfs(&self) -> &P108PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    ///0x62 - P108 Pin Function Control Register
    #[inline(always)]
    pub const fn p108pfs_ha(&self) -> &P108PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(98).cast() }
    }
    ///0x63 - P108 Pin Function Control Register
    #[inline(always)]
    pub const fn p108pfs_by(&self) -> &P108PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(99).cast() }
    }
    ///0x64 - P109 Pin Function Control Register
    #[inline(always)]
    pub const fn p109pfs(&self) -> &P109PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(100).cast() }
    }
    ///0x66 - P109 Pin Function Control Register
    #[inline(always)]
    pub const fn p109pfs_ha(&self) -> &P109PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(102).cast() }
    }
    ///0x67 - P109 Pin Function Control Register
    #[inline(always)]
    pub const fn p109pfs_by(&self) -> &P109PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(103).cast() }
    }
    ///0x68 - P110 Pin Function Control Register
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P110PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    ///0x6a - P110 Pin Function Control Register
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P110PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(106).cast() }
    }
    ///0x6b - P110 Pin Function Control Register
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P110PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(107).cast() }
    }
    ///0x6c - P111 Pin Function Control Register
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P111PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x6e - P111 Pin Function Control Register
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    ///0x6f - P111 Pin Function Control Register
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    ///0x6c - P112 Pin Function Control Register
    #[inline(always)]
    pub const fn p112pfs(&self) -> &P111PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x6e - P112 Pin Function Control Register
    #[inline(always)]
    pub const fn p112pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    ///0x6f - P112 Pin Function Control Register
    #[inline(always)]
    pub const fn p112pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    ///0x6c - P113 Pin Function Control Register
    #[inline(always)]
    pub const fn p113pfs(&self) -> &P111PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x6e - P113 Pin Function Control Register
    #[inline(always)]
    pub const fn p113pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    ///0x6f - P113 Pin Function Control Register
    #[inline(always)]
    pub const fn p113pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    ///0x6c - P114 Pin Function Control Register
    #[inline(always)]
    pub const fn p114pfs(&self) -> &P111PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x6e - P114 Pin Function Control Register
    #[inline(always)]
    pub const fn p114pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    ///0x6f - P114 Pin Function Control Register
    #[inline(always)]
    pub const fn p114pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    ///0x6c - P115 Pin Function Control Register
    #[inline(always)]
    pub const fn p115pfs(&self) -> &P111PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x6e - P115 Pin Function Control Register
    #[inline(always)]
    pub const fn p115pfs_ha(&self) -> &P111PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    ///0x6f - P115 Pin Function Control Register
    #[inline(always)]
    pub const fn p115pfs_by(&self) -> &P111PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    ///0x80 - P200 Pin Function Control Register
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P200PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    ///0x82 - P200 Pin Function Control Register
    #[inline(always)]
    pub const fn p200pfs_ha(&self) -> &P200PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(130).cast() }
    }
    ///0x83 - P200 Pin Function Control Register
    #[inline(always)]
    pub const fn p200pfs_by(&self) -> &P200PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(131).cast() }
    }
    ///0x84 - P201 Pin Function Control Register
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    ///0x86 - P201 Pin Function Control Register
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P201PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(134).cast() }
    }
    ///0x87 - P201 Pin Function Control Register
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P201PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(135).cast() }
    }
    ///0x88 - P202 Pin Function Control Register
    #[inline(always)]
    pub const fn p202pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P202 Pin Function Control Register
    #[inline(always)]
    pub const fn p202pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P202 Pin Function Control Register
    #[inline(always)]
    pub const fn p202pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P203 Pin Function Control Register
    #[inline(always)]
    pub const fn p203pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P203 Pin Function Control Register
    #[inline(always)]
    pub const fn p203pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P203 Pin Function Control Register
    #[inline(always)]
    pub const fn p203pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P204 Pin Function Control Register
    #[inline(always)]
    pub const fn p204pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P204 Pin Function Control Register
    #[inline(always)]
    pub const fn p204pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P204 Pin Function Control Register
    #[inline(always)]
    pub const fn p204pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P205 Pin Function Control Register
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P205 Pin Function Control Register
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P205 Pin Function Control Register
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P206 Pin Function Control Register
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P206 Pin Function Control Register
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P206 Pin Function Control Register
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P207 Pin Function Control Register
    #[inline(always)]
    pub const fn p207pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P207 Pin Function Control Register
    #[inline(always)]
    pub const fn p207pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P207 Pin Function Control Register
    #[inline(always)]
    pub const fn p207pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P208 Pin Function Control Register
    #[inline(always)]
    pub const fn p208pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P208 Pin Function Control Register
    #[inline(always)]
    pub const fn p208pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P208 Pin Function Control Register
    #[inline(always)]
    pub const fn p208pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0x88 - P209 Pin Function Control Register
    #[inline(always)]
    pub const fn p209pfs(&self) -> &P202PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    ///0x8a - P209 Pin Function Control Register
    #[inline(always)]
    pub const fn p209pfs_ha(&self) -> &P202PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    ///0x8b - P209 Pin Function Control Register
    #[inline(always)]
    pub const fn p209pfs_by(&self) -> &P202PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    ///0xa8 - P210 Pin Function Control Register
    #[inline(always)]
    pub const fn p210pfs(&self) -> &P210PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    ///0xaa - P210 Pin Function Control Register
    #[inline(always)]
    pub const fn p210pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(170).cast() }
    }
    ///0xab - P210 Pin Function Control Register
    #[inline(always)]
    pub const fn p210pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(171).cast() }
    }
    ///0xa8 - P211 Pin Function Control Register
    #[inline(always)]
    pub const fn p211pfs(&self) -> &P210PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    ///0xaa - P211 Pin Function Control Register
    #[inline(always)]
    pub const fn p211pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(170).cast() }
    }
    ///0xab - P211 Pin Function Control Register
    #[inline(always)]
    pub const fn p211pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(171).cast() }
    }
    ///0xa8 - P212 Pin Function Control Register
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P210PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    ///0xaa - P212 Pin Function Control Register
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(170).cast() }
    }
    ///0xab - P212 Pin Function Control Register
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(171).cast() }
    }
    ///0xa8 - P213 Pin Function Control Register
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P210PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    ///0xaa - P213 Pin Function Control Register
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(170).cast() }
    }
    ///0xab - P213 Pin Function Control Register
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(171).cast() }
    }
    ///0xa8 - P214 Pin Function Control Register
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P210PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    ///0xaa - P214 Pin Function Control Register
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P210PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(170).cast() }
    }
    ///0xab - P214 Pin Function Control Register
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P210PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(171).cast() }
    }
    ///0xc0 - P300 Pin Function Control Register
    #[inline(always)]
    pub const fn p300pfs(&self) -> &P300PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    ///0xc2 - P300 Pin Function Control Register
    #[inline(always)]
    pub const fn p300pfs_ha(&self) -> &P300PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(194).cast() }
    }
    ///0xc3 - P300 Pin Function Control Register
    #[inline(always)]
    pub const fn p300pfs_by(&self) -> &P300PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(195).cast() }
    }
    ///0xc4 - P301 Pin Function Control Register
    #[inline(always)]
    pub const fn p301pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P301 Pin Function Control Register
    #[inline(always)]
    pub const fn p301pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P301 Pin Function Control Register
    #[inline(always)]
    pub const fn p301pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P302 Pin Function Control Register
    #[inline(always)]
    pub const fn p302pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P302 Pin Function Control Register
    #[inline(always)]
    pub const fn p302pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P302 Pin Function Control Register
    #[inline(always)]
    pub const fn p302pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P303 Pin Function Control Register
    #[inline(always)]
    pub const fn p303pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P303 Pin Function Control Register
    #[inline(always)]
    pub const fn p303pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P303 Pin Function Control Register
    #[inline(always)]
    pub const fn p303pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P304 Pin Function Control Register
    #[inline(always)]
    pub const fn p304pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P304 Pin Function Control Register
    #[inline(always)]
    pub const fn p304pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P304 Pin Function Control Register
    #[inline(always)]
    pub const fn p304pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P305 Pin Function Control Register
    #[inline(always)]
    pub const fn p305pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P305 Pin Function Control Register
    #[inline(always)]
    pub const fn p305pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P305 Pin Function Control Register
    #[inline(always)]
    pub const fn p305pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P306 Pin Function Control Register
    #[inline(always)]
    pub const fn p306pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P306 Pin Function Control Register
    #[inline(always)]
    pub const fn p306pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P306 Pin Function Control Register
    #[inline(always)]
    pub const fn p306pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P307 Pin Function Control Register
    #[inline(always)]
    pub const fn p307pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P307 Pin Function Control Register
    #[inline(always)]
    pub const fn p307pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P307 Pin Function Control Register
    #[inline(always)]
    pub const fn p307pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P308 Pin Function Control Register
    #[inline(always)]
    pub const fn p308pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P308 Pin Function Control Register
    #[inline(always)]
    pub const fn p308pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P308 Pin Function Control Register
    #[inline(always)]
    pub const fn p308pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xc4 - P309 Pin Function Control Register
    #[inline(always)]
    pub const fn p309pfs(&self) -> &P301PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    ///0xc6 - P309 Pin Function Control Register
    #[inline(always)]
    pub const fn p309pfs_ha(&self) -> &P301PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    ///0xc7 - P309 Pin Function Control Register
    #[inline(always)]
    pub const fn p309pfs_by(&self) -> &P301PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    ///0xe8 - P310 Pin Function Control Register
    #[inline(always)]
    pub const fn p310pfs(&self) -> &P310PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    ///0xea - P3010 Pin Function Control Register
    #[inline(always)]
    pub const fn p310pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(234).cast() }
    }
    ///0xeb - P3010 Pin Function Control Register
    #[inline(always)]
    pub const fn p310pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(235).cast() }
    }
    ///0xe8 - P311 Pin Function Control Register
    #[inline(always)]
    pub const fn p311pfs(&self) -> &P310PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    ///0xea - P3011 Pin Function Control Register
    #[inline(always)]
    pub const fn p311pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(234).cast() }
    }
    ///0xeb - P3011 Pin Function Control Register
    #[inline(always)]
    pub const fn p311pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(235).cast() }
    }
    ///0xe8 - P312 Pin Function Control Register
    #[inline(always)]
    pub const fn p312pfs(&self) -> &P310PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    ///0xea - P3012 Pin Function Control Register
    #[inline(always)]
    pub const fn p312pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(234).cast() }
    }
    ///0xeb - P3012 Pin Function Control Register
    #[inline(always)]
    pub const fn p312pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(235).cast() }
    }
    ///0xe8 - P313 Pin Function Control Register
    #[inline(always)]
    pub const fn p313pfs(&self) -> &P310PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    ///0xea - P3013 Pin Function Control Register
    #[inline(always)]
    pub const fn p313pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(234).cast() }
    }
    ///0xeb - P3013 Pin Function Control Register
    #[inline(always)]
    pub const fn p313pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(235).cast() }
    }
    ///0xe8 - P314 Pin Function Control Register
    #[inline(always)]
    pub const fn p314pfs(&self) -> &P310PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    ///0xea - P3014 Pin Function Control Register
    #[inline(always)]
    pub const fn p314pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(234).cast() }
    }
    ///0xeb - P3014 Pin Function Control Register
    #[inline(always)]
    pub const fn p314pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(235).cast() }
    }
    ///0xe8 - P315 Pin Function Control Register
    #[inline(always)]
    pub const fn p315pfs(&self) -> &P310PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    ///0xea - P3015 Pin Function Control Register
    #[inline(always)]
    pub const fn p315pfs_ha(&self) -> &P310PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(234).cast() }
    }
    ///0xeb - P3015 Pin Function Control Register
    #[inline(always)]
    pub const fn p315pfs_by(&self) -> &P310PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(235).cast() }
    }
    ///0x100 - P400 Pin Function Control Register
    #[inline(always)]
    pub const fn p400pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P400 Pin Function Control Register
    #[inline(always)]
    pub const fn p400pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P400 Pin Function Control Register
    #[inline(always)]
    pub const fn p400pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P401 Pin Function Control Register
    #[inline(always)]
    pub const fn p401pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P401 Pin Function Control Register
    #[inline(always)]
    pub const fn p401pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P401 Pin Function Control Register
    #[inline(always)]
    pub const fn p401pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P402 Pin Function Control Register
    #[inline(always)]
    pub const fn p402pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P402 Pin Function Control Register
    #[inline(always)]
    pub const fn p402pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P402 Pin Function Control Register
    #[inline(always)]
    pub const fn p402pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P403 Pin Function Control Register
    #[inline(always)]
    pub const fn p403pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P403 Pin Function Control Register
    #[inline(always)]
    pub const fn p403pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P403 Pin Function Control Register
    #[inline(always)]
    pub const fn p403pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P404 Pin Function Control Register
    #[inline(always)]
    pub const fn p404pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P404 Pin Function Control Register
    #[inline(always)]
    pub const fn p404pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P404 Pin Function Control Register
    #[inline(always)]
    pub const fn p404pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P405 Pin Function Control Register
    #[inline(always)]
    pub const fn p405pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P405 Pin Function Control Register
    #[inline(always)]
    pub const fn p405pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P405 Pin Function Control Register
    #[inline(always)]
    pub const fn p405pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P406 Pin Function Control Register
    #[inline(always)]
    pub const fn p406pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P406 Pin Function Control Register
    #[inline(always)]
    pub const fn p406pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P406 Pin Function Control Register
    #[inline(always)]
    pub const fn p406pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P407 Pin Function Control Register
    #[inline(always)]
    pub const fn p407pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P407 Pin Function Control Register
    #[inline(always)]
    pub const fn p407pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P407 Pin Function Control Register
    #[inline(always)]
    pub const fn p407pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P408 Pin Function Control Register
    #[inline(always)]
    pub const fn p408pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P408 Pin Function Control Register
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P408 Pin Function Control Register
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x100 - P409 Pin Function Control Register
    #[inline(always)]
    pub const fn p409pfs(&self) -> &P400PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x102 - P409 Pin Function Control Register
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P400PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    ///0x103 - P409 Pin Function Control Register
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P400PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    ///0x128 - P410 Pin Function Control Register
    #[inline(always)]
    pub const fn p410pfs(&self) -> &P410PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    ///0x12a - P410 Pin Function Control Register
    #[inline(always)]
    pub const fn p410pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    ///0x12b - P410 Pin Function Control Register
    #[inline(always)]
    pub const fn p410pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    ///0x128 - P411 Pin Function Control Register
    #[inline(always)]
    pub const fn p411pfs(&self) -> &P410PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    ///0x12a - P411 Pin Function Control Register
    #[inline(always)]
    pub const fn p411pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    ///0x12b - P411 Pin Function Control Register
    #[inline(always)]
    pub const fn p411pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    ///0x128 - P412 Pin Function Control Register
    #[inline(always)]
    pub const fn p412pfs(&self) -> &P410PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    ///0x12a - P412 Pin Function Control Register
    #[inline(always)]
    pub const fn p412pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    ///0x12b - P412 Pin Function Control Register
    #[inline(always)]
    pub const fn p412pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    ///0x128 - P413 Pin Function Control Register
    #[inline(always)]
    pub const fn p413pfs(&self) -> &P410PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    ///0x12a - P413 Pin Function Control Register
    #[inline(always)]
    pub const fn p413pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    ///0x12b - P413 Pin Function Control Register
    #[inline(always)]
    pub const fn p413pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    ///0x128 - P414 Pin Function Control Register
    #[inline(always)]
    pub const fn p414pfs(&self) -> &P410PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    ///0x12a - P414 Pin Function Control Register
    #[inline(always)]
    pub const fn p414pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    ///0x12b - P414 Pin Function Control Register
    #[inline(always)]
    pub const fn p414pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    ///0x128 - P415 Pin Function Control Register
    #[inline(always)]
    pub const fn p415pfs(&self) -> &P410PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    ///0x12a - P415 Pin Function Control Register
    #[inline(always)]
    pub const fn p415pfs_ha(&self) -> &P410PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    ///0x12b - P415 Pin Function Control Register
    #[inline(always)]
    pub const fn p415pfs_by(&self) -> &P410PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    ///0x140 - P500 Pin Function Control Register
    #[inline(always)]
    pub const fn p500pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P500 Pin Function Control Register
    #[inline(always)]
    pub const fn p500pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P500 Pin Function Control Register
    #[inline(always)]
    pub const fn p500pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P501 Pin Function Control Register
    #[inline(always)]
    pub const fn p501pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P501 Pin Function Control Register
    #[inline(always)]
    pub const fn p501pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P501 Pin Function Control Register
    #[inline(always)]
    pub const fn p501pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P502 Pin Function Control Register
    #[inline(always)]
    pub const fn p502pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P502 Pin Function Control Register
    #[inline(always)]
    pub const fn p502pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P502 Pin Function Control Register
    #[inline(always)]
    pub const fn p502pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P503 Pin Function Control Register
    #[inline(always)]
    pub const fn p503pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P503 Pin Function Control Register
    #[inline(always)]
    pub const fn p503pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P503 Pin Function Control Register
    #[inline(always)]
    pub const fn p503pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P504 Pin Function Control Register
    #[inline(always)]
    pub const fn p504pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P504 Pin Function Control Register
    #[inline(always)]
    pub const fn p504pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P504 Pin Function Control Register
    #[inline(always)]
    pub const fn p504pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P505 Pin Function Control Register
    #[inline(always)]
    pub const fn p505pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P505 Pin Function Control Register
    #[inline(always)]
    pub const fn p505pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P505 Pin Function Control Register
    #[inline(always)]
    pub const fn p505pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P506 Pin Function Control Register
    #[inline(always)]
    pub const fn p506pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P506 Pin Function Control Register
    #[inline(always)]
    pub const fn p506pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P506 Pin Function Control Register
    #[inline(always)]
    pub const fn p506pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P507 Pin Function Control Register
    #[inline(always)]
    pub const fn p507pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P507 Pin Function Control Register
    #[inline(always)]
    pub const fn p507pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P507 Pin Function Control Register
    #[inline(always)]
    pub const fn p507pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x140 - P508 Pin Function Control Register
    #[inline(always)]
    pub const fn p508pfs(&self) -> &P500PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x142 - P508 Pin Function Control Register
    #[inline(always)]
    pub const fn p508pfs_ha(&self) -> &P500PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    ///0x143 - P508 Pin Function Control Register
    #[inline(always)]
    pub const fn p508pfs_by(&self) -> &P500PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    ///0x16c - P511 Pin Function Control Register
    #[inline(always)]
    pub const fn p511pfs(&self) -> &P511PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    ///0x16e - P511 Pin Function Control Register
    #[inline(always)]
    pub const fn p511pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(366).cast() }
    }
    ///0x16f - P511 Pin Function Control Register
    #[inline(always)]
    pub const fn p511pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(367).cast() }
    }
    ///0x16c - P512 Pin Function Control Register
    #[inline(always)]
    pub const fn p512pfs(&self) -> &P511PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    ///0x16e - P512 Pin Function Control Register
    #[inline(always)]
    pub const fn p512pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(366).cast() }
    }
    ///0x16f - P512 Pin Function Control Register
    #[inline(always)]
    pub const fn p512pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(367).cast() }
    }
    ///0x16c - P513 Pin Function Control Register
    #[inline(always)]
    pub const fn p513pfs(&self) -> &P511PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    ///0x16e - P513 Pin Function Control Register
    #[inline(always)]
    pub const fn p513pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(366).cast() }
    }
    ///0x16f - P513 Pin Function Control Register
    #[inline(always)]
    pub const fn p513pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(367).cast() }
    }
    ///0x16c - P514 Pin Function Control Register
    #[inline(always)]
    pub const fn p514pfs(&self) -> &P511PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    ///0x16e - P514 Pin Function Control Register
    #[inline(always)]
    pub const fn p514pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(366).cast() }
    }
    ///0x16f - P514 Pin Function Control Register
    #[inline(always)]
    pub const fn p514pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(367).cast() }
    }
    ///0x16c - P515 Pin Function Control Register
    #[inline(always)]
    pub const fn p515pfs(&self) -> &P511PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    ///0x16e - P515 Pin Function Control Register
    #[inline(always)]
    pub const fn p515pfs_ha(&self) -> &P511PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(366).cast() }
    }
    ///0x16f - P515 Pin Function Control Register
    #[inline(always)]
    pub const fn p515pfs_by(&self) -> &P511PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(367).cast() }
    }
    ///0x180 - P600 Pin Function Control Register
    #[inline(always)]
    pub const fn p600pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P600 Pin Function Control Register
    #[inline(always)]
    pub const fn p600pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P600 Pin Function Control Register
    #[inline(always)]
    pub const fn p600pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P601 Pin Function Control Register
    #[inline(always)]
    pub const fn p601pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P601 Pin Function Control Register
    #[inline(always)]
    pub const fn p601pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P601 Pin Function Control Register
    #[inline(always)]
    pub const fn p601pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P602 Pin Function Control Register
    #[inline(always)]
    pub const fn p602pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P602 Pin Function Control Register
    #[inline(always)]
    pub const fn p602pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P602 Pin Function Control Register
    #[inline(always)]
    pub const fn p602pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P603 Pin Function Control Register
    #[inline(always)]
    pub const fn p603pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P603 Pin Function Control Register
    #[inline(always)]
    pub const fn p603pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P603 Pin Function Control Register
    #[inline(always)]
    pub const fn p603pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P604 Pin Function Control Register
    #[inline(always)]
    pub const fn p604pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P604 Pin Function Control Register
    #[inline(always)]
    pub const fn p604pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P604 Pin Function Control Register
    #[inline(always)]
    pub const fn p604pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P605 Pin Function Control Register
    #[inline(always)]
    pub const fn p605pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P605 Pin Function Control Register
    #[inline(always)]
    pub const fn p605pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P605 Pin Function Control Register
    #[inline(always)]
    pub const fn p605pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P606 Pin Function Control Register
    #[inline(always)]
    pub const fn p606pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P606 Pin Function Control Register
    #[inline(always)]
    pub const fn p606pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P606 Pin Function Control Register
    #[inline(always)]
    pub const fn p606pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P607 Pin Function Control Register
    #[inline(always)]
    pub const fn p607pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P607 Pin Function Control Register
    #[inline(always)]
    pub const fn p607pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P607 Pin Function Control Register
    #[inline(always)]
    pub const fn p607pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P608 Pin Function Control Register
    #[inline(always)]
    pub const fn p608pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P608 Pin Function Control Register
    #[inline(always)]
    pub const fn p608pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P608 Pin Function Control Register
    #[inline(always)]
    pub const fn p608pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x180 - P609 Pin Function Control Register
    #[inline(always)]
    pub const fn p609pfs(&self) -> &P600PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    ///0x182 - P609 Pin Function Control Register
    #[inline(always)]
    pub const fn p609pfs_ha(&self) -> &P600PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    ///0x183 - P609 Pin Function Control Register
    #[inline(always)]
    pub const fn p609pfs_by(&self) -> &P600PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    ///0x1a8 - P610 Pin Function Control Register
    #[inline(always)]
    pub const fn p610pfs(&self) -> &P610PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    ///0x1aa - P610 Pin Function Control Register
    #[inline(always)]
    pub const fn p610pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    ///0x1ab - P610 Pin Function Control Register
    #[inline(always)]
    pub const fn p610pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    ///0x1a8 - P611 Pin Function Control Register
    #[inline(always)]
    pub const fn p611pfs(&self) -> &P610PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    ///0x1aa - P611 Pin Function Control Register
    #[inline(always)]
    pub const fn p611pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    ///0x1ab - P611 Pin Function Control Register
    #[inline(always)]
    pub const fn p611pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    ///0x1a8 - P612 Pin Function Control Register
    #[inline(always)]
    pub const fn p612pfs(&self) -> &P610PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    ///0x1aa - P612 Pin Function Control Register
    #[inline(always)]
    pub const fn p612pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    ///0x1ab - P612 Pin Function Control Register
    #[inline(always)]
    pub const fn p612pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    ///0x1a8 - P613 Pin Function Control Register
    #[inline(always)]
    pub const fn p613pfs(&self) -> &P610PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    ///0x1aa - P613 Pin Function Control Register
    #[inline(always)]
    pub const fn p613pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    ///0x1ab - P613 Pin Function Control Register
    #[inline(always)]
    pub const fn p613pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    ///0x1a8 - P614 Pin Function Control Register
    #[inline(always)]
    pub const fn p614pfs(&self) -> &P610PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    ///0x1aa - P614 Pin Function Control Register
    #[inline(always)]
    pub const fn p614pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    ///0x1ab - P614 Pin Function Control Register
    #[inline(always)]
    pub const fn p614pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    ///0x1a8 - P615 Pin Function Control Register
    #[inline(always)]
    pub const fn p615pfs(&self) -> &P610PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    ///0x1aa - P615 Pin Function Control Register
    #[inline(always)]
    pub const fn p615pfs_ha(&self) -> &P610PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    ///0x1ab - P615 Pin Function Control Register
    #[inline(always)]
    pub const fn p615pfs_by(&self) -> &P610PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    ///0x1c0 - P700 Pin Function Control Register
    #[inline(always)]
    pub const fn p700pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P700 Pin Function Control Register
    #[inline(always)]
    pub const fn p700pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P700 Pin Function Control Register
    #[inline(always)]
    pub const fn p700pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P701 Pin Function Control Register
    #[inline(always)]
    pub const fn p701pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P701 Pin Function Control Register
    #[inline(always)]
    pub const fn p701pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P701 Pin Function Control Register
    #[inline(always)]
    pub const fn p701pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P702 Pin Function Control Register
    #[inline(always)]
    pub const fn p702pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P702 Pin Function Control Register
    #[inline(always)]
    pub const fn p702pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P702 Pin Function Control Register
    #[inline(always)]
    pub const fn p702pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P703 Pin Function Control Register
    #[inline(always)]
    pub const fn p703pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P703 Pin Function Control Register
    #[inline(always)]
    pub const fn p703pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P703 Pin Function Control Register
    #[inline(always)]
    pub const fn p703pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P704 Pin Function Control Register
    #[inline(always)]
    pub const fn p704pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P704 Pin Function Control Register
    #[inline(always)]
    pub const fn p704pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P704 Pin Function Control Register
    #[inline(always)]
    pub const fn p704pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P705 Pin Function Control Register
    #[inline(always)]
    pub const fn p705pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P705 Pin Function Control Register
    #[inline(always)]
    pub const fn p705pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P705 Pin Function Control Register
    #[inline(always)]
    pub const fn p705pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P706 Pin Function Control Register
    #[inline(always)]
    pub const fn p706pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P706 Pin Function Control Register
    #[inline(always)]
    pub const fn p706pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P706 Pin Function Control Register
    #[inline(always)]
    pub const fn p706pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P707 Pin Function Control Register
    #[inline(always)]
    pub const fn p707pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P707 Pin Function Control Register
    #[inline(always)]
    pub const fn p707pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P707 Pin Function Control Register
    #[inline(always)]
    pub const fn p707pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x1c0 - P708 Pin Function Control Register
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P700PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    ///0x1c2 - P708 Pin Function Control Register
    #[inline(always)]
    pub const fn p708pfs_ha(&self) -> &P700PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(450).cast() }
    }
    ///0x1c3 - P708 Pin Function Control Register
    #[inline(always)]
    pub const fn p708pfs_by(&self) -> &P700PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(451).cast() }
    }
    ///0x200 - P800 Pin Function Control Register
    #[inline(always)]
    pub const fn p800pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P800 Pin Function Control Register
    #[inline(always)]
    pub const fn p800pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P800 Pin Function Control Register
    #[inline(always)]
    pub const fn p800pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x200 - P801 Pin Function Control Register
    #[inline(always)]
    pub const fn p801pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P801 Pin Function Control Register
    #[inline(always)]
    pub const fn p801pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P801 Pin Function Control Register
    #[inline(always)]
    pub const fn p801pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x200 - P802 Pin Function Control Register
    #[inline(always)]
    pub const fn p802pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P802 Pin Function Control Register
    #[inline(always)]
    pub const fn p802pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P802 Pin Function Control Register
    #[inline(always)]
    pub const fn p802pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x200 - P803 Pin Function Control Register
    #[inline(always)]
    pub const fn p803pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P803 Pin Function Control Register
    #[inline(always)]
    pub const fn p803pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P803 Pin Function Control Register
    #[inline(always)]
    pub const fn p803pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x200 - P804 Pin Function Control Register
    #[inline(always)]
    pub const fn p804pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P804 Pin Function Control Register
    #[inline(always)]
    pub const fn p804pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P804 Pin Function Control Register
    #[inline(always)]
    pub const fn p804pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x200 - P805 Pin Function Control Register
    #[inline(always)]
    pub const fn p805pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P805 Pin Function Control Register
    #[inline(always)]
    pub const fn p805pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P805 Pin Function Control Register
    #[inline(always)]
    pub const fn p805pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x200 - P806 Pin Function Control Register
    #[inline(always)]
    pub const fn p806pfs(&self) -> &P800PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    ///0x202 - P806 Pin Function Control Register
    #[inline(always)]
    pub const fn p806pfs_ha(&self) -> &P800PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    ///0x203 - P806 Pin Function Control Register
    #[inline(always)]
    pub const fn p806pfs_by(&self) -> &P800PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(515).cast() }
    }
    ///0x240 - P900 Pin Function Control Register
    #[inline(always)]
    pub const fn p900pfs(&self) -> &P900PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    ///0x242 - P900 Pin Function Control Register
    #[inline(always)]
    pub const fn p900pfs_ha(&self) -> &P900PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(578).cast() }
    }
    ///0x243 - P900 Pin Function Control Register
    #[inline(always)]
    pub const fn p900pfs_by(&self) -> &P900PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(579).cast() }
    }
    ///0x240 - P901 Pin Function Control Register
    #[inline(always)]
    pub const fn p901pfs(&self) -> &P900PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    ///0x242 - P901 Pin Function Control Register
    #[inline(always)]
    pub const fn p901pfs_ha(&self) -> &P900PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(578).cast() }
    }
    ///0x243 - P901 Pin Function Control Register
    #[inline(always)]
    pub const fn p901pfs_by(&self) -> &P900PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(579).cast() }
    }
    ///0x254 - P905 Pin Function Control Register
    #[inline(always)]
    pub const fn p905pfs(&self) -> &P905PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    ///0x256 - P905 Pin Function Control Register
    #[inline(always)]
    pub const fn p905pfs_ha(&self) -> &P905PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(598).cast() }
    }
    ///0x257 - P905 Pin Function Control Register
    #[inline(always)]
    pub const fn p905pfs_by(&self) -> &P905PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(599).cast() }
    }
    ///0x254 - P906 Pin Function Control Register
    #[inline(always)]
    pub const fn p906pfs(&self) -> &P905PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    ///0x256 - P906 Pin Function Control Register
    #[inline(always)]
    pub const fn p906pfs_ha(&self) -> &P905PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(598).cast() }
    }
    ///0x257 - P906 Pin Function Control Register
    #[inline(always)]
    pub const fn p906pfs_by(&self) -> &P905PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(599).cast() }
    }
    ///0x254 - P907 Pin Function Control Register
    #[inline(always)]
    pub const fn p907pfs(&self) -> &P905PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    ///0x256 - P907 Pin Function Control Register
    #[inline(always)]
    pub const fn p907pfs_ha(&self) -> &P905PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(598).cast() }
    }
    ///0x257 - P907 Pin Function Control Register
    #[inline(always)]
    pub const fn p907pfs_by(&self) -> &P905PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(599).cast() }
    }
    ///0x254 - P908 Pin Function Control Register
    #[inline(always)]
    pub const fn p908pfs(&self) -> &P905PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    ///0x256 - P908 Pin Function Control Register
    #[inline(always)]
    pub const fn p908pfs_ha(&self) -> &P905PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(598).cast() }
    }
    ///0x257 - P908 Pin Function Control Register
    #[inline(always)]
    pub const fn p908pfs_by(&self) -> &P905PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(599).cast() }
    }
    ///0x280 - PA00 Pin Function Control Register
    #[inline(always)]
    pub const fn pa00pfs(&self) -> &PA00PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(640).cast() }
    }
    ///0x282 - PA00 Pin Function Control Register
    #[inline(always)]
    pub const fn pa00pfs_ha(&self) -> &PA00PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(642).cast() }
    }
    ///0x283 - PA00 Pin Function Control Register
    #[inline(always)]
    pub const fn pa00pfs_by(&self) -> &PA00PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(643).cast() }
    }
    ///0x280 - PA01 Pin Function Control Register
    #[inline(always)]
    pub const fn pa01pfs(&self) -> &PA00PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(640).cast() }
    }
    ///0x282 - PA01 Pin Function Control Register
    #[inline(always)]
    pub const fn pa01pfs_ha(&self) -> &PA00PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(642).cast() }
    }
    ///0x283 - PA01 Pin Function Control Register
    #[inline(always)]
    pub const fn pa01pfs_by(&self) -> &PA00PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(643).cast() }
    }
    ///0x2a0 - PA08 Pin Function Control Register
    #[inline(always)]
    pub const fn pa08pfs(&self) -> &PA08PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(672).cast() }
    }
    ///0x2a2 - PA08 Pin Function Control Register
    #[inline(always)]
    pub const fn pa08pfs_ha(&self) -> &PA08PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(674).cast() }
    }
    ///0x2a3 - PA08 Pin Function Control Register
    #[inline(always)]
    pub const fn pa08pfs_by(&self) -> &PA08PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(675).cast() }
    }
    ///0x2a0 - PA09 Pin Function Control Register
    #[inline(always)]
    pub const fn pa09pfs(&self) -> &PA08PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(672).cast() }
    }
    ///0x2a2 - PA09 Pin Function Control Register
    #[inline(always)]
    pub const fn pa09pfs_ha(&self) -> &PA08PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(674).cast() }
    }
    ///0x2a3 - PA09 Pin Function Control Register
    #[inline(always)]
    pub const fn pa09pfs_by(&self) -> &PA08PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(675).cast() }
    }
    ///0x2a8 - PA10 Pin Function Control Register
    #[inline(always)]
    pub const fn pa10pfs(&self) -> &PA10PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(680).cast() }
    }
    ///0x2aa - PA10 Pin Function Control Register
    #[inline(always)]
    pub const fn pa10pfs_ha(&self) -> &PA10PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(682).cast() }
    }
    ///0x2ab - PA10 Pin Function Control Register
    #[inline(always)]
    pub const fn pa10pfs_by(&self) -> &PA10PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(683).cast() }
    }
    ///0x2c0 - PB00 Pin Function Control Register
    #[inline(always)]
    pub const fn pb00pfs(&self) -> &PB00PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(704).cast() }
    }
    ///0x2c2 - PB00 Pin Function Control Register
    #[inline(always)]
    pub const fn pb00pfs_ha(&self) -> &PB00PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(706).cast() }
    }
    ///0x2c3 - PB00 Pin Function Control Register
    #[inline(always)]
    pub const fn pb00pfs_by(&self) -> &PB00PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(707).cast() }
    }
    ///0x2c0 - PB01 Pin Function Control Register
    #[inline(always)]
    pub const fn pb01pfs(&self) -> &PB00PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(704).cast() }
    }
    ///0x2c2 - PB01 Pin Function Control Register
    #[inline(always)]
    pub const fn pb01pfs_ha(&self) -> &PB00PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(706).cast() }
    }
    ///0x2c3 - PB01 Pin Function Control Register
    #[inline(always)]
    pub const fn pb01pfs_by(&self) -> &PB00PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(707).cast() }
    }
}
/**P000PFS (rw) register accessor: P000 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p000pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p000pfs`] module*/
pub type P000PFS = crate::Reg<p000pfs::P000PFS_SPEC>;
///P000 Pin Function Control Register
pub mod p000pfs;
/**P000PFS_HA (rw) register accessor: P000 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p000pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p000pfs_ha`] module*/
pub type P000PFS_HA = crate::Reg<p000pfs_ha::P000PFS_HA_SPEC>;
///P000 Pin Function Control Register
pub mod p000pfs_ha;
/**P000PFS_BY (rw) register accessor: P000 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p000pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p000pfs_by`] module*/
pub type P000PFS_BY = crate::Reg<p000pfs_by::P000PFS_BY_SPEC>;
///P000 Pin Function Control Register
pub mod p000pfs_by;
pub use p000pfs as p001pfs;
pub use p000pfs as p008pfs;
pub use p000pfs as p010pfs;
pub use p000pfs as p014pfs;
pub use p000pfs_by as p001pfs_by;
pub use p000pfs_by as p008pfs_by;
pub use p000pfs_by as p010pfs_by;
pub use p000pfs_by as p014pfs_by;
pub use p000pfs_ha as p001pfs_ha;
pub use p000pfs_ha as p008pfs_ha;
pub use p000pfs_ha as p010pfs_ha;
pub use p000pfs_ha as p014pfs_ha;
pub use P000PFS as P001PFS;
pub use P000PFS as P008PFS;
pub use P000PFS as P010PFS;
pub use P000PFS as P014PFS;
pub use P000PFS_BY as P001PFS_BY;
pub use P000PFS_BY as P008PFS_BY;
pub use P000PFS_BY as P010PFS_BY;
pub use P000PFS_BY as P014PFS_BY;
pub use P000PFS_HA as P001PFS_HA;
pub use P000PFS_HA as P008PFS_HA;
pub use P000PFS_HA as P010PFS_HA;
pub use P000PFS_HA as P014PFS_HA;
/**P100PFS (rw) register accessor: P100 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p100pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p100pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p100pfs`] module*/
pub type P100PFS = crate::Reg<p100pfs::P100PFS_SPEC>;
///P100 Pin Function Control Register
pub mod p100pfs;
/**P100PFS_HA (rw) register accessor: P100 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p100pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p100pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p100pfs_ha`] module*/
pub type P100PFS_HA = crate::Reg<p100pfs_ha::P100PFS_HA_SPEC>;
///P100 Pin Function Control Register
pub mod p100pfs_ha;
/**P100PFS_BY (rw) register accessor: P100 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p100pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p100pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p100pfs_by`] module*/
pub type P100PFS_BY = crate::Reg<p100pfs_by::P100PFS_BY_SPEC>;
///P100 Pin Function Control Register
pub mod p100pfs_by;
pub use p100pfs as p101pfs;
pub use p100pfs_by as p101pfs_by;
pub use p100pfs_ha as p101pfs_ha;
pub use P100PFS as P101PFS;
pub use P100PFS_BY as P101PFS_BY;
pub use P100PFS_HA as P101PFS_HA;
/**P108PFS (rw) register accessor: P108 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p108pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p108pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p108pfs`] module*/
pub type P108PFS = crate::Reg<p108pfs::P108PFS_SPEC>;
///P108 Pin Function Control Register
pub mod p108pfs;
/**P108PFS_HA (rw) register accessor: P108 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p108pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p108pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p108pfs_ha`] module*/
pub type P108PFS_HA = crate::Reg<p108pfs_ha::P108PFS_HA_SPEC>;
///P108 Pin Function Control Register
pub mod p108pfs_ha;
/**P108PFS_BY (rw) register accessor: P108 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p108pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p108pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p108pfs_by`] module*/
pub type P108PFS_BY = crate::Reg<p108pfs_by::P108PFS_BY_SPEC>;
///P108 Pin Function Control Register
pub mod p108pfs_by;
pub use p108pfs as p109pfs;
pub use p108pfs_by as p109pfs_by;
pub use p108pfs_ha as p109pfs_ha;
pub use P108PFS as P109PFS;
pub use P108PFS_BY as P109PFS_BY;
pub use P108PFS_HA as P109PFS_HA;
/**P110PFS (rw) register accessor: P110 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p110pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p110pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p110pfs`] module*/
pub type P110PFS = crate::Reg<p110pfs::P110PFS_SPEC>;
///P110 Pin Function Control Register
pub mod p110pfs;
/**P110PFS_HA (rw) register accessor: P110 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p110pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p110pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p110pfs_ha`] module*/
pub type P110PFS_HA = crate::Reg<p110pfs_ha::P110PFS_HA_SPEC>;
///P110 Pin Function Control Register
pub mod p110pfs_ha;
/**P110PFS_BY (rw) register accessor: P110 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p110pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p110pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p110pfs_by`] module*/
pub type P110PFS_BY = crate::Reg<p110pfs_by::P110PFS_BY_SPEC>;
///P110 Pin Function Control Register
pub mod p110pfs_by;
pub use p000pfs as p111pfs;
pub use p000pfs as p200pfs;
pub use p000pfs_by as p111pfs_by;
pub use p000pfs_by as p200pfs_by;
pub use p000pfs_ha as p111pfs_ha;
pub use p000pfs_ha as p200pfs_ha;
pub use P000PFS as P111PFS;
pub use P000PFS as P200PFS;
pub use P000PFS_BY as P111PFS_BY;
pub use P000PFS_BY as P200PFS_BY;
pub use P000PFS_HA as P111PFS_HA;
pub use P000PFS_HA as P200PFS_HA;
/**P201PFS (rw) register accessor: P201 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p201pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p201pfs`] module*/
pub type P201PFS = crate::Reg<p201pfs::P201PFS_SPEC>;
///P201 Pin Function Control Register
pub mod p201pfs;
/**P201PFS_HA (rw) register accessor: P201 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p201pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p201pfs_ha`] module*/
pub type P201PFS_HA = crate::Reg<p201pfs_ha::P201PFS_HA_SPEC>;
///P201 Pin Function Control Register
pub mod p201pfs_ha;
/**P201PFS_BY (rw) register accessor: P201 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p201pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p201pfs_by`] module*/
pub type P201PFS_BY = crate::Reg<p201pfs_by::P201PFS_BY_SPEC>;
///P201 Pin Function Control Register
pub mod p201pfs_by;
pub use p000pfs as p202pfs;
pub use p000pfs as p210pfs;
pub use p000pfs as p301pfs;
pub use p000pfs as p310pfs;
pub use p000pfs as p500pfs;
pub use p000pfs as p511pfs;
pub use p000pfs as p600pfs;
pub use p000pfs as p610pfs;
pub use p000pfs as p700pfs;
pub use p000pfs as p800pfs;
pub use p000pfs as p900pfs;
pub use p000pfs as p905pfs;
pub use p000pfs as pa00pfs;
pub use p000pfs as pa08pfs;
pub use p000pfs as pa10pfs;
pub use p000pfs as pb00pfs;
pub use p000pfs_by as p202pfs_by;
pub use p000pfs_by as p210pfs_by;
pub use p000pfs_by as p301pfs_by;
pub use p000pfs_by as p310pfs_by;
pub use p000pfs_by as p500pfs_by;
pub use p000pfs_by as p511pfs_by;
pub use p000pfs_by as p600pfs_by;
pub use p000pfs_by as p610pfs_by;
pub use p000pfs_by as p700pfs_by;
pub use p000pfs_by as p800pfs_by;
pub use p000pfs_by as p900pfs_by;
pub use p000pfs_by as p905pfs_by;
pub use p000pfs_by as pa00pfs_by;
pub use p000pfs_by as pa08pfs_by;
pub use p000pfs_by as pa10pfs_by;
pub use p000pfs_by as pb00pfs_by;
pub use p000pfs_ha as p202pfs_ha;
pub use p000pfs_ha as p210pfs_ha;
pub use p000pfs_ha as p301pfs_ha;
pub use p000pfs_ha as p310pfs_ha;
pub use p000pfs_ha as p500pfs_ha;
pub use p000pfs_ha as p511pfs_ha;
pub use p000pfs_ha as p600pfs_ha;
pub use p000pfs_ha as p610pfs_ha;
pub use p000pfs_ha as p700pfs_ha;
pub use p000pfs_ha as p800pfs_ha;
pub use p000pfs_ha as p900pfs_ha;
pub use p000pfs_ha as p905pfs_ha;
pub use p000pfs_ha as pa00pfs_ha;
pub use p000pfs_ha as pa08pfs_ha;
pub use p000pfs_ha as pa10pfs_ha;
pub use p000pfs_ha as pb00pfs_ha;
pub use p100pfs as p400pfs;
pub use p100pfs as p410pfs;
pub use p100pfs_by as p400pfs_by;
pub use p100pfs_by as p410pfs_by;
pub use p100pfs_ha as p400pfs_ha;
pub use p100pfs_ha as p410pfs_ha;
pub use p110pfs as p300pfs;
pub use p110pfs_by as p300pfs_by;
pub use p110pfs_ha as p300pfs_ha;
pub use P000PFS as P202PFS;
pub use P000PFS as P210PFS;
pub use P000PFS as P301PFS;
pub use P000PFS as P310PFS;
pub use P000PFS as P500PFS;
pub use P000PFS as P511PFS;
pub use P000PFS as P600PFS;
pub use P000PFS as P610PFS;
pub use P000PFS as P700PFS;
pub use P000PFS as P800PFS;
pub use P000PFS as P900PFS;
pub use P000PFS as P905PFS;
pub use P000PFS as PA00PFS;
pub use P000PFS as PA08PFS;
pub use P000PFS as PA10PFS;
pub use P000PFS as PB00PFS;
pub use P000PFS_BY as P202PFS_BY;
pub use P000PFS_BY as P210PFS_BY;
pub use P000PFS_BY as P301PFS_BY;
pub use P000PFS_BY as P310PFS_BY;
pub use P000PFS_BY as P500PFS_BY;
pub use P000PFS_BY as P511PFS_BY;
pub use P000PFS_BY as P600PFS_BY;
pub use P000PFS_BY as P610PFS_BY;
pub use P000PFS_BY as P700PFS_BY;
pub use P000PFS_BY as P800PFS_BY;
pub use P000PFS_BY as P900PFS_BY;
pub use P000PFS_BY as P905PFS_BY;
pub use P000PFS_BY as PA00PFS_BY;
pub use P000PFS_BY as PA08PFS_BY;
pub use P000PFS_BY as PA10PFS_BY;
pub use P000PFS_BY as PB00PFS_BY;
pub use P000PFS_HA as P202PFS_HA;
pub use P000PFS_HA as P210PFS_HA;
pub use P000PFS_HA as P301PFS_HA;
pub use P000PFS_HA as P310PFS_HA;
pub use P000PFS_HA as P500PFS_HA;
pub use P000PFS_HA as P511PFS_HA;
pub use P000PFS_HA as P600PFS_HA;
pub use P000PFS_HA as P610PFS_HA;
pub use P000PFS_HA as P700PFS_HA;
pub use P000PFS_HA as P800PFS_HA;
pub use P000PFS_HA as P900PFS_HA;
pub use P000PFS_HA as P905PFS_HA;
pub use P000PFS_HA as PA00PFS_HA;
pub use P000PFS_HA as PA08PFS_HA;
pub use P000PFS_HA as PA10PFS_HA;
pub use P000PFS_HA as PB00PFS_HA;
pub use P100PFS as P400PFS;
pub use P100PFS as P410PFS;
pub use P100PFS_BY as P400PFS_BY;
pub use P100PFS_BY as P410PFS_BY;
pub use P100PFS_HA as P400PFS_HA;
pub use P100PFS_HA as P410PFS_HA;
pub use P110PFS as P300PFS;
pub use P110PFS_BY as P300PFS_BY;
pub use P110PFS_HA as P300PFS_HA;
