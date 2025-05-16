///Register `DADR%s` reader
pub type R = crate::R<DADR_SPEC>;
///Register `DADR%s` writer
pub type W = crate::W<DADR_SPEC>;
///Field `DADR` reader - D/A Data RegisterNOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
pub type DADR_R = crate::FieldReader<u16>;
///Field `DADR` writer - D/A Data RegisterNOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
pub type DADR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - D/A Data RegisterNOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - D/A Data RegisterNOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
    #[inline(always)]
    pub fn dadr(&mut self) -> DADR_W<DADR_SPEC> {
        DADR_W::new(self, 0)
    }
}
/**D/A Data Register %s

You can [`read`](crate::Reg::read) this register and get [`dadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DADR_SPEC;
impl crate::RegisterSpec for DADR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dadr::R`](R) reader structure
impl crate::Readable for DADR_SPEC {}
///`write(|w| ..)` method takes [`dadr::W`](W) writer structure
impl crate::Writable for DADR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DADR%s to value 0
impl crate::Resettable for DADR_SPEC {}
