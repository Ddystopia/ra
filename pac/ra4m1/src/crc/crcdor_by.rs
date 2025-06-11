///Register `CRCDOR_BY` reader
pub type R = crate::R<CRCDOR_BY_SPEC>;
///Register `CRCDOR_BY` writer
pub type W = crate::W<CRCDOR_BY_SPEC>;
///Field `CRCDOR_BY` reader - Calculation output Data (Case of CRC-8 )
pub type CRCDOR_BY_R = crate::FieldReader;
///Field `CRCDOR_BY` writer - Calculation output Data (Case of CRC-8 )
pub type CRCDOR_BY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Calculation output Data (Case of CRC-8 )
    #[inline(always)]
    pub fn crcdor_by(&self) -> CRCDOR_BY_R {
        CRCDOR_BY_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Calculation output Data (Case of CRC-8 )
    #[inline(always)]
    pub fn crcdor_by(&mut self) -> CRCDOR_BY_W<CRCDOR_BY_SPEC> {
        CRCDOR_BY_W::new(self, 0)
    }
}
/**CRC Data Output Register(byte access)

You can [`read`](crate::Reg::read) this register and get [`crcdor_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCDOR_BY_SPEC;
impl crate::RegisterSpec for CRCDOR_BY_SPEC {
    type Ux = u8;
}
///`read()` method returns [`crcdor_by::R`](R) reader structure
impl crate::Readable for CRCDOR_BY_SPEC {}
///`write(|w| ..)` method takes [`crcdor_by::W`](W) writer structure
impl crate::Writable for CRCDOR_BY_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDOR_BY to value 0
impl crate::Resettable for CRCDOR_BY_SPEC {}
