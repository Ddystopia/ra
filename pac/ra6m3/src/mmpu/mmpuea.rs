///Register `MMPUEA%s` reader
pub type R = crate::R<MMPUEA_SPEC>;
///Register `MMPUEA%s` writer
pub type W = crate::W<MMPUEA_SPEC>;
///Field `MMPUEA` reader - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
pub type MMPUEA_R = crate::FieldReader<u32>;
///Field `MMPUEA` writer - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
pub type MMPUEA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mmpuea(&self) -> MMPUEA_R {
        MMPUEA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mmpuea(&mut self) -> MMPUEA_W<MMPUEA_SPEC> {
        MMPUEA_W::new(self, 0)
    }
}
/**Group A Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpuea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUEA_SPEC;
impl crate::RegisterSpec for MMPUEA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmpuea::R`](R) reader structure
impl crate::Readable for MMPUEA_SPEC {}
///`write(|w| ..)` method takes [`mmpuea::W`](W) writer structure
impl crate::Writable for MMPUEA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUEA%s to value 0x03
impl crate::Resettable for MMPUEA_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
