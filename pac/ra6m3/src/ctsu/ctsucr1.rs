///Register `CTSUCR1` reader
pub type R = crate::R<CTSUCR1_SPEC>;
///Register `CTSUCR1` writer
pub type W = crate::W<CTSUCR1_SPEC>;
/**CTSU Power Supply Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUPON_A {
    ///0: Powered off the CTSU
    _0 = 0,
    ///1: Powered on the CTSU
    _1 = 1,
}
impl From<CTSUPON_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUPON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUPON` reader - CTSU Power Supply Enable
pub type CTSUPON_R = crate::BitReader<CTSUPON_A>;
impl CTSUPON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUPON_A {
        match self.bits {
            false => CTSUPON_A::_0,
            true => CTSUPON_A::_1,
        }
    }
    ///Powered off the CTSU
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUPON_A::_0
    }
    ///Powered on the CTSU
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUPON_A::_1
    }
}
///Field `CTSUPON` writer - CTSU Power Supply Enable
pub type CTSUPON_W<'a, REG> = crate::BitWriter<'a, REG, CTSUPON_A>;
impl<'a, REG> CTSUPON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Powered off the CTSU
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUPON_A::_0)
    }
    ///Powered on the CTSU
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUPON_A::_1)
    }
}
/**CTSU LPF Capacitance Charging Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUCSW_A {
    ///0: Turned off capacitance switch
    _0 = 0,
    ///1: Turned on capacitance switch
    _1 = 1,
}
impl From<CTSUCSW_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUCSW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUCSW` reader - CTSU LPF Capacitance Charging Control
pub type CTSUCSW_R = crate::BitReader<CTSUCSW_A>;
impl CTSUCSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCSW_A {
        match self.bits {
            false => CTSUCSW_A::_0,
            true => CTSUCSW_A::_1,
        }
    }
    ///Turned off capacitance switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCSW_A::_0
    }
    ///Turned on capacitance switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCSW_A::_1
    }
}
///Field `CTSUCSW` writer - CTSU LPF Capacitance Charging Control
pub type CTSUCSW_W<'a, REG> = crate::BitWriter<'a, REG, CTSUCSW_A>;
impl<'a, REG> CTSUCSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Turned off capacitance switch
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCSW_A::_0)
    }
    ///Turned on capacitance switch
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCSW_A::_1)
    }
}
/**CTSU Power Supply Capacity Adjustment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUATUNE1_A {
    ///0: Normal output
    _0 = 0,
    ///1: High-current output
    _1 = 1,
}
impl From<CTSUATUNE1_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUATUNE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUATUNE1` reader - CTSU Power Supply Capacity Adjustment
pub type CTSUATUNE1_R = crate::BitReader<CTSUATUNE1_A>;
impl CTSUATUNE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUATUNE1_A {
        match self.bits {
            false => CTSUATUNE1_A::_0,
            true => CTSUATUNE1_A::_1,
        }
    }
    ///Normal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUATUNE1_A::_0
    }
    ///High-current output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUATUNE1_A::_1
    }
}
///Field `CTSUATUNE1` writer - CTSU Power Supply Capacity Adjustment
pub type CTSUATUNE1_W<'a, REG> = crate::BitWriter<'a, REG, CTSUATUNE1_A>;
impl<'a, REG> CTSUATUNE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUATUNE1_A::_0)
    }
    ///High-current output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUATUNE1_A::_1)
    }
}
/**CTSU Operating Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCLK_A {
    ///0: PCLK
    _00 = 0,
    ///1: PCLK/2 (PCLK divided by 2)
    _01 = 1,
    ///2: PCLK/2 (PCLK divided by 4)
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<CTSUCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCLK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCLK_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCLK_A {}
///Field `CTSUCLK` reader - CTSU Operating Clock Select
pub type CTSUCLK_R = crate::FieldReader<CTSUCLK_A>;
impl CTSUCLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCLK_A {
        match self.bits {
            0 => CTSUCLK_A::_00,
            1 => CTSUCLK_A::_01,
            2 => CTSUCLK_A::_10,
            3 => CTSUCLK_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUCLK_A::_00
    }
    ///PCLK/2 (PCLK divided by 2)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUCLK_A::_01
    }
    ///PCLK/2 (PCLK divided by 4)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUCLK_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUCLK_A::_11
    }
}
///Field `CTSUCLK` writer - CTSU Operating Clock Select
pub type CTSUCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CTSUCLK_A, crate::Safe>;
impl<'a, REG> CTSUCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCLK_A::_00)
    }
    ///PCLK/2 (PCLK divided by 2)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCLK_A::_01)
    }
    ///PCLK/2 (PCLK divided by 4)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCLK_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCLK_A::_11)
    }
}
/**CTSU Measurement Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUMD_A {
    ///0: Self-capacitance single scan mode
    _00 = 0,
    ///1: Self-capacitance multi-scan mode
    _01 = 1,
    ///2: Mutual capacitance simple scan mode
    _10 = 2,
    ///3: Mutual capacitance full scan mode
    _11 = 3,
}
impl From<CTSUMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUMD_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUMD_A {}
///Field `CTSUMD` reader - CTSU Measurement Mode Select
pub type CTSUMD_R = crate::FieldReader<CTSUMD_A>;
impl CTSUMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUMD_A {
        match self.bits {
            0 => CTSUMD_A::_00,
            1 => CTSUMD_A::_01,
            2 => CTSUMD_A::_10,
            3 => CTSUMD_A::_11,
            _ => unreachable!(),
        }
    }
    ///Self-capacitance single scan mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUMD_A::_00
    }
    ///Self-capacitance multi-scan mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUMD_A::_01
    }
    ///Mutual capacitance simple scan mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUMD_A::_10
    }
    ///Mutual capacitance full scan mode
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUMD_A::_11
    }
}
///Field `CTSUMD` writer - CTSU Measurement Mode Select
pub type CTSUMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CTSUMD_A, crate::Safe>;
impl<'a, REG> CTSUMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Self-capacitance single scan mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUMD_A::_00)
    }
    ///Self-capacitance multi-scan mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUMD_A::_01)
    }
    ///Mutual capacitance simple scan mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUMD_A::_10)
    }
    ///Mutual capacitance full scan mode
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUMD_A::_11)
    }
}
impl R {
    ///Bit 0 - CTSU Power Supply Enable
    #[inline(always)]
    pub fn ctsupon(&self) -> CTSUPON_R {
        CTSUPON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTSU LPF Capacitance Charging Control
    #[inline(always)]
    pub fn ctsucsw(&self) -> CTSUCSW_R {
        CTSUCSW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - CTSU Power Supply Capacity Adjustment
    #[inline(always)]
    pub fn ctsuatune1(&self) -> CTSUATUNE1_R {
        CTSUATUNE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - CTSU Operating Clock Select
    #[inline(always)]
    pub fn ctsuclk(&self) -> CTSUCLK_R {
        CTSUCLK_R::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - CTSU Measurement Mode Select
    #[inline(always)]
    pub fn ctsumd(&self) -> CTSUMD_R {
        CTSUMD_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bit 0 - CTSU Power Supply Enable
    #[inline(always)]
    pub fn ctsupon(&mut self) -> CTSUPON_W<CTSUCR1_SPEC> {
        CTSUPON_W::new(self, 0)
    }
    ///Bit 1 - CTSU LPF Capacitance Charging Control
    #[inline(always)]
    pub fn ctsucsw(&mut self) -> CTSUCSW_W<CTSUCR1_SPEC> {
        CTSUCSW_W::new(self, 1)
    }
    ///Bit 3 - CTSU Power Supply Capacity Adjustment
    #[inline(always)]
    pub fn ctsuatune1(&mut self) -> CTSUATUNE1_W<CTSUCR1_SPEC> {
        CTSUATUNE1_W::new(self, 3)
    }
    ///Bits 4:5 - CTSU Operating Clock Select
    #[inline(always)]
    pub fn ctsuclk(&mut self) -> CTSUCLK_W<CTSUCR1_SPEC> {
        CTSUCLK_W::new(self, 4)
    }
    ///Bits 6:7 - CTSU Measurement Mode Select
    #[inline(always)]
    pub fn ctsumd(&mut self) -> CTSUMD_W<CTSUCR1_SPEC> {
        CTSUMD_W::new(self, 6)
    }
}
/**CTSU Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsucr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCR1_SPEC;
impl crate::RegisterSpec for CTSUCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsucr1::R`](R) reader structure
impl crate::Readable for CTSUCR1_SPEC {}
///`write(|w| ..)` method takes [`ctsucr1::W`](W) writer structure
impl crate::Writable for CTSUCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCR1 to value 0
impl crate::Resettable for CTSUCR1_SPEC {}
