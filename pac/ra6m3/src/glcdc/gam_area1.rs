///Register `GAM%s_AREA1` reader
pub type R = crate::R<GAM_AREA1_SPEC>;
///Register `GAM%s_AREA1` writer
pub type W = crate::W<GAM_AREA1_SPEC>;
///Field `TH03` reader - Start threshold of area 3Unsigned 10-bit integer
pub type TH03_R = crate::FieldReader<u16>;
///Field `TH03` writer - Start threshold of area 3Unsigned 10-bit integer
pub type TH03_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH02` reader - Start threshold of area 2Unsigned 10-bit integer
pub type TH02_R = crate::FieldReader<u16>;
///Field `TH02` writer - Start threshold of area 2Unsigned 10-bit integer
pub type TH02_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH01` reader - Start threshold of area 1Unsigned 10-bit integer
pub type TH01_R = crate::FieldReader<u16>;
///Field `TH01` writer - Start threshold of area 1Unsigned 10-bit integer
pub type TH01_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Start threshold of area 3Unsigned 10-bit integer
    #[inline(always)]
    pub fn th03(&self) -> TH03_R {
        TH03_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Start threshold of area 2Unsigned 10-bit integer
    #[inline(always)]
    pub fn th02(&self) -> TH02_R {
        TH02_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - Start threshold of area 1Unsigned 10-bit integer
    #[inline(always)]
    pub fn th01(&self) -> TH01_R {
        TH01_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Start threshold of area 3Unsigned 10-bit integer
    #[inline(always)]
    pub fn th03(&mut self) -> TH03_W<GAM_AREA1_SPEC> {
        TH03_W::new(self, 0)
    }
    ///Bits 10:19 - Start threshold of area 2Unsigned 10-bit integer
    #[inline(always)]
    pub fn th02(&mut self) -> TH02_W<GAM_AREA1_SPEC> {
        TH02_W::new(self, 10)
    }
    ///Bits 20:29 - Start threshold of area 1Unsigned 10-bit integer
    #[inline(always)]
    pub fn th01(&mut self) -> TH01_W<GAM_AREA1_SPEC> {
        TH01_W::new(self, 20)
    }
}
/**Gamma %s Correction Block Area Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`gam_area1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_AREA1_SPEC;
impl crate::RegisterSpec for GAM_AREA1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_area1::R`](R) reader structure
impl crate::Readable for GAM_AREA1_SPEC {}
///`write(|w| ..)` method takes [`gam_area1::W`](W) writer structure
impl crate::Writable for GAM_AREA1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_AREA1 to value 0
impl crate::Resettable for GAM_AREA1_SPEC {}
