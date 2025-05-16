///Register `FLLCR2` reader
pub type R = crate::R<FLLCR2_SPEC>;
///Register `FLLCR2` writer
pub type W = crate::W<FLLCR2_SPEC>;
///Field `FLLCNTL` reader - FLL Multiplication ControlMultiplication ratio of the FLL reference clock select
pub type FLLCNTL_R = crate::FieldReader<u16>;
///Field `FLLCNTL` writer - FLL Multiplication ControlMultiplication ratio of the FLL reference clock select
pub type FLLCNTL_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - FLL Multiplication ControlMultiplication ratio of the FLL reference clock select
    #[inline(always)]
    pub fn fllcntl(&self) -> FLLCNTL_R {
        FLLCNTL_R::new(self.bits & 0x07ff)
    }
}
impl W {
    ///Bits 0:10 - FLL Multiplication ControlMultiplication ratio of the FLL reference clock select
    #[inline(always)]
    pub fn fllcntl(&mut self) -> FLLCNTL_W<FLLCR2_SPEC> {
        FLLCNTL_W::new(self, 0)
    }
}
/**FLL Control Register 2

You can [`read`](crate::Reg::read) this register and get [`fllcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FLLCR2_SPEC;
impl crate::RegisterSpec for FLLCR2_SPEC {
    type Ux = u16;
}
///`read()` method returns [`fllcr2::R`](R) reader structure
impl crate::Readable for FLLCR2_SPEC {}
///`write(|w| ..)` method takes [`fllcr2::W`](W) writer structure
impl crate::Writable for FLLCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLLCR2 to value 0
impl crate::Resettable for FLLCR2_SPEC {}
