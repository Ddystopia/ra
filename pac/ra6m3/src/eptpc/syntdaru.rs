///Register `SYNTDARU` reader
pub type R = crate::R<SYNTDARU_SPEC>;
///Register `SYNTDARU` writer
pub type W = crate::W<SYNTDARU_SPEC>;
///Field `SYNTDARU` reader - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization.
pub type SYNTDARU_R = crate::FieldReader<u32>;
///Field `SYNTDARU` writer - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization.
pub type SYNTDARU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization.
    #[inline(always)]
    pub fn syntdaru(&self) -> SYNTDARU_R {
        SYNTDARU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization.
    #[inline(always)]
    pub fn syntdaru(&mut self) -> SYNTDARU_W<SYNTDARU_SPEC> {
        SYNTDARU_W::new(self, 0)
    }
}
/**Synchronization Loss Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdaru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdaru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNTDARU_SPEC;
impl crate::RegisterSpec for SYNTDARU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syntdaru::R`](R) reader structure
impl crate::Readable for SYNTDARU_SPEC {}
///`write(|w| ..)` method takes [`syntdaru::W`](W) writer structure
impl crate::Writable for SYNTDARU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTDARU to value 0
impl crate::Resettable for SYNTDARU_SPEC {}
