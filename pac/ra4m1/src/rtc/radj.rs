///Register `RADJ` reader
pub type R = crate::R<RADJ_SPEC>;
///Register `RADJ` writer
pub type W = crate::W<RADJ_SPEC>;
///Field `ADJ` reader - Adjustment Value These bits specify the adjustment value from the prescaler.
pub type ADJ_R = crate::FieldReader;
///Field `ADJ` writer - Adjustment Value These bits specify the adjustment value from the prescaler.
pub type ADJ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Plus-Minus

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMADJ_A {
    ///0: Adjustment is not performed.
    _00 = 0,
    ///1: Adjustment is performed by the addition to the prescaler.
    _01 = 1,
    ///2: Adjustment is performed by the subtraction from the prescaler.
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<PMADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: PMADJ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PMADJ_A {
    type Ux = u8;
}
impl crate::IsEnum for PMADJ_A {}
///Field `PMADJ` reader - Plus-Minus
pub type PMADJ_R = crate::FieldReader<PMADJ_A>;
impl PMADJ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMADJ_A {
        match self.bits {
            0 => PMADJ_A::_00,
            1 => PMADJ_A::_01,
            2 => PMADJ_A::_10,
            3 => PMADJ_A::_11,
            _ => unreachable!(),
        }
    }
    ///Adjustment is not performed.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PMADJ_A::_00
    }
    ///Adjustment is performed by the addition to the prescaler.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PMADJ_A::_01
    }
    ///Adjustment is performed by the subtraction from the prescaler.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PMADJ_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PMADJ_A::_11
    }
}
///Field `PMADJ` writer - Plus-Minus
pub type PMADJ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PMADJ_A, crate::Safe>;
impl<'a, REG> PMADJ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Adjustment is not performed.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_00)
    }
    ///Adjustment is performed by the addition to the prescaler.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_01)
    }
    ///Adjustment is performed by the subtraction from the prescaler.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_11)
    }
}
impl R {
    ///Bits 0:5 - Adjustment Value These bits specify the adjustment value from the prescaler.
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(self.bits & 0x3f)
    }
    ///Bits 6:7 - Plus-Minus
    #[inline(always)]
    pub fn pmadj(&self) -> PMADJ_R {
        PMADJ_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bits 0:5 - Adjustment Value These bits specify the adjustment value from the prescaler.
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W<RADJ_SPEC> {
        ADJ_W::new(self, 0)
    }
    ///Bits 6:7 - Plus-Minus
    #[inline(always)]
    pub fn pmadj(&mut self) -> PMADJ_W<RADJ_SPEC> {
        PMADJ_W::new(self, 6)
    }
}
/**Time Error Adjustment Register

You can [`read`](crate::Reg::read) this register and get [`radj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RADJ_SPEC;
impl crate::RegisterSpec for RADJ_SPEC {
    type Ux = u8;
}
///`read()` method returns [`radj::R`](R) reader structure
impl crate::Readable for RADJ_SPEC {}
///`write(|w| ..)` method takes [`radj::W`](W) writer structure
impl crate::Writable for RADJ_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RADJ to value 0
impl crate::Resettable for RADJ_SPEC {}
