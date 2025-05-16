///Register `CTSUSST` reader
pub type R = crate::R<CTSUSST_SPEC>;
///Register `CTSUSST` writer
pub type W = crate::W<CTSUSST_SPEC>;
///Field `CTSUSST` reader - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b.
pub type CTSUSST_R = crate::FieldReader;
///Field `CTSUSST` writer - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b.
pub type CTSUSST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b.
    #[inline(always)]
    pub fn ctsusst(&self) -> CTSUSST_R {
        CTSUSST_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b.
    #[inline(always)]
    pub fn ctsusst(&mut self) -> CTSUSST_W<CTSUSST_SPEC> {
        CTSUSST_W::new(self, 0)
    }
}
/**CTSU Sensor Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsusst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUSST_SPEC;
impl crate::RegisterSpec for CTSUSST_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsusst::R`](R) reader structure
impl crate::Readable for CTSUSST_SPEC {}
///`write(|w| ..)` method takes [`ctsusst::W`](W) writer structure
impl crate::Writable for CTSUSST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSST to value 0
impl crate::Resettable for CTSUSST_SPEC {}
