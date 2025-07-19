///Register `DADR0` reader
pub type R = crate::R<DADR0_SPEC>;
///Register `DADR0` writer
pub type W = crate::W<DADR0_SPEC>;
///Field `DADR` reader - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
pub type DADR_R = crate::FieldReader<u16>;
///Field `DADR` writer - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
pub type DADR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format.
    #[inline(always)]
    pub fn dadr(&mut self) -> DADR_W<'_, DADR0_SPEC> {
        DADR_W::new(self, 0)
    }
}
/**D/A Data Register 0

You can [`read`](crate::Reg::read) this register and get [`dadr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DADR0_SPEC;
impl crate::RegisterSpec for DADR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dadr0::R`](R) reader structure
impl crate::Readable for DADR0_SPEC {}
///`write(|w| ..)` method takes [`dadr0::W`](W) writer structure
impl crate::Writable for DADR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DADR0 to value 0
impl crate::Resettable for DADR0_SPEC {}
