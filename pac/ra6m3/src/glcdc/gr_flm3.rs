///Register `GR%s_FLM3` reader
pub type R = crate::R<GR_FLM3_SPEC>;
///Register `GR%s_FLM3` writer
pub type W = crate::W<GR_FLM3_SPEC>;
///Field `LNOFF` reader - Macro line offset address for accessing graphics data(frame buffer data)Signed; 16-bit integer
pub type LNOFF_R = crate::FieldReader<u16>;
///Field `LNOFF` writer - Macro line offset address for accessing graphics data(frame buffer data)Signed; 16-bit integer
pub type LNOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - Macro line offset address for accessing graphics data(frame buffer data)Signed; 16-bit integer
    #[inline(always)]
    pub fn lnoff(&self) -> LNOFF_R {
        LNOFF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - Macro line offset address for accessing graphics data(frame buffer data)Signed; 16-bit integer
    #[inline(always)]
    pub fn lnoff(&mut self) -> LNOFF_W<GR_FLM3_SPEC> {
        LNOFF_W::new(self, 16)
    }
}
/**Graphics %s Frame Buffer Control Register 3

You can [`read`](crate::Reg::read) this register and get [`gr_flm3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_FLM3_SPEC;
impl crate::RegisterSpec for GR_FLM3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_flm3::R`](R) reader structure
impl crate::Readable for GR_FLM3_SPEC {}
///`write(|w| ..)` method takes [`gr_flm3::W`](W) writer structure
impl crate::Writable for GR_FLM3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_FLM3 to value 0
impl crate::Resettable for GR_FLM3_SPEC {}
