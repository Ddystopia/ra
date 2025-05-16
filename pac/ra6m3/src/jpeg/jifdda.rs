///Register `JIFDDA` reader
pub type R = crate::R<JIFDDA_SPEC>;
///Register `JIFDDA` writer
pub type W = crate::W<JIFDDA_SPEC>;
///Field `DDA` reader - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0.
pub type DDA_R = crate::FieldReader<u32>;
///Field `DDA` writer - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0.
pub type DDA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn dda(&self) -> DDA_R {
        DDA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn dda(&mut self) -> DDA_W<JIFDDA_SPEC> {
        DDA_W::new(self, 0)
    }
}
/**JPEG Interface Decompression Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`jifdda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDDA_SPEC;
impl crate::RegisterSpec for JIFDDA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifdda::R`](R) reader structure
impl crate::Readable for JIFDDA_SPEC {}
///`write(|w| ..)` method takes [`jifdda::W`](W) writer structure
impl crate::Writable for JIFDDA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDDA to value 0
impl crate::Resettable for JIFDDA_SPEC {}
