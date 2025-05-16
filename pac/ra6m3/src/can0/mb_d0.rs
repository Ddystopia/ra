///Register `MB%s_D0` reader
pub type R = crate::R<MB_D0_SPEC>;
///Register `MB%s_D0` writer
pub type W = crate::W<MB_D0_SPEC>;
///Field `DATA0` reader - Data Bytes 0.DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA0_R = crate::FieldReader;
///Field `DATA0` writer - Data Bytes 0.DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data Bytes 0.DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Data Bytes 0.DATA0 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<MB_D0_SPEC> {
        DATA0_W::new(self, 0)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_D0_SPEC;
impl crate::RegisterSpec for MB_D0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mb_d0::R`](R) reader structure
impl crate::Readable for MB_D0_SPEC {}
///`write(|w| ..)` method takes [`mb_d0::W`](W) writer structure
impl crate::Writable for MB_D0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_D0 to value 0
impl crate::Resettable for MB_D0_SPEC {}
