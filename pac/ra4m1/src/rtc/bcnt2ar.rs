///Register `BCNT2AR` reader
pub type R = crate::R<BCNT2AR_SPEC>;
///Register `BCNT2AR` writer
pub type W = crate::W<BCNT2AR_SPEC>;
///Field `BCNT2AR` reader - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16.
pub type BCNT2AR_R = crate::FieldReader;
///Field `BCNT2AR` writer - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16.
pub type BCNT2AR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16.
    #[inline(always)]
    pub fn bcnt2ar(&self) -> BCNT2AR_R {
        BCNT2AR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16.
    #[inline(always)]
    pub fn bcnt2ar(&mut self) -> BCNT2AR_W<BCNT2AR_SPEC> {
        BCNT2AR_W::new(self, 0)
    }
}
/**Binary Counter 2 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT2AR_SPEC;
impl crate::RegisterSpec for BCNT2AR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt2ar::R`](R) reader structure
impl crate::Readable for BCNT2AR_SPEC {}
///`write(|w| ..)` method takes [`bcnt2ar::W`](W) writer structure
impl crate::Writable for BCNT2AR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT2AR to value 0
impl crate::Resettable for BCNT2AR_SPEC {}
