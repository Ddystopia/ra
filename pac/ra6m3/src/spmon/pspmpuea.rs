///Register `PSPMPUEA` reader
pub type R = crate::R<PSPMPUEA_SPEC>;
///Register `PSPMPUEA` writer
pub type W = crate::W<PSPMPUEA_SPEC>;
///Field `PSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
pub type PSPMPUEA_R = crate::FieldReader<u32>;
///Field `PSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
pub type PSPMPUEA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn pspmpuea(&self) -> PSPMPUEA_R {
        PSPMPUEA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn pspmpuea(&mut self) -> PSPMPUEA_W<PSPMPUEA_SPEC> {
        PSPMPUEA_W::new(self, 2)
    }
}
/**Process Stack Pointer Monitor End Address Register

You can [`read`](crate::Reg::read) this register and get [`pspmpuea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PSPMPUEA_SPEC;
impl crate::RegisterSpec for PSPMPUEA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pspmpuea::R`](R) reader structure
impl crate::Readable for PSPMPUEA_SPEC {}
///`write(|w| ..)` method takes [`pspmpuea::W`](W) writer structure
impl crate::Writable for PSPMPUEA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSPMPUEA to value 0x03
impl crate::Resettable for PSPMPUEA_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
