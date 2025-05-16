///Register `MB%s_D3` reader
pub type R = crate::R<MB_D3_SPEC>;
///Register `MB%s_D3` writer
pub type W = crate::W<MB_D3_SPEC>;
///Field `DATA3` reader - Data Bytes 3DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA3_R = crate::FieldReader;
///Field `DATA3` writer - Data Bytes 3DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data Bytes 3DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Data Bytes 3DATA3 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7.
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<MB_D3_SPEC> {
        DATA3_W::new(self, 0)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_d3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_D3_SPEC;
impl crate::RegisterSpec for MB_D3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mb_d3::R`](R) reader structure
impl crate::Readable for MB_D3_SPEC {}
///`write(|w| ..)` method takes [`mb_d3::W`](W) writer structure
impl crate::Writable for MB_D3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_D3 to value 0
impl crate::Resettable for MB_D3_SPEC {}
