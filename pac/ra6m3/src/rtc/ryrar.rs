///Register `RYRAR` reader
pub type R = crate::R<RYRAR_SPEC>;
///Register `RYRAR` writer
pub type W = crate::W<RYRAR_SPEC>;
///Field `YR1` reader - 1 Year Value for the ones place of years
pub type YR1_R = crate::FieldReader;
///Field `YR1` writer - 1 Year Value for the ones place of years
pub type YR1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `YR10` reader - 10 Years Value for the tens place of years
pub type YR10_R = crate::FieldReader;
///Field `YR10` writer - 10 Years Value for the tens place of years
pub type YR10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - 1 Year Value for the ones place of years
    #[inline(always)]
    pub fn yr1(&self) -> YR1_R {
        YR1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 10 Years Value for the tens place of years
    #[inline(always)]
    pub fn yr10(&self) -> YR10_R {
        YR10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - 1 Year Value for the ones place of years
    #[inline(always)]
    pub fn yr1(&mut self) -> YR1_W<RYRAR_SPEC> {
        YR1_W::new(self, 0)
    }
    ///Bits 4:7 - 10 Years Value for the tens place of years
    #[inline(always)]
    pub fn yr10(&mut self) -> YR10_W<RYRAR_SPEC> {
        YR10_W::new(self, 4)
    }
}
/**Year Alarm Register

You can [`read`](crate::Reg::read) this register and get [`ryrar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RYRAR_SPEC;
impl crate::RegisterSpec for RYRAR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ryrar::R`](R) reader structure
impl crate::Readable for RYRAR_SPEC {}
///`write(|w| ..)` method takes [`ryrar::W`](W) writer structure
impl crate::Writable for RYRAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RYRAR to value 0
impl crate::Resettable for RYRAR_SPEC {}
