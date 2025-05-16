///Register `MB%s_D1` reader
pub type R = crate::R<MB_D1_SPEC>;
///Register `MB%s_D1` writer
pub type W = crate::W<MB_D1_SPEC>;
///Field `DATA1` reader - Data Bytes 1DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA1_R = crate::FieldReader;
///Field `DATA1` writer - Data Bytes 1DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data Bytes 1DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Data Bytes 1DATA1 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<MB_D1_SPEC> {
        DATA1_W::new(self, 0)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_d1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_D1_SPEC;
impl crate::RegisterSpec for MB_D1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mb_d1::R`](R) reader structure
impl crate::Readable for MB_D1_SPEC {}
///`write(|w| ..)` method takes [`mb_d1::W`](W) writer structure
impl crate::Writable for MB_D1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_D1 to value 0
impl crate::Resettable for MB_D1_SPEC {}
