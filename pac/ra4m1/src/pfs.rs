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
    _reserved9: [u8; 0x04],
    _reserved_9_p010: [u8; 0x04],
    _reserved_10_p011: [u8; 0x04],
    _reserved_11_p012: [u8; 0x04],
    _reserved_12_p013: [u8; 0x04],
    _reserved_13_p014: [u8; 0x04],
    _reserved_14_p015: [u8; 0x04],
    _reserved_15_p100: [u8; 0x04],
    _reserved_16_p101: [u8; 0x04],
    _reserved_17_p102: [u8; 0x04],
    _reserved_18_p103: [u8; 0x04],
    _reserved_19_p104: [u8; 0x04],
    _reserved_20_p105: [u8; 0x04],
    _reserved_21_p106: [u8; 0x04],
    _reserved_22_p107: [u8; 0x04],
    _reserved_23_p108: [u8; 0x04],
    _reserved_24_p109: [u8; 0x04],
    _reserved_25_p110: [u8; 0x04],
    _reserved_26_p111: [u8; 0x04],
    _reserved_27_p112: [u8; 0x04],
    _reserved_28_p113: [u8; 0x04],
    _reserved_29_p114: [u8; 0x04],
    _reserved_30_p115: [u8; 0x04],
    _reserved_31_p200: [u8; 0x04],
    _reserved_32_p201: [u8; 0x04],
    _reserved_33_p202: [u8; 0x04],
    _reserved_34_p203: [u8; 0x04],
    _reserved_35_p204: [u8; 0x04],
    _reserved_36_p205: [u8; 0x04],
    _reserved_37_p206: [u8; 0x04],
    _reserved38: [u8; 0x14],
    _reserved_38_p212: [u8; 0x04],
    _reserved_39_p213: [u8; 0x04],
    _reserved_40_p214: [u8; 0x04],
    _reserved_41_p215: [u8; 0x04],
    _reserved_42_p300: [u8; 0x04],
    _reserved_43_p301: [u8; 0x04],
    _reserved_44_p302: [u8; 0x04],
    _reserved_45_p303: [u8; 0x04],
    _reserved_46_p304: [u8; 0x04],
    _reserved_47_p305: [u8; 0x04],
    _reserved_48_p306: [u8; 0x04],
    _reserved_49_p307: [u8; 0x04],
    _reserved50: [u8; 0x20],
    _reserved_50_p400: [u8; 0x04],
    _reserved_51_p401: [u8; 0x04],
    _reserved_52_p402: [u8; 0x04],
    _reserved_53_p403: [u8; 0x04],
    _reserved_54_p404: [u8; 0x04],
    _reserved_55_p405: [u8; 0x04],
    _reserved_56_p406: [u8; 0x04],
    _reserved_57_p407: [u8; 0x04],
    _reserved_58_p408: [u8; 0x04],
    _reserved_59_p409: [u8; 0x04],
    _reserved_60_p410: [u8; 0x04],
    _reserved_61_p411: [u8; 0x04],
    _reserved_62_p412: [u8; 0x04],
    _reserved_63_p413: [u8; 0x04],
    _reserved_64_p414: [u8; 0x04],
    _reserved_65_p415: [u8; 0x04],
    _reserved_66_p500: [u8; 0x04],
    _reserved_67_p501: [u8; 0x04],
    _reserved_68_p502: [u8; 0x04],
    _reserved_69_p503: [u8; 0x04],
    _reserved_70_p504: [u8; 0x04],
    _reserved_71_p505: [u8; 0x04],
    _reserved72: [u8; 0x28],
    _reserved_72_p600: [u8; 0x04],
    _reserved_73_p601: [u8; 0x04],
    _reserved_74_p602: [u8; 0x04],
    _reserved_75_p603: [u8; 0x04],
    _reserved76: [u8; 0x10],
    _reserved_76_p608: [u8; 0x04],
    _reserved_77_p609: [u8; 0x04],
    _reserved_78_p610: [u8; 0x04],
    _reserved79: [u8; 0x34],
    _reserved_79_p708: [u8; 0x04],
    _reserved80: [u8; 0x3c],
    _reserved_80_p808: [u8; 0x04],
    _reserved_81_p809: [u8; 0x04],
    _reserved82: [u8; 0x50],
    _reserved_82_p914: [u8; 0x04],
    _reserved_83_p915: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - P00%s Pin Function Control Register
    #[inline(always)]
    pub const fn p000pfs(&self) -> &P000PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x02 - P00%s Pin Function Control Register
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P000PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x03 - P00%s Pin Function Control Register
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
    ///0x04 - P008 Pin Function Control Register
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P001PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - P008 Pin Function Control Register
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P001PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x07 - P008 Pin Function Control Register
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P001PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    ///0x28 - P010 Pin Function Control Register
    #[inline(always)]
    pub const fn p010pfs(&self) -> &P010PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2a - P010 Pin Function Control Register
    #[inline(always)]
    pub const fn p010pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    ///0x2b - P010 Pin Function Control Register
    #[inline(always)]
    pub const fn p010pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    ///0x28 - P011 Pin Function Control Register
    #[inline(always)]
    pub const fn p011pfs(&self) -> &P010PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2a - P011 Pin Function Control Register
    #[inline(always)]
    pub const fn p011pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    ///0x2b - P011 Pin Function Control Register
    #[inline(always)]
    pub const fn p011pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    ///0x28 - P012 Pin Function Control Register
    #[inline(always)]
    pub const fn p012pfs(&self) -> &P010PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2a - P012 Pin Function Control Register
    #[inline(always)]
    pub const fn p012pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    ///0x2b - P012 Pin Function Control Register
    #[inline(always)]
    pub const fn p012pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    ///0x28 - P013 Pin Function Control Register
    #[inline(always)]
    pub const fn p013pfs(&self) -> &P010PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2a - P013 Pin Function Control Register
    #[inline(always)]
    pub const fn p013pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    ///0x2b - P013 Pin Function Control Register
    #[inline(always)]
    pub const fn p013pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    ///0x28 - P014 Pin Function Control Register
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P010PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2a - P014 Pin Function Control Register
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    ///0x2b - P014 Pin Function Control Register
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    ///0x28 - P015 Pin Function Control Register
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P010PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2a - P015 Pin Function Control Register
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P010PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    ///0x2b - P015 Pin Function Control Register
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P010PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
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
    ///0x40 - P101 Pin Function Control Register
    #[inline(always)]
    pub const fn p101pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P101 Pin Function Control Register
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P101 Pin Function Control Register
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x40 - P102 Pin Function Control Register
    #[inline(always)]
    pub const fn p102pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P102 Pin Function Control Register
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P102 Pin Function Control Register
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x40 - P103 Pin Function Control Register
    #[inline(always)]
    pub const fn p103pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P103 Pin Function Control Register
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P103 Pin Function Control Register
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x40 - P104 Pin Function Control Register
    #[inline(always)]
    pub const fn p104pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P104 Pin Function Control Register
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P104 Pin Function Control Register
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x40 - P105 Pin Function Control Register
    #[inline(always)]
    pub const fn p105pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P105 Pin Function Control Register
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P105 Pin Function Control Register
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x40 - P106 Pin Function Control Register
    #[inline(always)]
    pub const fn p106pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P106 Pin Function Control Register
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P106 Pin Function Control Register
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    ///0x40 - P107 Pin Function Control Register
    #[inline(always)]
    pub const fn p107pfs(&self) -> &P100PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x42 - P107 Pin Function Control Register
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P100PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    ///0x43 - P107 Pin Function Control Register
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P100PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
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
    ///0xb0 - P212 Pin Function Control Register
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P212PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    ///0xb2 - P212 Pin Function Control Register
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    ///0xb3 - P212 Pin Function Control Register
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    ///0xb0 - P213 Pin Function Control Register
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P212PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    ///0xb2 - P213 Pin Function Control Register
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    ///0xb3 - P213 Pin Function Control Register
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    ///0xb0 - P214 Pin Function Control Register
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P212PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    ///0xb2 - P214 Pin Function Control Register
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    ///0xb3 - P214 Pin Function Control Register
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    ///0xb0 - P215 Pin Function Control Register
    #[inline(always)]
    pub const fn p215pfs(&self) -> &P212PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    ///0xb2 - P215 Pin Function Control Register
    #[inline(always)]
    pub const fn p215pfs_ha(&self) -> &P212PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    ///0xb3 - P215 Pin Function Control Register
    #[inline(always)]
    pub const fn p215pfs_by(&self) -> &P212PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
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
    ///0x120 - P408 Pin Function Control Register
    #[inline(always)]
    pub const fn p408pfs(&self) -> &P408PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    ///0x122 - P408 Pin Function Control Register
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P408PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(290).cast() }
    }
    ///0x123 - P408 Pin Function Control Register
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P408PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(291).cast() }
    }
    ///0x124 - P409 Pin Function Control Register
    #[inline(always)]
    pub const fn p409pfs(&self) -> &P409PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    ///0x126 - P409 Pin Function Control Register
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P409PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(294).cast() }
    }
    ///0x127 - P409 Pin Function Control Register
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P409PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(295).cast() }
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
    ///0x1a0 - P608 Pin Function Control Register
    #[inline(always)]
    pub const fn p608pfs(&self) -> &P608PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    ///0x1a2 - P608 Pin Function Control Register
    #[inline(always)]
    pub const fn p608pfs_ha(&self) -> &P608PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(418).cast() }
    }
    ///0x1a3 - P608 Pin Function Control Register
    #[inline(always)]
    pub const fn p608pfs_by(&self) -> &P608PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(419).cast() }
    }
    ///0x1a0 - P609 Pin Function Control Register
    #[inline(always)]
    pub const fn p609pfs(&self) -> &P608PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    ///0x1a2 - P609 Pin Function Control Register
    #[inline(always)]
    pub const fn p609pfs_ha(&self) -> &P608PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(418).cast() }
    }
    ///0x1a3 - P609 Pin Function Control Register
    #[inline(always)]
    pub const fn p609pfs_by(&self) -> &P608PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(419).cast() }
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
    ///0x1e0 - P708 Pin Function Control Register
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P708PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    ///0x1e2 - P708 Pin Function Control Register
    #[inline(always)]
    pub const fn p708pfs_ha(&self) -> &P708PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(482).cast() }
    }
    ///0x1e3 - P708 Pin Function Control Register
    #[inline(always)]
    pub const fn p708pfs_by(&self) -> &P708PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(483).cast() }
    }
    ///0x220 - P808 Pin Function Control Register
    #[inline(always)]
    pub const fn p808pfs(&self) -> &P808PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    ///0x222 - P808 Pin Function Control Register
    #[inline(always)]
    pub const fn p808pfs_ha(&self) -> &P808PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(546).cast() }
    }
    ///0x223 - P808 Pin Function Control Register
    #[inline(always)]
    pub const fn p808pfs_by(&self) -> &P808PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(547).cast() }
    }
    ///0x220 - P809 Pin Function Control Register
    #[inline(always)]
    pub const fn p809pfs(&self) -> &P808PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    ///0x222 - P809 Pin Function Control Register
    #[inline(always)]
    pub const fn p809pfs_ha(&self) -> &P808PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(546).cast() }
    }
    ///0x223 - P809 Pin Function Control Register
    #[inline(always)]
    pub const fn p809pfs_by(&self) -> &P808PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(547).cast() }
    }
    ///0x278 - P914 Pin Function Control Register
    #[inline(always)]
    pub const fn p914pfs(&self) -> &P914PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    ///0x27a - P914 Pin Function Control Register
    #[inline(always)]
    pub const fn p914pfs_ha(&self) -> &P914PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(634).cast() }
    }
    ///0x27b - P914 Pin Function Control Register
    #[inline(always)]
    pub const fn p914pfs_by(&self) -> &P914PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(635).cast() }
    }
    ///0x278 - P915 Pin Function Control Register
    #[inline(always)]
    pub const fn p915pfs(&self) -> &P914PFS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    ///0x27a - P915 Pin Function Control Register
    #[inline(always)]
    pub const fn p915pfs_ha(&self) -> &P914PFS_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(634).cast() }
    }
    ///0x27b - P915 Pin Function Control Register
    #[inline(always)]
    pub const fn p915pfs_by(&self) -> &P914PFS_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(635).cast() }
    }
}
/**P000PFS (rw) register accessor: P00%s Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p000pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p000pfs`] module*/
pub type P000PFS = crate::Reg<p000pfs::P000PFS_SPEC>;
///P00%s Pin Function Control Register
pub mod p000pfs;
/**P000PFS_HA (rw) register accessor: P00%s Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p000pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p000pfs_ha`] module*/
pub type P000PFS_HA = crate::Reg<p000pfs_ha::P000PFS_HA_SPEC>;
///P00%s Pin Function Control Register
pub mod p000pfs_ha;
/**P000PFS_BY (rw) register accessor: P00%s Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p000pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p000pfs_by`] module*/
pub type P000PFS_BY = crate::Reg<p000pfs_by::P000PFS_BY_SPEC>;
///P00%s Pin Function Control Register
pub mod p000pfs_by;
pub use p000pfs as p001pfs;
pub use p000pfs as p010pfs;
pub use p000pfs as p100pfs;
pub use p000pfs_by as p001pfs_by;
pub use p000pfs_by as p010pfs_by;
pub use p000pfs_by as p100pfs_by;
pub use p000pfs_ha as p001pfs_ha;
pub use p000pfs_ha as p010pfs_ha;
pub use p000pfs_ha as p100pfs_ha;
pub use P000PFS as P001PFS;
pub use P000PFS as P010PFS;
pub use P000PFS as P100PFS;
pub use P000PFS_BY as P001PFS_BY;
pub use P000PFS_BY as P010PFS_BY;
pub use P000PFS_BY as P100PFS_BY;
pub use P000PFS_HA as P001PFS_HA;
pub use P000PFS_HA as P010PFS_HA;
pub use P000PFS_HA as P100PFS_HA;
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
/**P109PFS (rw) register accessor: P109 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p109pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p109pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p109pfs`] module*/
pub type P109PFS = crate::Reg<p109pfs::P109PFS_SPEC>;
///P109 Pin Function Control Register
pub mod p109pfs;
/**P109PFS_HA (rw) register accessor: P109 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p109pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p109pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p109pfs_ha`] module*/
pub type P109PFS_HA = crate::Reg<p109pfs_ha::P109PFS_HA_SPEC>;
///P109 Pin Function Control Register
pub mod p109pfs_ha;
pub use p000pfs as p111pfs;
pub use p000pfs as p200pfs;
pub use p000pfs_by as p109pfs_by;
pub use p000pfs_by as p111pfs_by;
pub use p000pfs_by as p200pfs_by;
pub use p000pfs_ha as p111pfs_ha;
pub use p000pfs_ha as p200pfs_ha;
pub use p108pfs as p110pfs;
pub use p108pfs_by as p110pfs_by;
pub use p108pfs_ha as p110pfs_ha;
pub use P000PFS as P111PFS;
pub use P000PFS as P200PFS;
pub use P000PFS_BY as P109PFS_BY;
pub use P000PFS_BY as P111PFS_BY;
pub use P000PFS_BY as P200PFS_BY;
pub use P000PFS_HA as P111PFS_HA;
pub use P000PFS_HA as P200PFS_HA;
pub use P108PFS as P110PFS;
pub use P108PFS_BY as P110PFS_BY;
pub use P108PFS_HA as P110PFS_HA;
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
pub use p000pfs as p212pfs;
pub use p000pfs as p301pfs;
pub use p000pfs as p400pfs;
pub use p000pfs_by as p202pfs_by;
pub use p000pfs_by as p212pfs_by;
pub use p000pfs_by as p301pfs_by;
pub use p000pfs_by as p400pfs_by;
pub use p000pfs_ha as p202pfs_ha;
pub use p000pfs_ha as p212pfs_ha;
pub use p000pfs_ha as p301pfs_ha;
pub use p000pfs_ha as p400pfs_ha;
pub use p108pfs as p300pfs;
pub use p108pfs_by as p300pfs_by;
pub use p108pfs_ha as p300pfs_ha;
pub use P000PFS as P202PFS;
pub use P000PFS as P212PFS;
pub use P000PFS as P301PFS;
pub use P000PFS as P400PFS;
pub use P000PFS_BY as P202PFS_BY;
pub use P000PFS_BY as P212PFS_BY;
pub use P000PFS_BY as P301PFS_BY;
pub use P000PFS_BY as P400PFS_BY;
pub use P000PFS_HA as P202PFS_HA;
pub use P000PFS_HA as P212PFS_HA;
pub use P000PFS_HA as P301PFS_HA;
pub use P000PFS_HA as P400PFS_HA;
pub use P108PFS as P300PFS;
pub use P108PFS_BY as P300PFS_BY;
pub use P108PFS_HA as P300PFS_HA;
/**P408PFS (rw) register accessor: P408 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p408pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p408pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p408pfs`] module*/
pub type P408PFS = crate::Reg<p408pfs::P408PFS_SPEC>;
///P408 Pin Function Control Register
pub mod p408pfs;
/**P408PFS_HA (rw) register accessor: P408 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p408pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p408pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p408pfs_ha`] module*/
pub type P408PFS_HA = crate::Reg<p408pfs_ha::P408PFS_HA_SPEC>;
///P408 Pin Function Control Register
pub mod p408pfs_ha;
pub use p000pfs as p409pfs;
pub use p000pfs as p410pfs;
pub use p000pfs as p500pfs;
pub use p000pfs as p600pfs;
pub use p000pfs as p608pfs;
pub use p000pfs as p610pfs;
pub use p000pfs as p708pfs;
pub use p000pfs as p808pfs;
pub use p000pfs_by as p408pfs_by;
pub use p000pfs_by as p409pfs_by;
pub use p000pfs_by as p410pfs_by;
pub use p000pfs_by as p500pfs_by;
pub use p000pfs_by as p600pfs_by;
pub use p000pfs_by as p608pfs_by;
pub use p000pfs_by as p610pfs_by;
pub use p000pfs_by as p708pfs_by;
pub use p000pfs_by as p808pfs_by;
pub use p000pfs_ha as p409pfs_ha;
pub use p000pfs_ha as p410pfs_ha;
pub use p000pfs_ha as p500pfs_ha;
pub use p000pfs_ha as p600pfs_ha;
pub use p000pfs_ha as p608pfs_ha;
pub use p000pfs_ha as p610pfs_ha;
pub use p000pfs_ha as p708pfs_ha;
pub use p000pfs_ha as p808pfs_ha;
pub use p109pfs as p914pfs;
pub use p109pfs_by as p914pfs_by;
pub use p109pfs_ha as p914pfs_ha;
pub use P000PFS as P409PFS;
pub use P000PFS as P410PFS;
pub use P000PFS as P500PFS;
pub use P000PFS as P600PFS;
pub use P000PFS as P608PFS;
pub use P000PFS as P610PFS;
pub use P000PFS as P708PFS;
pub use P000PFS as P808PFS;
pub use P000PFS_BY as P408PFS_BY;
pub use P000PFS_BY as P409PFS_BY;
pub use P000PFS_BY as P410PFS_BY;
pub use P000PFS_BY as P500PFS_BY;
pub use P000PFS_BY as P600PFS_BY;
pub use P000PFS_BY as P608PFS_BY;
pub use P000PFS_BY as P610PFS_BY;
pub use P000PFS_BY as P708PFS_BY;
pub use P000PFS_BY as P808PFS_BY;
pub use P000PFS_HA as P409PFS_HA;
pub use P000PFS_HA as P410PFS_HA;
pub use P000PFS_HA as P500PFS_HA;
pub use P000PFS_HA as P600PFS_HA;
pub use P000PFS_HA as P608PFS_HA;
pub use P000PFS_HA as P610PFS_HA;
pub use P000PFS_HA as P708PFS_HA;
pub use P000PFS_HA as P808PFS_HA;
pub use P109PFS as P914PFS;
pub use P109PFS_BY as P914PFS_BY;
pub use P109PFS_HA as P914PFS_HA;
