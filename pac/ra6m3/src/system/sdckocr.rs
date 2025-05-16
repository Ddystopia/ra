///Register `SDCKOCR` reader
pub type R = crate::R<SDCKOCR_SPEC>;
///Register `SDCKOCR` writer
pub type W = crate::W<SDCKOCR_SPEC>;
/**SDCLK Pin Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCKOEN_A {
    ///0: Disable SDCLK pin output (fixed high)
    _0 = 0,
    ///1: Enable SDCLK pin output
    _1 = 1,
}
impl From<SDCKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCKOEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCKOEN` reader - SDCLK Pin Output Control
pub type SDCKOEN_R = crate::BitReader<SDCKOEN_A>;
impl SDCKOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDCKOEN_A {
        match self.bits {
            false => SDCKOEN_A::_0,
            true => SDCKOEN_A::_1,
        }
    }
    ///Disable SDCLK pin output (fixed high)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCKOEN_A::_0
    }
    ///Enable SDCLK pin output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCKOEN_A::_1
    }
}
///Field `SDCKOEN` writer - SDCLK Pin Output Control
pub type SDCKOEN_W<'a, REG> = crate::BitWriter<'a, REG, SDCKOEN_A>;
impl<'a, REG> SDCKOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SDCLK pin output (fixed high)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDCKOEN_A::_0)
    }
    ///Enable SDCLK pin output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCKOEN_A::_1)
    }
}
impl R {
    ///Bit 0 - SDCLK Pin Output Control
    #[inline(always)]
    pub fn sdckoen(&self) -> SDCKOEN_R {
        SDCKOEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SDCLK Pin Output Control
    #[inline(always)]
    pub fn sdckoen(&mut self) -> SDCKOEN_W<SDCKOCR_SPEC> {
        SDCKOEN_W::new(self, 0)
    }
}
/**SDRAM Clock Output Control Register

You can [`read`](crate::Reg::read) this register and get [`sdckocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdckocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDCKOCR_SPEC;
impl crate::RegisterSpec for SDCKOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdckocr::R`](R) reader structure
impl crate::Readable for SDCKOCR_SPEC {}
///`write(|w| ..)` method takes [`sdckocr::W`](W) writer structure
impl crate::Writable for SDCKOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDCKOCR to value 0
impl crate::Resettable for SDCKOCR_SPEC {}
