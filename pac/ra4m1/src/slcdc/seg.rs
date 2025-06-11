///Register `SEG%s` reader
pub type R = crate::R<SEG_SPEC>;
///Register `SEG%s` writer
pub type W = crate::W<SEG_SPEC>;
///Field `SEG` reader - LCD Display Data
pub type SEG_R = crate::FieldReader;
///Field `SEG` writer - LCD Display Data
pub type SEG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - LCD Display Data
    #[inline(always)]
    pub fn seg(&self) -> SEG_R {
        SEG_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - LCD Display Data
    #[inline(always)]
    pub fn seg(&mut self) -> SEG_W<SEG_SPEC> {
        SEG_W::new(self, 0)
    }
}
/**LCD Display Data Register %s

You can [`read`](crate::Reg::read) this register and get [`seg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SEG_SPEC;
impl crate::RegisterSpec for SEG_SPEC {
    type Ux = u8;
}
///`read()` method returns [`seg::R`](R) reader structure
impl crate::Readable for SEG_SPEC {}
///`write(|w| ..)` method takes [`seg::W`](W) writer structure
impl crate::Writable for SEG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEG%s to value 0
impl crate::Resettable for SEG_SPEC {}
