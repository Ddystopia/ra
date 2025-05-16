///Register `CTSUCR0` reader
pub type R = crate::R<CTSUCR0_SPEC>;
///Register `CTSUCR0` writer
pub type W = crate::W<CTSUCR0_SPEC>;
/**CTSU Measurement Operation Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSTRT_A {
    ///0: Measurement operation stops.
    _0 = 0,
    ///1: Measurement operation starts.
    _1 = 1,
}
impl From<CTSUSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUSTRT` reader - CTSU Measurement Operation Start
pub type CTSUSTRT_R = crate::BitReader<CTSUSTRT_A>;
impl CTSUSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSTRT_A {
        match self.bits {
            false => CTSUSTRT_A::_0,
            true => CTSUSTRT_A::_1,
        }
    }
    ///Measurement operation stops.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSTRT_A::_0
    }
    ///Measurement operation starts.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSTRT_A::_1
    }
}
///Field `CTSUSTRT` writer - CTSU Measurement Operation Start
pub type CTSUSTRT_W<'a, REG> = crate::BitWriter<'a, REG, CTSUSTRT_A>;
impl<'a, REG> CTSUSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Measurement operation stops.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSTRT_A::_0)
    }
    ///Measurement operation starts.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSTRT_A::_1)
    }
}
/**CTSU Measurement Operation Start Trigger Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUCAP_A {
    ///0: Software trigger.
    _0 = 0,
    ///1: External trigger.
    _1 = 1,
}
impl From<CTSUCAP_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUCAP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUCAP` reader - CTSU Measurement Operation Start Trigger Select
pub type CTSUCAP_R = crate::BitReader<CTSUCAP_A>;
impl CTSUCAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCAP_A {
        match self.bits {
            false => CTSUCAP_A::_0,
            true => CTSUCAP_A::_1,
        }
    }
    ///Software trigger.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCAP_A::_0
    }
    ///External trigger.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCAP_A::_1
    }
}
///Field `CTSUCAP` writer - CTSU Measurement Operation Start Trigger Select
pub type CTSUCAP_W<'a, REG> = crate::BitWriter<'a, REG, CTSUCAP_A>;
impl<'a, REG> CTSUCAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCAP_A::_0)
    }
    ///External trigger.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCAP_A::_1)
    }
}
/**CTSU Wait State Power-Saving Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSNZ_A {
    ///0: Power-saving function during wait state is disabled.
    _0 = 0,
    ///1: Power-saving function during wait state is enabled.
    _1 = 1,
}
impl From<CTSUSNZ_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSNZ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUSNZ` reader - CTSU Wait State Power-Saving Enable
pub type CTSUSNZ_R = crate::BitReader<CTSUSNZ_A>;
impl CTSUSNZ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSNZ_A {
        match self.bits {
            false => CTSUSNZ_A::_0,
            true => CTSUSNZ_A::_1,
        }
    }
    ///Power-saving function during wait state is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSNZ_A::_0
    }
    ///Power-saving function during wait state is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSNZ_A::_1
    }
}
///Field `CTSUSNZ` writer - CTSU Wait State Power-Saving Enable
pub type CTSUSNZ_W<'a, REG> = crate::BitWriter<'a, REG, CTSUSNZ_A>;
impl<'a, REG> CTSUSNZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Power-saving function during wait state is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSNZ_A::_0)
    }
    ///Power-saving function during wait state is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSNZ_A::_1)
    }
}
/**CTSU Control Block Initialization

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUINIT_A {
    ///0: Writing a 0 has no effect, this bit is read as 0.
    _0 = 0,
    ///1: initializes the CTSU control block and registers.
    _1 = 1,
}
impl From<CTSUINIT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUINIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUINIT` reader - CTSU Control Block Initialization
pub type CTSUINIT_R = crate::BitReader<CTSUINIT_A>;
impl CTSUINIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUINIT_A {
        match self.bits {
            false => CTSUINIT_A::_0,
            true => CTSUINIT_A::_1,
        }
    }
    ///Writing a 0 has no effect, this bit is read as 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUINIT_A::_0
    }
    ///initializes the CTSU control block and registers.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUINIT_A::_1
    }
}
///Field `CTSUINIT` writer - CTSU Control Block Initialization
pub type CTSUINIT_W<'a, REG> = crate::BitWriter<'a, REG, CTSUINIT_A>;
impl<'a, REG> CTSUINIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing a 0 has no effect, this bit is read as 0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUINIT_A::_0)
    }
    ///initializes the CTSU control block and registers.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUINIT_A::_1)
    }
}
/**CTSU Transmission power supply selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUTXVSEL_A {
    ///0: Select Vcc
    _0 = 0,
    ///1: Select internal logic power supply
    _1 = 1,
}
impl From<CTSUTXVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUTXVSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUTXVSEL` reader - CTSU Transmission power supply selection
pub type CTSUTXVSEL_R = crate::BitReader<CTSUTXVSEL_A>;
impl CTSUTXVSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUTXVSEL_A {
        match self.bits {
            false => CTSUTXVSEL_A::_0,
            true => CTSUTXVSEL_A::_1,
        }
    }
    ///Select Vcc
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUTXVSEL_A::_0
    }
    ///Select internal logic power supply
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUTXVSEL_A::_1
    }
}
///Field `CTSUTXVSEL` writer - CTSU Transmission power supply selection
pub type CTSUTXVSEL_W<'a, REG> = crate::BitWriter<'a, REG, CTSUTXVSEL_A>;
impl<'a, REG> CTSUTXVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select Vcc
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUTXVSEL_A::_0)
    }
    ///Select internal logic power supply
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUTXVSEL_A::_1)
    }
}
impl R {
    ///Bit 0 - CTSU Measurement Operation Start
    #[inline(always)]
    pub fn ctsustrt(&self) -> CTSUSTRT_R {
        CTSUSTRT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTSU Measurement Operation Start Trigger Select
    #[inline(always)]
    pub fn ctsucap(&self) -> CTSUCAP_R {
        CTSUCAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTSU Wait State Power-Saving Enable
    #[inline(always)]
    pub fn ctsusnz(&self) -> CTSUSNZ_R {
        CTSUSNZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CTSU Control Block Initialization
    #[inline(always)]
    pub fn ctsuinit(&self) -> CTSUINIT_R {
        CTSUINIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - CTSU Transmission power supply selection
    #[inline(always)]
    pub fn ctsutxvsel(&self) -> CTSUTXVSEL_R {
        CTSUTXVSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CTSU Measurement Operation Start
    #[inline(always)]
    pub fn ctsustrt(&mut self) -> CTSUSTRT_W<CTSUCR0_SPEC> {
        CTSUSTRT_W::new(self, 0)
    }
    ///Bit 1 - CTSU Measurement Operation Start Trigger Select
    #[inline(always)]
    pub fn ctsucap(&mut self) -> CTSUCAP_W<CTSUCR0_SPEC> {
        CTSUCAP_W::new(self, 1)
    }
    ///Bit 2 - CTSU Wait State Power-Saving Enable
    #[inline(always)]
    pub fn ctsusnz(&mut self) -> CTSUSNZ_W<CTSUCR0_SPEC> {
        CTSUSNZ_W::new(self, 2)
    }
    ///Bit 4 - CTSU Control Block Initialization
    #[inline(always)]
    pub fn ctsuinit(&mut self) -> CTSUINIT_W<CTSUCR0_SPEC> {
        CTSUINIT_W::new(self, 4)
    }
    ///Bit 7 - CTSU Transmission power supply selection
    #[inline(always)]
    pub fn ctsutxvsel(&mut self) -> CTSUTXVSEL_W<CTSUCR0_SPEC> {
        CTSUTXVSEL_W::new(self, 7)
    }
}
/**CTSU Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsucr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCR0_SPEC;
impl crate::RegisterSpec for CTSUCR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsucr0::R`](R) reader structure
impl crate::Readable for CTSUCR0_SPEC {}
///`write(|w| ..)` method takes [`ctsucr0::W`](W) writer structure
impl crate::Writable for CTSUCR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCR0 to value 0
impl crate::Resettable for CTSUCR0_SPEC {}
