///Register `MMPUEC%s` reader
pub type R = crate::R<MMPUEC_SPEC>;
///Register `MMPUEC%s` writer
pub type W = crate::W<MMPUEC_SPEC>;
///Field `MMPUEC` reader - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
pub type MMPUEC_R = crate::FieldReader<u32>;
///Field `MMPUEC` writer - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
pub type MMPUEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mmpuec(&self) -> MMPUEC_R {
        MMPUEC_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mmpuec(&mut self) -> MMPUEC_W<MMPUEC_SPEC> {
        MMPUEC_W::new(self, 0)
    }
}
/**Group C Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpuec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUEC_SPEC;
impl crate::RegisterSpec for MMPUEC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmpuec::R`](R) reader structure
impl crate::Readable for MMPUEC_SPEC {}
///`write(|w| ..)` method takes [`mmpuec::W`](W) writer structure
impl crate::Writable for MMPUEC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUEC%s to value 0x03
impl crate::Resettable for MMPUEC_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
