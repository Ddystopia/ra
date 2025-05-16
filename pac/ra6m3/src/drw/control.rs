///Register `CONTROL` writer
pub type W = crate::W<CONTROL_SPEC>;
/**Enable limiter 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM1ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM1ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LIM1ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM1ENABLE` writer - Enable limiter 1
pub type LIM1ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LIM1ENABLE_A>;
impl<'a, REG> LIM1ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM1ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM1ENABLE_A::_1)
    }
}
/**Enable limiter 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM2ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM2ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LIM2ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM2ENABLE` writer - Enable limiter 2
pub type LIM2ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LIM2ENABLE_A>;
impl<'a, REG> LIM2ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM2ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM2ENABLE_A::_1)
    }
}
/**Enable limiter 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM3ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM3ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LIM3ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM3ENABLE` writer - Enable limiter 3
pub type LIM3ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LIM3ENABLE_A>;
impl<'a, REG> LIM3ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM3ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM3ENABLE_A::_1)
    }
}
/**Enable limiter 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM4ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM4ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LIM4ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM4ENABLE` writer - Enable limiter 4
pub type LIM4ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LIM4ENABLE_A>;
impl<'a, REG> LIM4ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM4ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM4ENABLE_A::_1)
    }
}
/**Enable limiter 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM5ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM5ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LIM5ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM5ENABLE` writer - Enable limiter 5
pub type LIM5ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LIM5ENABLE_A>;
impl<'a, REG> LIM5ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM5ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM5ENABLE_A::_1)
    }
}
/**Enable limiter 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM6ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM6ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LIM6ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM6ENABLE` writer - Enable limiter 6
pub type LIM6ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LIM6ENABLE_A>;
impl<'a, REG> LIM6ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM6ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM6ENABLE_A::_1)
    }
}
/**Enable quadratic coupling of limiters 1 and 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD1ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<QUAD1ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: QUAD1ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `QUAD1ENABLE` writer - Enable quadratic coupling of limiters 1 and 2
pub type QUAD1ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, QUAD1ENABLE_A>;
impl<'a, REG> QUAD1ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(QUAD1ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(QUAD1ENABLE_A::_1)
    }
}
/**Enable quadratic coupling of limiters 3 and 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD2ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<QUAD2ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: QUAD2ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `QUAD2ENABLE` writer - Enable quadratic coupling of limiters 3 and 4
pub type QUAD2ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, QUAD2ENABLE_A>;
impl<'a, REG> QUAD2ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(QUAD2ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(QUAD2ENABLE_A::_1)
    }
}
/**Enable quadratic coupling of limiters 5 and 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD3ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<QUAD3ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: QUAD3ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `QUAD3ENABLE` writer - Enable quadratic coupling of limiters 5 and 6
pub type QUAD3ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, QUAD3ENABLE_A>;
impl<'a, REG> QUAD3ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(QUAD3ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(QUAD3ENABLE_A::_1)
    }
}
/**Enable limiter 1 threshold mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM1THRESHOLD_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM1THRESHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LIM1THRESHOLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM1THRESHOLD` writer - Enable limiter 1 threshold mode
pub type LIM1THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG, LIM1THRESHOLD_A>;
impl<'a, REG> LIM1THRESHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM1THRESHOLD_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM1THRESHOLD_A::_1)
    }
}
/**Enable limiter 2 threshold mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM2THRESHOLD_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM2THRESHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LIM2THRESHOLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM2THRESHOLD` writer - Enable limiter 2 threshold mode
pub type LIM2THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG, LIM2THRESHOLD_A>;
impl<'a, REG> LIM2THRESHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM2THRESHOLD_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM2THRESHOLD_A::_1)
    }
}
/**Enable limiter 3 threshold mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM3THRESHOLD_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM3THRESHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LIM3THRESHOLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM3THRESHOLD` writer - Enable limiter 3 threshold mode
pub type LIM3THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG, LIM3THRESHOLD_A>;
impl<'a, REG> LIM3THRESHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM3THRESHOLD_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM3THRESHOLD_A::_1)
    }
}
/**Enable limiter 4 threshold mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM4THRESHOLD_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM4THRESHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LIM4THRESHOLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM4THRESHOLD` writer - Enable limiter 4 threshold mode
pub type LIM4THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG, LIM4THRESHOLD_A>;
impl<'a, REG> LIM4THRESHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM4THRESHOLD_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM4THRESHOLD_A::_1)
    }
}
/**Enable limiter 5 threshold mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM5THRESHOLD_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM5THRESHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LIM5THRESHOLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM5THRESHOLD` writer - Enable limiter 5 threshold mode
pub type LIM5THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG, LIM5THRESHOLD_A>;
impl<'a, REG> LIM5THRESHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM5THRESHOLD_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM5THRESHOLD_A::_1)
    }
}
/**Enable limiter 6 threshold mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIM6THRESHOLD_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<LIM6THRESHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LIM6THRESHOLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LIM6THRESHOLD` writer - Enable limiter 6 threshold mode
pub type LIM6THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG, LIM6THRESHOLD_A>;
impl<'a, REG> LIM6THRESHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LIM6THRESHOLD_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LIM6THRESHOLD_A::_1)
    }
}
/**Enable band postprocess for limiter 1 (see L1BAND)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BAND1ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<BAND1ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BAND1ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BAND1ENABLE` writer - Enable band postprocess for limiter 1 (see L1BAND)
pub type BAND1ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, BAND1ENABLE_A>;
impl<'a, REG> BAND1ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BAND1ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BAND1ENABLE_A::_1)
    }
}
/**Enable band postprocess for limiter 1 (see L1BAND)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BAND2ENABLE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<BAND2ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BAND2ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BAND2ENABLE` writer - Enable band postprocess for limiter 1 (see L1BAND)
pub type BAND2ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, BAND2ENABLE_A>;
impl<'a, REG> BAND2ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BAND2ENABLE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BAND2ENABLE_A::_1)
    }
}
/**Combine limter 1 & 2 as union (output is called A)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNION12_A {
    ///0: minimum/intersect
    _0 = 0,
    ///1: maximum/union
    _1 = 1,
}
impl From<UNION12_A> for bool {
    #[inline(always)]
    fn from(variant: UNION12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNION12` writer - Combine limter 1 & 2 as union (output is called A)
pub type UNION12_W<'a, REG> = crate::BitWriter<'a, REG, UNION12_A>;
impl<'a, REG> UNION12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///minimum/intersect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNION12_A::_0)
    }
    ///maximum/union
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNION12_A::_1)
    }
}
/**Combine limter 3 & 4 as union (output is called B)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNION34_A {
    ///0: minimum/intersect
    _0 = 0,
    ///1: maximum/union
    _1 = 1,
}
impl From<UNION34_A> for bool {
    #[inline(always)]
    fn from(variant: UNION34_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNION34` writer - Combine limter 3 & 4 as union (output is called B)
pub type UNION34_W<'a, REG> = crate::BitWriter<'a, REG, UNION34_A>;
impl<'a, REG> UNION34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///minimum/intersect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNION34_A::_0)
    }
    ///maximum/union
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNION34_A::_1)
    }
}
/**Combine limter 5 & 6 as union (output is called D)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNION56_A {
    ///0: minimum/intersect
    _0 = 0,
    ///1: maximum/union
    _1 = 1,
}
impl From<UNION56_A> for bool {
    #[inline(always)]
    fn from(variant: UNION56_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNION56` writer - Combine limter 5 & 6 as union (output is called D)
pub type UNION56_W<'a, REG> = crate::BitWriter<'a, REG, UNION56_A>;
impl<'a, REG> UNION56_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///minimum/intersect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNION56_A::_0)
    }
    ///maximum/union
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNION56_A::_1)
    }
}
/**Combine outputs A & B as union (output is called C)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNIONAB_A {
    ///0: minimum/intersect
    _0 = 0,
    ///1: maximum/union
    _1 = 1,
}
impl From<UNIONAB_A> for bool {
    #[inline(always)]
    fn from(variant: UNIONAB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNIONAB` writer - Combine outputs A & B as union (output is called C)
pub type UNIONAB_W<'a, REG> = crate::BitWriter<'a, REG, UNIONAB_A>;
impl<'a, REG> UNIONAB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///minimum/intersect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNIONAB_A::_0)
    }
    ///maximum/union
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNIONAB_A::_1)
    }
}
/**Combine outputs C & D as union (output is final)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNIONCD_A {
    ///0: minimum/intersect
    _0 = 0,
    ///1: maximum/union
    _1 = 1,
}
impl From<UNIONCD_A> for bool {
    #[inline(always)]
    fn from(variant: UNIONCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNIONCD` writer - Combine outputs C & D as union (output is final)
pub type UNIONCD_W<'a, REG> = crate::BitWriter<'a, REG, UNIONCD_A>;
impl<'a, REG> UNIONCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///minimum/intersect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNIONCD_A::_0)
    }
    ///maximum/union
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNIONCD_A::_1)
    }
}
/**Shape is horizontally convex, only a single span per scanline

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPANABORT_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<SPANABORT_A> for bool {
    #[inline(always)]
    fn from(variant: SPANABORT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPANABORT` writer - Shape is horizontally convex, only a single span per scanline
pub type SPANABORT_W<'a, REG> = crate::BitWriter<'a, REG, SPANABORT_A>;
impl<'a, REG> SPANABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPANABORT_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPANABORT_A::_1)
    }
}
/**Nextline span start is always equal or left to current-line span start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPANSTORE_A {
    ///0: disabled
    _0 = 0,
    ///1: enabled
    _1 = 1,
}
impl From<SPANSTORE_A> for bool {
    #[inline(always)]
    fn from(variant: SPANSTORE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPANSTORE` writer - Nextline span start is always equal or left to current-line span start
pub type SPANSTORE_W<'a, REG> = crate::BitWriter<'a, REG, SPANSTORE_A>;
impl<'a, REG> SPANSTORE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPANSTORE_A::_0)
    }
    ///enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPANSTORE_A::_1)
    }
}
impl W {
    ///Bit 0 - Enable limiter 1
    #[inline(always)]
    pub fn lim1enable(&mut self) -> LIM1ENABLE_W<CONTROL_SPEC> {
        LIM1ENABLE_W::new(self, 0)
    }
    ///Bit 1 - Enable limiter 2
    #[inline(always)]
    pub fn lim2enable(&mut self) -> LIM2ENABLE_W<CONTROL_SPEC> {
        LIM2ENABLE_W::new(self, 1)
    }
    ///Bit 2 - Enable limiter 3
    #[inline(always)]
    pub fn lim3enable(&mut self) -> LIM3ENABLE_W<CONTROL_SPEC> {
        LIM3ENABLE_W::new(self, 2)
    }
    ///Bit 3 - Enable limiter 4
    #[inline(always)]
    pub fn lim4enable(&mut self) -> LIM4ENABLE_W<CONTROL_SPEC> {
        LIM4ENABLE_W::new(self, 3)
    }
    ///Bit 4 - Enable limiter 5
    #[inline(always)]
    pub fn lim5enable(&mut self) -> LIM5ENABLE_W<CONTROL_SPEC> {
        LIM5ENABLE_W::new(self, 4)
    }
    ///Bit 5 - Enable limiter 6
    #[inline(always)]
    pub fn lim6enable(&mut self) -> LIM6ENABLE_W<CONTROL_SPEC> {
        LIM6ENABLE_W::new(self, 5)
    }
    ///Bit 6 - Enable quadratic coupling of limiters 1 and 2
    #[inline(always)]
    pub fn quad1enable(&mut self) -> QUAD1ENABLE_W<CONTROL_SPEC> {
        QUAD1ENABLE_W::new(self, 6)
    }
    ///Bit 7 - Enable quadratic coupling of limiters 3 and 4
    #[inline(always)]
    pub fn quad2enable(&mut self) -> QUAD2ENABLE_W<CONTROL_SPEC> {
        QUAD2ENABLE_W::new(self, 7)
    }
    ///Bit 8 - Enable quadratic coupling of limiters 5 and 6
    #[inline(always)]
    pub fn quad3enable(&mut self) -> QUAD3ENABLE_W<CONTROL_SPEC> {
        QUAD3ENABLE_W::new(self, 8)
    }
    ///Bit 9 - Enable limiter 1 threshold mode
    #[inline(always)]
    pub fn lim1threshold(&mut self) -> LIM1THRESHOLD_W<CONTROL_SPEC> {
        LIM1THRESHOLD_W::new(self, 9)
    }
    ///Bit 10 - Enable limiter 2 threshold mode
    #[inline(always)]
    pub fn lim2threshold(&mut self) -> LIM2THRESHOLD_W<CONTROL_SPEC> {
        LIM2THRESHOLD_W::new(self, 10)
    }
    ///Bit 11 - Enable limiter 3 threshold mode
    #[inline(always)]
    pub fn lim3threshold(&mut self) -> LIM3THRESHOLD_W<CONTROL_SPEC> {
        LIM3THRESHOLD_W::new(self, 11)
    }
    ///Bit 12 - Enable limiter 4 threshold mode
    #[inline(always)]
    pub fn lim4threshold(&mut self) -> LIM4THRESHOLD_W<CONTROL_SPEC> {
        LIM4THRESHOLD_W::new(self, 12)
    }
    ///Bit 13 - Enable limiter 5 threshold mode
    #[inline(always)]
    pub fn lim5threshold(&mut self) -> LIM5THRESHOLD_W<CONTROL_SPEC> {
        LIM5THRESHOLD_W::new(self, 13)
    }
    ///Bit 14 - Enable limiter 6 threshold mode
    #[inline(always)]
    pub fn lim6threshold(&mut self) -> LIM6THRESHOLD_W<CONTROL_SPEC> {
        LIM6THRESHOLD_W::new(self, 14)
    }
    ///Bit 15 - Enable band postprocess for limiter 1 (see L1BAND)
    #[inline(always)]
    pub fn band1enable(&mut self) -> BAND1ENABLE_W<CONTROL_SPEC> {
        BAND1ENABLE_W::new(self, 15)
    }
    ///Bit 16 - Enable band postprocess for limiter 1 (see L1BAND)
    #[inline(always)]
    pub fn band2enable(&mut self) -> BAND2ENABLE_W<CONTROL_SPEC> {
        BAND2ENABLE_W::new(self, 16)
    }
    ///Bit 17 - Combine limter 1 & 2 as union (output is called A)
    #[inline(always)]
    pub fn union12(&mut self) -> UNION12_W<CONTROL_SPEC> {
        UNION12_W::new(self, 17)
    }
    ///Bit 18 - Combine limter 3 & 4 as union (output is called B)
    #[inline(always)]
    pub fn union34(&mut self) -> UNION34_W<CONTROL_SPEC> {
        UNION34_W::new(self, 18)
    }
    ///Bit 19 - Combine limter 5 & 6 as union (output is called D)
    #[inline(always)]
    pub fn union56(&mut self) -> UNION56_W<CONTROL_SPEC> {
        UNION56_W::new(self, 19)
    }
    ///Bit 20 - Combine outputs A & B as union (output is called C)
    #[inline(always)]
    pub fn unionab(&mut self) -> UNIONAB_W<CONTROL_SPEC> {
        UNIONAB_W::new(self, 20)
    }
    ///Bit 21 - Combine outputs C & D as union (output is final)
    #[inline(always)]
    pub fn unioncd(&mut self) -> UNIONCD_W<CONTROL_SPEC> {
        UNIONCD_W::new(self, 21)
    }
    ///Bit 22 - Shape is horizontally convex, only a single span per scanline
    #[inline(always)]
    pub fn spanabort(&mut self) -> SPANABORT_W<CONTROL_SPEC> {
        SPANABORT_W::new(self, 22)
    }
    ///Bit 23 - Nextline span start is always equal or left to current-line span start
    #[inline(always)]
    pub fn spanstore(&mut self) -> SPANSTORE_W<CONTROL_SPEC> {
        SPANSTORE_W::new(self, 23)
    }
}
/**Geometry Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`control::W`](W) writer structure
impl crate::Writable for CONTROL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONTROL to value 0
impl crate::Resettable for CONTROL_SPEC {}
