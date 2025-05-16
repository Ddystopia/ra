///Register `GTADTBRB` reader
pub type R = crate::R<GTADTBRB_SPEC>;
///Register `GTADTBRB` writer
pub type W = crate::W<GTADTBRB_SPEC>;
///Field `GTADTBRB` reader - A/D Converter Start Request Timing Buffer Register B
pub type GTADTBRB_R = crate::FieldReader<u32>;
///Field `GTADTBRB` writer - A/D Converter Start Request Timing Buffer Register B
pub type GTADTBRB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A/D Converter Start Request Timing Buffer Register B
    #[inline(always)]
    pub fn gtadtbrb(&self) -> GTADTBRB_R {
        GTADTBRB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A/D Converter Start Request Timing Buffer Register B
    #[inline(always)]
    pub fn gtadtbrb(&mut self) -> GTADTBRB_W<GTADTBRB_SPEC> {
        GTADTBRB_W::new(self, 0)
    }
}
/**A/D Converter Start Request Timing Buffer Register B

You can [`read`](crate::Reg::read) this register and get [`gtadtbrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtbrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTADTBRB_SPEC;
impl crate::RegisterSpec for GTADTBRB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtadtbrb::R`](R) reader structure
impl crate::Readable for GTADTBRB_SPEC {}
///`write(|w| ..)` method takes [`gtadtbrb::W`](W) writer structure
impl crate::Writable for GTADTBRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTADTBRB to value 0xffff_ffff
impl crate::Resettable for GTADTBRB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
