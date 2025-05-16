///Register `GAM%s_AREA2` reader
pub type R = crate::R<GAM_AREA2_SPEC>;
///Register `GAM%s_AREA2` writer
pub type W = crate::W<GAM_AREA2_SPEC>;
///Field `TH06` reader - Start threshold of area 6Unsigned 10-bit integer
pub type TH06_R = crate::FieldReader<u16>;
///Field `TH06` writer - Start threshold of area 6Unsigned 10-bit integer
pub type TH06_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH05` reader - Start threshold of area 5Unsigned 10-bit integer
pub type TH05_R = crate::FieldReader<u16>;
///Field `TH05` writer - Start threshold of area 5Unsigned 10-bit integer
pub type TH05_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH04` reader - Start threshold of area 4Unsigned 10-bit integer
pub type TH04_R = crate::FieldReader<u16>;
///Field `TH04` writer - Start threshold of area 4Unsigned 10-bit integer
pub type TH04_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Start threshold of area 6Unsigned 10-bit integer
    #[inline(always)]
    pub fn th06(&self) -> TH06_R {
        TH06_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Start threshold of area 5Unsigned 10-bit integer
    #[inline(always)]
    pub fn th05(&self) -> TH05_R {
        TH05_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - Start threshold of area 4Unsigned 10-bit integer
    #[inline(always)]
    pub fn th04(&self) -> TH04_R {
        TH04_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Start threshold of area 6Unsigned 10-bit integer
    #[inline(always)]
    pub fn th06(&mut self) -> TH06_W<GAM_AREA2_SPEC> {
        TH06_W::new(self, 0)
    }
    ///Bits 10:19 - Start threshold of area 5Unsigned 10-bit integer
    #[inline(always)]
    pub fn th05(&mut self) -> TH05_W<GAM_AREA2_SPEC> {
        TH05_W::new(self, 10)
    }
    ///Bits 20:29 - Start threshold of area 4Unsigned 10-bit integer
    #[inline(always)]
    pub fn th04(&mut self) -> TH04_W<GAM_AREA2_SPEC> {
        TH04_W::new(self, 20)
    }
}
/**Gamma %s Correction Block Area Setting Register 2

You can [`read`](crate::Reg::read) this register and get [`gam_area2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_AREA2_SPEC;
impl crate::RegisterSpec for GAM_AREA2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_area2::R`](R) reader structure
impl crate::Readable for GAM_AREA2_SPEC {}
///`write(|w| ..)` method takes [`gam_area2::W`](W) writer structure
impl crate::Writable for GAM_AREA2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_AREA2 to value 0
impl crate::Resettable for GAM_AREA2_SPEC {}
