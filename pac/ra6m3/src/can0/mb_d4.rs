///Register `MB%s_D4` reader
pub type R = crate::R<MB_D4_SPEC>;
///Register `MB%s_D4` writer
pub type W = crate::W<MB_D4_SPEC>;
///Field `DATA4` reader - Data Bytes 4DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA4_R = crate::FieldReader;
///Field `DATA4` writer - Data Bytes 4DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data Bytes 4DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Data Bytes 4DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W<MB_D4_SPEC> {
        DATA4_W::new(self, 0)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_D4_SPEC;
impl crate::RegisterSpec for MB_D4_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mb_d4::R`](R) reader structure
impl crate::Readable for MB_D4_SPEC {}
///`write(|w| ..)` method takes [`mb_d4::W`](W) writer structure
impl crate::Writable for MB_D4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_D4 to value 0
impl crate::Resettable for MB_D4_SPEC {}
