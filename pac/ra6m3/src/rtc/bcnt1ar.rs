///Register `BCNT1AR` reader
pub type R = crate::R<BCNT1AR_SPEC>;
///Register `BCNT1AR` writer
pub type W = crate::W<BCNT1AR_SPEC>;
///Field `BCNT1AR` reader - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8.
pub type BCNT1AR_R = crate::FieldReader;
///Field `BCNT1AR` writer - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8.
pub type BCNT1AR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8.
    #[inline(always)]
    pub fn bcnt1ar(&self) -> BCNT1AR_R {
        BCNT1AR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8.
    #[inline(always)]
    pub fn bcnt1ar(&mut self) -> BCNT1AR_W<BCNT1AR_SPEC> {
        BCNT1AR_W::new(self, 0)
    }
}
/**Binary Counter 1 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT1AR_SPEC;
impl crate::RegisterSpec for BCNT1AR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt1ar::R`](R) reader structure
impl crate::Readable for BCNT1AR_SPEC {}
///`write(|w| ..)` method takes [`bcnt1ar::W`](W) writer structure
impl crate::Writable for BCNT1AR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT1AR to value 0
impl crate::Resettable for BCNT1AR_SPEC {}
