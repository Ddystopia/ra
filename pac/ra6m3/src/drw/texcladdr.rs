///Register `TEXCLADDR` writer
pub type W = crate::W<TEXCLADDR_SPEC>;
///Field `CLADDR` writer - Texture CLUT start address for indexed texture format
pub type CLADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Texture CLUT start address for indexed texture format
    #[inline(always)]
    pub fn claddr(&mut self) -> CLADDR_W<TEXCLADDR_SPEC> {
        CLADDR_W::new(self, 0)
    }
}
/**CLUT Start Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texcladdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXCLADDR_SPEC;
impl crate::RegisterSpec for TEXCLADDR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`texcladdr::W`](W) writer structure
impl crate::Writable for TEXCLADDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEXCLADDR to value 0
impl crate::Resettable for TEXCLADDR_SPEC {}
