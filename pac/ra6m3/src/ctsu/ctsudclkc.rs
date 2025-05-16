///Register `CTSUDCLKC` reader
pub type R = crate::R<CTSUDCLKC_SPEC>;
///Register `CTSUDCLKC` writer
pub type W = crate::W<CTSUDCLKC_SPEC>;
///Field `CTSUSSMOD` reader - CTSU Diffusion Clock Mode SelectNOTE: This bit should be set to 00b.
pub type CTSUSSMOD_R = crate::FieldReader;
///Field `CTSUSSMOD` writer - CTSU Diffusion Clock Mode SelectNOTE: This bit should be set to 00b.
pub type CTSUSSMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CTSUSSCNT` reader - CTSU Diffusion Clock Mode ControlNOTE: This bit should be set to 11b.
pub type CTSUSSCNT_R = crate::FieldReader;
///Field `CTSUSSCNT` writer - CTSU Diffusion Clock Mode ControlNOTE: This bit should be set to 11b.
pub type CTSUSSCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - CTSU Diffusion Clock Mode SelectNOTE: This bit should be set to 00b.
    #[inline(always)]
    pub fn ctsussmod(&self) -> CTSUSSMOD_R {
        CTSUSSMOD_R::new(self.bits & 3)
    }
    ///Bits 4:5 - CTSU Diffusion Clock Mode ControlNOTE: This bit should be set to 11b.
    #[inline(always)]
    pub fn ctsusscnt(&self) -> CTSUSSCNT_R {
        CTSUSSCNT_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    ///Bits 0:1 - CTSU Diffusion Clock Mode SelectNOTE: This bit should be set to 00b.
    #[inline(always)]
    pub fn ctsussmod(&mut self) -> CTSUSSMOD_W<CTSUDCLKC_SPEC> {
        CTSUSSMOD_W::new(self, 0)
    }
    ///Bits 4:5 - CTSU Diffusion Clock Mode ControlNOTE: This bit should be set to 11b.
    #[inline(always)]
    pub fn ctsusscnt(&mut self) -> CTSUSSCNT_W<CTSUDCLKC_SPEC> {
        CTSUSSCNT_W::new(self, 4)
    }
}
/**CTSU High-Pass Noise Reduction Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsudclkc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsudclkc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUDCLKC_SPEC;
impl crate::RegisterSpec for CTSUDCLKC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsudclkc::R`](R) reader structure
impl crate::Readable for CTSUDCLKC_SPEC {}
///`write(|w| ..)` method takes [`ctsudclkc::W`](W) writer structure
impl crate::Writable for CTSUDCLKC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUDCLKC to value 0
impl crate::Resettable for CTSUDCLKC_SPEC {}
