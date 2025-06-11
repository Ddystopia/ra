///Register `RMINCNT` reader
pub type R = crate::R<RMINCNT_SPEC>;
///Register `RMINCNT` writer
pub type W = crate::W<RMINCNT_SPEC>;
///Field `MIN1` reader - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place.
pub type MIN1_R = crate::FieldReader;
///Field `MIN1` writer - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place.
pub type MIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MIN10` reader - 10-Minute Count Counts from 0 to 5 for 60-minute counting.
pub type MIN10_R = crate::FieldReader;
///Field `MIN10` writer - 10-Minute Count Counts from 0 to 5 for 60-minute counting.
pub type MIN10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Minute Count Counts from 0 to 5 for 60-minute counting.
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    ///Bits 0:3 - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn min1(&mut self) -> MIN1_W<RMINCNT_SPEC> {
        MIN1_W::new(self, 0)
    }
    ///Bits 4:6 - 10-Minute Count Counts from 0 to 5 for 60-minute counting.
    #[inline(always)]
    pub fn min10(&mut self) -> MIN10_W<RMINCNT_SPEC> {
        MIN10_W::new(self, 4)
    }
}
/**Minute Counter

You can [`read`](crate::Reg::read) this register and get [`rmincnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmincnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMINCNT_SPEC;
impl crate::RegisterSpec for RMINCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rmincnt::R`](R) reader structure
impl crate::Readable for RMINCNT_SPEC {}
///`write(|w| ..)` method takes [`rmincnt::W`](W) writer structure
impl crate::Writable for RMINCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMINCNT to value 0
impl crate::Resettable for RMINCNT_SPEC {}
