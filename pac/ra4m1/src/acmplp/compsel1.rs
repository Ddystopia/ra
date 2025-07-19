///Register `COMPSEL1` reader
pub type R = crate::R<COMPSEL1_SPEC>;
///Register `COMPSEL1` writer
pub type W = crate::W<COMPSEL1_SPEC>;
/**ACMPLP0 Reference Voltage(IVREF0) Selection*

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS20_A {
    ///0: No input
    _000 = 0,
    ///1: CMPREF0 (P101)
    _001 = 1,
    ///2: DAC8 (ch0) output
    _010 = 2,
    ///4: CMPREF0 (P502)
    _100 = 4,
    ///3: settings prohibited.
    OTHERS = 3,
}
impl From<CRVS20_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS20_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRVS20_A {
    type Ux = u8;
}
impl crate::IsEnum for CRVS20_A {}
///Field `CRVS20` reader - ACMPLP0 Reference Voltage(IVREF0) Selection*
pub type CRVS20_R = crate::FieldReader<CRVS20_A>;
impl CRVS20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRVS20_A {
        match self.bits {
            0 => CRVS20_A::_000,
            1 => CRVS20_A::_001,
            2 => CRVS20_A::_010,
            4 => CRVS20_A::_100,
            _ => CRVS20_A::OTHERS,
        }
    }
    ///No input
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CRVS20_A::_000
    }
    ///CMPREF0 (P101)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CRVS20_A::_001
    }
    ///DAC8 (ch0) output
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CRVS20_A::_010
    }
    ///CMPREF0 (P502)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CRVS20_A::_100
    }
    ///settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CRVS20_A::OTHERS)
    }
}
///Field `CRVS20` writer - ACMPLP0 Reference Voltage(IVREF0) Selection*
pub type CRVS20_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CRVS20_A, crate::Safe>;
impl<'a, REG> CRVS20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No input
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS20_A::_000)
    }
    ///CMPREF0 (P101)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS20_A::_001)
    }
    ///DAC8 (ch0) output
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS20_A::_010)
    }
    ///CMPREF0 (P502)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS20_A::_100)
    }
    ///settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS20_A::OTHERS)
    }
}
/**ACMPLP1 Reference Voltage(IVREF1) Selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS64_A {
    ///0: No input
    _000 = 0,
    ///1: CMPREF1 (P103)
    _001 = 1,
    ///2: DAC8 (ch1) output
    _010 = 2,
    ///4: CMPREF1 (P500)
    _100 = 4,
    ///3: settings prohibited.
    OTHERS = 3,
}
impl From<CRVS64_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS64_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRVS64_A {
    type Ux = u8;
}
impl crate::IsEnum for CRVS64_A {}
///Field `CRVS64` reader - ACMPLP1 Reference Voltage(IVREF1) Selection
pub type CRVS64_R = crate::FieldReader<CRVS64_A>;
impl CRVS64_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRVS64_A {
        match self.bits {
            0 => CRVS64_A::_000,
            1 => CRVS64_A::_001,
            2 => CRVS64_A::_010,
            4 => CRVS64_A::_100,
            _ => CRVS64_A::OTHERS,
        }
    }
    ///No input
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CRVS64_A::_000
    }
    ///CMPREF1 (P103)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CRVS64_A::_001
    }
    ///DAC8 (ch1) output
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CRVS64_A::_010
    }
    ///CMPREF1 (P500)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CRVS64_A::_100
    }
    ///settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CRVS64_A::OTHERS)
    }
}
///Field `CRVS64` writer - ACMPLP1 Reference Voltage(IVREF1) Selection
pub type CRVS64_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CRVS64_A, crate::Safe>;
impl<'a, REG> CRVS64_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No input
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS64_A::_000)
    }
    ///CMPREF1 (P103)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS64_A::_001)
    }
    ///DAC8 (ch1) output
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS64_A::_010)
    }
    ///CMPREF1 (P500)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS64_A::_100)
    }
    ///settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS64_A::OTHERS)
    }
}
/**ACMPLP1 Reference Voltage Selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1VRF2_A {
    ///0: IVREF0 selected
    _0 = 0,
    ///1: IVREF1 selected.
    _1 = 1,
}
impl From<C1VRF2_A> for bool {
    #[inline(always)]
    fn from(variant: C1VRF2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1VRF2` reader - ACMPLP1 Reference Voltage Selection
pub type C1VRF2_R = crate::BitReader<C1VRF2_A>;
impl C1VRF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1VRF2_A {
        match self.bits {
            false => C1VRF2_A::_0,
            true => C1VRF2_A::_1,
        }
    }
    ///IVREF0 selected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1VRF2_A::_0
    }
    ///IVREF1 selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1VRF2_A::_1
    }
}
///Field `C1VRF2` writer - ACMPLP1 Reference Voltage Selection
pub type C1VRF2_W<'a, REG> = crate::BitWriter<'a, REG, C1VRF2_A>;
impl<'a, REG> C1VRF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IVREF0 selected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1VRF2_A::_0)
    }
    ///IVREF1 selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1VRF2_A::_1)
    }
}
impl R {
    ///Bits 0:2 - ACMPLP0 Reference Voltage(IVREF0) Selection*
    #[inline(always)]
    pub fn crvs20(&self) -> CRVS20_R {
        CRVS20_R::new(self.bits & 7)
    }
    ///Bits 4:6 - ACMPLP1 Reference Voltage(IVREF1) Selection
    #[inline(always)]
    pub fn crvs64(&self) -> CRVS64_R {
        CRVS64_R::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - ACMPLP1 Reference Voltage Selection
    #[inline(always)]
    pub fn c1vrf2(&self) -> C1VRF2_R {
        C1VRF2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - ACMPLP0 Reference Voltage(IVREF0) Selection*
    #[inline(always)]
    pub fn crvs20(&mut self) -> CRVS20_W<'_, COMPSEL1_SPEC> {
        CRVS20_W::new(self, 0)
    }
    ///Bits 4:6 - ACMPLP1 Reference Voltage(IVREF1) Selection
    #[inline(always)]
    pub fn crvs64(&mut self) -> CRVS64_W<'_, COMPSEL1_SPEC> {
        CRVS64_W::new(self, 4)
    }
    ///Bit 7 - ACMPLP1 Reference Voltage Selection
    #[inline(always)]
    pub fn c1vrf2(&mut self) -> C1VRF2_W<'_, COMPSEL1_SPEC> {
        C1VRF2_W::new(self, 7)
    }
}
/**Comparator Reference Voltage Select Register

You can [`read`](crate::Reg::read) this register and get [`compsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMPSEL1_SPEC;
impl crate::RegisterSpec for COMPSEL1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`compsel1::R`](R) reader structure
impl crate::Readable for COMPSEL1_SPEC {}
///`write(|w| ..)` method takes [`compsel1::W`](W) writer structure
impl crate::Writable for COMPSEL1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMPSEL1 to value 0x91
impl crate::Resettable for COMPSEL1_SPEC {
    const RESET_VALUE: u8 = 0x91;
}
