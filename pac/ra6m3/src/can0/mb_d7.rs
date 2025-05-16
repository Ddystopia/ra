///Register `MB%s_D7` reader
pub type R = crate::R<MB_D7_SPEC>;
///Register `MB%s_D7` writer
pub type W = crate::W<MB_D7_SPEC>;
///Field `DATA7` reader - Data Bytes 7DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA7_R = crate::FieldReader;
///Field `DATA7` writer - Data Bytes 7DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data Bytes 7DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Data Bytes 7DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W<MB_D7_SPEC> {
        DATA7_W::new(self, 0)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_d7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_D7_SPEC;
impl crate::RegisterSpec for MB_D7_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mb_d7::R`](R) reader structure
impl crate::Readable for MB_D7_SPEC {}
///`write(|w| ..)` method takes [`mb_d7::W`](W) writer structure
impl crate::Writable for MB_D7_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_D7 to value 0
impl crate::Resettable for MB_D7_SPEC {}
