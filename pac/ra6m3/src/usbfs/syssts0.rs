///Register `SYSSTS0` reader
pub type R = crate::R<SYSSTS0_SPEC>;
/**USB Data Line Status Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LNST_A {
    ///0: SE0
    _00 = 0,
    ///1: J-State
    _01 = 1,
    ///2: K-State
    _10 = 2,
    ///3: SE1
    _11 = 3,
}
impl From<LNST_A> for u8 {
    #[inline(always)]
    fn from(variant: LNST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LNST_A {
    type Ux = u8;
}
impl crate::IsEnum for LNST_A {}
///Field `LNST` reader - USB Data Line Status Monitor
pub type LNST_R = crate::FieldReader<LNST_A>;
impl LNST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LNST_A {
        match self.bits {
            0 => LNST_A::_00,
            1 => LNST_A::_01,
            2 => LNST_A::_10,
            3 => LNST_A::_11,
            _ => unreachable!(),
        }
    }
    ///SE0
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LNST_A::_00
    }
    ///J-State
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LNST_A::_01
    }
    ///K-State
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LNST_A::_10
    }
    ///SE1
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LNST_A::_11
    }
}
/**External ID0 Input Pin Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMON_A {
    ///0: USB0_ID pin is low
    _0 = 0,
    ///1: USB0_ID pin is high
    _1 = 1,
}
impl From<IDMON_A> for bool {
    #[inline(always)]
    fn from(variant: IDMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDMON` reader - External ID0 Input Pin Monitor
pub type IDMON_R = crate::BitReader<IDMON_A>;
impl IDMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDMON_A {
        match self.bits {
            false => IDMON_A::_0,
            true => IDMON_A::_1,
        }
    }
    ///USB0_ID pin is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDMON_A::_0
    }
    ///USB0_ID pin is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDMON_A::_1
    }
}
/**Active Monitor When the Host Controller is Selected.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFEA_A {
    ///0: SOF output is stopped.
    _0 = 0,
    ///1: SOF output is operating.
    _1 = 1,
}
impl From<SOFEA_A> for bool {
    #[inline(always)]
    fn from(variant: SOFEA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOFEA` reader - Active Monitor When the Host Controller is Selected.
pub type SOFEA_R = crate::BitReader<SOFEA_A>;
impl SOFEA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOFEA_A {
        match self.bits {
            false => SOFEA_A::_0,
            true => SOFEA_A::_1,
        }
    }
    ///SOF output is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFEA_A::_0
    }
    ///SOF output is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFEA_A::_1
    }
}
/**USB Host Sequencer Status Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTACT_A {
    ///0: Host sequencer of the USB is completely stopped.
    _0 = 0,
    ///1: Host sequencer of the USB is not completely stopped.
    _1 = 1,
}
impl From<HTACT_A> for bool {
    #[inline(always)]
    fn from(variant: HTACT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTACT` reader - USB Host Sequencer Status Monitor
pub type HTACT_R = crate::BitReader<HTACT_A>;
impl HTACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTACT_A {
        match self.bits {
            false => HTACT_A::_0,
            true => HTACT_A::_1,
        }
    }
    ///Host sequencer of the USB is completely stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTACT_A::_0
    }
    ///Host sequencer of the USB is not completely stopped.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTACT_A::_1
    }
}
///Field `OVCMON` reader - External USB0_OVRCURA/ USB0_OVRCURB Input Pin MonitorThe OCVMON\[1\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\[0\] bit indicates the status of the USBHS_OVRCURB pin.
pub type OVCMON_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - USB Data Line Status Monitor
    #[inline(always)]
    pub fn lnst(&self) -> LNST_R {
        LNST_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - External ID0 Input Pin Monitor
    #[inline(always)]
    pub fn idmon(&self) -> IDMON_R {
        IDMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Active Monitor When the Host Controller is Selected.
    #[inline(always)]
    pub fn sofea(&self) -> SOFEA_R {
        SOFEA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - USB Host Sequencer Status Monitor
    #[inline(always)]
    pub fn htact(&self) -> HTACT_R {
        HTACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 14:15 - External USB0_OVRCURA/ USB0_OVRCURB Input Pin MonitorThe OCVMON\[1\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\[0\] bit indicates the status of the USBHS_OVRCURB pin.
    #[inline(always)]
    pub fn ovcmon(&self) -> OVCMON_R {
        OVCMON_R::new(((self.bits >> 14) & 3) as u8)
    }
}
/**System Configuration Status Register 0

You can [`read`](crate::Reg::read) this register and get [`syssts0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSSTS0_SPEC;
impl crate::RegisterSpec for SYSSTS0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`syssts0::R`](R) reader structure
impl crate::Readable for SYSSTS0_SPEC {}
///`reset()` method sets SYSSTS0 to value 0
impl crate::Resettable for SYSSTS0_SPEC {}
