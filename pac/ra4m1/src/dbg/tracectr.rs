///Register `TRACECTR` reader
pub type R = crate::R<TRACECTR_SPEC>;
///Register `TRACECTR` writer
pub type W = crate::W<TRACECTR_SPEC>;
/**Enable bit for halt request by ETB full

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENETBFULL_A {
    ///0: ETB full does not cause CPU halt
    _0 = 0,
    ///1: ETB full cause CPU halt
    _1 = 1,
}
impl From<ENETBFULL_A> for bool {
    #[inline(always)]
    fn from(variant: ENETBFULL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENETBFULL` reader - Enable bit for halt request by ETB full
pub type ENETBFULL_R = crate::BitReader<ENETBFULL_A>;
impl ENETBFULL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENETBFULL_A {
        match self.bits {
            false => ENETBFULL_A::_0,
            true => ENETBFULL_A::_1,
        }
    }
    ///ETB full does not cause CPU halt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENETBFULL_A::_0
    }
    ///ETB full cause CPU halt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENETBFULL_A::_1
    }
}
///Field `ENETBFULL` writer - Enable bit for halt request by ETB full
pub type ENETBFULL_W<'a, REG> = crate::BitWriter<'a, REG, ENETBFULL_A>;
impl<'a, REG> ENETBFULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ETB full does not cause CPU halt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENETBFULL_A::_0)
    }
    ///ETB full cause CPU halt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENETBFULL_A::_1)
    }
}
impl R {
    ///Bit 31 - Enable bit for halt request by ETB full
    #[inline(always)]
    pub fn enetbfull(&self) -> ENETBFULL_R {
        ENETBFULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 31 - Enable bit for halt request by ETB full
    #[inline(always)]
    pub fn enetbfull(&mut self) -> ENETBFULL_W<TRACECTR_SPEC> {
        ENETBFULL_W::new(self, 31)
    }
}
/**Trace Control Register

You can [`read`](crate::Reg::read) this register and get [`tracectr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tracectr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TRACECTR_SPEC;
impl crate::RegisterSpec for TRACECTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tracectr::R`](R) reader structure
impl crate::Readable for TRACECTR_SPEC {}
///`write(|w| ..)` method takes [`tracectr::W`](W) writer structure
impl crate::Writable for TRACECTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRACECTR to value 0
impl crate::Resettable for TRACECTR_SPEC {}
