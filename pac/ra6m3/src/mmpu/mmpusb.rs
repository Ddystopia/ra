///Register `MMPUSB%s` reader
pub type R = crate::R<MMPUSB_SPEC>;
///Register `MMPUSB%s` writer
pub type W = crate::W<MMPUSB_SPEC>;
///Field `MMPUSB` reader - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
pub type MMPUSB_R = crate::FieldReader<u32>;
///Field `MMPUSB` writer - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
pub type MMPUSB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn mmpusb(&self) -> MMPUSB_R {
        MMPUSB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn mmpusb(&mut self) -> MMPUSB_W<MMPUSB_SPEC> {
        MMPUSB_W::new(self, 0)
    }
}
/**Group B Region %s Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpusb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUSB_SPEC;
impl crate::RegisterSpec for MMPUSB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmpusb::R`](R) reader structure
impl crate::Readable for MMPUSB_SPEC {}
///`write(|w| ..)` method takes [`mmpusb::W`](W) writer structure
impl crate::Writable for MMPUSB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUSB%s to value 0
impl crate::Resettable for MMPUSB_SPEC {}
