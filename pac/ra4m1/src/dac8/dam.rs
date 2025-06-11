///Register `DAM` reader
pub type R = crate::R<DAM_SPEC>;
///Register `DAM` writer
pub type W = crate::W<DAM_SPEC>;
/**D/A Operation Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACE0_A {
    ///0: D/A conversion disabled for channel 0
    _0 = 0,
    ///1: D/A conversion enabled for channel 0.
    _1 = 1,
}
impl From<DACE0_A> for bool {
    #[inline(always)]
    fn from(variant: DACE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DACE0` reader - D/A Operation Enable 0
pub type DACE0_R = crate::BitReader<DACE0_A>;
impl DACE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DACE0_A {
        match self.bits {
            false => DACE0_A::_0,
            true => DACE0_A::_1,
        }
    }
    ///D/A conversion disabled for channel 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACE0_A::_0
    }
    ///D/A conversion enabled for channel 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACE0_A::_1
    }
}
///Field `DACE0` writer - D/A Operation Enable 0
pub type DACE0_W<'a, REG> = crate::BitWriter<'a, REG, DACE0_A>;
impl<'a, REG> DACE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///D/A conversion disabled for channel 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DACE0_A::_0)
    }
    ///D/A conversion enabled for channel 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DACE0_A::_1)
    }
}
/**D/A Operation Enable 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACE1_A {
    ///0: D/A conversion disabled for channel 1
    _0 = 0,
    ///1: D/A conversion enabled for channel 1
    _1 = 1,
}
impl From<DACE1_A> for bool {
    #[inline(always)]
    fn from(variant: DACE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DACE1` reader - D/A Operation Enable 1
pub type DACE1_R = crate::BitReader<DACE1_A>;
impl DACE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DACE1_A {
        match self.bits {
            false => DACE1_A::_0,
            true => DACE1_A::_1,
        }
    }
    ///D/A conversion disabled for channel 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACE1_A::_0
    }
    ///D/A conversion enabled for channel 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACE1_A::_1
    }
}
///Field `DACE1` writer - D/A Operation Enable 1
pub type DACE1_W<'a, REG> = crate::BitWriter<'a, REG, DACE1_A>;
impl<'a, REG> DACE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///D/A conversion disabled for channel 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DACE1_A::_0)
    }
    ///D/A conversion enabled for channel 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DACE1_A::_1)
    }
}
impl R {
    ///Bit 4 - D/A Operation Enable 0
    #[inline(always)]
    pub fn dace0(&self) -> DACE0_R {
        DACE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - D/A Operation Enable 1
    #[inline(always)]
    pub fn dace1(&self) -> DACE1_R {
        DACE1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - D/A Operation Enable 0
    #[inline(always)]
    pub fn dace0(&mut self) -> DACE0_W<DAM_SPEC> {
        DACE0_W::new(self, 4)
    }
    ///Bit 5 - D/A Operation Enable 1
    #[inline(always)]
    pub fn dace1(&mut self) -> DACE1_W<DAM_SPEC> {
        DACE1_W::new(self, 5)
    }
}
/**D/A Converter Mode Register

You can [`read`](crate::Reg::read) this register and get [`dam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAM_SPEC;
impl crate::RegisterSpec for DAM_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dam::R`](R) reader structure
impl crate::Readable for DAM_SPEC {}
///`write(|w| ..)` method takes [`dam::W`](W) writer structure
impl crate::Writable for DAM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAM to value 0
impl crate::Resettable for DAM_SPEC {}
