///Register `EBCKOCR` reader
pub type R = crate::R<EBCKOCR_SPEC>;
///Register `EBCKOCR` writer
pub type W = crate::W<EBCKOCR_SPEC>;
/**BCLK Pin Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBCKOEN_A {
    ///0: Disable EBCLK pin output (fixed high)
    _0 = 0,
    ///1: Enable EBCLK pin output
    _1 = 1,
}
impl From<EBCKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: EBCKOEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EBCKOEN` reader - BCLK Pin Output Control
pub type EBCKOEN_R = crate::BitReader<EBCKOEN_A>;
impl EBCKOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EBCKOEN_A {
        match self.bits {
            false => EBCKOEN_A::_0,
            true => EBCKOEN_A::_1,
        }
    }
    ///Disable EBCLK pin output (fixed high)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EBCKOEN_A::_0
    }
    ///Enable EBCLK pin output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EBCKOEN_A::_1
    }
}
///Field `EBCKOEN` writer - BCLK Pin Output Control
pub type EBCKOEN_W<'a, REG> = crate::BitWriter<'a, REG, EBCKOEN_A>;
impl<'a, REG> EBCKOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable EBCLK pin output (fixed high)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EBCKOEN_A::_0)
    }
    ///Enable EBCLK pin output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EBCKOEN_A::_1)
    }
}
impl R {
    ///Bit 0 - BCLK Pin Output Control
    #[inline(always)]
    pub fn ebckoen(&self) -> EBCKOEN_R {
        EBCKOEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BCLK Pin Output Control
    #[inline(always)]
    pub fn ebckoen(&mut self) -> EBCKOEN_W<EBCKOCR_SPEC> {
        EBCKOEN_W::new(self, 0)
    }
}
/**External Bus Clock Output Control Register

You can [`read`](crate::Reg::read) this register and get [`ebckocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebckocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EBCKOCR_SPEC;
impl crate::RegisterSpec for EBCKOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ebckocr::R`](R) reader structure
impl crate::Readable for EBCKOCR_SPEC {}
///`write(|w| ..)` method takes [`ebckocr::W`](W) writer structure
impl crate::Writable for EBCKOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EBCKOCR to value 0
impl crate::Resettable for EBCKOCR_SPEC {}
