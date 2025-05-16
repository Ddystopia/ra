///Register `CTSUCHTRC0` reader
pub type R = crate::R<CTSUCHTRC0_SPEC>;
///Register `CTSUCHTRC0` writer
pub type W = crate::W<CTSUCHTRC0_SPEC>;
///Field `CTSUCHTRC0` reader - CTSU Channel Transmit/Receive Control 0CTSUCHTRC0\[0\] corresponds to TS00 and CTSUCHTRC0\[7\] corresponds to TS07. ( 0: Reception / 1: Transmission )
pub type CTSUCHTRC0_R = crate::FieldReader;
///Field `CTSUCHTRC0` writer - CTSU Channel Transmit/Receive Control 0CTSUCHTRC0\[0\] corresponds to TS00 and CTSUCHTRC0\[7\] corresponds to TS07. ( 0: Reception / 1: Transmission )
pub type CTSUCHTRC0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 0CTSUCHTRC0\[0\] corresponds to TS00 and CTSUCHTRC0\[7\] corresponds to TS07. ( 0: Reception / 1: Transmission )
    #[inline(always)]
    pub fn ctsuchtrc0(&self) -> CTSUCHTRC0_R {
        CTSUCHTRC0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 0CTSUCHTRC0\[0\] corresponds to TS00 and CTSUCHTRC0\[7\] corresponds to TS07. ( 0: Reception / 1: Transmission )
    #[inline(always)]
    pub fn ctsuchtrc0(&mut self) -> CTSUCHTRC0_W<CTSUCHTRC0_SPEC> {
        CTSUCHTRC0_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC0_SPEC;
impl crate::RegisterSpec for CTSUCHTRC0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc0::R`](R) reader structure
impl crate::Readable for CTSUCHTRC0_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc0::W`](W) writer structure
impl crate::Writable for CTSUCHTRC0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC0 to value 0
impl crate::Resettable for CTSUCHTRC0_SPEC {}
