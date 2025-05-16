///Register `BG_MON` reader
pub type R = crate::R<BG_MON_SPEC>;
/**Background plane generation module operation state monitor.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///1: Operation is in progress.
    _1 = 1,
    ///0: Operation is stopped.
    _0 = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Background plane generation module operation state monitor.
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::_1,
            false => EN_A::_0,
        }
    }
    ///Operation is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
    ///Operation is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
}
/**Entire module internal operation reflection control signal monitor.The signal state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEN_A {
    ///1: The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is asserted.
    _1 = 1,
    ///0: The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is negated.
    _0 = 0,
}
impl From<VEN_A> for bool {
    #[inline(always)]
    fn from(variant: VEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VEN` reader - Entire module internal operation reflection control signal monitor.The signal state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal.
pub type VEN_R = crate::BitReader<VEN_A>;
impl VEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VEN_A {
        match self.bits {
            true => VEN_A::_1,
            false => VEN_A::_0,
        }
    }
    ///The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is asserted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VEN_A::_1
    }
    ///The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is negated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VEN_A::_0
    }
}
/**Entire module SW reset state monitor.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    ///1: The entire module is released from the SW reset state.
    _1 = 1,
    ///0: The entire module is in the SW reset state.
    _0 = 0,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWRST` reader - Entire module SW reset state monitor.
pub type SWRST_R = crate::BitReader<SWRST_A>;
impl SWRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWRST_A {
        match self.bits {
            true => SWRST_A::_1,
            false => SWRST_A::_0,
        }
    }
    ///The entire module is released from the SW reset state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRST_A::_1
    }
    ///The entire module is in the SW reset state.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRST_A::_0
    }
}
impl R {
    ///Bit 0 - Background plane generation module operation state monitor.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Entire module internal operation reflection control signal monitor.The signal state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal.
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Entire module SW reset state monitor.
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
/**Background Plane Setting Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`bg_mon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_MON_SPEC;
impl crate::RegisterSpec for BG_MON_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_mon::R`](R) reader structure
impl crate::Readable for BG_MON_SPEC {}
///`reset()` method sets BG_MON to value 0
impl crate::Resettable for BG_MON_SPEC {}
