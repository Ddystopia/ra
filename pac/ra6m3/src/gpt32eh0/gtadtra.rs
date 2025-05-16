///Register `GTADTRA` reader
pub type R = crate::R<GTADTRA_SPEC>;
///Register `GTADTRA` writer
pub type W = crate::W<GTADTRA_SPEC>;
///Field `GTADTRA` reader - A/D Converter Start Request Timing Register A
pub type GTADTRA_R = crate::FieldReader<u32>;
///Field `GTADTRA` writer - A/D Converter Start Request Timing Register A
pub type GTADTRA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A/D Converter Start Request Timing Register A
    #[inline(always)]
    pub fn gtadtra(&self) -> GTADTRA_R {
        GTADTRA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A/D Converter Start Request Timing Register A
    #[inline(always)]
    pub fn gtadtra(&mut self) -> GTADTRA_W<GTADTRA_SPEC> {
        GTADTRA_W::new(self, 0)
    }
}
/**A/D Converter Start Request Timing Register A

You can [`read`](crate::Reg::read) this register and get [`gtadtra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTADTRA_SPEC;
impl crate::RegisterSpec for GTADTRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtadtra::R`](R) reader structure
impl crate::Readable for GTADTRA_SPEC {}
///`write(|w| ..)` method takes [`gtadtra::W`](W) writer structure
impl crate::Writable for GTADTRA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTADTRA to value 0xffff_ffff
impl crate::Resettable for GTADTRA_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
