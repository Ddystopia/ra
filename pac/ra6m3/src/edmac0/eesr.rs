///Register `EESR` reader
pub type R = crate::R<EESR_SPEC>;
///Register `EESR` writer
pub type W = crate::W<EESR_SPEC>;
/**CRC Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERF_A {
    ///0: CRC error has not been detected.
    _0 = 0,
    ///1: CRC error has been detected.
    _1 = 1,
}
impl From<CERF_A> for bool {
    #[inline(always)]
    fn from(variant: CERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CERF` reader - CRC Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CERF_R = crate::BitReader<CERF_A>;
impl CERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CERF_A {
        match self.bits {
            false => CERF_A::_0,
            true => CERF_A::_1,
        }
    }
    ///CRC error has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERF_A::_0
    }
    ///CRC error has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERF_A::_1
    }
}
///Field `CERF` writer - CRC Error Flag
pub type CERF_W<'a, REG> = crate::BitWriter1C<'a, REG, CERF_A>;
impl<'a, REG> CERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CERF_A::_0)
    }
    ///CRC error has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CERF_A::_1)
    }
}
/**PHY-LSI Receive Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRE_A {
    ///0: PHY-LSI receive error has not been detected.
    _0 = 0,
    ///1: PHY-LSI receive error has been detected.
    _1 = 1,
}
impl From<PRE_A> for bool {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PRE` reader - PHY-LSI Receive Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PRE_R = crate::BitReader<PRE_A>;
impl PRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRE_A {
        match self.bits {
            false => PRE_A::_0,
            true => PRE_A::_1,
        }
    }
    ///PHY-LSI receive error has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRE_A::_0
    }
    ///PHY-LSI receive error has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRE_A::_1
    }
}
///Field `PRE` writer - PHY-LSI Receive Error Flag
pub type PRE_W<'a, REG> = crate::BitWriter1C<'a, REG, PRE_A>;
impl<'a, REG> PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PHY-LSI receive error has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_A::_0)
    }
    ///PHY-LSI receive error has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_A::_1)
    }
}
/**Frame-Too-Short Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSF_A {
    ///0: Frame-too-short error has not been detected.
    _0 = 0,
    ///1: Frame-too-short error has been detected.
    _1 = 1,
}
impl From<RTSF_A> for bool {
    #[inline(always)]
    fn from(variant: RTSF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RTSF` reader - Frame-Too-Short Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RTSF_R = crate::BitReader<RTSF_A>;
impl RTSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTSF_A {
        match self.bits {
            false => RTSF_A::_0,
            true => RTSF_A::_1,
        }
    }
    ///Frame-too-short error has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTSF_A::_0
    }
    ///Frame-too-short error has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTSF_A::_1
    }
}
///Field `RTSF` writer - Frame-Too-Short Error Flag
pub type RTSF_W<'a, REG> = crate::BitWriter1C<'a, REG, RTSF_A>;
impl<'a, REG> RTSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame-too-short error has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTSF_A::_0)
    }
    ///Frame-too-short error has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTSF_A::_1)
    }
}
/**Frame-Too-Long Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTLF_A {
    ///0: Frame-too-long error has not been detected.
    _0 = 0,
    ///1: Frame-too-long error has been detected.
    _1 = 1,
}
impl From<RTLF_A> for bool {
    #[inline(always)]
    fn from(variant: RTLF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RTLF` reader - Frame-Too-Long Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RTLF_R = crate::BitReader<RTLF_A>;
impl RTLF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTLF_A {
        match self.bits {
            false => RTLF_A::_0,
            true => RTLF_A::_1,
        }
    }
    ///Frame-too-long error has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTLF_A::_0
    }
    ///Frame-too-long error has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTLF_A::_1
    }
}
///Field `RTLF` writer - Frame-Too-Long Error Flag
pub type RTLF_W<'a, REG> = crate::BitWriter1C<'a, REG, RTLF_A>;
impl<'a, REG> RTLF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame-too-long error has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTLF_A::_0)
    }
    ///Frame-too-long error has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTLF_A::_1)
    }
}
/**Alignment Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRF_A {
    ///0: Alignment error has not been detected.
    _0 = 0,
    ///1: Alignment error has been detected.
    _1 = 1,
}
impl From<RRF_A> for bool {
    #[inline(always)]
    fn from(variant: RRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RRF` reader - Alignment Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RRF_R = crate::BitReader<RRF_A>;
impl RRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRF_A {
        match self.bits {
            false => RRF_A::_0,
            true => RRF_A::_1,
        }
    }
    ///Alignment error has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRF_A::_0
    }
    ///Alignment error has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRF_A::_1
    }
}
///Field `RRF` writer - Alignment Error Flag
pub type RRF_W<'a, REG> = crate::BitWriter1C<'a, REG, RRF_A>;
impl<'a, REG> RRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alignment error has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RRF_A::_0)
    }
    ///Alignment error has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RRF_A::_1)
    }
}
/**Multicast Address Frame Receive Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMAF_A {
    ///0: Multicast address frame has not been received.
    _0 = 0,
    ///1: Multicast address frame has been received.
    _1 = 1,
}
impl From<RMAF_A> for bool {
    #[inline(always)]
    fn from(variant: RMAF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RMAF` reader - Multicast Address Frame Receive Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RMAF_R = crate::BitReader<RMAF_A>;
impl RMAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMAF_A {
        match self.bits {
            false => RMAF_A::_0,
            true => RMAF_A::_1,
        }
    }
    ///Multicast address frame has not been received.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMAF_A::_0
    }
    ///Multicast address frame has been received.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMAF_A::_1
    }
}
///Field `RMAF` writer - Multicast Address Frame Receive Flag
pub type RMAF_W<'a, REG> = crate::BitWriter1C<'a, REG, RMAF_A>;
impl<'a, REG> RMAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Multicast address frame has not been received.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RMAF_A::_0)
    }
    ///Multicast address frame has been received.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RMAF_A::_1)
    }
}
/**Transmit Retry Over Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRO_A {
    ///0: Transmit retry-over condition has not been detected.
    _0 = 0,
    ///1: Transmit retry-over condition has been detected.
    _1 = 1,
}
impl From<TRO_A> for bool {
    #[inline(always)]
    fn from(variant: TRO_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TRO` reader - Transmit Retry Over Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TRO_R = crate::BitReader<TRO_A>;
impl TRO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRO_A {
        match self.bits {
            false => TRO_A::_0,
            true => TRO_A::_1,
        }
    }
    ///Transmit retry-over condition has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRO_A::_0
    }
    ///Transmit retry-over condition has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRO_A::_1
    }
}
///Field `TRO` writer - Transmit Retry Over Flag
pub type TRO_W<'a, REG> = crate::BitWriter1C<'a, REG, TRO_A>;
impl<'a, REG> TRO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit retry-over condition has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRO_A::_0)
    }
    ///Transmit retry-over condition has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRO_A::_1)
    }
}
/**Late Collision Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CD_A {
    ///0: Late collision has not been detected.
    _0 = 0,
    ///1: Late collision has been detected during frame transmission.
    _1 = 1,
}
impl From<CD_A> for bool {
    #[inline(always)]
    fn from(variant: CD_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CD` reader - Late Collision Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CD_R = crate::BitReader<CD_A>;
impl CD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CD_A {
        match self.bits {
            false => CD_A::_0,
            true => CD_A::_1,
        }
    }
    ///Late collision has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CD_A::_0
    }
    ///Late collision has been detected during frame transmission.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CD_A::_1
    }
}
///Field `CD` writer - Late Collision Detect Flag
pub type CD_W<'a, REG> = crate::BitWriter1C<'a, REG, CD_A>;
impl<'a, REG> CD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Late collision has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CD_A::_0)
    }
    ///Late collision has been detected during frame transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CD_A::_1)
    }
}
/**Loss of Carrier Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLC_A {
    ///0: Loss of carrier has not been detected.
    _0 = 0,
    ///1: Loss of carrier has been detected during frame transmission.
    _1 = 1,
}
impl From<DLC_A> for bool {
    #[inline(always)]
    fn from(variant: DLC_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DLC` reader - Loss of Carrier Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DLC_R = crate::BitReader<DLC_A>;
impl DLC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLC_A {
        match self.bits {
            false => DLC_A::_0,
            true => DLC_A::_1,
        }
    }
    ///Loss of carrier has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLC_A::_0
    }
    ///Loss of carrier has been detected during frame transmission.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLC_A::_1
    }
}
///Field `DLC` writer - Loss of Carrier Detect Flag
pub type DLC_W<'a, REG> = crate::BitWriter1C<'a, REG, DLC_A>;
impl<'a, REG> DLC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Loss of carrier has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0)
    }
    ///Loss of carrier has been detected during frame transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_1)
    }
}
/**Carrier Not Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CND_A {
    ///0: A carrier has been detected when transmission starts.
    _0 = 0,
    ///1: A carrier has not been detected during preamble transmission.
    _1 = 1,
}
impl From<CND_A> for bool {
    #[inline(always)]
    fn from(variant: CND_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CND` reader - Carrier Not Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CND_R = crate::BitReader<CND_A>;
impl CND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CND_A {
        match self.bits {
            false => CND_A::_0,
            true => CND_A::_1,
        }
    }
    ///A carrier has been detected when transmission starts.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CND_A::_0
    }
    ///A carrier has not been detected during preamble transmission.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CND_A::_1
    }
}
///Field `CND` writer - Carrier Not Detect Flag
pub type CND_W<'a, REG> = crate::BitWriter1C<'a, REG, CND_A>;
impl<'a, REG> CND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A carrier has been detected when transmission starts.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CND_A::_0)
    }
    ///A carrier has not been detected during preamble transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CND_A::_1)
    }
}
/**Receive FIFO Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOF_A {
    ///0: Overflow has not occurred.
    _0 = 0,
    ///1: Overflow has occurred.
    _1 = 1,
}
impl From<RFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RFOF` reader - Receive FIFO Overflow Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RFOF_R = crate::BitReader<RFOF_A>;
impl RFOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFOF_A {
        match self.bits {
            false => RFOF_A::_0,
            true => RFOF_A::_1,
        }
    }
    ///Overflow has not occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOF_A::_0
    }
    ///Overflow has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOF_A::_1
    }
}
///Field `RFOF` writer - Receive FIFO Overflow Flag
pub type RFOF_W<'a, REG> = crate::BitWriter1C<'a, REG, RFOF_A>;
impl<'a, REG> RFOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overflow has not occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFOF_A::_0)
    }
    ///Overflow has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFOF_A::_1)
    }
}
/**Receive Descriptor Empty Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDE_A {
    ///0: The EDMAC detects that the receive descriptor valid bit (RDn.RACT) is 1.
    _0 = 0,
    ///1: The EDMAC detects that the receive descriptor valid bit (RDn.RACT) is 0.
    _1 = 1,
}
impl From<RDE_A> for bool {
    #[inline(always)]
    fn from(variant: RDE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RDE` reader - Receive Descriptor Empty Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RDE_R = crate::BitReader<RDE_A>;
impl RDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDE_A {
        match self.bits {
            false => RDE_A::_0,
            true => RDE_A::_1,
        }
    }
    ///The EDMAC detects that the receive descriptor valid bit (RDn.RACT) is 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDE_A::_0
    }
    ///The EDMAC detects that the receive descriptor valid bit (RDn.RACT) is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDE_A::_1
    }
}
///Field `RDE` writer - Receive Descriptor Empty Flag
pub type RDE_W<'a, REG> = crate::BitWriter1C<'a, REG, RDE_A>;
impl<'a, REG> RDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The EDMAC detects that the receive descriptor valid bit (RDn.RACT) is 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDE_A::_0)
    }
    ///The EDMAC detects that the receive descriptor valid bit (RDn.RACT) is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDE_A::_1)
    }
}
/**Frame Receive Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FR_A {
    ///0: Frame has not been received.
    _0 = 0,
    ///1: Frame has been received. Update of the receive descriptor is complete.
    _1 = 1,
}
impl From<FR_A> for bool {
    #[inline(always)]
    fn from(variant: FR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `FR` reader - Frame Receive Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type FR_R = crate::BitReader<FR_A>;
impl FR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FR_A {
        match self.bits {
            false => FR_A::_0,
            true => FR_A::_1,
        }
    }
    ///Frame has not been received.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FR_A::_0
    }
    ///Frame has been received. Update of the receive descriptor is complete.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FR_A::_1
    }
}
///Field `FR` writer - Frame Receive Flag
pub type FR_W<'a, REG> = crate::BitWriter1C<'a, REG, FR_A>;
impl<'a, REG> FR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame has not been received.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FR_A::_0)
    }
    ///Frame has been received. Update of the receive descriptor is complete.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FR_A::_1)
    }
}
/**Transmit FIFO Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUF_A {
    ///0: Underflow has not occurred.
    _0 = 0,
    ///1: Underflow has occurred.
    _1 = 1,
}
impl From<TFUF_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TFUF` reader - Transmit FIFO Underflow Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TFUF_R = crate::BitReader<TFUF_A>;
impl TFUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFUF_A {
        match self.bits {
            false => TFUF_A::_0,
            true => TFUF_A::_1,
        }
    }
    ///Underflow has not occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUF_A::_0
    }
    ///Underflow has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUF_A::_1
    }
}
///Field `TFUF` writer - Transmit FIFO Underflow Flag
pub type TFUF_W<'a, REG> = crate::BitWriter1C<'a, REG, TFUF_A>;
impl<'a, REG> TFUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Underflow has not occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFUF_A::_0)
    }
    ///Underflow has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFUF_A::_1)
    }
}
/**Transmit Descriptor Empty Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    ///0: The EDMAC detects that the transmit descriptor valid bit (TDn.TACT) is 1.
    _0 = 0,
    ///1: The EDMAC detects that the transmit descriptor valid bit (TDn.TACT) is 0.
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TDE` reader - Transmit Descriptor Empty Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TDE_R = crate::BitReader<TDE_A>;
impl TDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    ///The EDMAC detects that the transmit descriptor valid bit (TDn.TACT) is 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    ///The EDMAC detects that the transmit descriptor valid bit (TDn.TACT) is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
///Field `TDE` writer - Transmit Descriptor Empty Flag
pub type TDE_W<'a, REG> = crate::BitWriter1C<'a, REG, TDE_A>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The EDMAC detects that the transmit descriptor valid bit (TDn.TACT) is 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    ///The EDMAC detects that the transmit descriptor valid bit (TDn.TACT) is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
/**Frame Transfer Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    ///0: Transfer have not been completed, or no transfer has been requested.
    _0 = 0,
    ///1: All frames indicated by the transmit descriptor have been completely transferred to the transmit FIFO.
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TC` reader - Frame Transfer Complete Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TC_R = crate::BitReader<TC_A>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    ///Transfer have not been completed, or no transfer has been requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    ///All frames indicated by the transmit descriptor have been completely transferred to the transmit FIFO.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
///Field `TC` writer - Frame Transfer Complete Flag
pub type TC_W<'a, REG> = crate::BitWriter1C<'a, REG, TC_A>;
impl<'a, REG> TC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer have not been completed, or no transfer has been requested.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TC_A::_0)
    }
    ///All frames indicated by the transmit descriptor have been completely transferred to the transmit FIFO.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TC_A::_1)
    }
}
/**ETHERC Status Register Source FlagNOTE: When the source in the ETHERCn.ECSR register is cleared, the ECI flag is also cleared.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECI_A {
    ///0: ETHERC status interrupt source has not been detected.
    _0 = 0,
    ///1: ETHERC status interrupt source has been detected.
    _1 = 1,
}
impl From<ECI_A> for bool {
    #[inline(always)]
    fn from(variant: ECI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ECI` reader - ETHERC Status Register Source FlagNOTE: When the source in the ETHERCn.ECSR register is cleared, the ECI flag is also cleared.
pub type ECI_R = crate::BitReader<ECI_A>;
impl ECI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECI_A {
        match self.bits {
            false => ECI_A::_0,
            true => ECI_A::_1,
        }
    }
    ///ETHERC status interrupt source has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECI_A::_0
    }
    ///ETHERC status interrupt source has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECI_A::_1
    }
}
/**Address Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADE_A {
    ///0: Invalid memory address has not been detected (normal operation).
    _0 = 0,
    ///1: Invalid memory address has been detected.
    _1 = 1,
}
impl From<ADE_A> for bool {
    #[inline(always)]
    fn from(variant: ADE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ADE` reader - Address Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ADE_R = crate::BitReader<ADE_A>;
impl ADE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADE_A {
        match self.bits {
            false => ADE_A::_0,
            true => ADE_A::_1,
        }
    }
    ///Invalid memory address has not been detected (normal operation).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADE_A::_0
    }
    ///Invalid memory address has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADE_A::_1
    }
}
///Field `ADE` writer - Address Error Flag
pub type ADE_W<'a, REG> = crate::BitWriter1C<'a, REG, ADE_A>;
impl<'a, REG> ADE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid memory address has not been detected (normal operation).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADE_A::_0)
    }
    ///Invalid memory address has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADE_A::_1)
    }
}
/**Receive Frame Counter Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCOF_A {
    ///0: Receive frame counter has not overflowed.
    _0 = 0,
    ///1: Receive frame counter has overflowed.
    _1 = 1,
}
impl From<RFCOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFCOF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RFCOF` reader - Receive Frame Counter Overflow Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RFCOF_R = crate::BitReader<RFCOF_A>;
impl RFCOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFCOF_A {
        match self.bits {
            false => RFCOF_A::_0,
            true => RFCOF_A::_1,
        }
    }
    ///Receive frame counter has not overflowed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFCOF_A::_0
    }
    ///Receive frame counter has overflowed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFCOF_A::_1
    }
}
///Field `RFCOF` writer - Receive Frame Counter Overflow Flag
pub type RFCOF_W<'a, REG> = crate::BitWriter1C<'a, REG, RFCOF_A>;
impl<'a, REG> RFCOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive frame counter has not overflowed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFCOF_A::_0)
    }
    ///Receive frame counter has overflowed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFCOF_A::_1)
    }
}
/**Receive Abort Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RABT_A {
    ///0: Frame reception has not been aborted or no reception has been requested.
    _0 = 0,
    ///1: Frame reception has been aborted.
    _1 = 1,
}
impl From<RABT_A> for bool {
    #[inline(always)]
    fn from(variant: RABT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RABT` reader - Receive Abort Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RABT_R = crate::BitReader<RABT_A>;
impl RABT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RABT_A {
        match self.bits {
            false => RABT_A::_0,
            true => RABT_A::_1,
        }
    }
    ///Frame reception has not been aborted or no reception has been requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RABT_A::_0
    }
    ///Frame reception has been aborted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RABT_A::_1
    }
}
///Field `RABT` writer - Receive Abort Detect Flag
pub type RABT_W<'a, REG> = crate::BitWriter1C<'a, REG, RABT_A>;
impl<'a, REG> RABT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame reception has not been aborted or no reception has been requested.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RABT_A::_0)
    }
    ///Frame reception has been aborted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RABT_A::_1)
    }
}
/**Transmit Abort Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABT_A {
    ///0: Frame transmission has not been aborted or no transmission has been requested.
    _0 = 0,
    ///1: Frame transmission has been aborted.
    _1 = 1,
}
impl From<TABT_A> for bool {
    #[inline(always)]
    fn from(variant: TABT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TABT` reader - Transmit Abort Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TABT_R = crate::BitReader<TABT_A>;
impl TABT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TABT_A {
        match self.bits {
            false => TABT_A::_0,
            true => TABT_A::_1,
        }
    }
    ///Frame transmission has not been aborted or no transmission has been requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABT_A::_0
    }
    ///Frame transmission has been aborted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABT_A::_1
    }
}
///Field `TABT` writer - Transmit Abort Detect Flag
pub type TABT_W<'a, REG> = crate::BitWriter1C<'a, REG, TABT_A>;
impl<'a, REG> TABT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame transmission has not been aborted or no transmission has been requested.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TABT_A::_0)
    }
    ///Frame transmission has been aborted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TABT_A::_1)
    }
}
/**Write-Back Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWB_A {
    ///0: Write-back has not been completed, or no transmission has been requested.
    _0 = 0,
    ///1: Write-back to the transmit descriptor has been completed.
    _1 = 1,
}
impl From<TWB_A> for bool {
    #[inline(always)]
    fn from(variant: TWB_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TWB` reader - Write-Back Complete Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TWB_R = crate::BitReader<TWB_A>;
impl TWB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TWB_A {
        match self.bits {
            false => TWB_A::_0,
            true => TWB_A::_1,
        }
    }
    ///Write-back has not been completed, or no transmission has been requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWB_A::_0
    }
    ///Write-back to the transmit descriptor has been completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWB_A::_1
    }
}
///Field `TWB` writer - Write-Back Complete Flag
pub type TWB_W<'a, REG> = crate::BitWriter1C<'a, REG, TWB_A>;
impl<'a, REG> TWB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write-back has not been completed, or no transmission has been requested.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TWB_A::_0)
    }
    ///Write-back to the transmit descriptor has been completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TWB_A::_1)
    }
}
impl R {
    ///Bit 0 - CRC Error Flag
    #[inline(always)]
    pub fn cerf(&self) -> CERF_R {
        CERF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Flag
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame-Too-Short Error Flag
    #[inline(always)]
    pub fn rtsf(&self) -> RTSF_R {
        RTSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Frame-Too-Long Error Flag
    #[inline(always)]
    pub fn rtlf(&self) -> RTLF_R {
        RTLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alignment Error Flag
    #[inline(always)]
    pub fn rrf(&self) -> RRF_R {
        RRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Multicast Address Frame Receive Flag
    #[inline(always)]
    pub fn rmaf(&self) -> RMAF_R {
        RMAF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit Retry Over Flag
    #[inline(always)]
    pub fn tro(&self) -> TRO_R {
        TRO_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Late Collision Detect Flag
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Loss of Carrier Detect Flag
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Carrier Not Detect Flag
    #[inline(always)]
    pub fn cnd(&self) -> CND_R {
        CND_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Receive FIFO Overflow Flag
    #[inline(always)]
    pub fn rfof(&self) -> RFOF_R {
        RFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Descriptor Empty Flag
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame Receive Flag
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transmit FIFO Underflow Flag
    #[inline(always)]
    pub fn tfuf(&self) -> TFUF_R {
        TFUF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Transmit Descriptor Empty Flag
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Frame Transfer Complete Flag
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ETHERC Status Register Source FlagNOTE: When the source in the ETHERCn.ECSR register is cleared, the ECI flag is also cleared.
    #[inline(always)]
    pub fn eci(&self) -> ECI_R {
        ECI_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Address Error Flag
    #[inline(always)]
    pub fn ade(&self) -> ADE_R {
        ADE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Receive Frame Counter Overflow Flag
    #[inline(always)]
    pub fn rfcof(&self) -> RFCOF_R {
        RFCOF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive Abort Detect Flag
    #[inline(always)]
    pub fn rabt(&self) -> RABT_R {
        RABT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Transmit Abort Detect Flag
    #[inline(always)]
    pub fn tabt(&self) -> TABT_R {
        TABT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 30 - Write-Back Complete Flag
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CRC Error Flag
    #[inline(always)]
    pub fn cerf(&mut self) -> CERF_W<EESR_SPEC> {
        CERF_W::new(self, 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Flag
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W<EESR_SPEC> {
        PRE_W::new(self, 1)
    }
    ///Bit 2 - Frame-Too-Short Error Flag
    #[inline(always)]
    pub fn rtsf(&mut self) -> RTSF_W<EESR_SPEC> {
        RTSF_W::new(self, 2)
    }
    ///Bit 3 - Frame-Too-Long Error Flag
    #[inline(always)]
    pub fn rtlf(&mut self) -> RTLF_W<EESR_SPEC> {
        RTLF_W::new(self, 3)
    }
    ///Bit 4 - Alignment Error Flag
    #[inline(always)]
    pub fn rrf(&mut self) -> RRF_W<EESR_SPEC> {
        RRF_W::new(self, 4)
    }
    ///Bit 7 - Multicast Address Frame Receive Flag
    #[inline(always)]
    pub fn rmaf(&mut self) -> RMAF_W<EESR_SPEC> {
        RMAF_W::new(self, 7)
    }
    ///Bit 8 - Transmit Retry Over Flag
    #[inline(always)]
    pub fn tro(&mut self) -> TRO_W<EESR_SPEC> {
        TRO_W::new(self, 8)
    }
    ///Bit 9 - Late Collision Detect Flag
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W<EESR_SPEC> {
        CD_W::new(self, 9)
    }
    ///Bit 10 - Loss of Carrier Detect Flag
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W<EESR_SPEC> {
        DLC_W::new(self, 10)
    }
    ///Bit 11 - Carrier Not Detect Flag
    #[inline(always)]
    pub fn cnd(&mut self) -> CND_W<EESR_SPEC> {
        CND_W::new(self, 11)
    }
    ///Bit 16 - Receive FIFO Overflow Flag
    #[inline(always)]
    pub fn rfof(&mut self) -> RFOF_W<EESR_SPEC> {
        RFOF_W::new(self, 16)
    }
    ///Bit 17 - Receive Descriptor Empty Flag
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<EESR_SPEC> {
        RDE_W::new(self, 17)
    }
    ///Bit 18 - Frame Receive Flag
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W<EESR_SPEC> {
        FR_W::new(self, 18)
    }
    ///Bit 19 - Transmit FIFO Underflow Flag
    #[inline(always)]
    pub fn tfuf(&mut self) -> TFUF_W<EESR_SPEC> {
        TFUF_W::new(self, 19)
    }
    ///Bit 20 - Transmit Descriptor Empty Flag
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<EESR_SPEC> {
        TDE_W::new(self, 20)
    }
    ///Bit 21 - Frame Transfer Complete Flag
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<EESR_SPEC> {
        TC_W::new(self, 21)
    }
    ///Bit 23 - Address Error Flag
    #[inline(always)]
    pub fn ade(&mut self) -> ADE_W<EESR_SPEC> {
        ADE_W::new(self, 23)
    }
    ///Bit 24 - Receive Frame Counter Overflow Flag
    #[inline(always)]
    pub fn rfcof(&mut self) -> RFCOF_W<EESR_SPEC> {
        RFCOF_W::new(self, 24)
    }
    ///Bit 25 - Receive Abort Detect Flag
    #[inline(always)]
    pub fn rabt(&mut self) -> RABT_W<EESR_SPEC> {
        RABT_W::new(self, 25)
    }
    ///Bit 26 - Transmit Abort Detect Flag
    #[inline(always)]
    pub fn tabt(&mut self) -> TABT_W<EESR_SPEC> {
        TABT_W::new(self, 26)
    }
    ///Bit 30 - Write-Back Complete Flag
    #[inline(always)]
    pub fn twb(&mut self) -> TWB_W<EESR_SPEC> {
        TWB_W::new(self, 30)
    }
}
/**ETHERC/EDMAC Status Register

You can [`read`](crate::Reg::read) this register and get [`eesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EESR_SPEC;
impl crate::RegisterSpec for EESR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`eesr::R`](R) reader structure
impl crate::Readable for EESR_SPEC {}
///`write(|w| ..)` method takes [`eesr::W`](W) writer structure
impl crate::Writable for EESR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x47bf_0f9f;
}
///`reset()` method sets EESR to value 0
impl crate::Resettable for EESR_SPEC {}
