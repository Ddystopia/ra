///Register `RWKCNT` reader
pub type R = crate::R<RWKCNT_SPEC>;
///Register `RWKCNT` writer
pub type W = crate::W<RWKCNT_SPEC>;
/**Day-of-Week Counting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAYW_A {
    ///0: Sunday
    _000 = 0,
    ///1: Monday
    _001 = 1,
    ///2: Tuesday
    _010 = 2,
    ///3: Wednesday
    _011 = 3,
    ///4: Thursday
    _100 = 4,
    ///5: Friday
    _101 = 5,
    ///6: Saturday
    _110 = 6,
    ///7: Setting Prohibited
    _111 = 7,
}
impl From<DAYW_A> for u8 {
    #[inline(always)]
    fn from(variant: DAYW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAYW_A {
    type Ux = u8;
}
impl crate::IsEnum for DAYW_A {}
///Field `DAYW` reader - Day-of-Week Counting
pub type DAYW_R = crate::FieldReader<DAYW_A>;
impl DAYW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAYW_A {
        match self.bits {
            0 => DAYW_A::_000,
            1 => DAYW_A::_001,
            2 => DAYW_A::_010,
            3 => DAYW_A::_011,
            4 => DAYW_A::_100,
            5 => DAYW_A::_101,
            6 => DAYW_A::_110,
            7 => DAYW_A::_111,
            _ => unreachable!(),
        }
    }
    ///Sunday
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DAYW_A::_000
    }
    ///Monday
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DAYW_A::_001
    }
    ///Tuesday
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DAYW_A::_010
    }
    ///Wednesday
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DAYW_A::_011
    }
    ///Thursday
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DAYW_A::_100
    }
    ///Friday
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DAYW_A::_101
    }
    ///Saturday
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DAYW_A::_110
    }
    ///Setting Prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DAYW_A::_111
    }
}
///Field `DAYW` writer - Day-of-Week Counting
pub type DAYW_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DAYW_A, crate::Safe>;
impl<'a, REG> DAYW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sunday
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_000)
    }
    ///Monday
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_001)
    }
    ///Tuesday
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_010)
    }
    ///Wednesday
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_011)
    }
    ///Thursday
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_100)
    }
    ///Friday
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_101)
    }
    ///Saturday
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_110)
    }
    ///Setting Prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_111)
    }
}
impl R {
    ///Bits 0:2 - Day-of-Week Counting
    #[inline(always)]
    pub fn dayw(&self) -> DAYW_R {
        DAYW_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - Day-of-Week Counting
    #[inline(always)]
    pub fn dayw(&mut self) -> DAYW_W<RWKCNT_SPEC> {
        DAYW_W::new(self, 0)
    }
}
/**Day-of-Week Counter

You can [`read`](crate::Reg::read) this register and get [`rwkcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RWKCNT_SPEC;
impl crate::RegisterSpec for RWKCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rwkcnt::R`](R) reader structure
impl crate::Readable for RWKCNT_SPEC {}
///`write(|w| ..)` method takes [`rwkcnt::W`](W) writer structure
impl crate::Writable for RWKCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RWKCNT to value 0
impl crate::Resettable for RWKCNT_SPEC {}
