///Register `BCNT3AR` reader
pub type R = crate::R<BCNT3AR_SPEC>;
///Register `BCNT3AR` writer
pub type W = crate::W<BCNT3AR_SPEC>;
///Field `BCNT3AR` reader - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24.
pub type BCNT3AR_R = crate::FieldReader;
///Field `BCNT3AR` writer - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24.
pub type BCNT3AR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24.
    #[inline(always)]
    pub fn bcnt3ar(&self) -> BCNT3AR_R {
        BCNT3AR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24.
    #[inline(always)]
    pub fn bcnt3ar(&mut self) -> BCNT3AR_W<BCNT3AR_SPEC> {
        BCNT3AR_W::new(self, 0)
    }
}
/**Binary Counter 3 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt3ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT3AR_SPEC;
impl crate::RegisterSpec for BCNT3AR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt3ar::R`](R) reader structure
impl crate::Readable for BCNT3AR_SPEC {}
///`write(|w| ..)` method takes [`bcnt3ar::W`](W) writer structure
impl crate::Writable for BCNT3AR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT3AR to value 0
impl crate::Resettable for BCNT3AR_SPEC {}
