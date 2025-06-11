///Register `FCACHEE` reader
pub type R = crate::R<FCACHEE_SPEC>;
///Register `FCACHEE` writer
pub type W = crate::W<FCACHEE_SPEC>;
/**FCACHE Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCACHEEN_A {
    ///0: FCACHE is disabled
    _0 = 0,
    ///1: FCACHE is enabled
    _1 = 1,
}
impl From<FCACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: FCACHEEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FCACHEEN` reader - FCACHE Enable
pub type FCACHEEN_R = crate::BitReader<FCACHEEN_A>;
impl FCACHEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FCACHEEN_A {
        match self.bits {
            false => FCACHEEN_A::_0,
            true => FCACHEEN_A::_1,
        }
    }
    ///FCACHE is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCACHEEN_A::_0
    }
    ///FCACHE is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCACHEEN_A::_1
    }
}
///Field `FCACHEEN` writer - FCACHE Enable
pub type FCACHEEN_W<'a, REG> = crate::BitWriter<'a, REG, FCACHEEN_A>;
impl<'a, REG> FCACHEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FCACHE is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEEN_A::_0)
    }
    ///FCACHE is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEEN_A::_1)
    }
}
impl R {
    ///Bit 0 - FCACHE Enable
    #[inline(always)]
    pub fn fcacheen(&self) -> FCACHEEN_R {
        FCACHEEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FCACHE Enable
    #[inline(always)]
    pub fn fcacheen(&mut self) -> FCACHEEN_W<FCACHEE_SPEC> {
        FCACHEEN_W::new(self, 0)
    }
}
/**Flash Cache Enable Register

You can [`read`](crate::Reg::read) this register and get [`fcachee::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcachee::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCACHEE_SPEC;
impl crate::RegisterSpec for FCACHEE_SPEC {
    type Ux = u16;
}
///`read()` method returns [`fcachee::R`](R) reader structure
impl crate::Readable for FCACHEE_SPEC {}
///`write(|w| ..)` method takes [`fcachee::W`](W) writer structure
impl crate::Writable for FCACHEE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCACHEE to value 0
impl crate::Resettable for FCACHEE_SPEC {}
