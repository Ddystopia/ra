///Register `PIPE%sTRN` reader
pub type R = crate::R<PIPETRN_SPEC>;
///Register `PIPE%sTRN` writer
pub type W = crate::W<PIPETRN_SPEC>;
///Field `TRNCNT` reader - Transaction Counter
pub type TRNCNT_R = crate::FieldReader<u16>;
///Field `TRNCNT` writer - Transaction Counter
pub type TRNCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Transaction Counter
    #[inline(always)]
    pub fn trncnt(&self) -> TRNCNT_R {
        TRNCNT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Transaction Counter
    #[inline(always)]
    pub fn trncnt(&mut self) -> TRNCNT_W<PIPETRN_SPEC> {
        TRNCNT_W::new(self, 0)
    }
}
/**Pipe %s Transaction Counter Register

You can [`read`](crate::Reg::read) this register and get [`pipetrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPETRN_SPEC;
impl crate::RegisterSpec for PIPETRN_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipetrn::R`](R) reader structure
impl crate::Readable for PIPETRN_SPEC {}
///`write(|w| ..)` method takes [`pipetrn::W`](W) writer structure
impl crate::Writable for PIPETRN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPE%sTRN to value 0
impl crate::Resettable for PIPETRN_SPEC {}
