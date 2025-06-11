///Register `ICDRT` reader
pub type R = crate::R<ICDRT_SPEC>;
///Register `ICDRT` writer
pub type W = crate::W<ICDRT_SPEC>;
///Field `ICDRT` reader - 8-bit read-write register that stores transmit data.
pub type ICDRT_R = crate::FieldReader;
///Field `ICDRT` writer - 8-bit read-write register that stores transmit data.
pub type ICDRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - 8-bit read-write register that stores transmit data.
    #[inline(always)]
    pub fn icdrt(&self) -> ICDRT_R {
        ICDRT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - 8-bit read-write register that stores transmit data.
    #[inline(always)]
    pub fn icdrt(&mut self) -> ICDRT_W<ICDRT_SPEC> {
        ICDRT_W::new(self, 0)
    }
}
/**I2C Bus Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`icdrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICDRT_SPEC;
impl crate::RegisterSpec for ICDRT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icdrt::R`](R) reader structure
impl crate::Readable for ICDRT_SPEC {}
///`write(|w| ..)` method takes [`icdrt::W`](W) writer structure
impl crate::Writable for ICDRT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICDRT to value 0xff
impl crate::Resettable for ICDRT_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
