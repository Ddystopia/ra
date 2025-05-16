///Register `BUSMCNTDMA` reader
pub type R = crate::R<BUSMCNTDMA_SPEC>;
///Register `BUSMCNTDMA` writer
pub type W = crate::W<BUSMCNTDMA_SPEC>;
/**Ignore Error Responses

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERES_A {
    ///0: Bus error will be reported.
    _0 = 0,
    ///1: Bus error will not be reported.
    _1 = 1,
}
impl From<IERES_A> for bool {
    #[inline(always)]
    fn from(variant: IERES_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IERES` reader - Ignore Error Responses
pub type IERES_R = crate::BitReader<IERES_A>;
impl IERES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IERES_A {
        match self.bits {
            false => IERES_A::_0,
            true => IERES_A::_1,
        }
    }
    ///Bus error will be reported.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IERES_A::_0
    }
    ///Bus error will not be reported.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IERES_A::_1
    }
}
///Field `IERES` writer - Ignore Error Responses
pub type IERES_W<'a, REG> = crate::BitWriter<'a, REG, IERES_A>;
impl<'a, REG> IERES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus error will be reported.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IERES_A::_0)
    }
    ///Bus error will not be reported.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IERES_A::_1)
    }
}
impl R {
    ///Bit 15 - Ignore Error Responses
    #[inline(always)]
    pub fn ieres(&self) -> IERES_R {
        IERES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - Ignore Error Responses
    #[inline(always)]
    pub fn ieres(&mut self) -> IERES_W<BUSMCNTDMA_SPEC> {
        IERES_W::new(self, 15)
    }
}
/**Master Bus Control Register DMA

You can [`read`](crate::Reg::read) this register and get [`busmcntdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUSMCNTDMA_SPEC;
impl crate::RegisterSpec for BUSMCNTDMA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`busmcntdma::R`](R) reader structure
impl crate::Readable for BUSMCNTDMA_SPEC {}
///`write(|w| ..)` method takes [`busmcntdma::W`](W) writer structure
impl crate::Writable for BUSMCNTDMA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSMCNTDMA to value 0
impl crate::Resettable for BUSMCNTDMA_SPEC {}
