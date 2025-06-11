///Register `AMPMON` reader
pub type R = crate::R<AMPMON_SPEC>;
/**Operational amplifier status(UNIT0)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON0_A {
    ///0: Operational amplifier 0 is stopped.
    _0 = 0,
    ///1: Operational amplifier 0 is operating.
    _1 = 1,
}
impl From<AMPMON0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPMON0` reader - Operational amplifier status(UNIT0)
pub type AMPMON0_R = crate::BitReader<AMPMON0_A>;
impl AMPMON0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPMON0_A {
        match self.bits {
            false => AMPMON0_A::_0,
            true => AMPMON0_A::_1,
        }
    }
    ///Operational amplifier 0 is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON0_A::_0
    }
    ///Operational amplifier 0 is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON0_A::_1
    }
}
/**Operational amplifier status(UNIT1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON1_A {
    ///0: Operational amplifier 1 is stopped.
    _0 = 0,
    ///1: Operational amplifier 1 is operating.
    _1 = 1,
}
impl From<AMPMON1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPMON1` reader - Operational amplifier status(UNIT1)
pub type AMPMON1_R = crate::BitReader<AMPMON1_A>;
impl AMPMON1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPMON1_A {
        match self.bits {
            false => AMPMON1_A::_0,
            true => AMPMON1_A::_1,
        }
    }
    ///Operational amplifier 1 is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON1_A::_0
    }
    ///Operational amplifier 1 is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON1_A::_1
    }
}
/**Operational amplifier status(UNIT2)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON2_A {
    ///0: Operational amplifier 2 is stopped.
    _0 = 0,
    ///1: Operational amplifier 2 is operating.
    _1 = 1,
}
impl From<AMPMON2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPMON2` reader - Operational amplifier status(UNIT2)
pub type AMPMON2_R = crate::BitReader<AMPMON2_A>;
impl AMPMON2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPMON2_A {
        match self.bits {
            false => AMPMON2_A::_0,
            true => AMPMON2_A::_1,
        }
    }
    ///Operational amplifier 2 is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON2_A::_0
    }
    ///Operational amplifier 2 is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON2_A::_1
    }
}
/**Operational amplifier status(UNIT3)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPMON3_A {
    ///0: Operational amplifier 3 is stopped.
    _0 = 0,
    ///1: Operational amplifier 3 is operating.
    _1 = 1,
}
impl From<AMPMON3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPMON3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPMON3` reader - Operational amplifier status(UNIT3)
pub type AMPMON3_R = crate::BitReader<AMPMON3_A>;
impl AMPMON3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPMON3_A {
        match self.bits {
            false => AMPMON3_A::_0,
            true => AMPMON3_A::_1,
        }
    }
    ///Operational amplifier 3 is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPMON3_A::_0
    }
    ///Operational amplifier 3 is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPMON3_A::_1
    }
}
impl R {
    ///Bit 0 - Operational amplifier status(UNIT0)
    #[inline(always)]
    pub fn ampmon0(&self) -> AMPMON0_R {
        AMPMON0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operational amplifier status(UNIT1)
    #[inline(always)]
    pub fn ampmon1(&self) -> AMPMON1_R {
        AMPMON1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Operational amplifier status(UNIT2)
    #[inline(always)]
    pub fn ampmon2(&self) -> AMPMON2_R {
        AMPMON2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Operational amplifier status(UNIT3)
    #[inline(always)]
    pub fn ampmon3(&self) -> AMPMON3_R {
        AMPMON3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
/**Operational amplifier monitor register

You can [`read`](crate::Reg::read) this register and get [`ampmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AMPMON_SPEC;
impl crate::RegisterSpec for AMPMON_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ampmon::R`](R) reader structure
impl crate::Readable for AMPMON_SPEC {}
///`reset()` method sets AMPMON to value 0
impl crate::Resettable for AMPMON_SPEC {}
