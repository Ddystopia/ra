///Register `MDDR` reader
pub type R = crate::R<MDDR_SPEC>;
///Register `MDDR` writer
pub type W = crate::W<MDDR_SPEC>;
///Field `MDDR` reader - MDDR corrects the bit rate adjusted by the BRR register.
pub type MDDR_R = crate::FieldReader;
///Field `MDDR` writer - MDDR corrects the bit rate adjusted by the BRR register.
pub type MDDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - MDDR corrects the bit rate adjusted by the BRR register.
    #[inline(always)]
    pub fn mddr(&self) -> MDDR_R {
        MDDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - MDDR corrects the bit rate adjusted by the BRR register.
    #[inline(always)]
    pub fn mddr(&mut self) -> MDDR_W<MDDR_SPEC> {
        MDDR_W::new(self, 0)
    }
}
/**Modulation Duty Register

You can [`read`](crate::Reg::read) this register and get [`mddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MDDR_SPEC;
impl crate::RegisterSpec for MDDR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mddr::R`](R) reader structure
impl crate::Readable for MDDR_SPEC {}
///`write(|w| ..)` method takes [`mddr::W`](W) writer structure
impl crate::Writable for MDDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MDDR to value 0xff
impl crate::Resettable for MDDR_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
