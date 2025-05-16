///Register `GTADTRB` reader
pub type R = crate::R<GTADTRB_SPEC>;
///Register `GTADTRB` writer
pub type W = crate::W<GTADTRB_SPEC>;
///Field `GTADTRB` reader - A/D Converter Start Request Timing Register B
pub type GTADTRB_R = crate::FieldReader<u32>;
///Field `GTADTRB` writer - A/D Converter Start Request Timing Register B
pub type GTADTRB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A/D Converter Start Request Timing Register B
    #[inline(always)]
    pub fn gtadtrb(&self) -> GTADTRB_R {
        GTADTRB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A/D Converter Start Request Timing Register B
    #[inline(always)]
    pub fn gtadtrb(&mut self) -> GTADTRB_W<GTADTRB_SPEC> {
        GTADTRB_W::new(self, 0)
    }
}
/**A/D Converter Start Request Timing Register B

You can [`read`](crate::Reg::read) this register and get [`gtadtrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTADTRB_SPEC;
impl crate::RegisterSpec for GTADTRB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtadtrb::R`](R) reader structure
impl crate::Readable for GTADTRB_SPEC {}
///`write(|w| ..)` method takes [`gtadtrb::W`](W) writer structure
impl crate::Writable for GTADTRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTADTRB to value 0xffff_ffff
impl crate::Resettable for GTADTRB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
