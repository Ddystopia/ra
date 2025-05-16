///Register `TDR` reader
pub type R = crate::R<TDR_SPEC>;
///Register `TDR` writer
pub type W = crate::W<TDR_SPEC>;
///Field `TDR` reader - TDR is an 8-bit register that stores transmit data.
pub type TDR_R = crate::FieldReader;
///Field `TDR` writer - TDR is an 8-bit register that stores transmit data.
pub type TDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - TDR is an 8-bit register that stores transmit data.
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - TDR is an 8-bit register that stores transmit data.
    #[inline(always)]
    pub fn tdr(&mut self) -> TDR_W<TDR_SPEC> {
        TDR_W::new(self, 0)
    }
}
/**Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tdr::R`](R) reader structure
impl crate::Readable for TDR_SPEC {}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDR to value 0xff
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
