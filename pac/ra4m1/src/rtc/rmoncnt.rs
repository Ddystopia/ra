///Register `RMONCNT` reader
pub type R = crate::R<RMONCNT_SPEC>;
///Register `RMONCNT` writer
pub type W = crate::W<RMONCNT_SPEC>;
///Field `MON1` reader - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place.
pub type MON1_R = crate::FieldReader;
///Field `MON1` writer - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place.
pub type MON1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MON10` reader - 10-Month Count Counts from 0 to 1 once per carry from the ones place.
pub type MON10_R = crate::BitReader;
///Field `MON10` writer - 10-Month Count Counts from 0 to 1 once per carry from the ones place.
pub type MON10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    ///Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place.
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn mon1(&mut self) -> MON1_W<RMONCNT_SPEC> {
        MON1_W::new(self, 0)
    }
    ///Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place.
    #[inline(always)]
    pub fn mon10(&mut self) -> MON10_W<RMONCNT_SPEC> {
        MON10_W::new(self, 4)
    }
}
/**Month Counter

You can [`read`](crate::Reg::read) this register and get [`rmoncnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmoncnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMONCNT_SPEC;
impl crate::RegisterSpec for RMONCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rmoncnt::R`](R) reader structure
impl crate::Readable for RMONCNT_SPEC {}
///`write(|w| ..)` method takes [`rmoncnt::W`](W) writer structure
impl crate::Writable for RMONCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMONCNT to value 0
impl crate::Resettable for RMONCNT_SPEC {}
