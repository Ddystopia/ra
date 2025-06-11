///Register `BCNT0AR` reader
pub type R = crate::R<BCNT0AR_SPEC>;
///Register `BCNT0AR` writer
pub type W = crate::W<BCNT0AR_SPEC>;
///Field `BCNT0AR` reader - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0.
pub type BCNT0AR_R = crate::FieldReader;
///Field `BCNT0AR` writer - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0.
pub type BCNT0AR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0.
    #[inline(always)]
    pub fn bcnt0ar(&self) -> BCNT0AR_R {
        BCNT0AR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0.
    #[inline(always)]
    pub fn bcnt0ar(&mut self) -> BCNT0AR_W<BCNT0AR_SPEC> {
        BCNT0AR_W::new(self, 0)
    }
}
/**Binary Counter 0 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT0AR_SPEC;
impl crate::RegisterSpec for BCNT0AR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt0ar::R`](R) reader structure
impl crate::Readable for BCNT0AR_SPEC {}
///`write(|w| ..)` method takes [`bcnt0ar::W`](W) writer structure
impl crate::Writable for BCNT0AR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT0AR to value 0
impl crate::Resettable for BCNT0AR_SPEC {}
