///Register `RFLR` reader
pub type R = crate::R<RFLR_SPEC>;
///Register `RFLR` writer
pub type W = crate::W<RFLR_SPEC>;
///Field `RFL` reader - Receive Frame Maximum LengthThe set value becomes the maximum frame length. The minimum value that can be set is 1,518 bytes, and the maximum value that can be set is 2,048 bytes. Values that are less than 1,518 bytes are regarded as 1,518 bytes, and values larger than 2,048 bytes are regarded as 2,048 bytes.
pub type RFL_R = crate::FieldReader<u16>;
///Field `RFL` writer - Receive Frame Maximum LengthThe set value becomes the maximum frame length. The minimum value that can be set is 1,518 bytes, and the maximum value that can be set is 2,048 bytes. Values that are less than 1,518 bytes are regarded as 1,518 bytes, and values larger than 2,048 bytes are regarded as 2,048 bytes.
pub type RFL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Receive Frame Maximum LengthThe set value becomes the maximum frame length. The minimum value that can be set is 1,518 bytes, and the maximum value that can be set is 2,048 bytes. Values that are less than 1,518 bytes are regarded as 1,518 bytes, and values larger than 2,048 bytes are regarded as 2,048 bytes.
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Receive Frame Maximum LengthThe set value becomes the maximum frame length. The minimum value that can be set is 1,518 bytes, and the maximum value that can be set is 2,048 bytes. Values that are less than 1,518 bytes are regarded as 1,518 bytes, and values larger than 2,048 bytes are regarded as 2,048 bytes.
    #[inline(always)]
    pub fn rfl(&mut self) -> RFL_W<RFLR_SPEC> {
        RFL_W::new(self, 0)
    }
}
/**Receive Frame Maximum Length Register

You can [`read`](crate::Reg::read) this register and get [`rflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFLR_SPEC;
impl crate::RegisterSpec for RFLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rflr::R`](R) reader structure
impl crate::Readable for RFLR_SPEC {}
///`write(|w| ..)` method takes [`rflr::W`](W) writer structure
impl crate::Writable for RFLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFLR to value 0
impl crate::Resettable for RFLR_SPEC {}
