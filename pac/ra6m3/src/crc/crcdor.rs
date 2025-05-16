///Register `CRCDOR` reader
pub type R = crate::R<CRCDOR_SPEC>;
///Register `CRCDOR` writer
pub type W = crate::W<CRCDOR_SPEC>;
///Field `CRCDOR` reader - Calculation output Data (Case of CRC-32, CRC-32C )
pub type CRCDOR_R = crate::FieldReader<u32>;
///Field `CRCDOR` writer - Calculation output Data (Case of CRC-32, CRC-32C )
pub type CRCDOR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )
    #[inline(always)]
    pub fn crcdor(&self) -> CRCDOR_R {
        CRCDOR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )
    #[inline(always)]
    pub fn crcdor(&mut self) -> CRCDOR_W<CRCDOR_SPEC> {
        CRCDOR_W::new(self, 0)
    }
}
/**CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCDOR_SPEC;
impl crate::RegisterSpec for CRCDOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`crcdor::R`](R) reader structure
impl crate::Readable for CRCDOR_SPEC {}
///`write(|w| ..)` method takes [`crcdor::W`](W) writer structure
impl crate::Writable for CRCDOR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDOR to value 0
impl crate::Resettable for CRCDOR_SPEC {}
