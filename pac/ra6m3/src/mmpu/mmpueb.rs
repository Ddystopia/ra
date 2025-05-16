///Register `MMPUEB%s` reader
pub type R = crate::R<MMPUEB_SPEC>;
///Register `MMPUEB%s` writer
pub type W = crate::W<MMPUEB_SPEC>;
///Field `MMPUEB` reader - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
pub type MMPUEB_R = crate::FieldReader<u32>;
///Field `MMPUEB` writer - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
pub type MMPUEB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mmpueb(&self) -> MMPUEB_R {
        MMPUEB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mmpueb(&mut self) -> MMPUEB_W<MMPUEB_SPEC> {
        MMPUEB_W::new(self, 0)
    }
}
/**Group B Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpueb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpueb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUEB_SPEC;
impl crate::RegisterSpec for MMPUEB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmpueb::R`](R) reader structure
impl crate::Readable for MMPUEB_SPEC {}
///`write(|w| ..)` method takes [`mmpueb::W`](W) writer structure
impl crate::Writable for MMPUEB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUEB%s to value 0x03
impl crate::Resettable for MMPUEB_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
