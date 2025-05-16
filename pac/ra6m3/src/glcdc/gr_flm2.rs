///Register `GR%s_FLM2` reader
pub type R = crate::R<GR_FLM2_SPEC>;
///Register `GR%s_FLM2` writer
pub type W = crate::W<GR_FLM2_SPEC>;
///Field `BASE` reader - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\[5:0\] should be fixed to 0 during 64-byte burst transfer.
pub type BASE_R = crate::FieldReader<u32>;
///Field `BASE` writer - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\[5:0\] should be fixed to 0 during 64-byte burst transfer.
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\[5:0\] should be fixed to 0 during 64-byte burst transfer.
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\[5:0\] should be fixed to 0 during 64-byte burst transfer.
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W<GR_FLM2_SPEC> {
        BASE_W::new(self, 0)
    }
}
/**Graphics %s Frame Buffer Control Register 2

You can [`read`](crate::Reg::read) this register and get [`gr_flm2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_FLM2_SPEC;
impl crate::RegisterSpec for GR_FLM2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_flm2::R`](R) reader structure
impl crate::Readable for GR_FLM2_SPEC {}
///`write(|w| ..)` method takes [`gr_flm2::W`](W) writer structure
impl crate::Writable for GR_FLM2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_FLM2 to value 0
impl crate::Resettable for GR_FLM2_SPEC {}
