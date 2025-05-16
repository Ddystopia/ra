///Register `RTCCR%s` reader
pub type R = crate::R<RTCCR_SPEC>;
///Register `RTCCR%s` writer
pub type W = crate::W<RTCCR_SPEC>;
/**Time Capture Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCT_A {
    ///0: No event is detected.
    _00 = 0,
    ///1: Rising edge is detected.
    _01 = 1,
    ///2: Falling edge is detected.
    _10 = 2,
    ///3: Both edges are detected.
    _11 = 3,
}
impl From<TCCT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCT_A {
    type Ux = u8;
}
impl crate::IsEnum for TCCT_A {}
///Field `TCCT` reader - Time Capture Control
pub type TCCT_R = crate::FieldReader<TCCT_A>;
impl TCCT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCCT_A {
        match self.bits {
            0 => TCCT_A::_00,
            1 => TCCT_A::_01,
            2 => TCCT_A::_10,
            3 => TCCT_A::_11,
            _ => unreachable!(),
        }
    }
    ///No event is detected.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCCT_A::_00
    }
    ///Rising edge is detected.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCCT_A::_01
    }
    ///Falling edge is detected.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCCT_A::_10
    }
    ///Both edges are detected.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCCT_A::_11
    }
}
///Field `TCCT` writer - Time Capture Control
pub type TCCT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCT_A, crate::Safe>;
impl<'a, REG> TCCT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event is detected.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_00)
    }
    ///Rising edge is detected.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_01)
    }
    ///Falling edge is detected.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_10)
    }
    ///Both edges are detected.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_11)
    }
}
/**Time Capture Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCST_A {
    ///0: No event is detected.
    _0 = 0,
    ///1: An event is detected.
    _1 = 1,
}
impl From<TCST_A> for bool {
    #[inline(always)]
    fn from(variant: TCST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCST` reader - Time Capture Status
pub type TCST_R = crate::BitReader<TCST_A>;
impl TCST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCST_A {
        match self.bits {
            false => TCST_A::_0,
            true => TCST_A::_1,
        }
    }
    ///No event is detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCST_A::_0
    }
    ///An event is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCST_A::_1
    }
}
/**Time Capture Noise Filter Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCNF_A {
    ///0: The noise filter is off.
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: The noise filter is on (count source).
    _10 = 2,
    ///3: The noise filter is on (count source by divided by 32).
    _11 = 3,
}
impl From<TCNF_A> for u8 {
    #[inline(always)]
    fn from(variant: TCNF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCNF_A {
    type Ux = u8;
}
impl crate::IsEnum for TCNF_A {}
///Field `TCNF` reader - Time Capture Noise Filter Control
pub type TCNF_R = crate::FieldReader<TCNF_A>;
impl TCNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCNF_A {
        match self.bits {
            0 => TCNF_A::_00,
            1 => TCNF_A::_01,
            2 => TCNF_A::_10,
            3 => TCNF_A::_11,
            _ => unreachable!(),
        }
    }
    ///The noise filter is off.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCNF_A::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCNF_A::_01
    }
    ///The noise filter is on (count source).
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCNF_A::_10
    }
    ///The noise filter is on (count source by divided by 32).
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCNF_A::_11
    }
}
///Field `TCNF` writer - Time Capture Noise Filter Control
pub type TCNF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCNF_A, crate::Safe>;
impl<'a, REG> TCNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The noise filter is off.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_01)
    }
    ///The noise filter is on (count source).
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_10)
    }
    ///The noise filter is on (count source by divided by 32).
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Time Capture Control
    #[inline(always)]
    pub fn tcct(&self) -> TCCT_R {
        TCCT_R::new(self.bits & 3)
    }
    ///Bit 2 - Time Capture Status
    #[inline(always)]
    pub fn tcst(&self) -> TCST_R {
        TCST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Time Capture Noise Filter Control
    #[inline(always)]
    pub fn tcnf(&self) -> TCNF_R {
        TCNF_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    ///Bits 0:1 - Time Capture Control
    #[inline(always)]
    pub fn tcct(&mut self) -> TCCT_W<RTCCR_SPEC> {
        TCCT_W::new(self, 0)
    }
    ///Bits 4:5 - Time Capture Noise Filter Control
    #[inline(always)]
    pub fn tcnf(&mut self) -> TCNF_W<RTCCR_SPEC> {
        TCNF_W::new(self, 4)
    }
}
/**Time Capture Control Register %s

You can [`read`](crate::Reg::read) this register and get [`rtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTCCR_SPEC;
impl crate::RegisterSpec for RTCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rtccr::R`](R) reader structure
impl crate::Readable for RTCCR_SPEC {}
///`write(|w| ..)` method takes [`rtccr::W`](W) writer structure
impl crate::Writable for RTCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTCCR%s to value 0
impl crate::Resettable for RTCCR_SPEC {}
