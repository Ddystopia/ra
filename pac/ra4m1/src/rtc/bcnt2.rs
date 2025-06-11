///Register `BCNT2` reader
pub type R = crate::R<BCNT2_SPEC>;
///Register `BCNT2` writer
pub type W = crate::W<BCNT2_SPEC>;
///Field `BCNT2` reader - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16.
pub type BCNT2_R = crate::FieldReader;
///Field `BCNT2` writer - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16.
pub type BCNT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16.
    #[inline(always)]
    pub fn bcnt2(&self) -> BCNT2_R {
        BCNT2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16.
    #[inline(always)]
    pub fn bcnt2(&mut self) -> BCNT2_W<BCNT2_SPEC> {
        BCNT2_W::new(self, 0)
    }
}
/**Binary Counter 2

You can [`read`](crate::Reg::read) this register and get [`bcnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT2_SPEC;
impl crate::RegisterSpec for BCNT2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt2::R`](R) reader structure
impl crate::Readable for BCNT2_SPEC {}
///`write(|w| ..)` method takes [`bcnt2::W`](W) writer structure
impl crate::Writable for BCNT2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT2 to value 0
impl crate::Resettable for BCNT2_SPEC {}
