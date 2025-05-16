///Register `CTSUSDPRS` reader
pub type R = crate::R<CTSUSDPRS_SPEC>;
///Register `CTSUSDPRS` writer
pub type W = crate::W<CTSUSDPRS_SPEC>;
///Field `CTSUPRRATIO` reader - CTSU Measurement Time and Pulse Count AdjustmentRecommended setting: 3 (0011b)
pub type CTSUPRRATIO_R = crate::FieldReader;
///Field `CTSUPRRATIO` writer - CTSU Measurement Time and Pulse Count AdjustmentRecommended setting: 3 (0011b)
pub type CTSUPRRATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**CTSU Base Period and Pulse Count Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUPRMODE_A {
    ///0: 510 pulses
    _00 = 0,
    ///1: 126 pulses
    _01 = 1,
    ///2: 62 pulses (recommended setting value)
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<CTSUPRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUPRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUPRMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUPRMODE_A {}
///Field `CTSUPRMODE` reader - CTSU Base Period and Pulse Count Setting
pub type CTSUPRMODE_R = crate::FieldReader<CTSUPRMODE_A>;
impl CTSUPRMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUPRMODE_A {
        match self.bits {
            0 => CTSUPRMODE_A::_00,
            1 => CTSUPRMODE_A::_01,
            2 => CTSUPRMODE_A::_10,
            3 => CTSUPRMODE_A::_11,
            _ => unreachable!(),
        }
    }
    ///510 pulses
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUPRMODE_A::_00
    }
    ///126 pulses
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUPRMODE_A::_01
    }
    ///62 pulses (recommended setting value)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUPRMODE_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUPRMODE_A::_11
    }
}
///Field `CTSUPRMODE` writer - CTSU Base Period and Pulse Count Setting
pub type CTSUPRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CTSUPRMODE_A, crate::Safe>;
impl<'a, REG> CTSUPRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///510 pulses
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUPRMODE_A::_00)
    }
    ///126 pulses
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUPRMODE_A::_01)
    }
    ///62 pulses (recommended setting value)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUPRMODE_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUPRMODE_A::_11)
    }
}
/**CTSU High-Pass Noise Reduction Function Off Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSOFF_A {
    ///0: High-pass noise reduction function turned on
    _0 = 0,
    ///1: High-pass noise reduction function turned off
    _1 = 1,
}
impl From<CTSUSOFF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUSOFF` reader - CTSU High-Pass Noise Reduction Function Off Setting
pub type CTSUSOFF_R = crate::BitReader<CTSUSOFF_A>;
impl CTSUSOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSOFF_A {
        match self.bits {
            false => CTSUSOFF_A::_0,
            true => CTSUSOFF_A::_1,
        }
    }
    ///High-pass noise reduction function turned on
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSOFF_A::_0
    }
    ///High-pass noise reduction function turned off
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSOFF_A::_1
    }
}
///Field `CTSUSOFF` writer - CTSU High-Pass Noise Reduction Function Off Setting
pub type CTSUSOFF_W<'a, REG> = crate::BitWriter<'a, REG, CTSUSOFF_A>;
impl<'a, REG> CTSUSOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-pass noise reduction function turned on
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSOFF_A::_0)
    }
    ///High-pass noise reduction function turned off
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSOFF_A::_1)
    }
}
impl R {
    ///Bits 0:3 - CTSU Measurement Time and Pulse Count AdjustmentRecommended setting: 3 (0011b)
    #[inline(always)]
    pub fn ctsuprratio(&self) -> CTSUPRRATIO_R {
        CTSUPRRATIO_R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - CTSU Base Period and Pulse Count Setting
    #[inline(always)]
    pub fn ctsuprmode(&self) -> CTSUPRMODE_R {
        CTSUPRMODE_R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - CTSU High-Pass Noise Reduction Function Off Setting
    #[inline(always)]
    pub fn ctsusoff(&self) -> CTSUSOFF_R {
        CTSUSOFF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - CTSU Measurement Time and Pulse Count AdjustmentRecommended setting: 3 (0011b)
    #[inline(always)]
    pub fn ctsuprratio(&mut self) -> CTSUPRRATIO_W<CTSUSDPRS_SPEC> {
        CTSUPRRATIO_W::new(self, 0)
    }
    ///Bits 4:5 - CTSU Base Period and Pulse Count Setting
    #[inline(always)]
    pub fn ctsuprmode(&mut self) -> CTSUPRMODE_W<CTSUSDPRS_SPEC> {
        CTSUPRMODE_W::new(self, 4)
    }
    ///Bit 6 - CTSU High-Pass Noise Reduction Function Off Setting
    #[inline(always)]
    pub fn ctsusoff(&mut self) -> CTSUSOFF_W<CTSUSDPRS_SPEC> {
        CTSUSOFF_W::new(self, 6)
    }
}
/**CTSU Synchronous Noise Reduction Setting Register

You can [`read`](crate::Reg::read) this register and get [`ctsusdprs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusdprs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUSDPRS_SPEC;
impl crate::RegisterSpec for CTSUSDPRS_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsusdprs::R`](R) reader structure
impl crate::Readable for CTSUSDPRS_SPEC {}
///`write(|w| ..)` method takes [`ctsusdprs::W`](W) writer structure
impl crate::Writable for CTSUSDPRS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSDPRS to value 0
impl crate::Resettable for CTSUSDPRS_SPEC {}
