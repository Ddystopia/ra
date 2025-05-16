///Register `MSPMPUEA` reader
pub type R = crate::R<MSPMPUEA_SPEC>;
///Register `MSPMPUEA` writer
pub type W = crate::W<MSPMPUEA_SPEC>;
///Field `MSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
pub type MSPMPUEA_R = crate::FieldReader<u32>;
///Field `MSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
pub type MSPMPUEA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mspmpuea(&self) -> MSPMPUEA_R {
        MSPMPUEA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - Region end address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1.
    #[inline(always)]
    pub fn mspmpuea(&mut self) -> MSPMPUEA_W<MSPMPUEA_SPEC> {
        MSPMPUEA_W::new(self, 2)
    }
}
/**Main Stack Pointer Monitor End Address Register

You can [`read`](crate::Reg::read) this register and get [`mspmpuea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSPMPUEA_SPEC;
impl crate::RegisterSpec for MSPMPUEA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mspmpuea::R`](R) reader structure
impl crate::Readable for MSPMPUEA_SPEC {}
///`write(|w| ..)` method takes [`mspmpuea::W`](W) writer structure
impl crate::Writable for MSPMPUEA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSPMPUEA to value 0x03
impl crate::Resettable for MSPMPUEA_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
