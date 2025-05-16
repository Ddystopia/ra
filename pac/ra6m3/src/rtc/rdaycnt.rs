///Register `RDAYCNT` reader
pub type R = crate::R<RDAYCNT_SPEC>;
///Register `RDAYCNT` writer
pub type W = crate::W<RDAYCNT_SPEC>;
///Field `DATE1` reader - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place.
pub type DATE1_R = crate::FieldReader;
///Field `DATE1` writer - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place.
pub type DATE1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATE10` reader - 10-Day Count Counts from 0 to 3 once per carry from the ones place.
pub type DATE10_R = crate::FieldReader;
///Field `DATE10` writer - 10-Day Count Counts from 0 to 3 once per carry from the ones place.
pub type DATE10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Day Count Counts from 0 to 3 once per carry from the ones place.
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    ///Bits 0:3 - 1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn date1(&mut self) -> DATE1_W<RDAYCNT_SPEC> {
        DATE1_W::new(self, 0)
    }
    ///Bits 4:5 - 10-Day Count Counts from 0 to 3 once per carry from the ones place.
    #[inline(always)]
    pub fn date10(&mut self) -> DATE10_W<RDAYCNT_SPEC> {
        DATE10_W::new(self, 4)
    }
}
/**Day Counter

You can [`read`](crate::Reg::read) this register and get [`rdaycnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdaycnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDAYCNT_SPEC;
impl crate::RegisterSpec for RDAYCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rdaycnt::R`](R) reader structure
impl crate::Readable for RDAYCNT_SPEC {}
///`write(|w| ..)` method takes [`rdaycnt::W`](W) writer structure
impl crate::Writable for RDAYCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDAYCNT to value 0
impl crate::Resettable for RDAYCNT_SPEC {}
