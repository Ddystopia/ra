///Register `GTITC` reader
pub type R = crate::R<GTITC_SPEC>;
///Register `GTITC` writer
pub type W = crate::W<GTITC_SPEC>;
/**GTCCRA Compare Match/Input Capture Interrupt Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLA_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ITLA_A> for bool {
    #[inline(always)]
    fn from(variant: ITLA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITLA` reader - GTCCRA Compare Match/Input Capture Interrupt Link
pub type ITLA_R = crate::BitReader<ITLA_A>;
impl ITLA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITLA_A {
        match self.bits {
            false => ITLA_A::_0,
            true => ITLA_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLA_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLA_A::_1
    }
}
///Field `ITLA` writer - GTCCRA Compare Match/Input Capture Interrupt Link
pub type ITLA_W<'a, REG> = crate::BitWriter<'a, REG, ITLA_A>;
impl<'a, REG> ITLA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ITLA_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ITLA_A::_1)
    }
}
/**GTCCRB Compare Match/Input Capture Interrupt Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLB_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ITLB_A> for bool {
    #[inline(always)]
    fn from(variant: ITLB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITLB` reader - GTCCRB Compare Match/Input Capture Interrupt Link
pub type ITLB_R = crate::BitReader<ITLB_A>;
impl ITLB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITLB_A {
        match self.bits {
            false => ITLB_A::_0,
            true => ITLB_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLB_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLB_A::_1
    }
}
///Field `ITLB` writer - GTCCRB Compare Match/Input Capture Interrupt Link
pub type ITLB_W<'a, REG> = crate::BitWriter<'a, REG, ITLB_A>;
impl<'a, REG> ITLB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ITLB_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ITLB_A::_1)
    }
}
/**GTCCRC Compare Match Interrupt Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLC_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ITLC_A> for bool {
    #[inline(always)]
    fn from(variant: ITLC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITLC` reader - GTCCRC Compare Match Interrupt Link
pub type ITLC_R = crate::BitReader<ITLC_A>;
impl ITLC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITLC_A {
        match self.bits {
            false => ITLC_A::_0,
            true => ITLC_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLC_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLC_A::_1
    }
}
///Field `ITLC` writer - GTCCRC Compare Match Interrupt Link
pub type ITLC_W<'a, REG> = crate::BitWriter<'a, REG, ITLC_A>;
impl<'a, REG> ITLC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ITLC_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ITLC_A::_1)
    }
}
/**GTCCRD Compare Match Interrupt Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLD_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ITLD_A> for bool {
    #[inline(always)]
    fn from(variant: ITLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITLD` reader - GTCCRD Compare Match Interrupt Link
pub type ITLD_R = crate::BitReader<ITLD_A>;
impl ITLD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITLD_A {
        match self.bits {
            false => ITLD_A::_0,
            true => ITLD_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLD_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLD_A::_1
    }
}
///Field `ITLD` writer - GTCCRD Compare Match Interrupt Link
pub type ITLD_W<'a, REG> = crate::BitWriter<'a, REG, ITLD_A>;
impl<'a, REG> ITLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ITLD_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ITLD_A::_1)
    }
}
/**GTCCRE Compare Match Interrupt Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLE_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ITLE_A> for bool {
    #[inline(always)]
    fn from(variant: ITLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITLE` reader - GTCCRE Compare Match Interrupt Link
pub type ITLE_R = crate::BitReader<ITLE_A>;
impl ITLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITLE_A {
        match self.bits {
            false => ITLE_A::_0,
            true => ITLE_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLE_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLE_A::_1
    }
}
///Field `ITLE` writer - GTCCRE Compare Match Interrupt Link
pub type ITLE_W<'a, REG> = crate::BitWriter<'a, REG, ITLE_A>;
impl<'a, REG> ITLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ITLE_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ITLE_A::_1)
    }
}
/**GTCCRF Compare Match Interrupt Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITLF_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ITLF_A> for bool {
    #[inline(always)]
    fn from(variant: ITLF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITLF` reader - GTCCRF Compare Match Interrupt Link
pub type ITLF_R = crate::BitReader<ITLF_A>;
impl ITLF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITLF_A {
        match self.bits {
            false => ITLF_A::_0,
            true => ITLF_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITLF_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITLF_A::_1
    }
}
///Field `ITLF` writer - GTCCRF Compare Match Interrupt Link
pub type ITLF_W<'a, REG> = crate::BitWriter<'a, REG, ITLF_A>;
impl<'a, REG> ITLF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ITLF_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ITLF_A::_1)
    }
}
/**GPT_OVF/GPT_UDF Interrupt Skipping Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IVTC_A {
    ///0: Do not perform skipping
    _00 = 0,
    ///1: Count and skip both overflow and underflow for saw waves and crest for triangle waves
    _01 = 1,
    ///2: Count and skip both overflow and underflow for saw waves and trough for triangle waves
    _10 = 2,
    ///3: Count and skip both overflow and underflow for saw waves and both crest and trough for triangle waves.
    _11 = 3,
}
impl From<IVTC_A> for u8 {
    #[inline(always)]
    fn from(variant: IVTC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IVTC_A {
    type Ux = u8;
}
impl crate::IsEnum for IVTC_A {}
///Field `IVTC` reader - GPT_OVF/GPT_UDF Interrupt Skipping Function Select
pub type IVTC_R = crate::FieldReader<IVTC_A>;
impl IVTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IVTC_A {
        match self.bits {
            0 => IVTC_A::_00,
            1 => IVTC_A::_01,
            2 => IVTC_A::_10,
            3 => IVTC_A::_11,
            _ => unreachable!(),
        }
    }
    ///Do not perform skipping
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IVTC_A::_00
    }
    ///Count and skip both overflow and underflow for saw waves and crest for triangle waves
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IVTC_A::_01
    }
    ///Count and skip both overflow and underflow for saw waves and trough for triangle waves
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IVTC_A::_10
    }
    ///Count and skip both overflow and underflow for saw waves and both crest and trough for triangle waves.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IVTC_A::_11
    }
}
///Field `IVTC` writer - GPT_OVF/GPT_UDF Interrupt Skipping Function Select
pub type IVTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IVTC_A, crate::Safe>;
impl<'a, REG> IVTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not perform skipping
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IVTC_A::_00)
    }
    ///Count and skip both overflow and underflow for saw waves and crest for triangle waves
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IVTC_A::_01)
    }
    ///Count and skip both overflow and underflow for saw waves and trough for triangle waves
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IVTC_A::_10)
    }
    ///Count and skip both overflow and underflow for saw waves and both crest and trough for triangle waves.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IVTC_A::_11)
    }
}
/**GPT_OVF/GPT_UDF Interrupt Skipping Count Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IVTT_A {
    ///0: No skipping
    _000 = 0,
    ///1: Skipping count of 1
    _001 = 1,
    ///2: Skipping count of 2
    _010 = 2,
    ///3: Skipping count of 3
    _011 = 3,
    ///4: Skipping count of 4
    _100 = 4,
    ///5: Skipping count of 5
    _101 = 5,
    ///6: Skipping count of 6
    _110 = 6,
    ///7: Skipping count of 7.
    _111 = 7,
}
impl From<IVTT_A> for u8 {
    #[inline(always)]
    fn from(variant: IVTT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IVTT_A {
    type Ux = u8;
}
impl crate::IsEnum for IVTT_A {}
///Field `IVTT` reader - GPT_OVF/GPT_UDF Interrupt Skipping Count Select
pub type IVTT_R = crate::FieldReader<IVTT_A>;
impl IVTT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IVTT_A {
        match self.bits {
            0 => IVTT_A::_000,
            1 => IVTT_A::_001,
            2 => IVTT_A::_010,
            3 => IVTT_A::_011,
            4 => IVTT_A::_100,
            5 => IVTT_A::_101,
            6 => IVTT_A::_110,
            7 => IVTT_A::_111,
            _ => unreachable!(),
        }
    }
    ///No skipping
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == IVTT_A::_000
    }
    ///Skipping count of 1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == IVTT_A::_001
    }
    ///Skipping count of 2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == IVTT_A::_010
    }
    ///Skipping count of 3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == IVTT_A::_011
    }
    ///Skipping count of 4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == IVTT_A::_100
    }
    ///Skipping count of 5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == IVTT_A::_101
    }
    ///Skipping count of 6
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == IVTT_A::_110
    }
    ///Skipping count of 7.
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == IVTT_A::_111
    }
}
///Field `IVTT` writer - GPT_OVF/GPT_UDF Interrupt Skipping Count Select
pub type IVTT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, IVTT_A, crate::Safe>;
impl<'a, REG> IVTT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No skipping
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_000)
    }
    ///Skipping count of 1
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_001)
    }
    ///Skipping count of 2
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_010)
    }
    ///Skipping count of 3
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_011)
    }
    ///Skipping count of 4
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_100)
    }
    ///Skipping count of 5
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_101)
    }
    ///Skipping count of 6
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_110)
    }
    ///Skipping count of 7.
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(IVTT_A::_111)
    }
}
/**GTADTRA A/D Converter Start Request Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTAL_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _1 = 1,
}
impl From<ADTAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADTAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTAL` reader - GTADTRA A/D Converter Start Request Link
pub type ADTAL_R = crate::BitReader<ADTAL_A>;
impl ADTAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTAL_A {
        match self.bits {
            false => ADTAL_A::_0,
            true => ADTAL_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTAL_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTAL_A::_1
    }
}
///Field `ADTAL` writer - GTADTRA A/D Converter Start Request Link
pub type ADTAL_W<'a, REG> = crate::BitWriter<'a, REG, ADTAL_A>;
impl<'a, REG> ADTAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTAL_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTAL_A::_1)
    }
}
/**GTADTRB A/D Converter Start Request Link

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTBL_A {
    ///0: Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    _0 = 0,
    ///1: Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    _1 = 1,
}
impl From<ADTBL_A> for bool {
    #[inline(always)]
    fn from(variant: ADTBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTBL` reader - GTADTRB A/D Converter Start Request Link
pub type ADTBL_R = crate::BitReader<ADTBL_A>;
impl ADTBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTBL_A {
        match self.bits {
            false => ADTBL_A::_0,
            true => ADTBL_A::_1,
        }
    }
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTBL_A::_0
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTBL_A::_1
    }
}
///Field `ADTBL` writer - GTADTRB A/D Converter Start Request Link
pub type ADTBL_W<'a, REG> = crate::BitWriter<'a, REG, ADTBL_A>;
impl<'a, REG> ADTBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not link with GPTn_OVF/GPTn_UDF interrupt skipping function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTBL_A::_0)
    }
    ///Link with GPTn_OVF/GPTn_UDF interrupt skipping function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTBL_A::_1)
    }
}
impl R {
    ///Bit 0 - GTCCRA Compare Match/Input Capture Interrupt Link
    #[inline(always)]
    pub fn itla(&self) -> ITLA_R {
        ITLA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTCCRB Compare Match/Input Capture Interrupt Link
    #[inline(always)]
    pub fn itlb(&self) -> ITLB_R {
        ITLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTCCRC Compare Match Interrupt Link
    #[inline(always)]
    pub fn itlc(&self) -> ITLC_R {
        ITLC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTCCRD Compare Match Interrupt Link
    #[inline(always)]
    pub fn itld(&self) -> ITLD_R {
        ITLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTCCRE Compare Match Interrupt Link
    #[inline(always)]
    pub fn itle(&self) -> ITLE_R {
        ITLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTCCRF Compare Match Interrupt Link
    #[inline(always)]
    pub fn itlf(&self) -> ITLF_R {
        ITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - GPT_OVF/GPT_UDF Interrupt Skipping Function Select
    #[inline(always)]
    pub fn ivtc(&self) -> IVTC_R {
        IVTC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - GPT_OVF/GPT_UDF Interrupt Skipping Count Select
    #[inline(always)]
    pub fn ivtt(&self) -> IVTT_R {
        IVTT_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - GTADTRA A/D Converter Start Request Link
    #[inline(always)]
    pub fn adtal(&self) -> ADTAL_R {
        ADTAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - GTADTRB A/D Converter Start Request Link
    #[inline(always)]
    pub fn adtbl(&self) -> ADTBL_R {
        ADTBL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTCCRA Compare Match/Input Capture Interrupt Link
    #[inline(always)]
    pub fn itla(&mut self) -> ITLA_W<GTITC_SPEC> {
        ITLA_W::new(self, 0)
    }
    ///Bit 1 - GTCCRB Compare Match/Input Capture Interrupt Link
    #[inline(always)]
    pub fn itlb(&mut self) -> ITLB_W<GTITC_SPEC> {
        ITLB_W::new(self, 1)
    }
    ///Bit 2 - GTCCRC Compare Match Interrupt Link
    #[inline(always)]
    pub fn itlc(&mut self) -> ITLC_W<GTITC_SPEC> {
        ITLC_W::new(self, 2)
    }
    ///Bit 3 - GTCCRD Compare Match Interrupt Link
    #[inline(always)]
    pub fn itld(&mut self) -> ITLD_W<GTITC_SPEC> {
        ITLD_W::new(self, 3)
    }
    ///Bit 4 - GTCCRE Compare Match Interrupt Link
    #[inline(always)]
    pub fn itle(&mut self) -> ITLE_W<GTITC_SPEC> {
        ITLE_W::new(self, 4)
    }
    ///Bit 5 - GTCCRF Compare Match Interrupt Link
    #[inline(always)]
    pub fn itlf(&mut self) -> ITLF_W<GTITC_SPEC> {
        ITLF_W::new(self, 5)
    }
    ///Bits 6:7 - GPT_OVF/GPT_UDF Interrupt Skipping Function Select
    #[inline(always)]
    pub fn ivtc(&mut self) -> IVTC_W<GTITC_SPEC> {
        IVTC_W::new(self, 6)
    }
    ///Bits 8:10 - GPT_OVF/GPT_UDF Interrupt Skipping Count Select
    #[inline(always)]
    pub fn ivtt(&mut self) -> IVTT_W<GTITC_SPEC> {
        IVTT_W::new(self, 8)
    }
    ///Bit 12 - GTADTRA A/D Converter Start Request Link
    #[inline(always)]
    pub fn adtal(&mut self) -> ADTAL_W<GTITC_SPEC> {
        ADTAL_W::new(self, 12)
    }
    ///Bit 14 - GTADTRB A/D Converter Start Request Link
    #[inline(always)]
    pub fn adtbl(&mut self) -> ADTBL_W<GTITC_SPEC> {
        ADTBL_W::new(self, 14)
    }
}
/**General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtitc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtitc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTITC_SPEC;
impl crate::RegisterSpec for GTITC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtitc::R`](R) reader structure
impl crate::Readable for GTITC_SPEC {}
///`write(|w| ..)` method takes [`gtitc::W`](W) writer structure
impl crate::Writable for GTITC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTITC to value 0
impl crate::Resettable for GTITC_SPEC {}
