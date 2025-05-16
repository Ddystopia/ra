///Register `GTADTBRA` reader
pub type R = crate::R<GTADTBRA_SPEC>;
///Register `GTADTBRA` writer
pub type W = crate::W<GTADTBRA_SPEC>;
///Field `GTADTBRA` reader - A/D Converter Start Request Timing Buffer Register A
pub type GTADTBRA_R = crate::FieldReader<u32>;
///Field `GTADTBRA` writer - A/D Converter Start Request Timing Buffer Register A
pub type GTADTBRA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A/D Converter Start Request Timing Buffer Register A
    #[inline(always)]
    pub fn gtadtbra(&self) -> GTADTBRA_R {
        GTADTBRA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A/D Converter Start Request Timing Buffer Register A
    #[inline(always)]
    pub fn gtadtbra(&mut self) -> GTADTBRA_W<GTADTBRA_SPEC> {
        GTADTBRA_W::new(self, 0)
    }
}
/**A/D Converter Start Request Timing Buffer Register A

You can [`read`](crate::Reg::read) this register and get [`gtadtbra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtbra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTADTBRA_SPEC;
impl crate::RegisterSpec for GTADTBRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtadtbra::R`](R) reader structure
impl crate::Readable for GTADTBRA_SPEC {}
///`write(|w| ..)` method takes [`gtadtbra::W`](W) writer structure
impl crate::Writable for GTADTBRA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTADTBRA to value 0xffff_ffff
impl crate::Resettable for GTADTBRA_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
