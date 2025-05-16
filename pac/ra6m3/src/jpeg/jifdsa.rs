///Register `JIFDSA` reader
pub type R = crate::R<JIFDSA_SPEC>;
///Register `JIFDSA` writer
pub type W = crate::W<JIFDSA_SPEC>;
///Field `DSA` reader - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0.
pub type DSA_R = crate::FieldReader<u32>;
///Field `DSA` writer - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0.
pub type DSA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn dsa(&self) -> DSA_R {
        DSA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn dsa(&mut self) -> DSA_W<JIFDSA_SPEC> {
        DSA_W::new(self, 0)
    }
}
/**JPEG Interface Decompression Source Address Register

You can [`read`](crate::Reg::read) this register and get [`jifdsa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdsa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDSA_SPEC;
impl crate::RegisterSpec for JIFDSA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifdsa::R`](R) reader structure
impl crate::Readable for JIFDSA_SPEC {}
///`write(|w| ..)` method takes [`jifdsa::W`](W) writer structure
impl crate::Writable for JIFDSA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDSA to value 0
impl crate::Resettable for JIFDSA_SPEC {}
