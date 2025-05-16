///Register `MSPMPUSA` reader
pub type R = crate::R<MSPMPUSA_SPEC>;
///Register `MSPMPUSA` writer
pub type W = crate::W<MSPMPUSA_SPEC>;
///Field `MSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
pub type MSPMPUSA_R = crate::FieldReader<u32>;
///Field `MSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
pub type MSPMPUSA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn mspmpusa(&self) -> MSPMPUSA_R {
        MSPMPUSA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn mspmpusa(&mut self) -> MSPMPUSA_W<MSPMPUSA_SPEC> {
        MSPMPUSA_W::new(self, 2)
    }
}
/**Main Stack Pointer Monitor Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mspmpusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSPMPUSA_SPEC;
impl crate::RegisterSpec for MSPMPUSA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mspmpusa::R`](R) reader structure
impl crate::Readable for MSPMPUSA_SPEC {}
///`write(|w| ..)` method takes [`mspmpusa::W`](W) writer structure
impl crate::Writable for MSPMPUSA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSPMPUSA to value 0
impl crate::Resettable for MSPMPUSA_SPEC {}
