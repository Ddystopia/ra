///Register `EESR` reader
pub type R = crate::R<EESR_SPEC>;
///Register `EESR` writer
pub type W = crate::W<EESR_SPEC>;
/**PTP v2 Message Type Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    ///0: Sync
    _0000 = 0,
    ///1: Delay_Req
    _0001 = 1,
    ///2: Pdelay_Req
    _0010 = 2,
    ///3: Pdelay_Resp
    _0011 = 3,
    ///8: Follow_Up
    _1000 = 8,
    ///9: Delay_Resp
    _1001 = 9,
    ///10: Pdelay_Resp_Follow_Up
    _1010 = 10,
    ///11: Announce
    _1011 = 11,
    ///12: Signaling
    _1100 = 12,
    ///13: Management
    _1101 = 13,
    ///4: Settings other than above are prohibited.
    OTHERS = 4,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for TYPE_A {}
/**Field `TYPE` reader - PTP v2 Message Type Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TYPE_R = crate::FieldReader<TYPE_A>;
impl TYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TYPE_A {
        match self.bits {
            0 => TYPE_A::_0000,
            1 => TYPE_A::_0001,
            2 => TYPE_A::_0010,
            3 => TYPE_A::_0011,
            8 => TYPE_A::_1000,
            9 => TYPE_A::_1001,
            10 => TYPE_A::_1010,
            11 => TYPE_A::_1011,
            12 => TYPE_A::_1100,
            13 => TYPE_A::_1101,
            _ => TYPE_A::OTHERS,
        }
    }
    ///Sync
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TYPE_A::_0000
    }
    ///Delay_Req
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TYPE_A::_0001
    }
    ///Pdelay_Req
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TYPE_A::_0010
    }
    ///Pdelay_Resp
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TYPE_A::_0011
    }
    ///Follow_Up
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TYPE_A::_1000
    }
    ///Delay_Resp
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TYPE_A::_1001
    }
    ///Pdelay_Resp_Follow_Up
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TYPE_A::_1010
    }
    ///Announce
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == TYPE_A::_1011
    }
    ///Signaling
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == TYPE_A::_1100
    }
    ///Management
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TYPE_A::_1101
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TYPE_A::OTHERS)
    }
}
///Field `TYPE` writer - PTP v2 Message Type Flag
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TYPE_A, crate::Safe>;
impl<'a, REG> TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sync
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_0000)
    }
    ///Delay_Req
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_0001)
    }
    ///Pdelay_Req
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_0010)
    }
    ///Pdelay_Resp
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_0011)
    }
    ///Follow_Up
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_1000)
    }
    ///Delay_Resp
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_1001)
    }
    ///Pdelay_Resp_Follow_Up
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_1010)
    }
    ///Announce
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_1011)
    }
    ///Signaling
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_1100)
    }
    ///Management
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_1101)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::OTHERS)
    }
}
/**PTP v2 Packet Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVER_A {
    ///0: The current packet is not a PTP v2 packet.
    _0 = 0,
    ///1: The current packet is a PTP v2 packet.
    _1 = 1,
}
impl From<PVER_A> for bool {
    #[inline(always)]
    fn from(variant: PVER_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PVER` reader - PTP v2 Packet Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PVER_R = crate::BitReader<PVER_A>;
impl PVER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVER_A {
        match self.bits {
            false => PVER_A::_0,
            true => PVER_A::_1,
        }
    }
    ///The current packet is not a PTP v2 packet.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PVER_A::_0
    }
    ///The current packet is a PTP v2 packet.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PVER_A::_1
    }
}
///Field `PVER` writer - PTP v2 Packet Flag
pub type PVER_W<'a, REG> = crate::BitWriter1C<'a, REG, PVER_A>;
impl<'a, REG> PVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The current packet is not a PTP v2 packet.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PVER_A::_0)
    }
    ///The current packet is a PTP v2 packet.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PVER_A::_1)
    }
}
/**MAC Address Mismatch Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MACE_A {
    ///0: The source MAC address of transmit frame data matches the set value.
    _0 = 0,
    ///1: The source MAC address of transmit frame data does not match the set value.
    _1 = 1,
}
impl From<MACE_A> for bool {
    #[inline(always)]
    fn from(variant: MACE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `MACE` reader - MAC Address Mismatch Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type MACE_R = crate::BitReader<MACE_A>;
impl MACE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MACE_A {
        match self.bits {
            false => MACE_A::_0,
            true => MACE_A::_1,
        }
    }
    ///The source MAC address of transmit frame data matches the set value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MACE_A::_0
    }
    ///The source MAC address of transmit frame data does not match the set value.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MACE_A::_1
    }
}
///Field `MACE` writer - MAC Address Mismatch Flag
pub type MACE_W<'a, REG> = crate::BitWriter1C<'a, REG, MACE_A>;
impl<'a, REG> MACE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The source MAC address of transmit frame data matches the set value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MACE_A::_0)
    }
    ///The source MAC address of transmit frame data does not match the set value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MACE_A::_1)
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
    ///0: The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 1.
    _0 = 0,
    ///1: The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 0.
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
    ///The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDE_A::_0
    }
    ///The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 0.
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
    ///The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDE_A::_0)
    }
    ///The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 0.
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
    ///1: Frame has been received. The receive descriptor has been updated.
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
    ///Frame has been received. The receive descriptor has been updated.
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
    ///Frame has been received. The receive descriptor has been updated.
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
    ///0: The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 1.
    _0 = 0,
    ///1: The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 0.
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
    ///The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    ///The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 0.
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
    ///The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    ///The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
/**Frame Transfer Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    ///0: Transfer has not been completed, or transfer has not been requested.
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
    ///Transfer has not been completed, or transfer has not been requested.
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
    ///Transfer has not been completed, or transfer has not been requested.
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
/**Transmit Abort Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABT_A {
    ///0: Frame transmission has not been aborted or transmission has not been requested.
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
    ///Frame transmission has not been aborted or transmission has not been requested.
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
    ///Frame transmission has not been aborted or transmission has not been requested.
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
    ///0: Write-back has not been completed, or transmission has not been requested.
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
    ///Write-back has not been completed, or transmission has not been requested.
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
    ///Write-back has not been completed, or transmission has not been requested.
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
    ///Bits 0:3 - PTP v2 Message Type Flag
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - PTP v2 Packet Flag
    #[inline(always)]
    pub fn pver(&self) -> PVER_R {
        PVER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - MAC Address Mismatch Flag
    #[inline(always)]
    pub fn mace(&self) -> MACE_R {
        MACE_R::new(((self.bits >> 8) & 1) != 0)
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
    ///Bits 0:3 - PTP v2 Message Type Flag
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<EESR_SPEC> {
        TYPE_W::new(self, 0)
    }
    ///Bit 4 - PTP v2 Packet Flag
    #[inline(always)]
    pub fn pver(&mut self) -> PVER_W<EESR_SPEC> {
        PVER_W::new(self, 4)
    }
    ///Bit 8 - MAC Address Mismatch Flag
    #[inline(always)]
    pub fn mace(&mut self) -> MACE_W<EESR_SPEC> {
        MACE_W::new(self, 8)
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
/**PTP/EDMAC Status Register

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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x45bf_011f;
}
///`reset()` method sets EESR to value 0
impl crate::Resettable for EESR_SPEC {}
