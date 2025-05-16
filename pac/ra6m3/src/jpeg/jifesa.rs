///Register `JIFESA` reader
pub type R = crate::R<JIFESA_SPEC>;
///Register `JIFESA` writer
pub type W = crate::W<JIFESA_SPEC>;
///Field `ESA` reader - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0.
pub type ESA_R = crate::FieldReader<u32>;
///Field `ESA` writer - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0.
pub type ESA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn esa(&self) -> ESA_R {
        ESA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn esa(&mut self) -> ESA_W<JIFESA_SPEC> {
        ESA_W::new(self, 0)
    }
}
/**JPEG Interface Compression Source Address Register

You can [`read`](crate::Reg::read) this register and get [`jifesa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifesa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFESA_SPEC;
impl crate::RegisterSpec for JIFESA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifesa::R`](R) reader structure
impl crate::Readable for JIFESA_SPEC {}
///`write(|w| ..)` method takes [`jifesa::W`](W) writer structure
impl crate::Writable for JIFESA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFESA to value 0
impl crate::Resettable for JIFESA_SPEC {}
