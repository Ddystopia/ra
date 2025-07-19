///Register `AMPC` reader
pub type R = crate::R<AMPC_SPEC>;
///Register `AMPC` writer
pub type W = crate::W<AMPC_SPEC>;
/**Operation control of operational amplifier(UNIT0)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE0_A {
    ///0: Operation amplifier is stopped.
    _0 = 0,
    ///1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled.
    _1 = 1,
}
impl From<AMPE0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPE0` reader - Operation control of operational amplifier(UNIT0)
pub type AMPE0_R = crate::BitReader<AMPE0_A>;
impl AMPE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPE0_A {
        match self.bits {
            false => AMPE0_A::_0,
            true => AMPE0_A::_1,
        }
    }
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE0_A::_0
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE0_A::_1
    }
}
///Field `AMPE0` writer - Operation control of operational amplifier(UNIT0)
pub type AMPE0_W<'a, REG> = crate::BitWriter<'a, REG, AMPE0_A>;
impl<'a, REG> AMPE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE0_A::_0)
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE0_A::_1)
    }
}
/**Operation control of operational amplifier(UNIT1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE1_A {
    ///0: Operation amplifier is stopped.
    _0 = 0,
    ///1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    _1 = 1,
}
impl From<AMPE1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPE1` reader - Operation control of operational amplifier(UNIT1)
pub type AMPE1_R = crate::BitReader<AMPE1_A>;
impl AMPE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPE1_A {
        match self.bits {
            false => AMPE1_A::_0,
            true => AMPE1_A::_1,
        }
    }
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE1_A::_0
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE1_A::_1
    }
}
///Field `AMPE1` writer - Operation control of operational amplifier(UNIT1)
pub type AMPE1_W<'a, REG> = crate::BitWriter<'a, REG, AMPE1_A>;
impl<'a, REG> AMPE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE1_A::_0)
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE1_A::_1)
    }
}
/**Operation control of operational amplifier(UNIT2)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE2_A {
    ///0: Operation amplifier is stopped.
    _0 = 0,
    ///1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    _1 = 1,
}
impl From<AMPE2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPE2` reader - Operation control of operational amplifier(UNIT2)
pub type AMPE2_R = crate::BitReader<AMPE2_A>;
impl AMPE2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPE2_A {
        match self.bits {
            false => AMPE2_A::_0,
            true => AMPE2_A::_1,
        }
    }
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE2_A::_0
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE2_A::_1
    }
}
///Field `AMPE2` writer - Operation control of operational amplifier(UNIT2)
pub type AMPE2_W<'a, REG> = crate::BitWriter<'a, REG, AMPE2_A>;
impl<'a, REG> AMPE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE2_A::_0)
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE2_A::_1)
    }
}
/**Operation control of operational amplifier(UNIT3)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE3_A {
    ///0: Operation amplifier is stopped.
    _0 = 0,
    ///1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    _1 = 1,
}
impl From<AMPE3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPE3` reader - Operation control of operational amplifier(UNIT3)
pub type AMPE3_R = crate::BitReader<AMPE3_A>;
impl AMPE3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPE3_A {
        match self.bits {
            false => AMPE3_A::_0,
            true => AMPE3_A::_1,
        }
    }
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE3_A::_0
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE3_A::_1
    }
}
///Field `AMPE3` writer - Operation control of operational amplifier(UNIT3)
pub type AMPE3_W<'a, REG> = crate::BitWriter<'a, REG, AMPE3_A>;
impl<'a, REG> AMPE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation amplifier is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE3_A::_0)
    }
    ///Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPE3_A::_1)
    }
}
/**Operation control of operational amplifier reference current circuit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREFE_A {
    ///0: Operational amplifier reference current circuit is stopped.
    _0 = 0,
    ///1: Operation of operational amplifier reference current circuit is enabled.
    _1 = 1,
}
impl From<IREFE_A> for bool {
    #[inline(always)]
    fn from(variant: IREFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IREFE` reader - Operation control of operational amplifier reference current circuit
pub type IREFE_R = crate::BitReader<IREFE_A>;
impl IREFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IREFE_A {
        match self.bits {
            false => IREFE_A::_0,
            true => IREFE_A::_1,
        }
    }
    ///Operational amplifier reference current circuit is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFE_A::_0
    }
    ///Operation of operational amplifier reference current circuit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFE_A::_1
    }
}
///Field `IREFE` writer - Operation control of operational amplifier reference current circuit
pub type IREFE_W<'a, REG> = crate::BitWriter<'a, REG, IREFE_A>;
impl<'a, REG> IREFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operational amplifier reference current circuit is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IREFE_A::_0)
    }
    ///Operation of operational amplifier reference current circuit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IREFE_A::_1)
    }
}
impl R {
    ///Bit 0 - Operation control of operational amplifier(UNIT0)
    #[inline(always)]
    pub fn ampe0(&self) -> AMPE0_R {
        AMPE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation control of operational amplifier(UNIT1)
    #[inline(always)]
    pub fn ampe1(&self) -> AMPE1_R {
        AMPE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Operation control of operational amplifier(UNIT2)
    #[inline(always)]
    pub fn ampe2(&self) -> AMPE2_R {
        AMPE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Operation control of operational amplifier(UNIT3)
    #[inline(always)]
    pub fn ampe3(&self) -> AMPE3_R {
        AMPE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Operation control of operational amplifier reference current circuit
    #[inline(always)]
    pub fn irefe(&self) -> IREFE_R {
        IREFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operation control of operational amplifier(UNIT0)
    #[inline(always)]
    pub fn ampe0(&mut self) -> AMPE0_W<'_, AMPC_SPEC> {
        AMPE0_W::new(self, 0)
    }
    ///Bit 1 - Operation control of operational amplifier(UNIT1)
    #[inline(always)]
    pub fn ampe1(&mut self) -> AMPE1_W<'_, AMPC_SPEC> {
        AMPE1_W::new(self, 1)
    }
    ///Bit 2 - Operation control of operational amplifier(UNIT2)
    #[inline(always)]
    pub fn ampe2(&mut self) -> AMPE2_W<'_, AMPC_SPEC> {
        AMPE2_W::new(self, 2)
    }
    ///Bit 3 - Operation control of operational amplifier(UNIT3)
    #[inline(always)]
    pub fn ampe3(&mut self) -> AMPE3_W<'_, AMPC_SPEC> {
        AMPE3_W::new(self, 3)
    }
    ///Bit 7 - Operation control of operational amplifier reference current circuit
    #[inline(always)]
    pub fn irefe(&mut self) -> IREFE_W<'_, AMPC_SPEC> {
        IREFE_W::new(self, 7)
    }
}
/**Operational amplifier control register

You can [`read`](crate::Reg::read) this register and get [`ampc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AMPC_SPEC;
impl crate::RegisterSpec for AMPC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ampc::R`](R) reader structure
impl crate::Readable for AMPC_SPEC {}
///`write(|w| ..)` method takes [`ampc::W`](W) writer structure
impl crate::Writable for AMPC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AMPC to value 0
impl crate::Resettable for AMPC_SPEC {}
