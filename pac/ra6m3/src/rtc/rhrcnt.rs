///Register `RHRCNT` reader
pub type R = crate::R<RHRCNT_SPEC>;
///Register `RHRCNT` writer
pub type W = crate::W<RHRCNT_SPEC>;
///Field `HR1` reader - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place.
pub type HR1_R = crate::FieldReader;
///Field `HR1` writer - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place.
pub type HR1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HR10` reader - 10-Hour Count Counts from 0 to 2 once per carry from the ones place.
pub type HR10_R = crate::FieldReader;
///Field `HR10` writer - 10-Hour Count Counts from 0 to 2 once per carry from the ones place.
pub type HR10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Time Counter Setting for a.m./p.m.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    ///0: a.m.
    _0 = 0,
    ///1: p.m.
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - Time Counter Setting for a.m./p.m.
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    ///a.m.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    ///p.m.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
///Field `PM` writer - Time Counter Setting for a.m./p.m.
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM_A>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///a.m.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_0)
    }
    ///p.m.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_1)
    }
}
impl R {
    ///Bits 0:3 - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn hr1(&self) -> HR1_R {
        HR1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Hour Count Counts from 0 to 2 once per carry from the ones place.
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - Time Counter Setting for a.m./p.m.
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place.
    #[inline(always)]
    pub fn hr1(&mut self) -> HR1_W<RHRCNT_SPEC> {
        HR1_W::new(self, 0)
    }
    ///Bits 4:5 - 10-Hour Count Counts from 0 to 2 once per carry from the ones place.
    #[inline(always)]
    pub fn hr10(&mut self) -> HR10_W<RHRCNT_SPEC> {
        HR10_W::new(self, 4)
    }
    ///Bit 6 - Time Counter Setting for a.m./p.m.
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<RHRCNT_SPEC> {
        PM_W::new(self, 6)
    }
}
/**Hour Counter

You can [`read`](crate::Reg::read) this register and get [`rhrcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RHRCNT_SPEC;
impl crate::RegisterSpec for RHRCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rhrcnt::R`](R) reader structure
impl crate::Readable for RHRCNT_SPEC {}
///`write(|w| ..)` method takes [`rhrcnt::W`](W) writer structure
impl crate::Writable for RHRCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RHRCNT to value 0
impl crate::Resettable for RHRCNT_SPEC {}
