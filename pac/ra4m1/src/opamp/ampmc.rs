///Register `AMPMC` reader
pub type R = crate::R<AMPMC_SPEC>;
///Register `AMPMC` writer
pub type W = crate::W<AMPMC_SPEC>;
/**Operational amplifier precharge control status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC0_A {
    ///0: Precharging is stopped.
    _0 = 0,
    ///1: Precharging is enabled.
    _1 = 1,
}
impl From<AMPPC0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPPC0` reader - Operational amplifier precharge control status
pub type AMPPC0_R = crate::BitReader<AMPPC0_A>;
impl AMPPC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPPC0_A {
        match self.bits {
            false => AMPPC0_A::_0,
            true => AMPPC0_A::_1,
        }
    }
    ///Precharging is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC0_A::_0
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC0_A::_1
    }
}
///Field `AMPPC0` writer - Operational amplifier precharge control status
pub type AMPPC0_W<'a, REG> = crate::BitWriter<'a, REG, AMPPC0_A>;
impl<'a, REG> AMPPC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharging is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC0_A::_0)
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC0_A::_1)
    }
}
/**Operational amplifier precharge control status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC1_A {
    ///0: Precharging is stopped.
    _0 = 0,
    ///1: Precharging is enabled.
    _1 = 1,
}
impl From<AMPPC1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPPC1` reader - Operational amplifier precharge control status
pub type AMPPC1_R = crate::BitReader<AMPPC1_A>;
impl AMPPC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPPC1_A {
        match self.bits {
            false => AMPPC1_A::_0,
            true => AMPPC1_A::_1,
        }
    }
    ///Precharging is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC1_A::_0
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC1_A::_1
    }
}
///Field `AMPPC1` writer - Operational amplifier precharge control status
pub type AMPPC1_W<'a, REG> = crate::BitWriter<'a, REG, AMPPC1_A>;
impl<'a, REG> AMPPC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharging is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC1_A::_0)
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC1_A::_1)
    }
}
/**Operational amplifier precharge control status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC2_A {
    ///0: Precharging is stopped.
    _0 = 0,
    ///1: Precharging is enabled.
    _1 = 1,
}
impl From<AMPPC2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPPC2` reader - Operational amplifier precharge control status
pub type AMPPC2_R = crate::BitReader<AMPPC2_A>;
impl AMPPC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPPC2_A {
        match self.bits {
            false => AMPPC2_A::_0,
            true => AMPPC2_A::_1,
        }
    }
    ///Precharging is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC2_A::_0
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC2_A::_1
    }
}
///Field `AMPPC2` writer - Operational amplifier precharge control status
pub type AMPPC2_W<'a, REG> = crate::BitWriter<'a, REG, AMPPC2_A>;
impl<'a, REG> AMPPC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharging is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC2_A::_0)
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC2_A::_1)
    }
}
/**Operational amplifier precharge control status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPPC3_A {
    ///0: Precharging is stopped.
    _0 = 0,
    ///1: Precharging is enabled.
    _1 = 1,
}
impl From<AMPPC3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPPC3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPPC3` reader - Operational amplifier precharge control status
pub type AMPPC3_R = crate::BitReader<AMPPC3_A>;
impl AMPPC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPPC3_A {
        match self.bits {
            false => AMPPC3_A::_0,
            true => AMPPC3_A::_1,
        }
    }
    ///Precharging is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPPC3_A::_0
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPPC3_A::_1
    }
}
///Field `AMPPC3` writer - Operational amplifier precharge control status
pub type AMPPC3_W<'a, REG> = crate::BitWriter<'a, REG, AMPPC3_A>;
impl<'a, REG> AMPPC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharging is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC3_A::_0)
    }
    ///Precharging is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPPC3_A::_1)
    }
}
/**Operation mode selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPSP_A {
    ///0: Low-power mode (low-speed).
    _0 = 0,
    ///1: High-speed mode.
    _1 = 1,
}
impl From<AMPSP_A> for bool {
    #[inline(always)]
    fn from(variant: AMPSP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPSP` reader - Operation mode selection
pub type AMPSP_R = crate::BitReader<AMPSP_A>;
impl AMPSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPSP_A {
        match self.bits {
            false => AMPSP_A::_0,
            true => AMPSP_A::_1,
        }
    }
    ///Low-power mode (low-speed).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPSP_A::_0
    }
    ///High-speed mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPSP_A::_1
    }
}
///Field `AMPSP` writer - Operation mode selection
pub type AMPSP_W<'a, REG> = crate::BitWriter<'a, REG, AMPSP_A>;
impl<'a, REG> AMPSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low-power mode (low-speed).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPSP_A::_0)
    }
    ///High-speed mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPSP_A::_1)
    }
}
impl R {
    ///Bit 0 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc0(&self) -> AMPPC0_R {
        AMPPC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc1(&self) -> AMPPC1_R {
        AMPPC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc2(&self) -> AMPPC2_R {
        AMPPC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc3(&self) -> AMPPC3_R {
        AMPPC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Operation mode selection
    #[inline(always)]
    pub fn ampsp(&self) -> AMPSP_R {
        AMPSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc0(&mut self) -> AMPPC0_W<AMPMC_SPEC> {
        AMPPC0_W::new(self, 0)
    }
    ///Bit 1 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc1(&mut self) -> AMPPC1_W<AMPMC_SPEC> {
        AMPPC1_W::new(self, 1)
    }
    ///Bit 2 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc2(&mut self) -> AMPPC2_W<AMPMC_SPEC> {
        AMPPC2_W::new(self, 2)
    }
    ///Bit 3 - Operational amplifier precharge control status
    #[inline(always)]
    pub fn amppc3(&mut self) -> AMPPC3_W<AMPMC_SPEC> {
        AMPPC3_W::new(self, 3)
    }
    ///Bit 7 - Operation mode selection
    #[inline(always)]
    pub fn ampsp(&mut self) -> AMPSP_W<AMPMC_SPEC> {
        AMPSP_W::new(self, 7)
    }
}
/**Operational amplifier mode control register

You can [`read`](crate::Reg::read) this register and get [`ampmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AMPMC_SPEC;
impl crate::RegisterSpec for AMPMC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ampmc::R`](R) reader structure
impl crate::Readable for AMPMC_SPEC {}
///`write(|w| ..)` method takes [`ampmc::W`](W) writer structure
impl crate::Writable for AMPMC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AMPMC to value 0
impl crate::Resettable for AMPMC_SPEC {}
