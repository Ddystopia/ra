///Register `DODIR` reader
pub type R = crate::R<DODIR_SPEC>;
///Register `DODIR` writer
pub type W = crate::W<DODIR_SPEC>;
///Field `DODIR` reader - 16-bit read-write register in which 16-bit data for use in the operations are stored.
pub type DODIR_R = crate::FieldReader<u16>;
///Field `DODIR` writer - 16-bit read-write register in which 16-bit data for use in the operations are stored.
pub type DODIR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 16-bit read-write register in which 16-bit data for use in the operations are stored.
    #[inline(always)]
    pub fn dodir(&self) -> DODIR_R {
        DODIR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - 16-bit read-write register in which 16-bit data for use in the operations are stored.
    #[inline(always)]
    pub fn dodir(&mut self) -> DODIR_W<DODIR_SPEC> {
        DODIR_W::new(self, 0)
    }
}
/**DOC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`dodir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DODIR_SPEC;
impl crate::RegisterSpec for DODIR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dodir::R`](R) reader structure
impl crate::Readable for DODIR_SPEC {}
///`write(|w| ..)` method takes [`dodir::W`](W) writer structure
impl crate::Writable for DODIR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DODIR to value 0
impl crate::Resettable for DODIR_SPEC {}
