///Register `SYIPADDRR` reader
pub type R = crate::R<SYIPADDRR_SPEC>;
///Register `SYIPADDRR` writer
pub type W = crate::W<SYIPADDRR_SPEC>;
///Field `SYIPADDRR` reader - These bits hold the setting for the local IP address.
pub type SYIPADDRR_R = crate::FieldReader<u32>;
///Field `SYIPADDRR` writer - These bits hold the setting for the local IP address.
pub type SYIPADDRR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the local IP address.
    #[inline(always)]
    pub fn syipaddrr(&self) -> SYIPADDRR_R {
        SYIPADDRR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the local IP address.
    #[inline(always)]
    pub fn syipaddrr(&mut self) -> SYIPADDRR_W<SYIPADDRR_SPEC> {
        SYIPADDRR_W::new(self, 0)
    }
}
/**SYNFP Local IP Address Register

You can [`read`](crate::Reg::read) this register and get [`syipaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syipaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYIPADDRR_SPEC;
impl crate::RegisterSpec for SYIPADDRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syipaddrr::R`](R) reader structure
impl crate::Readable for SYIPADDRR_SPEC {}
///`write(|w| ..)` method takes [`syipaddrr::W`](W) writer structure
impl crate::Writable for SYIPADDRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYIPADDRR to value 0
impl crate::Resettable for SYIPADDRR_SPEC {}
