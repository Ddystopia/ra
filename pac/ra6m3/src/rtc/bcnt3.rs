///Register `BCNT3` reader
pub type R = crate::R<BCNT3_SPEC>;
///Register `BCNT3` writer
pub type W = crate::W<BCNT3_SPEC>;
///Field `BCNT3` reader - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24.
pub type BCNT3_R = crate::FieldReader;
///Field `BCNT3` writer - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24.
pub type BCNT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24.
    #[inline(always)]
    pub fn bcnt3(&self) -> BCNT3_R {
        BCNT3_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24.
    #[inline(always)]
    pub fn bcnt3(&mut self) -> BCNT3_W<BCNT3_SPEC> {
        BCNT3_W::new(self, 0)
    }
}
/**Binary Counter 3

You can [`read`](crate::Reg::read) this register and get [`bcnt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT3_SPEC;
impl crate::RegisterSpec for BCNT3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt3::R`](R) reader structure
impl crate::Readable for BCNT3_SPEC {}
///`write(|w| ..)` method takes [`bcnt3::W`](W) writer structure
impl crate::Writable for BCNT3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT3 to value 0
impl crate::Resettable for BCNT3_SPEC {}
