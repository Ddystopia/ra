///Register `CALLVR` reader
pub type R = crate::R<CALLVR_SPEC>;
///Register `CALLVR` writer
pub type W = crate::W<CALLVR_SPEC>;
///Field `CALLVR` reader - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency.
pub type CALLVR_R = crate::FieldReader<u16>;
///Field `CALLVR` writer - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency.
pub type CALLVR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency.
    #[inline(always)]
    pub fn callvr(&self) -> CALLVR_R {
        CALLVR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency.
    #[inline(always)]
    pub fn callvr(&mut self) -> CALLVR_W<CALLVR_SPEC> {
        CALLVR_W::new(self, 0)
    }
}
/**CAC Lower-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`callvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`callvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CALLVR_SPEC;
impl crate::RegisterSpec for CALLVR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`callvr::R`](R) reader structure
impl crate::Readable for CALLVR_SPEC {}
///`write(|w| ..)` method takes [`callvr::W`](W) writer structure
impl crate::Writable for CALLVR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALLVR to value 0
impl crate::Resettable for CALLVR_SPEC {}
