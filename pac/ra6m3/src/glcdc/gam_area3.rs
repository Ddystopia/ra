///Register `GAM%s_AREA3` reader
pub type R = crate::R<GAM_AREA3_SPEC>;
///Register `GAM%s_AREA3` writer
pub type W = crate::W<GAM_AREA3_SPEC>;
///Field `TH09` reader - Start threshold of area 9Unsigned 10-bit integer
pub type TH09_R = crate::FieldReader<u16>;
///Field `TH09` writer - Start threshold of area 9Unsigned 10-bit integer
pub type TH09_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH08` reader - Start threshold of area 8Unsigned 10-bit integer
pub type TH08_R = crate::FieldReader<u16>;
///Field `TH08` writer - Start threshold of area 8Unsigned 10-bit integer
pub type TH08_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TH07` reader - Start threshold of area 7Unsigned 10-bit integer
pub type TH07_R = crate::FieldReader<u16>;
///Field `TH07` writer - Start threshold of area 7Unsigned 10-bit integer
pub type TH07_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Start threshold of area 9Unsigned 10-bit integer
    #[inline(always)]
    pub fn th09(&self) -> TH09_R {
        TH09_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Start threshold of area 8Unsigned 10-bit integer
    #[inline(always)]
    pub fn th08(&self) -> TH08_R {
        TH08_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - Start threshold of area 7Unsigned 10-bit integer
    #[inline(always)]
    pub fn th07(&self) -> TH07_R {
        TH07_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Start threshold of area 9Unsigned 10-bit integer
    #[inline(always)]
    pub fn th09(&mut self) -> TH09_W<GAM_AREA3_SPEC> {
        TH09_W::new(self, 0)
    }
    ///Bits 10:19 - Start threshold of area 8Unsigned 10-bit integer
    #[inline(always)]
    pub fn th08(&mut self) -> TH08_W<GAM_AREA3_SPEC> {
        TH08_W::new(self, 10)
    }
    ///Bits 20:29 - Start threshold of area 7Unsigned 10-bit integer
    #[inline(always)]
    pub fn th07(&mut self) -> TH07_W<GAM_AREA3_SPEC> {
        TH07_W::new(self, 20)
    }
}
/**Gamma %s Correction Block Area Setting Register 3

You can [`read`](crate::Reg::read) this register and get [`gam_area3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_AREA3_SPEC;
impl crate::RegisterSpec for GAM_AREA3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_area3::R`](R) reader structure
impl crate::Readable for GAM_AREA3_SPEC {}
///`write(|w| ..)` method takes [`gam_area3::W`](W) writer structure
impl crate::Writable for GAM_AREA3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_AREA3 to value 0
impl crate::Resettable for GAM_AREA3_SPEC {}
