///Register `DMAST` reader
pub type R = crate::R<DMAST_SPEC>;
///Register `DMAST` writer
pub type W = crate::W<DMAST_SPEC>;
/**DMAC Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMST_A {
    ///0: DMAC activation is disabled.
    _0 = 0,
    ///1: DMAC activation is enabled.
    _1 = 1,
}
impl From<DMST_A> for bool {
    #[inline(always)]
    fn from(variant: DMST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMST` reader - DMAC Operation Enable
pub type DMST_R = crate::BitReader<DMST_A>;
impl DMST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMST_A {
        match self.bits {
            false => DMST_A::_0,
            true => DMST_A::_1,
        }
    }
    ///DMAC activation is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMST_A::_0
    }
    ///DMAC activation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMST_A::_1
    }
}
///Field `DMST` writer - DMAC Operation Enable
pub type DMST_W<'a, REG> = crate::BitWriter<'a, REG, DMST_A>;
impl<'a, REG> DMST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAC activation is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DMST_A::_0)
    }
    ///DMAC activation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DMST_A::_1)
    }
}
impl R {
    ///Bit 0 - DMAC Operation Enable
    #[inline(always)]
    pub fn dmst(&self) -> DMST_R {
        DMST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMAC Operation Enable
    #[inline(always)]
    pub fn dmst(&mut self) -> DMST_W<DMAST_SPEC> {
        DMST_W::new(self, 0)
    }
}
/**DMA Module Activation Register

You can [`read`](crate::Reg::read) this register and get [`dmast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMAST_SPEC;
impl crate::RegisterSpec for DMAST_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dmast::R`](R) reader structure
impl crate::Readable for DMAST_SPEC {}
///`write(|w| ..)` method takes [`dmast::W`](W) writer structure
impl crate::Writable for DMAST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAST to value 0
impl crate::Resettable for DMAST_SPEC {}
