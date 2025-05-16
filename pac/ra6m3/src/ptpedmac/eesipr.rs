///Register `EESIPR` reader
pub type R = crate::R<EESIPR_SPEC>;
///Register `EESIPR` writer
pub type W = crate::W<EESIPR_SPEC>;
/**PTP v2 Packet Receive Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVERIP_A {
    ///0: PTP v2 packet receive interrupt request is disabled.
    _0 = 0,
    ///1: PTP v2 packet receive interrupt request is enabled.
    _1 = 1,
}
impl From<PVERIP_A> for bool {
    #[inline(always)]
    fn from(variant: PVERIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PVERIP` reader - PTP v2 Packet Receive Interrupt Request Enable
pub type PVERIP_R = crate::BitReader<PVERIP_A>;
impl PVERIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVERIP_A {
        match self.bits {
            false => PVERIP_A::_0,
            true => PVERIP_A::_1,
        }
    }
    ///PTP v2 packet receive interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PVERIP_A::_0
    }
    ///PTP v2 packet receive interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PVERIP_A::_1
    }
}
///Field `PVERIP` writer - PTP v2 Packet Receive Interrupt Request Enable
pub type PVERIP_W<'a, REG> = crate::BitWriter<'a, REG, PVERIP_A>;
impl<'a, REG> PVERIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PTP v2 packet receive interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PVERIP_A::_0)
    }
    ///PTP v2 packet receive interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PVERIP_A::_1)
    }
}
/**MAC Address Mismatch Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MACEIP_A {
    ///0: This bit disables an interrupt request generated when the source MAC address of transmit frame data does not match the set value.
    _0 = 0,
    ///1: This bit enables an interrupt request generated when the source MAC address of transmit frame data does not match the set value.
    _1 = 1,
}
impl From<MACEIP_A> for bool {
    #[inline(always)]
    fn from(variant: MACEIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MACEIP` reader - MAC Address Mismatch Interrupt Request Enable
pub type MACEIP_R = crate::BitReader<MACEIP_A>;
impl MACEIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MACEIP_A {
        match self.bits {
            false => MACEIP_A::_0,
            true => MACEIP_A::_1,
        }
    }
    ///This bit disables an interrupt request generated when the source MAC address of transmit frame data does not match the set value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MACEIP_A::_0
    }
    ///This bit enables an interrupt request generated when the source MAC address of transmit frame data does not match the set value.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MACEIP_A::_1
    }
}
///Field `MACEIP` writer - MAC Address Mismatch Interrupt Request Enable
pub type MACEIP_W<'a, REG> = crate::BitWriter<'a, REG, MACEIP_A>;
impl<'a, REG> MACEIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit disables an interrupt request generated when the source MAC address of transmit frame data does not match the set value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MACEIP_A::_0)
    }
    ///This bit enables an interrupt request generated when the source MAC address of transmit frame data does not match the set value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MACEIP_A::_1)
    }
}
/**Receive FIFO Overflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOFIP_A {
    ///0: Overflow interrupt request is disabled.
    _0 = 0,
    ///1: Overflow interrupt request is enabled.
    _1 = 1,
}
impl From<RFOFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RFOFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFOFIP` reader - Receive FIFO Overflow Interrupt Request Enable
pub type RFOFIP_R = crate::BitReader<RFOFIP_A>;
impl RFOFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFOFIP_A {
        match self.bits {
            false => RFOFIP_A::_0,
            true => RFOFIP_A::_1,
        }
    }
    ///Overflow interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOFIP_A::_0
    }
    ///Overflow interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOFIP_A::_1
    }
}
///Field `RFOFIP` writer - Receive FIFO Overflow Interrupt Request Enable
pub type RFOFIP_W<'a, REG> = crate::BitWriter<'a, REG, RFOFIP_A>;
impl<'a, REG> RFOFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overflow interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFOFIP_A::_0)
    }
    ///Overflow interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFOFIP_A::_1)
    }
}
/**Receive Descriptor Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDEIP_A {
    ///0: Receive descriptor empty interrupt request is disabled.
    _0 = 0,
    ///1: Receive descriptor empty interrupt request is enabled.
    _1 = 1,
}
impl From<RDEIP_A> for bool {
    #[inline(always)]
    fn from(variant: RDEIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDEIP` reader - Receive Descriptor Empty Interrupt Request Enable
pub type RDEIP_R = crate::BitReader<RDEIP_A>;
impl RDEIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDEIP_A {
        match self.bits {
            false => RDEIP_A::_0,
            true => RDEIP_A::_1,
        }
    }
    ///Receive descriptor empty interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDEIP_A::_0
    }
    ///Receive descriptor empty interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDEIP_A::_1
    }
}
///Field `RDEIP` writer - Receive Descriptor Empty Interrupt Request Enable
pub type RDEIP_W<'a, REG> = crate::BitWriter<'a, REG, RDEIP_A>;
impl<'a, REG> RDEIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive descriptor empty interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDEIP_A::_0)
    }
    ///Receive descriptor empty interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDEIP_A::_1)
    }
}
/**Frame Receive Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRIP_A {
    ///0: Frame receive interrupt request is disabled.
    _0 = 0,
    ///1: Frame receive interrupt request is enabled.
    _1 = 1,
}
impl From<FRIP_A> for bool {
    #[inline(always)]
    fn from(variant: FRIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRIP` reader - Frame Receive Interrupt Request Enable
pub type FRIP_R = crate::BitReader<FRIP_A>;
impl FRIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRIP_A {
        match self.bits {
            false => FRIP_A::_0,
            true => FRIP_A::_1,
        }
    }
    ///Frame receive interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRIP_A::_0
    }
    ///Frame receive interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRIP_A::_1
    }
}
///Field `FRIP` writer - Frame Receive Interrupt Request Enable
pub type FRIP_W<'a, REG> = crate::BitWriter<'a, REG, FRIP_A>;
impl<'a, REG> FRIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame receive interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FRIP_A::_0)
    }
    ///Frame receive interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FRIP_A::_1)
    }
}
/**Transmit FIFO Underflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUFIP_A {
    ///0: Underflow interrupt request is disabled.
    _0 = 0,
    ///1: Underflow interrupt request is enabled.
    _1 = 1,
}
impl From<TFUFIP_A> for bool {
    #[inline(always)]
    fn from(variant: TFUFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFUFIP` reader - Transmit FIFO Underflow Interrupt Request Enable
pub type TFUFIP_R = crate::BitReader<TFUFIP_A>;
impl TFUFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFUFIP_A {
        match self.bits {
            false => TFUFIP_A::_0,
            true => TFUFIP_A::_1,
        }
    }
    ///Underflow interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUFIP_A::_0
    }
    ///Underflow interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUFIP_A::_1
    }
}
///Field `TFUFIP` writer - Transmit FIFO Underflow Interrupt Request Enable
pub type TFUFIP_W<'a, REG> = crate::BitWriter<'a, REG, TFUFIP_A>;
impl<'a, REG> TFUFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Underflow interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFUFIP_A::_0)
    }
    ///Underflow interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFUFIP_A::_1)
    }
}
/**Transmit Descriptor Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDEIP_A {
    ///0: Transmit descriptor empty interrupt request is disabled.
    _0 = 0,
    ///1: Transmit descriptor empty interrupt request is enabled.
    _1 = 1,
}
impl From<TDEIP_A> for bool {
    #[inline(always)]
    fn from(variant: TDEIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDEIP` reader - Transmit Descriptor Empty Interrupt Request Enable
pub type TDEIP_R = crate::BitReader<TDEIP_A>;
impl TDEIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDEIP_A {
        match self.bits {
            false => TDEIP_A::_0,
            true => TDEIP_A::_1,
        }
    }
    ///Transmit descriptor empty interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDEIP_A::_0
    }
    ///Transmit descriptor empty interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDEIP_A::_1
    }
}
///Field `TDEIP` writer - Transmit Descriptor Empty Interrupt Request Enable
pub type TDEIP_W<'a, REG> = crate::BitWriter<'a, REG, TDEIP_A>;
impl<'a, REG> TDEIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit descriptor empty interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDEIP_A::_0)
    }
    ///Transmit descriptor empty interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDEIP_A::_1)
    }
}
/**Frame Transfer Complete Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIP_A {
    ///0: Frame transmission complete interrupt request is disabled.
    _0 = 0,
    ///1: Frame transmission complete interrupt request is enabled.
    _1 = 1,
}
impl From<TCIP_A> for bool {
    #[inline(always)]
    fn from(variant: TCIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIP` reader - Frame Transfer Complete Interrupt Request Enable
pub type TCIP_R = crate::BitReader<TCIP_A>;
impl TCIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIP_A {
        match self.bits {
            false => TCIP_A::_0,
            true => TCIP_A::_1,
        }
    }
    ///Frame transmission complete interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIP_A::_0
    }
    ///Frame transmission complete interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIP_A::_1
    }
}
///Field `TCIP` writer - Frame Transfer Complete Interrupt Request Enable
pub type TCIP_W<'a, REG> = crate::BitWriter<'a, REG, TCIP_A>;
impl<'a, REG> TCIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame transmission complete interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIP_A::_0)
    }
    ///Frame transmission complete interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIP_A::_1)
    }
}
/**Address Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEIP_A {
    ///0: Address error interrupt request is disabled.
    _0 = 0,
    ///1: Address error interrupt request is enabled.
    _1 = 1,
}
impl From<ADEIP_A> for bool {
    #[inline(always)]
    fn from(variant: ADEIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEIP` reader - Address Error Interrupt Request Enable
pub type ADEIP_R = crate::BitReader<ADEIP_A>;
impl ADEIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADEIP_A {
        match self.bits {
            false => ADEIP_A::_0,
            true => ADEIP_A::_1,
        }
    }
    ///Address error interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEIP_A::_0
    }
    ///Address error interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEIP_A::_1
    }
}
///Field `ADEIP` writer - Address Error Interrupt Request Enable
pub type ADEIP_W<'a, REG> = crate::BitWriter<'a, REG, ADEIP_A>;
impl<'a, REG> ADEIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address error interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADEIP_A::_0)
    }
    ///Address error interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADEIP_A::_1)
    }
}
/**Receive Frame Counter Overflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCOFIP_A {
    ///0: Receive frame counter overflow interrupt request is disabled.
    _0 = 0,
    ///1: Receive frame counter overflow interrupt request is enabled.
    _1 = 1,
}
impl From<RFCOFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RFCOFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCOFIP` reader - Receive Frame Counter Overflow Interrupt Request Enable
pub type RFCOFIP_R = crate::BitReader<RFCOFIP_A>;
impl RFCOFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFCOFIP_A {
        match self.bits {
            false => RFCOFIP_A::_0,
            true => RFCOFIP_A::_1,
        }
    }
    ///Receive frame counter overflow interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFCOFIP_A::_0
    }
    ///Receive frame counter overflow interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFCOFIP_A::_1
    }
}
///Field `RFCOFIP` writer - Receive Frame Counter Overflow Interrupt Request Enable
pub type RFCOFIP_W<'a, REG> = crate::BitWriter<'a, REG, RFCOFIP_A>;
impl<'a, REG> RFCOFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive frame counter overflow interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFCOFIP_A::_0)
    }
    ///Receive frame counter overflow interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFCOFIP_A::_1)
    }
}
/**Transmit Abort Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTIP_A {
    ///0: Transmit abort detect interrupt request is disabled.
    _0 = 0,
    ///1: Transmit abort detect interrupt request is enabled.
    _1 = 1,
}
impl From<TABTIP_A> for bool {
    #[inline(always)]
    fn from(variant: TABTIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TABTIP` reader - Transmit Abort Detect Interrupt Request Enable
pub type TABTIP_R = crate::BitReader<TABTIP_A>;
impl TABTIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TABTIP_A {
        match self.bits {
            false => TABTIP_A::_0,
            true => TABTIP_A::_1,
        }
    }
    ///Transmit abort detect interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABTIP_A::_0
    }
    ///Transmit abort detect interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABTIP_A::_1
    }
}
///Field `TABTIP` writer - Transmit Abort Detect Interrupt Request Enable
pub type TABTIP_W<'a, REG> = crate::BitWriter<'a, REG, TABTIP_A>;
impl<'a, REG> TABTIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit abort detect interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TABTIP_A::_0)
    }
    ///Transmit abort detect interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TABTIP_A::_1)
    }
}
/**Write-Back Complete Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWBIP_A {
    ///0: Write-back complete interrupt request is disabled.
    _0 = 0,
    ///1: Write-back complete interrupt request is enabled.
    _1 = 1,
}
impl From<TWBIP_A> for bool {
    #[inline(always)]
    fn from(variant: TWBIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TWBIP` reader - Write-Back Complete Interrupt Request Enable
pub type TWBIP_R = crate::BitReader<TWBIP_A>;
impl TWBIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TWBIP_A {
        match self.bits {
            false => TWBIP_A::_0,
            true => TWBIP_A::_1,
        }
    }
    ///Write-back complete interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWBIP_A::_0
    }
    ///Write-back complete interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWBIP_A::_1
    }
}
///Field `TWBIP` writer - Write-Back Complete Interrupt Request Enable
pub type TWBIP_W<'a, REG> = crate::BitWriter<'a, REG, TWBIP_A>;
impl<'a, REG> TWBIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write-back complete interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TWBIP_A::_0)
    }
    ///Write-back complete interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TWBIP_A::_1)
    }
}
impl R {
    ///Bit 4 - PTP v2 Packet Receive Interrupt Request Enable
    #[inline(always)]
    pub fn pverip(&self) -> PVERIP_R {
        PVERIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - MAC Address Mismatch Interrupt Request Enable
    #[inline(always)]
    pub fn maceip(&self) -> MACEIP_R {
        MACEIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Receive FIFO Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfofip(&self) -> RFOFIP_R {
        RFOFIP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn rdeip(&self) -> RDEIP_R {
        RDEIP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn frip(&self) -> FRIP_R {
        FRIP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transmit FIFO Underflow Interrupt Request Enable
    #[inline(always)]
    pub fn tfufip(&self) -> TFUFIP_R {
        TFUFIP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Transmit Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tdeip(&self) -> TDEIP_R {
        TDEIP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Frame Transfer Complete Interrupt Request Enable
    #[inline(always)]
    pub fn tcip(&self) -> TCIP_R {
        TCIP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Address Error Interrupt Request Enable
    #[inline(always)]
    pub fn adeip(&self) -> ADEIP_R {
        ADEIP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfcofip(&self) -> RFCOFIP_R {
        RFCOFIP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Transmit Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn tabtip(&self) -> TABTIP_R {
        TABTIP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 30 - Write-Back Complete Interrupt Request Enable
    #[inline(always)]
    pub fn twbip(&self) -> TWBIP_R {
        TWBIP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - PTP v2 Packet Receive Interrupt Request Enable
    #[inline(always)]
    pub fn pverip(&mut self) -> PVERIP_W<EESIPR_SPEC> {
        PVERIP_W::new(self, 4)
    }
    ///Bit 8 - MAC Address Mismatch Interrupt Request Enable
    #[inline(always)]
    pub fn maceip(&mut self) -> MACEIP_W<EESIPR_SPEC> {
        MACEIP_W::new(self, 8)
    }
    ///Bit 16 - Receive FIFO Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfofip(&mut self) -> RFOFIP_W<EESIPR_SPEC> {
        RFOFIP_W::new(self, 16)
    }
    ///Bit 17 - Receive Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn rdeip(&mut self) -> RDEIP_W<EESIPR_SPEC> {
        RDEIP_W::new(self, 17)
    }
    ///Bit 18 - Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn frip(&mut self) -> FRIP_W<EESIPR_SPEC> {
        FRIP_W::new(self, 18)
    }
    ///Bit 19 - Transmit FIFO Underflow Interrupt Request Enable
    #[inline(always)]
    pub fn tfufip(&mut self) -> TFUFIP_W<EESIPR_SPEC> {
        TFUFIP_W::new(self, 19)
    }
    ///Bit 20 - Transmit Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tdeip(&mut self) -> TDEIP_W<EESIPR_SPEC> {
        TDEIP_W::new(self, 20)
    }
    ///Bit 21 - Frame Transfer Complete Interrupt Request Enable
    #[inline(always)]
    pub fn tcip(&mut self) -> TCIP_W<EESIPR_SPEC> {
        TCIP_W::new(self, 21)
    }
    ///Bit 23 - Address Error Interrupt Request Enable
    #[inline(always)]
    pub fn adeip(&mut self) -> ADEIP_W<EESIPR_SPEC> {
        ADEIP_W::new(self, 23)
    }
    ///Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfcofip(&mut self) -> RFCOFIP_W<EESIPR_SPEC> {
        RFCOFIP_W::new(self, 24)
    }
    ///Bit 26 - Transmit Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn tabtip(&mut self) -> TABTIP_W<EESIPR_SPEC> {
        TABTIP_W::new(self, 26)
    }
    ///Bit 30 - Write-Back Complete Interrupt Request Enable
    #[inline(always)]
    pub fn twbip(&mut self) -> TWBIP_W<EESIPR_SPEC> {
        TWBIP_W::new(self, 30)
    }
}
/**PTP/EDMAC Status Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`eesipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EESIPR_SPEC;
impl crate::RegisterSpec for EESIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`eesipr::R`](R) reader structure
impl crate::Readable for EESIPR_SPEC {}
///`write(|w| ..)` method takes [`eesipr::W`](W) writer structure
impl crate::Writable for EESIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EESIPR to value 0
impl crate::Resettable for EESIPR_SPEC {}
