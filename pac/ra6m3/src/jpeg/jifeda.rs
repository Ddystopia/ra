///Register `JIFEDA` reader
pub type R = crate::R<JIFEDA_SPEC>;
///Register `JIFEDA` writer
pub type W = crate::W<JIFEDA_SPEC>;
///Field `EDA` reader - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
pub type EDA_R = crate::FieldReader<u32>;
///Field `EDA` writer - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
pub type EDA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn eda(&self) -> EDA_R {
        EDA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn eda(&mut self) -> EDA_W<JIFEDA_SPEC> {
        EDA_W::new(self, 0)
    }
}
/**JPEG Interface Compression Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`jifeda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifeda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFEDA_SPEC;
impl crate::RegisterSpec for JIFEDA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifeda::R`](R) reader structure
impl crate::Readable for JIFEDA_SPEC {}
///`write(|w| ..)` method takes [`jifeda::W`](W) writer structure
impl crate::Writable for JIFEDA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFEDA to value 0
impl crate::Resettable for JIFEDA_SPEC {}
