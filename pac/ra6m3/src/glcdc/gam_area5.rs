///Register `GAM%s_AREA5` reader
pub type R = crate::R<GAM_AREA5_SPEC>;
///Register `GAM%s_AREA5` writer
pub type W = crate::W<GAM_AREA5_SPEC>;
///Field `TH15` reader - Start threshold of area 15Unsigned 10-bit integer
pub type TH15_R = crate::FieldReader<u16>;
///Field `TH15` writer - Start threshold of area 15Unsigned 10-bit integer
pub type TH15_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH14` reader - Start threshold of area 14Unsigned 10-bit integer
pub type TH14_R = crate::FieldReader<u16>;
///Field `TH14` writer - Start threshold of area 14Unsigned 10-bit integer
pub type TH14_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH13` reader - Start threshold of area 13Unsigned 10-bit integer
pub type TH13_R = crate::FieldReader<u16>;
///Field `TH13` writer - Start threshold of area 13Unsigned 10-bit integer
pub type TH13_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Start threshold of area 15Unsigned 10-bit integer
    #[inline(always)]
    pub fn th15(&self) -> TH15_R {
        TH15_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Start threshold of area 14Unsigned 10-bit integer
    #[inline(always)]
    pub fn th14(&self) -> TH14_R {
        TH14_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - Start threshold of area 13Unsigned 10-bit integer
    #[inline(always)]
    pub fn th13(&self) -> TH13_R {
        TH13_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Start threshold of area 15Unsigned 10-bit integer
    #[inline(always)]
    pub fn th15(&mut self) -> TH15_W<GAM_AREA5_SPEC> {
        TH15_W::new(self, 0)
    }
    ///Bits 10:19 - Start threshold of area 14Unsigned 10-bit integer
    #[inline(always)]
    pub fn th14(&mut self) -> TH14_W<GAM_AREA5_SPEC> {
        TH14_W::new(self, 10)
    }
    ///Bits 20:29 - Start threshold of area 13Unsigned 10-bit integer
    #[inline(always)]
    pub fn th13(&mut self) -> TH13_W<GAM_AREA5_SPEC> {
        TH13_W::new(self, 20)
    }
}
/**Gamma %s Correction Block Area Setting Register 5

You can [`read`](crate::Reg::read) this register and get [`gam_area5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_AREA5_SPEC;
impl crate::RegisterSpec for GAM_AREA5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_area5::R`](R) reader structure
impl crate::Readable for GAM_AREA5_SPEC {}
///`write(|w| ..)` method takes [`gam_area5::W`](W) writer structure
impl crate::Writable for GAM_AREA5_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_AREA5 to value 0
impl crate::Resettable for GAM_AREA5_SPEC {}
