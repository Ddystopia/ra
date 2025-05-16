///Register `MKR[%s]` reader
pub type R = crate::R<MKR_SPEC>;
///Register `MKR[%s]` writer
pub type W = crate::W<MKR_SPEC>;
///Field `EID` reader - Extended ID
pub type EID_R = crate::FieldReader<u32>;
///Field `EID` writer - Extended ID
pub type EID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `SID` reader - Standard ID
pub type SID_R = crate::FieldReader<u16>;
///Field `SID` writer - Standard ID
pub type SID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:17 - Extended ID
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:28 - Standard ID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:17 - Extended ID
    #[inline(always)]
    pub fn eid(&mut self) -> EID_W<MKR_SPEC> {
        EID_W::new(self, 0)
    }
    ///Bits 18:28 - Standard ID
    #[inline(always)]
    pub fn sid(&mut self) -> SID_W<MKR_SPEC> {
        SID_W::new(self, 18)
    }
}
/**Mask Register

You can [`read`](crate::Reg::read) this register and get [`mkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MKR_SPEC;
impl crate::RegisterSpec for MKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mkr::R`](R) reader structure
impl crate::Readable for MKR_SPEC {}
///`write(|w| ..)` method takes [`mkr::W`](W) writer structure
impl crate::Writable for MKR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKR[%s] to value 0
impl crate::Resettable for MKR_SPEC {}
