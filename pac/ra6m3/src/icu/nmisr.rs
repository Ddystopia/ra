///Register `NMISR` reader
pub type R = crate::R<NMISR_SPEC>;
/**IWDT Underflow/Refresh Error Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTST_A {
    ///0: IWDT underflow/refresh error interrupt is not requested.
    _0 = 0,
    ///1: IWDT underflow/refresh error interrupt is requested.
    _1 = 1,
}
impl From<IWDTST_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTST` reader - IWDT Underflow/Refresh Error Status Flag
pub type IWDTST_R = crate::BitReader<IWDTST_A>;
impl IWDTST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDTST_A {
        match self.bits {
            false => IWDTST_A::_0,
            true => IWDTST_A::_1,
        }
    }
    ///IWDT underflow/refresh error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTST_A::_0
    }
    ///IWDT underflow/refresh error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTST_A::_1
    }
}
/**WDT Underflow/Refresh Error Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTST_A {
    ///0: WDT underflow/refresh error interrupt is not requested.
    _0 = 0,
    ///1: WDT underflow/refresh error interrupt is requested.
    _1 = 1,
}
impl From<WDTST_A> for bool {
    #[inline(always)]
    fn from(variant: WDTST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTST` reader - WDT Underflow/Refresh Error Status Flag
pub type WDTST_R = crate::BitReader<WDTST_A>;
impl WDTST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDTST_A {
        match self.bits {
            false => WDTST_A::_0,
            true => WDTST_A::_1,
        }
    }
    ///WDT underflow/refresh error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTST_A::_0
    }
    ///WDT underflow/refresh error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTST_A::_1
    }
}
/**Voltage-Monitoring 1 Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1ST_A {
    ///0: Voltage-monitoring 1 interrupt is not requested.
    _0 = 0,
    ///1: Voltage-monitoring 1 interrupt is requested.
    _1 = 1,
}
impl From<LVD1ST_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1ST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1ST` reader - Voltage-Monitoring 1 Interrupt Status Flag
pub type LVD1ST_R = crate::BitReader<LVD1ST_A>;
impl LVD1ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1ST_A {
        match self.bits {
            false => LVD1ST_A::_0,
            true => LVD1ST_A::_1,
        }
    }
    ///Voltage-monitoring 1 interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1ST_A::_0
    }
    ///Voltage-monitoring 1 interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1ST_A::_1
    }
}
/**Voltage-Monitoring 2 Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2ST_A {
    ///0: Voltage-monitoring 2 interrupt is not requested.
    _0 = 0,
    ///1: Voltage-monitoring 2 interrupt is requested.
    _1 = 1,
}
impl From<LVD2ST_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2ST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2ST` reader - Voltage-Monitoring 2 Interrupt Status Flag
pub type LVD2ST_R = crate::BitReader<LVD2ST_A>;
impl LVD2ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2ST_A {
        match self.bits {
            false => LVD2ST_A::_0,
            true => LVD2ST_A::_1,
        }
    }
    ///Voltage-monitoring 2 interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2ST_A::_0
    }
    ///Voltage-monitoring 2 interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2ST_A::_1
    }
}
/**Oscillation Stop Detection Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTST_A {
    ///0: Oscillation stop detection interrupt is not requested.
    _0 = 0,
    ///1: Oscillation stop detection interrupt is requested.
    _1 = 1,
}
impl From<OSTST_A> for bool {
    #[inline(always)]
    fn from(variant: OSTST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTST` reader - Oscillation Stop Detection Interrupt Status Flag
pub type OSTST_R = crate::BitReader<OSTST_A>;
impl OSTST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTST_A {
        match self.bits {
            false => OSTST_A::_0,
            true => OSTST_A::_1,
        }
    }
    ///Oscillation stop detection interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTST_A::_0
    }
    ///Oscillation stop detection interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTST_A::_1
    }
}
/**NMI Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIST_A {
    ///0: NMI pin interrupt is not requested.
    _0 = 0,
    ///1: NMI pin interrupt is requested.
    _1 = 1,
}
impl From<NMIST_A> for bool {
    #[inline(always)]
    fn from(variant: NMIST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NMIST` reader - NMI Status Flag
pub type NMIST_R = crate::BitReader<NMIST_A>;
impl NMIST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NMIST_A {
        match self.bits {
            false => NMIST_A::_0,
            true => NMIST_A::_1,
        }
    }
    ///NMI pin interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIST_A::_0
    }
    ///NMI pin interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIST_A::_1
    }
}
/**RAM Parity Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPEST_A {
    ///0: RAM Parity Error interrupt is not requested.
    _0 = 0,
    ///1: RAM Parity Error interrupt is requested.
    _1 = 1,
}
impl From<RPEST_A> for bool {
    #[inline(always)]
    fn from(variant: RPEST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPEST` reader - RAM Parity Error Interrupt Status Flag
pub type RPEST_R = crate::BitReader<RPEST_A>;
impl RPEST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPEST_A {
        match self.bits {
            false => RPEST_A::_0,
            true => RPEST_A::_1,
        }
    }
    ///RAM Parity Error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPEST_A::_0
    }
    ///RAM Parity Error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPEST_A::_1
    }
}
/**RAM ECC Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECCST_A {
    ///0: RAM ECC Error interrupt is not requested.
    _0 = 0,
    ///1: RAM ECC Error interrupt is requested.
    _1 = 1,
}
impl From<RECCST_A> for bool {
    #[inline(always)]
    fn from(variant: RECCST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RECCST` reader - RAM ECC Error Interrupt Status Flag
pub type RECCST_R = crate::BitReader<RECCST_A>;
impl RECCST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RECCST_A {
        match self.bits {
            false => RECCST_A::_0,
            true => RECCST_A::_1,
        }
    }
    ///RAM ECC Error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECCST_A::_0
    }
    ///RAM ECC Error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECCST_A::_1
    }
}
/**MPU Bus Slave Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSST_A {
    ///0: MPU Bus Slave Error interrupt is not requested.
    _0 = 0,
    ///1: MPU Bus Slave Error interrupt is requested.
    _1 = 1,
}
impl From<BUSSST_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSSST` reader - MPU Bus Slave Error Interrupt Status Flag
pub type BUSSST_R = crate::BitReader<BUSSST_A>;
impl BUSSST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSSST_A {
        match self.bits {
            false => BUSSST_A::_0,
            true => BUSSST_A::_1,
        }
    }
    ///MPU Bus Slave Error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSST_A::_0
    }
    ///MPU Bus Slave Error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSST_A::_1
    }
}
/**MPU Bus Master Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMST_A {
    ///0: MPU Bus Master Error interrupt is not requested.
    _0 = 0,
    ///1: MPU Bus Master Error interrupt is requested.
    _1 = 1,
}
impl From<BUSMST_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMST` reader - MPU Bus Master Error Interrupt Status Flag
pub type BUSMST_R = crate::BitReader<BUSMST_A>;
impl BUSMST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSMST_A {
        match self.bits {
            false => BUSMST_A::_0,
            true => BUSMST_A::_1,
        }
    }
    ///MPU Bus Master Error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMST_A::_0
    }
    ///MPU Bus Master Error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMST_A::_1
    }
}
/**MPU Stack Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEST_A {
    ///0: MPU Stack Error interrupt is not requested.
    _0 = 0,
    ///1: MPU Stack Error interrupt is requested.
    _1 = 1,
}
impl From<SPEST_A> for bool {
    #[inline(always)]
    fn from(variant: SPEST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPEST` reader - MPU Stack Error Interrupt Status Flag
pub type SPEST_R = crate::BitReader<SPEST_A>;
impl SPEST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPEST_A {
        match self.bits {
            false => SPEST_A::_0,
            true => SPEST_A::_1,
        }
    }
    ///MPU Stack Error interrupt is not requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEST_A::_0
    }
    ///MPU Stack Error interrupt is requested.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEST_A::_1
    }
}
impl R {
    ///Bit 0 - IWDT Underflow/Refresh Error Status Flag
    #[inline(always)]
    pub fn iwdtst(&self) -> IWDTST_R {
        IWDTST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Status Flag
    #[inline(always)]
    pub fn wdtst(&self) -> WDTST_R {
        WDTST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage-Monitoring 1 Interrupt Status Flag
    #[inline(always)]
    pub fn lvd1st(&self) -> LVD1ST_R {
        LVD1ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage-Monitoring 2 Interrupt Status Flag
    #[inline(always)]
    pub fn lvd2st(&self) -> LVD2ST_R {
        LVD2ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Oscillation Stop Detection Interrupt Status Flag
    #[inline(always)]
    pub fn ostst(&self) -> OSTST_R {
        OSTST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NMI Status Flag
    #[inline(always)]
    pub fn nmist(&self) -> NMIST_R {
        NMIST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RAM Parity Error Interrupt Status Flag
    #[inline(always)]
    pub fn rpest(&self) -> RPEST_R {
        RPEST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RAM ECC Error Interrupt Status Flag
    #[inline(always)]
    pub fn reccst(&self) -> RECCST_R {
        RECCST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MPU Bus Slave Error Interrupt Status Flag
    #[inline(always)]
    pub fn bussst(&self) -> BUSSST_R {
        BUSSST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MPU Bus Master Error Interrupt Status Flag
    #[inline(always)]
    pub fn busmst(&self) -> BUSMST_R {
        BUSMST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - MPU Stack Error Interrupt Status Flag
    #[inline(always)]
    pub fn spest(&self) -> SPEST_R {
        SPEST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
/**Non-Maskable Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nmisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NMISR_SPEC;
impl crate::RegisterSpec for NMISR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nmisr::R`](R) reader structure
impl crate::Readable for NMISR_SPEC {}
///`reset()` method sets NMISR to value 0
impl crate::Resettable for NMISR_SPEC {}
