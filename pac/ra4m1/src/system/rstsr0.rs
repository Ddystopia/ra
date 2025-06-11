///Register `RSTSR0` reader
pub type R = crate::R<RSTSR0_SPEC>;
///Register `RSTSR0` writer
pub type W = crate::W<RSTSR0_SPEC>;
/**Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORF_A {
    ///0: Power-on reset not detected.
    _0 = 0,
    ///1: Power-on reset detected.
    _1 = 1,
}
impl From<PORF_A> for bool {
    #[inline(always)]
    fn from(variant: PORF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PORF` reader - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PORF_R = crate::BitReader<PORF_A>;
impl PORF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PORF_A {
        match self.bits {
            false => PORF_A::_0,
            true => PORF_A::_1,
        }
    }
    ///Power-on reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORF_A::_0
    }
    ///Power-on reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORF_A::_1
    }
}
///Field `PORF` writer - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
pub type PORF_W<'a, REG> = crate::BitWriter0C<'a, REG, PORF_A>;
impl<'a, REG> PORF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Power-on reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PORF_A::_0)
    }
    ///Power-on reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PORF_A::_1)
    }
}
/**Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD0RF_A {
    ///0: Voltage Monitor 0 reset not detected.
    _0 = 0,
    ///1: Voltage Monitor 0 reset detected.
    _1 = 1,
}
impl From<LVD0RF_A> for bool {
    #[inline(always)]
    fn from(variant: LVD0RF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `LVD0RF` reader - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type LVD0RF_R = crate::BitReader<LVD0RF_A>;
impl LVD0RF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD0RF_A {
        match self.bits {
            false => LVD0RF_A::_0,
            true => LVD0RF_A::_1,
        }
    }
    ///Voltage Monitor 0 reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD0RF_A::_0
    }
    ///Voltage Monitor 0 reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD0RF_A::_1
    }
}
///Field `LVD0RF` writer - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
pub type LVD0RF_W<'a, REG> = crate::BitWriter0C<'a, REG, LVD0RF_A>;
impl<'a, REG> LVD0RF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage Monitor 0 reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD0RF_A::_0)
    }
    ///Voltage Monitor 0 reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD0RF_A::_1)
    }
}
/**Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1RF_A {
    ///0: Voltage Monitor 1 reset not detected.
    _0 = 0,
    ///1: Voltage Monitor 1 reset detected.
    _1 = 1,
}
impl From<LVD1RF_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1RF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `LVD1RF` reader - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type LVD1RF_R = crate::BitReader<LVD1RF_A>;
impl LVD1RF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1RF_A {
        match self.bits {
            false => LVD1RF_A::_0,
            true => LVD1RF_A::_1,
        }
    }
    ///Voltage Monitor 1 reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1RF_A::_0
    }
    ///Voltage Monitor 1 reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1RF_A::_1
    }
}
///Field `LVD1RF` writer - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
pub type LVD1RF_W<'a, REG> = crate::BitWriter0C<'a, REG, LVD1RF_A>;
impl<'a, REG> LVD1RF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage Monitor 1 reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1RF_A::_0)
    }
    ///Voltage Monitor 1 reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1RF_A::_1)
    }
}
/**Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2RF_A {
    ///0: Voltage Monitor 2 reset not detected.
    _0 = 0,
    ///1: Voltage Monitor 2 reset detected.
    _1 = 1,
}
impl From<LVD2RF_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2RF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `LVD2RF` reader - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type LVD2RF_R = crate::BitReader<LVD2RF_A>;
impl LVD2RF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2RF_A {
        match self.bits {
            false => LVD2RF_A::_0,
            true => LVD2RF_A::_1,
        }
    }
    ///Voltage Monitor 2 reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2RF_A::_0
    }
    ///Voltage Monitor 2 reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2RF_A::_1
    }
}
///Field `LVD2RF` writer - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
pub type LVD2RF_W<'a, REG> = crate::BitWriter0C<'a, REG, LVD2RF_A>;
impl<'a, REG> LVD2RF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage Monitor 2 reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2RF_A::_0)
    }
    ///Voltage Monitor 2 reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2RF_A::_1)
    }
}
impl R {
    ///Bit 0 - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn lvd0rf(&self) -> LVD0RF_R {
        LVD0RF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn lvd1rf(&self) -> LVD1RF_R {
        LVD1RF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn lvd2rf(&self) -> LVD2RF_R {
        LVD2RF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Power-On Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn porf(&mut self) -> PORF_W<RSTSR0_SPEC> {
        PORF_W::new(self, 0)
    }
    ///Bit 1 - Voltage Monitor 0 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn lvd0rf(&mut self) -> LVD0RF_W<RSTSR0_SPEC> {
        LVD0RF_W::new(self, 1)
    }
    ///Bit 2 - Voltage Monitor 1 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn lvd1rf(&mut self) -> LVD1RF_W<RSTSR0_SPEC> {
        LVD1RF_W::new(self, 2)
    }
    ///Bit 3 - Voltage Monitor 2 Reset Detect Flag Note: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1.
    #[inline(always)]
    pub fn lvd2rf(&mut self) -> LVD2RF_W<RSTSR0_SPEC> {
        LVD2RF_W::new(self, 3)
    }
}
/**Reset Status Register 0

You can [`read`](crate::Reg::read) this register and get [`rstsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSTSR0_SPEC;
impl crate::RegisterSpec for RSTSR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rstsr0::R`](R) reader structure
impl crate::Readable for RSTSR0_SPEC {}
///`write(|w| ..)` method takes [`rstsr0::W`](W) writer structure
impl crate::Writable for RSTSR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x0f;
}
///`reset()` method sets RSTSR0 to value 0
impl crate::Resettable for RSTSR0_SPEC {}
