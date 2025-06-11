///Register `BCNT0` reader
pub type R = crate::R<BCNT0_SPEC>;
///Register `BCNT0` writer
pub type W = crate::W<BCNT0_SPEC>;
///Field `BCNT0` reader - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0.
pub type BCNT0_R = crate::FieldReader;
///Field `BCNT0` writer - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0.
pub type BCNT0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0.
    #[inline(always)]
    pub fn bcnt0(&self) -> BCNT0_R {
        BCNT0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0.
    #[inline(always)]
    pub fn bcnt0(&mut self) -> BCNT0_W<BCNT0_SPEC> {
        BCNT0_W::new(self, 0)
    }
}
/**Binary Counter 0

You can [`read`](crate::Reg::read) this register and get [`bcnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT0_SPEC;
impl crate::RegisterSpec for BCNT0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt0::R`](R) reader structure
impl crate::Readable for BCNT0_SPEC {}
///`write(|w| ..)` method takes [`bcnt0::W`](W) writer structure
impl crate::Writable for BCNT0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT0 to value 0
impl crate::Resettable for BCNT0_SPEC {}
