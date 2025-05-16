///Register `PSPMPUSA` reader
pub type R = crate::R<PSPMPUSA_SPEC>;
///Register `PSPMPUSA` writer
pub type W = crate::W<PSPMPUSA_SPEC>;
///Field `PSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
pub type PSPMPUSA_R = crate::FieldReader<u32>;
///Field `PSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
pub type PSPMPUSA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn pspmpusa(&self) -> PSPMPUSA_R {
        PSPMPUSA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - Region start address register Address where the region starts, for use in region determination.NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn pspmpusa(&mut self) -> PSPMPUSA_W<PSPMPUSA_SPEC> {
        PSPMPUSA_W::new(self, 2)
    }
}
/**Process Stack Pointer Monitor Start Address Register

You can [`read`](crate::Reg::read) this register and get [`pspmpusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PSPMPUSA_SPEC;
impl crate::RegisterSpec for PSPMPUSA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pspmpusa::R`](R) reader structure
impl crate::Readable for PSPMPUSA_SPEC {}
///`write(|w| ..)` method takes [`pspmpusa::W`](W) writer structure
impl crate::Writable for PSPMPUSA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSPMPUSA to value 0
impl crate::Resettable for PSPMPUSA_SPEC {}
