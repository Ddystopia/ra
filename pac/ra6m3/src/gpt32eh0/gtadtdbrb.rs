///Register `GTADTDBRB` reader
pub type R = crate::R<GTADTDBRB_SPEC>;
///Register `GTADTDBRB` writer
pub type W = crate::W<GTADTDBRB_SPEC>;
///Field `GTADTDBRB` reader - A/D Converter Start Request Timing Double-Buffer Register B
pub type GTADTDBRB_R = crate::FieldReader<u32>;
///Field `GTADTDBRB` writer - A/D Converter Start Request Timing Double-Buffer Register B
pub type GTADTDBRB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A/D Converter Start Request Timing Double-Buffer Register B
    #[inline(always)]
    pub fn gtadtdbrb(&self) -> GTADTDBRB_R {
        GTADTDBRB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A/D Converter Start Request Timing Double-Buffer Register B
    #[inline(always)]
    pub fn gtadtdbrb(&mut self) -> GTADTDBRB_W<GTADTDBRB_SPEC> {
        GTADTDBRB_W::new(self, 0)
    }
}
/**A/D Converter Start Request Timing Double-Buffer Register B

You can [`read`](crate::Reg::read) this register and get [`gtadtdbrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtdbrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTADTDBRB_SPEC;
impl crate::RegisterSpec for GTADTDBRB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtadtdbrb::R`](R) reader structure
impl crate::Readable for GTADTDBRB_SPEC {}
///`write(|w| ..)` method takes [`gtadtdbrb::W`](W) writer structure
impl crate::Writable for GTADTDBRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTADTDBRB to value 0xffff_ffff
impl crate::Resettable for GTADTDBRB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
