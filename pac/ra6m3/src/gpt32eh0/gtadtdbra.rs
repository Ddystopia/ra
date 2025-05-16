///Register `GTADTDBRA` reader
pub type R = crate::R<GTADTDBRA_SPEC>;
///Register `GTADTDBRA` writer
pub type W = crate::W<GTADTDBRA_SPEC>;
///Field `GTADTDBRA` reader - A/D Converter Start Request Timing Double-Buffer Register A
pub type GTADTDBRA_R = crate::FieldReader<u32>;
///Field `GTADTDBRA` writer - A/D Converter Start Request Timing Double-Buffer Register A
pub type GTADTDBRA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A/D Converter Start Request Timing Double-Buffer Register A
    #[inline(always)]
    pub fn gtadtdbra(&self) -> GTADTDBRA_R {
        GTADTDBRA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A/D Converter Start Request Timing Double-Buffer Register A
    #[inline(always)]
    pub fn gtadtdbra(&mut self) -> GTADTDBRA_W<GTADTDBRA_SPEC> {
        GTADTDBRA_W::new(self, 0)
    }
}
/**A/D Converter Start Request Timing Double-Buffer Register A

You can [`read`](crate::Reg::read) this register and get [`gtadtdbra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtdbra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTADTDBRA_SPEC;
impl crate::RegisterSpec for GTADTDBRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtadtdbra::R`](R) reader structure
impl crate::Readable for GTADTDBRA_SPEC {}
///`write(|w| ..)` method takes [`gtadtdbra::W`](W) writer structure
impl crate::Writable for GTADTDBRA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTADTDBRA to value 0xffff_ffff
impl crate::Resettable for GTADTDBRA_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
