///Register `CTSUCHTRC1` reader
pub type R = crate::R<CTSUCHTRC1_SPEC>;
///Register `CTSUCHTRC1` writer
pub type W = crate::W<CTSUCHTRC1_SPEC>;
///Field `CTSUCHTRC1` reader - CTSU Channel Transmit/Receive Control 1CTSUCHTRC1\[0\] corresponds to TS08 and CTSUCHTRC1\[7\] corresponds to TS15. ( 0: Reception / 1: Transmission )
pub type CTSUCHTRC1_R = crate::FieldReader;
///Field `CTSUCHTRC1` writer - CTSU Channel Transmit/Receive Control 1CTSUCHTRC1\[0\] corresponds to TS08 and CTSUCHTRC1\[7\] corresponds to TS15. ( 0: Reception / 1: Transmission )
pub type CTSUCHTRC1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 1CTSUCHTRC1\[0\] corresponds to TS08 and CTSUCHTRC1\[7\] corresponds to TS15. ( 0: Reception / 1: Transmission )
    #[inline(always)]
    pub fn ctsuchtrc1(&self) -> CTSUCHTRC1_R {
        CTSUCHTRC1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 1CTSUCHTRC1\[0\] corresponds to TS08 and CTSUCHTRC1\[7\] corresponds to TS15. ( 0: Reception / 1: Transmission )
    #[inline(always)]
    pub fn ctsuchtrc1(&mut self) -> CTSUCHTRC1_W<CTSUCHTRC1_SPEC> {
        CTSUCHTRC1_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC1_SPEC;
impl crate::RegisterSpec for CTSUCHTRC1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc1::R`](R) reader structure
impl crate::Readable for CTSUCHTRC1_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc1::W`](W) writer structure
impl crate::Writable for CTSUCHTRC1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC1 to value 0
impl crate::Resettable for CTSUCHTRC1_SPEC {}
