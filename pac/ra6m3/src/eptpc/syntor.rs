///Register `SYNTOR` reader
pub type R = crate::R<SYNTOR_SPEC>;
///Register `SYNTOR` writer
pub type W = crate::W<SYNTOR_SPEC>;
///Field `SYNTOR` reader - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1.
pub type SYNTOR_R = crate::FieldReader<u32>;
///Field `SYNTOR` writer - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1.
pub type SYNTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1.
    #[inline(always)]
    pub fn syntor(&self) -> SYNTOR_R {
        SYNTOR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1.
    #[inline(always)]
    pub fn syntor(&mut self) -> SYNTOR_W<SYNTOR_SPEC> {
        SYNTOR_W::new(self, 0)
    }
}
/**Sync Message Reception Timeout Register

You can [`read`](crate::Reg::read) this register and get [`syntor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNTOR_SPEC;
impl crate::RegisterSpec for SYNTOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syntor::R`](R) reader structure
impl crate::Readable for SYNTOR_SPEC {}
///`write(|w| ..)` method takes [`syntor::W`](W) writer structure
impl crate::Writable for SYNTOR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTOR to value 0
impl crate::Resettable for SYNTOR_SPEC {}
