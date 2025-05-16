///Register `CTSUSO0` reader
pub type R = crate::R<CTSUSO0_SPEC>;
///Register `CTSUSO0` writer
pub type W = crate::W<CTSUSO0_SPEC>;
///Field `CTSUSO` reader - CTSU Sensor Offset AdjustmentCurrent offset amount is CTSUSO ( 0 to 1023 )
pub type CTSUSO_R = crate::FieldReader<u16>;
///Field `CTSUSO` writer - CTSU Sensor Offset AdjustmentCurrent offset amount is CTSUSO ( 0 to 1023 )
pub type CTSUSO_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CTSUSNUM` reader - CTSU Measurement Count Setting
pub type CTSUSNUM_R = crate::FieldReader;
///Field `CTSUSNUM` writer - CTSU Measurement Count Setting
pub type CTSUSNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:9 - CTSU Sensor Offset AdjustmentCurrent offset amount is CTSUSO ( 0 to 1023 )
    #[inline(always)]
    pub fn ctsuso(&self) -> CTSUSO_R {
        CTSUSO_R::new(self.bits & 0x03ff)
    }
    ///Bits 10:15 - CTSU Measurement Count Setting
    #[inline(always)]
    pub fn ctsusnum(&self) -> CTSUSNUM_R {
        CTSUSNUM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:9 - CTSU Sensor Offset AdjustmentCurrent offset amount is CTSUSO ( 0 to 1023 )
    #[inline(always)]
    pub fn ctsuso(&mut self) -> CTSUSO_W<CTSUSO0_SPEC> {
        CTSUSO_W::new(self, 0)
    }
    ///Bits 10:15 - CTSU Measurement Count Setting
    #[inline(always)]
    pub fn ctsusnum(&mut self) -> CTSUSNUM_W<CTSUSO0_SPEC> {
        CTSUSNUM_W::new(self, 10)
    }
}
/**CTSU Sensor Offset Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuso0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUSO0_SPEC;
impl crate::RegisterSpec for CTSUSO0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctsuso0::R`](R) reader structure
impl crate::Readable for CTSUSO0_SPEC {}
///`write(|w| ..)` method takes [`ctsuso0::W`](W) writer structure
impl crate::Writable for CTSUSO0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSO0 to value 0
impl crate::Resettable for CTSUSO0_SPEC {}
