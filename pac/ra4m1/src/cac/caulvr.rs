///Register `CAULVR` reader
pub type R = crate::R<CAULVR_SPEC>;
///Register `CAULVR` writer
pub type W = crate::W<CAULVR_SPEC>;
///Field `CAULVR` reader - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency.
pub type CAULVR_R = crate::FieldReader<u16>;
///Field `CAULVR` writer - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency.
pub type CAULVR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency.
    #[inline(always)]
    pub fn caulvr(&self) -> CAULVR_R {
        CAULVR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency.
    #[inline(always)]
    pub fn caulvr(&mut self) -> CAULVR_W<CAULVR_SPEC> {
        CAULVR_W::new(self, 0)
    }
}
/**CAC Upper-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`caulvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caulvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CAULVR_SPEC;
impl crate::RegisterSpec for CAULVR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`caulvr::R`](R) reader structure
impl crate::Readable for CAULVR_SPEC {}
///`write(|w| ..)` method takes [`caulvr::W`](W) writer structure
impl crate::Writable for CAULVR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAULVR to value 0
impl crate::Resettable for CAULVR_SPEC {}
