///Register `RYRCNT` reader
pub type R = crate::R<RYRCNT_SPEC>;
///Register `RYRCNT` writer
pub type W = crate::W<RYRCNT_SPEC>;
///Field `YR1` reader - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place.
pub type YR1_R = crate::FieldReader;
///Field `YR1` writer - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place.
pub type YR1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `YR10` reader - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place.
pub type YR10_R = crate::FieldReader;
///Field `YR10` writer - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place.
pub type YR10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn yr1(&self) -> YR1_R {
        YR1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place.
    #[inline(always)]
    pub fn yr10(&self) -> YR10_R {
        YR10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - 1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn yr1(&mut self) -> YR1_W<RYRCNT_SPEC> {
        YR1_W::new(self, 0)
    }
    ///Bits 4:7 - 10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place.
    #[inline(always)]
    pub fn yr10(&mut self) -> YR10_W<RYRCNT_SPEC> {
        YR10_W::new(self, 4)
    }
}
/**Year Counter

You can [`read`](crate::Reg::read) this register and get [`ryrcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RYRCNT_SPEC;
impl crate::RegisterSpec for RYRCNT_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ryrcnt::R`](R) reader structure
impl crate::Readable for RYRCNT_SPEC {}
///`write(|w| ..)` method takes [`ryrcnt::W`](W) writer structure
impl crate::Writable for RYRCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RYRCNT to value 0
impl crate::Resettable for RYRCNT_SPEC {}
