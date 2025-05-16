///Register `GAM%s_AREA4` reader
pub type R = crate::R<GAM_AREA4_SPEC>;
///Register `GAM%s_AREA4` writer
pub type W = crate::W<GAM_AREA4_SPEC>;
///Field `TH12` reader - Start threshold of area 12Unsigned 10-bit integer
pub type TH12_R = crate::FieldReader<u16>;
///Field `TH12` writer - Start threshold of area 12Unsigned 10-bit integer
pub type TH12_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH11` reader - Start threshold of area 11Unsigned 10-bit integer
pub type TH11_R = crate::FieldReader<u16>;
///Field `TH11` writer - Start threshold of area 11Unsigned 10-bit integer
pub type TH11_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH10` reader - Start threshold of area 10Unsigned 10-bit integer
pub type TH10_R = crate::FieldReader<u16>;
///Field `TH10` writer - Start threshold of area 10Unsigned 10-bit integer
pub type TH10_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Start threshold of area 12Unsigned 10-bit integer
    #[inline(always)]
    pub fn th12(&self) -> TH12_R {
        TH12_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Start threshold of area 11Unsigned 10-bit integer
    #[inline(always)]
    pub fn th11(&self) -> TH11_R {
        TH11_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - Start threshold of area 10Unsigned 10-bit integer
    #[inline(always)]
    pub fn th10(&self) -> TH10_R {
        TH10_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Start threshold of area 12Unsigned 10-bit integer
    #[inline(always)]
    pub fn th12(&mut self) -> TH12_W<GAM_AREA4_SPEC> {
        TH12_W::new(self, 0)
    }
    ///Bits 10:19 - Start threshold of area 11Unsigned 10-bit integer
    #[inline(always)]
    pub fn th11(&mut self) -> TH11_W<GAM_AREA4_SPEC> {
        TH11_W::new(self, 10)
    }
    ///Bits 20:29 - Start threshold of area 10Unsigned 10-bit integer
    #[inline(always)]
    pub fn th10(&mut self) -> TH10_W<GAM_AREA4_SPEC> {
        TH10_W::new(self, 20)
    }
}
/**Gamma %s Correction Block Area Setting Register 4

You can [`read`](crate::Reg::read) this register and get [`gam_area4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_AREA4_SPEC;
impl crate::RegisterSpec for GAM_AREA4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_area4::R`](R) reader structure
impl crate::Readable for GAM_AREA4_SPEC {}
///`write(|w| ..)` method takes [`gam_area4::W`](W) writer structure
impl crate::Writable for GAM_AREA4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_AREA4 to value 0
impl crate::Resettable for GAM_AREA4_SPEC {}
