///Register `SYNTDARL` reader
pub type R = crate::R<SYNTDARL_SPEC>;
///Register `SYNTDARL` writer
pub type W = crate::W<SYNTDARL_SPEC>;
///Field `SYNTDARL` reader - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization.
pub type SYNTDARL_R = crate::FieldReader<u32>;
///Field `SYNTDARL` writer - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization.
pub type SYNTDARL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization.
    #[inline(always)]
    pub fn syntdarl(&self) -> SYNTDARL_R {
        SYNTDARL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization.
    #[inline(always)]
    pub fn syntdarl(&mut self) -> SYNTDARL_W<SYNTDARL_SPEC> {
        SYNTDARL_W::new(self, 0)
    }
}
/**Synchronization Loss Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdarl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdarl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNTDARL_SPEC;
impl crate::RegisterSpec for SYNTDARL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syntdarl::R`](R) reader structure
impl crate::Readable for SYNTDARL_SPEC {}
///`write(|w| ..)` method takes [`syntdarl::W`](W) writer structure
impl crate::Writable for SYNTDARL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTDARL to value 0
impl crate::Resettable for SYNTDARL_SPEC {}
