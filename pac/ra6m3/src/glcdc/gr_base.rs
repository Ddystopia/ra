///Register `GR%s_BASE` reader
pub type R = crate::R<GR_BASE_SPEC>;
///Register `GR%s_BASE` writer
pub type W = crate::W<GR_BASE_SPEC>;
///Field `R` reader - Background color R valueUnsigned; 8 bits
pub type R_R = crate::FieldReader;
///Field `R` writer - Background color R valueUnsigned; 8 bits
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `B` reader - Background color B valueUnsigned; 8 bits
pub type B_R = crate::FieldReader;
///Field `B` writer - Background color B valueUnsigned; 8 bits
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `G` reader - Background color G valueUnsigned; 8 bits
pub type G_R = crate::FieldReader;
///Field `G` writer - Background color G valueUnsigned; 8 bits
pub type G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Background color R valueUnsigned; 8 bits
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Background color B valueUnsigned; 8 bits
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Background color G valueUnsigned; 8 bits
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Background color R valueUnsigned; 8 bits
    #[inline(always)]
    pub fn r(&mut self) -> R_W<GR_BASE_SPEC> {
        R_W::new(self, 0)
    }
    ///Bits 8:15 - Background color B valueUnsigned; 8 bits
    #[inline(always)]
    pub fn b(&mut self) -> B_W<GR_BASE_SPEC> {
        B_W::new(self, 8)
    }
    ///Bits 16:23 - Background color G valueUnsigned; 8 bits
    #[inline(always)]
    pub fn g(&mut self) -> G_W<GR_BASE_SPEC> {
        G_W::new(self, 16)
    }
}
/**Graphics %s Background Color Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_BASE_SPEC;
impl crate::RegisterSpec for GR_BASE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_base::R`](R) reader structure
impl crate::Readable for GR_BASE_SPEC {}
///`write(|w| ..)` method takes [`gr_base::W`](W) writer structure
impl crate::Writable for GR_BASE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_BASE to value 0
impl crate::Resettable for GR_BASE_SPEC {}
