///Register `MMPUSC%s` reader
pub type R = crate::R<MMPUSC_SPEC>;
///Register `MMPUSC%s` writer
pub type W = crate::W<MMPUSC_SPEC>;
///Field `MMPUSC` reader - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
pub type MMPUSC_R = crate::FieldReader<u32>;
///Field `MMPUSC` writer - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
pub type MMPUSC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn mmpusc(&self) -> MMPUSC_R {
        MMPUSC_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0.
    #[inline(always)]
    pub fn mmpusc(&mut self) -> MMPUSC_W<MMPUSC_SPEC> {
        MMPUSC_W::new(self, 0)
    }
}
/**Group C Region %s Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpusc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUSC_SPEC;
impl crate::RegisterSpec for MMPUSC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmpusc::R`](R) reader structure
impl crate::Readable for MMPUSC_SPEC {}
///`write(|w| ..)` method takes [`mmpusc::W`](W) writer structure
impl crate::Writable for MMPUSC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUSC%s to value 0
impl crate::Resettable for MMPUSC_SPEC {}
