///Register `BCNT1` reader
pub type R = crate::R<BCNT1_SPEC>;
///Register `BCNT1` writer
pub type W = crate::W<BCNT1_SPEC>;
///Field `BCNT1` reader - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8.
pub type BCNT1_R = crate::FieldReader;
///Field `BCNT1` writer - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8.
pub type BCNT1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8.
    #[inline(always)]
    pub fn bcnt1(&self) -> BCNT1_R {
        BCNT1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8.
    #[inline(always)]
    pub fn bcnt1(&mut self) -> BCNT1_W<BCNT1_SPEC> {
        BCNT1_W::new(self, 0)
    }
}
/**Binary Counter 1

You can [`read`](crate::Reg::read) this register and get [`bcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT1_SPEC;
impl crate::RegisterSpec for BCNT1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt1::R`](R) reader structure
impl crate::Readable for BCNT1_SPEC {}
///`write(|w| ..)` method takes [`bcnt1::W`](W) writer structure
impl crate::Writable for BCNT1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT1 to value 0
impl crate::Resettable for BCNT1_SPEC {}
