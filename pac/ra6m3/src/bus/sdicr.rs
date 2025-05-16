///Register `SDICR` reader
pub type R = crate::R<SDICR_SPEC>;
///Register `SDICR` writer
pub type W = crate::W<SDICR_SPEC>;
/**Initialization Sequence Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIRQ_A {
    ///0: Invalid
    _0 = 0,
    ///1: Initialization sequence starts
    _1 = 1,
}
impl From<INIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: INIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INIRQ` reader - Initialization Sequence Start
pub type INIRQ_R = crate::BitReader<INIRQ_A>;
impl INIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INIRQ_A {
        match self.bits {
            false => INIRQ_A::_0,
            true => INIRQ_A::_1,
        }
    }
    ///Invalid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIRQ_A::_0
    }
    ///Initialization sequence starts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIRQ_A::_1
    }
}
///Field `INIRQ` writer - Initialization Sequence Start
pub type INIRQ_W<'a, REG> = crate::BitWriter<'a, REG, INIRQ_A>;
impl<'a, REG> INIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INIRQ_A::_0)
    }
    ///Initialization sequence starts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INIRQ_A::_1)
    }
}
impl R {
    ///Bit 0 - Initialization Sequence Start
    #[inline(always)]
    pub fn inirq(&self) -> INIRQ_R {
        INIRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Initialization Sequence Start
    #[inline(always)]
    pub fn inirq(&mut self) -> INIRQ_W<SDICR_SPEC> {
        INIRQ_W::new(self, 0)
    }
}
/**SDRAM Initialization Sequence Control Register

You can [`read`](crate::Reg::read) this register and get [`sdicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDICR_SPEC;
impl crate::RegisterSpec for SDICR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdicr::R`](R) reader structure
impl crate::Readable for SDICR_SPEC {}
///`write(|w| ..)` method takes [`sdicr::W`](W) writer structure
impl crate::Writable for SDICR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDICR to value 0
impl crate::Resettable for SDICR_SPEC {}
