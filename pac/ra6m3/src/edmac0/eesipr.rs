///Register `EESIPR` reader
pub type R = crate::R<EESIPR_SPEC>;
///Register `EESIPR` writer
pub type W = crate::W<EESIPR_SPEC>;
/**CRC Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERFIP_A {
    ///0: CRC error interrupt request is disabled.
    _0 = 0,
    ///1: CRC error interrupt request is enabled.
    _1 = 1,
}
impl From<CERFIP_A> for bool {
    #[inline(always)]
    fn from(variant: CERFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CERFIP` reader - CRC Error Interrupt Request Enable
pub type CERFIP_R = crate::BitReader<CERFIP_A>;
impl CERFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CERFIP_A {
        match self.bits {
            false => CERFIP_A::_0,
            true => CERFIP_A::_1,
        }
    }
    ///CRC error interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERFIP_A::_0
    }
    ///CRC error interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERFIP_A::_1
    }
}
///Field `CERFIP` writer - CRC Error Interrupt Request Enable
pub type CERFIP_W<'a, REG> = crate::BitWriter<'a, REG, CERFIP_A>;
impl<'a, REG> CERFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CERFIP_A::_0)
    }
    ///CRC error interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CERFIP_A::_1)
    }
}
/**PHY-LSI Receive Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREIP_A {
    ///0: PHY-LSI receive error interrupt request is disabled.
    _0 = 0,
    ///1: PHY-LSI receive error interrupt request is enabled.
    _1 = 1,
}
impl From<PREIP_A> for bool {
    #[inline(always)]
    fn from(variant: PREIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PREIP` reader - PHY-LSI Receive Error Interrupt Request Enable
pub type PREIP_R = crate::BitReader<PREIP_A>;
impl PREIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PREIP_A {
        match self.bits {
            false => PREIP_A::_0,
            true => PREIP_A::_1,
        }
    }
    ///PHY-LSI receive error interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PREIP_A::_0
    }
    ///PHY-LSI receive error interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PREIP_A::_1
    }
}
///Field `PREIP` writer - PHY-LSI Receive Error Interrupt Request Enable
pub type PREIP_W<'a, REG> = crate::BitWriter<'a, REG, PREIP_A>;
impl<'a, REG> PREIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PHY-LSI receive error interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PREIP_A::_0)
    }
    ///PHY-LSI receive error interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PREIP_A::_1)
    }
}
/**Frame-Too-Short Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSFIP_A {
    ///0: Frame-too-short error interrupt request is disabled.
    _0 = 0,
    ///1: Frame-too-short error interrupt request is enabled.
    _1 = 1,
}
impl From<RTSFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RTSFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSFIP` reader - Frame-Too-Short Error Interrupt Request Enable
pub type RTSFIP_R = crate::BitReader<RTSFIP_A>;
impl RTSFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTSFIP_A {
        match self.bits {
            false => RTSFIP_A::_0,
            true => RTSFIP_A::_1,
        }
    }
    ///Frame-too-short error interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTSFIP_A::_0
    }
    ///Frame-too-short error interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTSFIP_A::_1
    }
}
///Field `RTSFIP` writer - Frame-Too-Short Error Interrupt Request Enable
pub type RTSFIP_W<'a, REG> = crate::BitWriter<'a, REG, RTSFIP_A>;
impl<'a, REG> RTSFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame-too-short error interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTSFIP_A::_0)
    }
    ///Frame-too-short error interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTSFIP_A::_1)
    }
}
/**Frame-Too-Long Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTLFIP_A {
    ///0: Frame-too-long error interrupt request is disabled.
    _0 = 0,
    ///1: Frame-too-long error interrupt request is enabled.
    _1 = 1,
}
impl From<RTLFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RTLFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTLFIP` reader - Frame-Too-Long Error Interrupt Request Enable
pub type RTLFIP_R = crate::BitReader<RTLFIP_A>;
impl RTLFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTLFIP_A {
        match self.bits {
            false => RTLFIP_A::_0,
            true => RTLFIP_A::_1,
        }
    }
    ///Frame-too-long error interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTLFIP_A::_0
    }
    ///Frame-too-long error interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTLFIP_A::_1
    }
}
///Field `RTLFIP` writer - Frame-Too-Long Error Interrupt Request Enable
pub type RTLFIP_W<'a, REG> = crate::BitWriter<'a, REG, RTLFIP_A>;
impl<'a, REG> RTLFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame-too-long error interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTLFIP_A::_0)
    }
    ///Frame-too-long error interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTLFIP_A::_1)
    }
}
/**Alignment Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFIP_A {
    ///0: Alignment error interrupt request is disabled.
    _0 = 0,
    ///1: Alignment error interrupt request is enabled.
    _1 = 1,
}
impl From<RRFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RRFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFIP` reader - Alignment Error Interrupt Request Enable
pub type RRFIP_R = crate::BitReader<RRFIP_A>;
impl RRFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRFIP_A {
        match self.bits {
            false => RRFIP_A::_0,
            true => RRFIP_A::_1,
        }
    }
    ///Alignment error interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRFIP_A::_0
    }
    ///Alignment error interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRFIP_A::_1
    }
}
///Field `RRFIP` writer - Alignment Error Interrupt Request Enable
pub type RRFIP_W<'a, REG> = crate::BitWriter<'a, REG, RRFIP_A>;
impl<'a, REG> RRFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alignment error interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RRFIP_A::_0)
    }
    ///Alignment error interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RRFIP_A::_1)
    }
}
/**Multicast Address Frame Receive Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMAFIP_A {
    ///0: Multicast address frame receive interrupt request is disabled.
    _0 = 0,
    ///1: Multicast address frame receive interrupt request is enabled.
    _1 = 1,
}
impl From<RMAFIP_A> for bool {
    #[inline(always)]
    fn from(variant: RMAFIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RMAFIP` reader - Multicast Address Frame Receive Interrupt Request Enable
pub type RMAFIP_R = crate::BitReader<RMAFIP_A>;
impl RMAFIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMAFIP_A {
        match self.bits {
            false => RMAFIP_A::_0,
            true => RMAFIP_A::_1,
        }
    }
    ///Multicast address frame receive interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMAFIP_A::_0
    }
    ///Multicast address frame receive interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMAFIP_A::_1
    }
}
///Field `RMAFIP` writer - Multicast Address Frame Receive Interrupt Request Enable
pub type RMAFIP_W<'a, REG> = crate::BitWriter<'a, REG, RMAFIP_A>;
impl<'a, REG> RMAFIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Multicast address frame receive interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RMAFIP_A::_0)
    }
    ///Multicast address frame receive interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RMAFIP_A::_1)
    }
}
/**Transmit Retry Over Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROIP_A {
    ///0: Transmit retry over interrupt request is disabled.
    _0 = 0,
    ///1: Transmit retry over interrupt request is enabled.
    _1 = 1,
}
impl From<TROIP_A> for bool {
    #[inline(always)]
    fn from(variant: TROIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TROIP` reader - Transmit Retry Over Interrupt Request Enable
pub type TROIP_R = crate::BitReader<TROIP_A>;
impl TROIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TROIP_A {
        match self.bits {
            false => TROIP_A::_0,
            true => TROIP_A::_1,
        }
    }
    ///Transmit retry over interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TROIP_A::_0
    }
    ///Transmit retry over interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TROIP_A::_1
    }
}
///Field `TROIP` writer - Transmit Retry Over Interrupt Request Enable
pub type TROIP_W<'a, REG> = crate::BitWriter<'a, REG, TROIP_A>;
impl<'a, REG> TROIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit retry over interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TROIP_A::_0)
    }
    ///Transmit retry over interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TROIP_A::_1)
    }
}
/**Late Collision Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDIP_A {
    ///0: Late collision detect interrupt request is disabled.
    _0 = 0,
    ///1: Late collision detect interrupt request is enabled.
    _1 = 1,
}
impl From<CDIP_A> for bool {
    #[inline(always)]
    fn from(variant: CDIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDIP` reader - Late Collision Detect Interrupt Request Enable
pub type CDIP_R = crate::BitReader<CDIP_A>;
impl CDIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDIP_A {
        match self.bits {
            false => CDIP_A::_0,
            true => CDIP_A::_1,
        }
    }
    ///Late collision detect interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDIP_A::_0
    }
    ///Late collision detect interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDIP_A::_1
    }
}
///Field `CDIP` writer - Late Collision Detect Interrupt Request Enable
pub type CDIP_W<'a, REG> = crate::BitWriter<'a, REG, CDIP_A>;
impl<'a, REG> CDIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Late collision detect interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDIP_A::_0)
    }
    ///Late collision detect interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDIP_A::_1)
    }
}
/**Loss of Carrier Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLCIP_A {
    ///0: Loss of carrier detect interrupt request is disabled.
    _0 = 0,
    ///1: Loss of carrier detect interrupt request is enabled.
    _1 = 1,
}
impl From<DLCIP_A> for bool {
    #[inline(always)]
    fn from(variant: DLCIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLCIP` reader - Loss of Carrier Detect Interrupt Request Enable
pub type DLCIP_R = crate::BitReader<DLCIP_A>;
impl DLCIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLCIP_A {
        match self.bits {
            false => DLCIP_A::_0,
            true => DLCIP_A::_1,
        }
    }
    ///Loss of carrier detect interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLCIP_A::_0
    }
    ///Loss of carrier detect interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLCIP_A::_1
    }
}
///Field `DLCIP` writer - Loss of Carrier Detect Interrupt Request Enable
pub type DLCIP_W<'a, REG> = crate::BitWriter<'a, REG, DLCIP_A>;
impl<'a, REG> DLCIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Loss of carrier detect interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLCIP_A::_0)
    }
    ///Loss of carrier detect interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLCIP_A::_1)
    }
}
/**Carrier Not Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNDIP_A {
    ///0: Carrier not detect interrupt request is disabled.
    _0 = 0,
    ///1: Carrier not detect interrupt request is enabled.
    _1 = 1,
}
impl From<CNDIP_A> for bool {
    #[inline(always)]
    fn from(variant: CNDIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CNDIP` reader - Carrier Not Detect Interrupt Request Enable
pub type CNDIP_R = crate::BitReader<CNDIP_A>;
impl CNDIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNDIP_A {
        match self.bits {
            false => CNDIP_A::_0,
            true => CNDIP_A::_1,
        }
    }
    ///Carrier not detect interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNDIP_A::_0
    }
    ///Carrier not detect interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNDIP_A::_1
    }
}
///Field `CNDIP` writer - Carrier Not Detect Interrupt Request Enable
pub type CNDIP_W<'a, REG> = crate::BitWriter<'a, REG, CNDIP_A>;
impl<'a, REG> CNDIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Carrier not detect interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CNDIP_A::_0)
    }
    ///Carrier not detect interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CNDIP_A::_1)
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
    ///0: Frame reception interrupt request is disabled.
    _0 = 0,
    ///1: Frame reception interrupt request is enabled.
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
    ///Frame reception interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRIP_A::_0
    }
    ///Frame reception interrupt request is enabled.
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
    ///Frame reception interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FRIP_A::_0)
    }
    ///Frame reception interrupt request is enabled.
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
/**ETHERC Status Register Source Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECIIP_A {
    ///0: ETHERC status interrupt request is disabled.
    _0 = 0,
    ///1: ETHERC status interrupt request is enabled.
    _1 = 1,
}
impl From<ECIIP_A> for bool {
    #[inline(always)]
    fn from(variant: ECIIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ECIIP` reader - ETHERC Status Register Source Interrupt Request Enable
pub type ECIIP_R = crate::BitReader<ECIIP_A>;
impl ECIIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECIIP_A {
        match self.bits {
            false => ECIIP_A::_0,
            true => ECIIP_A::_1,
        }
    }
    ///ETHERC status interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECIIP_A::_0
    }
    ///ETHERC status interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECIIP_A::_1
    }
}
///Field `ECIIP` writer - ETHERC Status Register Source Interrupt Request Enable
pub type ECIIP_W<'a, REG> = crate::BitWriter<'a, REG, ECIIP_A>;
impl<'a, REG> ECIIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ETHERC status interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECIIP_A::_0)
    }
    ///ETHERC status interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECIIP_A::_1)
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
/**Receive Abort Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RABTIP_A {
    ///0: Receive abort detect interrupt request is disabled.
    _0 = 0,
    ///1: Receive abort detect interrupt request is enabled.
    _1 = 1,
}
impl From<RABTIP_A> for bool {
    #[inline(always)]
    fn from(variant: RABTIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RABTIP` reader - Receive Abort Detect Interrupt Request Enable
pub type RABTIP_R = crate::BitReader<RABTIP_A>;
impl RABTIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RABTIP_A {
        match self.bits {
            false => RABTIP_A::_0,
            true => RABTIP_A::_1,
        }
    }
    ///Receive abort detect interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RABTIP_A::_0
    }
    ///Receive abort detect interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RABTIP_A::_1
    }
}
///Field `RABTIP` writer - Receive Abort Detect Interrupt Request Enable
pub type RABTIP_W<'a, REG> = crate::BitWriter<'a, REG, RABTIP_A>;
impl<'a, REG> RABTIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive abort detect interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RABTIP_A::_0)
    }
    ///Receive abort detect interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RABTIP_A::_1)
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
    ///Bit 0 - CRC Error Interrupt Request Enable
    #[inline(always)]
    pub fn cerfip(&self) -> CERFIP_R {
        CERFIP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Interrupt Request Enable
    #[inline(always)]
    pub fn preip(&self) -> PREIP_R {
        PREIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame-Too-Short Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtsfip(&self) -> RTSFIP_R {
        RTSFIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Frame-Too-Long Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtlfip(&self) -> RTLFIP_R {
        RTLFIP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alignment Error Interrupt Request Enable
    #[inline(always)]
    pub fn rrfip(&self) -> RRFIP_R {
        RRFIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Multicast Address Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn rmafip(&self) -> RMAFIP_R {
        RMAFIP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit Retry Over Interrupt Request Enable
    #[inline(always)]
    pub fn troip(&self) -> TROIP_R {
        TROIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Late Collision Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cdip(&self) -> CDIP_R {
        CDIP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Loss of Carrier Detect Interrupt Request Enable
    #[inline(always)]
    pub fn dlcip(&self) -> DLCIP_R {
        DLCIP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Carrier Not Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cndip(&self) -> CNDIP_R {
        CNDIP_R::new(((self.bits >> 11) & 1) != 0)
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
    ///Bit 22 - ETHERC Status Register Source Interrupt Request Enable
    #[inline(always)]
    pub fn eciip(&self) -> ECIIP_R {
        ECIIP_R::new(((self.bits >> 22) & 1) != 0)
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
    ///Bit 25 - Receive Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn rabtip(&self) -> RABTIP_R {
        RABTIP_R::new(((self.bits >> 25) & 1) != 0)
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
    ///Bit 0 - CRC Error Interrupt Request Enable
    #[inline(always)]
    pub fn cerfip(&mut self) -> CERFIP_W<EESIPR_SPEC> {
        CERFIP_W::new(self, 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Interrupt Request Enable
    #[inline(always)]
    pub fn preip(&mut self) -> PREIP_W<EESIPR_SPEC> {
        PREIP_W::new(self, 1)
    }
    ///Bit 2 - Frame-Too-Short Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtsfip(&mut self) -> RTSFIP_W<EESIPR_SPEC> {
        RTSFIP_W::new(self, 2)
    }
    ///Bit 3 - Frame-Too-Long Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtlfip(&mut self) -> RTLFIP_W<EESIPR_SPEC> {
        RTLFIP_W::new(self, 3)
    }
    ///Bit 4 - Alignment Error Interrupt Request Enable
    #[inline(always)]
    pub fn rrfip(&mut self) -> RRFIP_W<EESIPR_SPEC> {
        RRFIP_W::new(self, 4)
    }
    ///Bit 7 - Multicast Address Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn rmafip(&mut self) -> RMAFIP_W<EESIPR_SPEC> {
        RMAFIP_W::new(self, 7)
    }
    ///Bit 8 - Transmit Retry Over Interrupt Request Enable
    #[inline(always)]
    pub fn troip(&mut self) -> TROIP_W<EESIPR_SPEC> {
        TROIP_W::new(self, 8)
    }
    ///Bit 9 - Late Collision Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cdip(&mut self) -> CDIP_W<EESIPR_SPEC> {
        CDIP_W::new(self, 9)
    }
    ///Bit 10 - Loss of Carrier Detect Interrupt Request Enable
    #[inline(always)]
    pub fn dlcip(&mut self) -> DLCIP_W<EESIPR_SPEC> {
        DLCIP_W::new(self, 10)
    }
    ///Bit 11 - Carrier Not Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cndip(&mut self) -> CNDIP_W<EESIPR_SPEC> {
        CNDIP_W::new(self, 11)
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
    ///Bit 22 - ETHERC Status Register Source Interrupt Request Enable
    #[inline(always)]
    pub fn eciip(&mut self) -> ECIIP_W<EESIPR_SPEC> {
        ECIIP_W::new(self, 22)
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
    ///Bit 25 - Receive Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn rabtip(&mut self) -> RABTIP_W<EESIPR_SPEC> {
        RABTIP_W::new(self, 25)
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
/**ETHERC/EDMAC Status Interrupt Enable Register

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
