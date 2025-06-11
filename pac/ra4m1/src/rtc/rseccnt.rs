///Register `RSECCNT` reader
pub type R = crate::R<RSECCNT_SPEC>;
///Register `RSECCNT` writer
pub type W = crate::W<RSECCNT_SPEC>;
///Field `SEC1` reader - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place.
pub type SEC1_R = crate::FieldReader;
///Field `SEC1` writer - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place.
pub type SEC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEC10` reader - 10-Second Count Counts from 0 to 5 for 60-second counting.
pub type SEC10_R = crate::FieldReader;
///Field `SEC10` writer - 10-Second Count Counts from 0 to 5 for 60-second counting.
pub type SEC10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Second Count Counts from 0 to 5 for 60-second counting.
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    ///Bits 0:3 - 1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<RSECCNT_SPEC> {
        SEC1_W::new(self, 0)
    }
    ///Bits 4:6 - 10-Second Count Counts from 0 to 5 for 60-second counting.
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W<RSECCNT_SPEC> {
        SEC10_W::new(self, 4)
    }
}
/**Second Counter

You can [`read`](crate::Reg::read) this register and get [`rseccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rseccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSECCNT_SPEC;
impl crate::RegisterSpec for RSECCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rseccnt::R`](R) reader structure
impl crate::Readable for RSECCNT_SPEC {}
///`write(|w| ..)` method takes [`rseccnt::W`](W) writer structure
impl crate::Writable for RSECCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSECCNT to value 0
impl crate::Resettable for RSECCNT_SPEC {}
