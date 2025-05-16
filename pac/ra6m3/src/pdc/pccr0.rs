///Register `PCCR0` reader
pub type R = crate::R<PCCR0_SPEC>;
///Register `PCCR0` writer
pub type W = crate::W<PCCR0_SPEC>;
/**Channel 0 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCKE_A {
    ///0: Operations for reception are stopped.
    _0 = 0,
    ///1: Operations for reception are ongoing.
    _1 = 1,
}
impl From<PCKE_A> for bool {
    #[inline(always)]
    fn from(variant: PCKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCKE` reader - Channel 0 GTCNT Count Clear
pub type PCKE_R = crate::BitReader<PCKE_A>;
impl PCKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKE_A {
        match self.bits {
            false => PCKE_A::_0,
            true => PCKE_A::_1,
        }
    }
    ///Operations for reception are stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCKE_A::_0
    }
    ///Operations for reception are ongoing.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCKE_A::_1
    }
}
///Field `PCKE` writer - Channel 0 GTCNT Count Clear
pub type PCKE_W<'a, REG> = crate::BitWriter<'a, REG, PCKE_A>;
impl<'a, REG> PCKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operations for reception are stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PCKE_A::_0)
    }
    ///Operations for reception are ongoing.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PCKE_A::_1)
    }
}
/**VSYNC Signal Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPS_A {
    ///0: VSYNC signal is active high.
    _0 = 0,
    ///1: VSYNC signal is active low.
    _1 = 1,
}
impl From<VPS_A> for bool {
    #[inline(always)]
    fn from(variant: VPS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VPS` reader - VSYNC Signal Polarity Select
pub type VPS_R = crate::BitReader<VPS_A>;
impl VPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VPS_A {
        match self.bits {
            false => VPS_A::_0,
            true => VPS_A::_1,
        }
    }
    ///VSYNC signal is active high.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPS_A::_0
    }
    ///VSYNC signal is active low.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPS_A::_1
    }
}
///Field `VPS` writer - VSYNC Signal Polarity Select
pub type VPS_W<'a, REG> = crate::BitWriter<'a, REG, VPS_A>;
impl<'a, REG> VPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VSYNC signal is active high.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_A::_0)
    }
    ///VSYNC signal is active low.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_A::_1)
    }
}
/**HSYNC Signal Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPS_A {
    ///0: HSYNC signal is active high.
    _0 = 0,
    ///1: HSYNC signal is active low.
    _1 = 1,
}
impl From<HPS_A> for bool {
    #[inline(always)]
    fn from(variant: HPS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HPS` reader - HSYNC Signal Polarity Select
pub type HPS_R = crate::BitReader<HPS_A>;
impl HPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPS_A {
        match self.bits {
            false => HPS_A::_0,
            true => HPS_A::_1,
        }
    }
    ///HSYNC signal is active high.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HPS_A::_0
    }
    ///HSYNC signal is active low.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HPS_A::_1
    }
}
///Field `HPS` writer - HSYNC Signal Polarity Select
pub type HPS_W<'a, REG> = crate::BitWriter<'a, REG, HPS_A>;
impl<'a, REG> HPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSYNC signal is active high.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HPS_A::_0)
    }
    ///HSYNC signal is active low.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HPS_A::_1)
    }
}
/**PDC Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRST_A {
    ///0: PDC reset is not applied.
    _0 = 0,
    ///1: PDC is reset.
    _1 = 1,
}
impl From<PRST_A> for bool {
    #[inline(always)]
    fn from(variant: PRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRST` writer - PDC Reset
pub type PRST_W<'a, REG> = crate::BitWriter<'a, REG, PRST_A>;
impl<'a, REG> PRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PDC reset is not applied.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRST_A::_0)
    }
    ///PDC is reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRST_A::_1)
    }
}
/**Receive Data Ready Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFIE_A {
    ///0: Generation of receive data ready interrupt requests is disabled.
    _0 = 0,
    ///1: Generation of receive data ready interrupt requests is enabled.
    _1 = 1,
}
impl From<DFIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DFIE` reader - Receive Data Ready Interrupt Enable
pub type DFIE_R = crate::BitReader<DFIE_A>;
impl DFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFIE_A {
        match self.bits {
            false => DFIE_A::_0,
            true => DFIE_A::_1,
        }
    }
    ///Generation of receive data ready interrupt requests is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFIE_A::_0
    }
    ///Generation of receive data ready interrupt requests is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFIE_A::_1
    }
}
///Field `DFIE` writer - Receive Data Ready Interrupt Enable
pub type DFIE_W<'a, REG> = crate::BitWriter<'a, REG, DFIE_A>;
impl<'a, REG> DFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of receive data ready interrupt requests is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DFIE_A::_0)
    }
    ///Generation of receive data ready interrupt requests is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DFIE_A::_1)
    }
}
/**Frame End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    ///0: Generation of frame end interrupt requests is disabled.
    _0 = 0,
    ///1: Generation of frame end interrupt requests is enabled.
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FEIE` reader - Frame End Interrupt Enable
pub type FEIE_R = crate::BitReader<FEIE_A>;
impl FEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    ///Generation of frame end interrupt requests is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    ///Generation of frame end interrupt requests is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
///Field `FEIE` writer - Frame End Interrupt Enable
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG, FEIE_A>;
impl<'a, REG> FEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of frame end interrupt requests is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FEIE_A::_0)
    }
    ///Generation of frame end interrupt requests is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FEIE_A::_1)
    }
}
/**Overrun Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVIE_A {
    ///0: Generation of overrun interrupt requests is disabled.
    _0 = 0,
    ///1: Generation of overrun interrupt requests is enabled.
    _1 = 1,
}
impl From<OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVIE` reader - Overrun Interrupt Enable
pub type OVIE_R = crate::BitReader<OVIE_A>;
impl OVIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVIE_A {
        match self.bits {
            false => OVIE_A::_0,
            true => OVIE_A::_1,
        }
    }
    ///Generation of overrun interrupt requests is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVIE_A::_0
    }
    ///Generation of overrun interrupt requests is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVIE_A::_1
    }
}
///Field `OVIE` writer - Overrun Interrupt Enable
pub type OVIE_W<'a, REG> = crate::BitWriter<'a, REG, OVIE_A>;
impl<'a, REG> OVIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of overrun interrupt requests is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVIE_A::_0)
    }
    ///Generation of overrun interrupt requests is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVIE_A::_1)
    }
}
/**Underrun Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRIE_A {
    ///0: Generation of underrun interrupt requests is disabled.
    _0 = 0,
    ///1: Generation of underrun interrupt requests is enabled.
    _1 = 1,
}
impl From<UDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: UDRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UDRIE` reader - Underrun Interrupt Enable
pub type UDRIE_R = crate::BitReader<UDRIE_A>;
impl UDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDRIE_A {
        match self.bits {
            false => UDRIE_A::_0,
            true => UDRIE_A::_1,
        }
    }
    ///Generation of underrun interrupt requests is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRIE_A::_0
    }
    ///Generation of underrun interrupt requests is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRIE_A::_1
    }
}
///Field `UDRIE` writer - Underrun Interrupt Enable
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG, UDRIE_A>;
impl<'a, REG> UDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of underrun interrupt requests is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE_A::_0)
    }
    ///Generation of underrun interrupt requests is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE_A::_1)
    }
}
/**Vertical Line Number Setting Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VERIE_A {
    ///0: Generation of vertical line number setting error interrupt requests is disabled.
    _0 = 0,
    ///1: Generation of vertical line number setting error interrupt requests is enabled.
    _1 = 1,
}
impl From<VERIE_A> for bool {
    #[inline(always)]
    fn from(variant: VERIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VERIE` reader - Vertical Line Number Setting Error Interrupt Enable
pub type VERIE_R = crate::BitReader<VERIE_A>;
impl VERIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VERIE_A {
        match self.bits {
            false => VERIE_A::_0,
            true => VERIE_A::_1,
        }
    }
    ///Generation of vertical line number setting error interrupt requests is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VERIE_A::_0
    }
    ///Generation of vertical line number setting error interrupt requests is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VERIE_A::_1
    }
}
///Field `VERIE` writer - Vertical Line Number Setting Error Interrupt Enable
pub type VERIE_W<'a, REG> = crate::BitWriter<'a, REG, VERIE_A>;
impl<'a, REG> VERIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of vertical line number setting error interrupt requests is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VERIE_A::_0)
    }
    ///Generation of vertical line number setting error interrupt requests is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VERIE_A::_1)
    }
}
/**Horizontal Byte Number Setting Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HERIE_A {
    ///0: Generation of horizontal byte number setting error interrupt requests is disabled.
    _0 = 0,
    ///1: Generation of horizontal byte number setting error interrupt requests is enabled.
    _1 = 1,
}
impl From<HERIE_A> for bool {
    #[inline(always)]
    fn from(variant: HERIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HERIE` reader - Horizontal Byte Number Setting Error Interrupt Enable
pub type HERIE_R = crate::BitReader<HERIE_A>;
impl HERIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HERIE_A {
        match self.bits {
            false => HERIE_A::_0,
            true => HERIE_A::_1,
        }
    }
    ///Generation of horizontal byte number setting error interrupt requests is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HERIE_A::_0
    }
    ///Generation of horizontal byte number setting error interrupt requests is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HERIE_A::_1
    }
}
///Field `HERIE` writer - Horizontal Byte Number Setting Error Interrupt Enable
pub type HERIE_W<'a, REG> = crate::BitWriter<'a, REG, HERIE_A>;
impl<'a, REG> HERIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of horizontal byte number setting error interrupt requests is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HERIE_A::_0)
    }
    ///Generation of horizontal byte number setting error interrupt requests is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HERIE_A::_1)
    }
}
/**PCKO Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCKOE_A {
    ///0: PCKO output is disabled (fixed to the high level)
    _0 = 0,
    ///1: PCKO output is enabled.
    _1 = 1,
}
impl From<PCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: PCKOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCKOE` reader - PCKO Output Enable
pub type PCKOE_R = crate::BitReader<PCKOE_A>;
impl PCKOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKOE_A {
        match self.bits {
            false => PCKOE_A::_0,
            true => PCKOE_A::_1,
        }
    }
    ///PCKO output is disabled (fixed to the high level)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCKOE_A::_0
    }
    ///PCKO output is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCKOE_A::_1
    }
}
///Field `PCKOE` writer - PCKO Output Enable
pub type PCKOE_W<'a, REG> = crate::BitWriter<'a, REG, PCKOE_A>;
impl<'a, REG> PCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCKO output is disabled (fixed to the high level)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PCKOE_A::_0)
    }
    ///PCKO output is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PCKOE_A::_1)
    }
}
/**PCKO Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKDIV_A {
    ///0: PCKO/2
    _000 = 0,
    ///1: PCKO/4
    _001 = 1,
    ///2: PCKO/6
    _010 = 2,
    ///3: PCKO/8
    _011 = 3,
    ///4: PCKO/10
    _100 = 4,
    ///5: PCKO/12
    _101 = 5,
    ///6: PCKO/14
    _110 = 6,
    ///7: PCKO/16
    _111 = 7,
}
impl From<PCKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCKDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for PCKDIV_A {}
///Field `PCKDIV` reader - PCKO Frequency Division Ratio Select
pub type PCKDIV_R = crate::FieldReader<PCKDIV_A>;
impl PCKDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKDIV_A {
        match self.bits {
            0 => PCKDIV_A::_000,
            1 => PCKDIV_A::_001,
            2 => PCKDIV_A::_010,
            3 => PCKDIV_A::_011,
            4 => PCKDIV_A::_100,
            5 => PCKDIV_A::_101,
            6 => PCKDIV_A::_110,
            7 => PCKDIV_A::_111,
            _ => unreachable!(),
        }
    }
    ///PCKO/2
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKDIV_A::_000
    }
    ///PCKO/4
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKDIV_A::_001
    }
    ///PCKO/6
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKDIV_A::_010
    }
    ///PCKO/8
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKDIV_A::_011
    }
    ///PCKO/10
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKDIV_A::_100
    }
    ///PCKO/12
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKDIV_A::_101
    }
    ///PCKO/14
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKDIV_A::_110
    }
    ///PCKO/16
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PCKDIV_A::_111
    }
}
///Field `PCKDIV` writer - PCKO Frequency Division Ratio Select
pub type PCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PCKDIV_A, crate::Safe>;
impl<'a, REG> PCKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCKO/2
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_000)
    }
    ///PCKO/4
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_001)
    }
    ///PCKO/6
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_010)
    }
    ///PCKO/8
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_011)
    }
    ///PCKO/10
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_100)
    }
    ///PCKO/12
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_101)
    }
    ///PCKO/14
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_110)
    }
    ///PCKO/16
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(PCKDIV_A::_111)
    }
}
/**Endian Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDS_A {
    ///0: Little endian
    _0 = 0,
    ///1: Big endian
    _1 = 1,
}
impl From<EDS_A> for bool {
    #[inline(always)]
    fn from(variant: EDS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EDS` reader - Endian Select
pub type EDS_R = crate::BitReader<EDS_A>;
impl EDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDS_A {
        match self.bits {
            false => EDS_A::_0,
            true => EDS_A::_1,
        }
    }
    ///Little endian
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDS_A::_0
    }
    ///Big endian
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDS_A::_1
    }
}
///Field `EDS` writer - Endian Select
pub type EDS_W<'a, REG> = crate::BitWriter<'a, REG, EDS_A>;
impl<'a, REG> EDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little endian
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EDS_A::_0)
    }
    ///Big endian
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EDS_A::_1)
    }
}
impl R {
    ///Bit 0 - Channel 0 GTCNT Count Clear
    #[inline(always)]
    pub fn pcke(&self) -> PCKE_R {
        PCKE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSYNC Signal Polarity Select
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSYNC Signal Polarity Select
    #[inline(always)]
    pub fn hps(&self) -> HPS_R {
        HPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Receive Data Ready Interrupt Enable
    #[inline(always)]
    pub fn dfie(&self) -> DFIE_R {
        DFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Frame End Interrupt Enable
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Underrun Interrupt Enable
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Vertical Line Number Setting Error Interrupt Enable
    #[inline(always)]
    pub fn verie(&self) -> VERIE_R {
        VERIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Horizontal Byte Number Setting Error Interrupt Enable
    #[inline(always)]
    pub fn herie(&self) -> HERIE_R {
        HERIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PCKO Output Enable
    #[inline(always)]
    pub fn pckoe(&self) -> PCKOE_R {
        PCKOE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13 - PCKO Frequency Division Ratio Select
    #[inline(always)]
    pub fn pckdiv(&self) -> PCKDIV_R {
        PCKDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 14 - Endian Select
    #[inline(always)]
    pub fn eds(&self) -> EDS_R {
        EDS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel 0 GTCNT Count Clear
    #[inline(always)]
    pub fn pcke(&mut self) -> PCKE_W<PCCR0_SPEC> {
        PCKE_W::new(self, 0)
    }
    ///Bit 1 - VSYNC Signal Polarity Select
    #[inline(always)]
    pub fn vps(&mut self) -> VPS_W<PCCR0_SPEC> {
        VPS_W::new(self, 1)
    }
    ///Bit 2 - HSYNC Signal Polarity Select
    #[inline(always)]
    pub fn hps(&mut self) -> HPS_W<PCCR0_SPEC> {
        HPS_W::new(self, 2)
    }
    ///Bit 3 - PDC Reset
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W<PCCR0_SPEC> {
        PRST_W::new(self, 3)
    }
    ///Bit 4 - Receive Data Ready Interrupt Enable
    #[inline(always)]
    pub fn dfie(&mut self) -> DFIE_W<PCCR0_SPEC> {
        DFIE_W::new(self, 4)
    }
    ///Bit 5 - Frame End Interrupt Enable
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<PCCR0_SPEC> {
        FEIE_W::new(self, 5)
    }
    ///Bit 6 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn ovie(&mut self) -> OVIE_W<PCCR0_SPEC> {
        OVIE_W::new(self, 6)
    }
    ///Bit 7 - Underrun Interrupt Enable
    #[inline(always)]
    pub fn udrie(&mut self) -> UDRIE_W<PCCR0_SPEC> {
        UDRIE_W::new(self, 7)
    }
    ///Bit 8 - Vertical Line Number Setting Error Interrupt Enable
    #[inline(always)]
    pub fn verie(&mut self) -> VERIE_W<PCCR0_SPEC> {
        VERIE_W::new(self, 8)
    }
    ///Bit 9 - Horizontal Byte Number Setting Error Interrupt Enable
    #[inline(always)]
    pub fn herie(&mut self) -> HERIE_W<PCCR0_SPEC> {
        HERIE_W::new(self, 9)
    }
    ///Bit 10 - PCKO Output Enable
    #[inline(always)]
    pub fn pckoe(&mut self) -> PCKOE_W<PCCR0_SPEC> {
        PCKOE_W::new(self, 10)
    }
    ///Bits 11:13 - PCKO Frequency Division Ratio Select
    #[inline(always)]
    pub fn pckdiv(&mut self) -> PCKDIV_W<PCCR0_SPEC> {
        PCKDIV_W::new(self, 11)
    }
    ///Bit 14 - Endian Select
    #[inline(always)]
    pub fn eds(&mut self) -> EDS_W<PCCR0_SPEC> {
        EDS_W::new(self, 14)
    }
}
/**PDC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`pccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCCR0_SPEC;
impl crate::RegisterSpec for PCCR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pccr0::R`](R) reader structure
impl crate::Readable for PCCR0_SPEC {}
///`write(|w| ..)` method takes [`pccr0::W`](W) writer structure
impl crate::Writable for PCCR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCCR0 to value 0
impl crate::Resettable for PCCR0_SPEC {}
