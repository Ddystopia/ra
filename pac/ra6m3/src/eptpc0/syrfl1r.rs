///Register `SYRFL1R` reader
pub type R = crate::R<SYRFL1R_SPEC>;
///Register `SYRFL1R` writer
pub type W = crate::W<SYRFL1R_SPEC>;
/**Announce Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANCE0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<ANCE0_A> for bool {
    #[inline(always)]
    fn from(variant: ANCE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANCE0` reader - Announce Message Processing
pub type ANCE0_R = crate::BitReader<ANCE0_A>;
impl ANCE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANCE0_A {
        match self.bits {
            false => ANCE0_A::_0,
            true => ANCE0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANCE0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANCE0_A::_1
    }
}
///Field `ANCE0` writer - Announce Message Processing
pub type ANCE0_W<'a, REG> = crate::BitWriter<'a, REG, ANCE0_A>;
impl<'a, REG> ANCE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANCE0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANCE0_A::_1)
    }
}
/**Sync Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<SYNC0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC0` reader - Sync Message Processing
pub type SYNC0_R = crate::BitReader<SYNC0_A>;
impl SYNC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNC0_A {
        match self.bits {
            false => SYNC0_A::_0,
            true => SYNC0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC0_A::_1
    }
}
///Field `SYNC0` writer - Sync Message Processing
pub type SYNC0_W<'a, REG> = crate::BitWriter<'a, REG, SYNC0_A>;
impl<'a, REG> SYNC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC0_A::_1)
    }
}
/**Sync Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP processes messages.
    _1 = 1,
}
impl From<SYNC2_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC2` reader - Sync Message Processing
pub type SYNC2_R = crate::BitReader<SYNC2_A>;
impl SYNC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNC2_A {
        match self.bits {
            false => SYNC2_A::_0,
            true => SYNC2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC2_A::_0
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC2_A::_1
    }
}
///Field `SYNC2` writer - Sync Message Processing
pub type SYNC2_W<'a, REG> = crate::BitWriter<'a, REG, SYNC2_A>;
impl<'a, REG> SYNC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC2_A::_0)
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC2_A::_1)
    }
}
/**Follow_Up Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUP0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<FUP0_A> for bool {
    #[inline(always)]
    fn from(variant: FUP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FUP0` reader - Follow_Up Message Processing
pub type FUP0_R = crate::BitReader<FUP0_A>;
impl FUP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FUP0_A {
        match self.bits {
            false => FUP0_A::_0,
            true => FUP0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUP0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUP0_A::_1
    }
}
///Field `FUP0` writer - Follow_Up Message Processing
pub type FUP0_W<'a, REG> = crate::BitWriter<'a, REG, FUP0_A>;
impl<'a, REG> FUP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FUP0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FUP0_A::_1)
    }
}
/**Follow_Up Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUP2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP processes messages.
    _1 = 1,
}
impl From<FUP2_A> for bool {
    #[inline(always)]
    fn from(variant: FUP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FUP2` reader - Follow_Up Message Processing
pub type FUP2_R = crate::BitReader<FUP2_A>;
impl FUP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FUP2_A {
        match self.bits {
            false => FUP2_A::_0,
            true => FUP2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUP2_A::_0
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUP2_A::_1
    }
}
///Field `FUP2` writer - Follow_Up Message Processing
pub type FUP2_W<'a, REG> = crate::BitWriter<'a, REG, FUP2_A>;
impl<'a, REG> FUP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FUP2_A::_0)
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FUP2_A::_1)
    }
}
/**Delay_Req Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<DRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRQ0` reader - Delay_Req Message Processing
pub type DRQ0_R = crate::BitReader<DRQ0_A>;
impl DRQ0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRQ0_A {
        match self.bits {
            false => DRQ0_A::_0,
            true => DRQ0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQ0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQ0_A::_1
    }
}
///Field `DRQ0` writer - Delay_Req Message Processing
pub type DRQ0_W<'a, REG> = crate::BitWriter<'a, REG, DRQ0_A>;
impl<'a, REG> DRQ0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ0_A::_1)
    }
}
/**Delay_Req Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP processes messages.
    _1 = 1,
}
impl From<DRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRQ2` reader - Delay_Req Message Processing
pub type DRQ2_R = crate::BitReader<DRQ2_A>;
impl DRQ2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRQ2_A {
        match self.bits {
            false => DRQ2_A::_0,
            true => DRQ2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQ2_A::_0
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQ2_A::_1
    }
}
///Field `DRQ2` writer - Delay_Req Message Processing
pub type DRQ2_W<'a, REG> = crate::BitWriter<'a, REG, DRQ2_A>;
impl<'a, REG> DRQ2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ2_A::_0)
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ2_A::_1)
    }
}
/**Delay_Resp Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRP0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<DRP0_A> for bool {
    #[inline(always)]
    fn from(variant: DRP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRP0` reader - Delay_Resp Message Processing
pub type DRP0_R = crate::BitReader<DRP0_A>;
impl DRP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRP0_A {
        match self.bits {
            false => DRP0_A::_0,
            true => DRP0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRP0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRP0_A::_1
    }
}
///Field `DRP0` writer - Delay_Resp Message Processing
pub type DRP0_W<'a, REG> = crate::BitWriter<'a, REG, DRP0_A>;
impl<'a, REG> DRP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRP0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRP0_A::_1)
    }
}
/**Delay_Resp Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRP2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP processes messages.
    _1 = 1,
}
impl From<DRP2_A> for bool {
    #[inline(always)]
    fn from(variant: DRP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRP2` reader - Delay_Resp Message Processing
pub type DRP2_R = crate::BitReader<DRP2_A>;
impl DRP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRP2_A {
        match self.bits {
            false => DRP2_A::_0,
            true => DRP2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRP2_A::_0
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRP2_A::_1
    }
}
///Field `DRP2` writer - Delay_Resp Message Processing
pub type DRP2_W<'a, REG> = crate::BitWriter<'a, REG, DRP2_A>;
impl<'a, REG> DRP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRP2_A::_0)
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRP2_A::_1)
    }
}
/**Pdelay_Req Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRQ0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<PDRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: PDRQ0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDRQ0` reader - Pdelay_Req Message Processing
pub type PDRQ0_R = crate::BitReader<PDRQ0_A>;
impl PDRQ0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDRQ0_A {
        match self.bits {
            false => PDRQ0_A::_0,
            true => PDRQ0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRQ0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRQ0_A::_1
    }
}
///Field `PDRQ0` writer - Pdelay_Req Message Processing
pub type PDRQ0_W<'a, REG> = crate::BitWriter<'a, REG, PDRQ0_A>;
impl<'a, REG> PDRQ0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDRQ0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDRQ0_A::_1)
    }
}
/**Pdelay_Req Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRQ2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP processes messages.
    _1 = 1,
}
impl From<PDRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: PDRQ2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDRQ2` reader - Pdelay_Req Message Processing
pub type PDRQ2_R = crate::BitReader<PDRQ2_A>;
impl PDRQ2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDRQ2_A {
        match self.bits {
            false => PDRQ2_A::_0,
            true => PDRQ2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRQ2_A::_0
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRQ2_A::_1
    }
}
///Field `PDRQ2` writer - Pdelay_Req Message Processing
pub type PDRQ2_W<'a, REG> = crate::BitWriter<'a, REG, PDRQ2_A>;
impl<'a, REG> PDRQ2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDRQ2_A::_0)
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDRQ2_A::_1)
    }
}
/**Pdelay_Resp Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRP0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<PDRP0_A> for bool {
    #[inline(always)]
    fn from(variant: PDRP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDRP0` reader - Pdelay_Resp Message Processing
pub type PDRP0_R = crate::BitReader<PDRP0_A>;
impl PDRP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDRP0_A {
        match self.bits {
            false => PDRP0_A::_0,
            true => PDRP0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRP0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRP0_A::_1
    }
}
///Field `PDRP0` writer - Pdelay_Resp Message Processing
pub type PDRP0_W<'a, REG> = crate::BitWriter<'a, REG, PDRP0_A>;
impl<'a, REG> PDRP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDRP0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDRP0_A::_1)
    }
}
/**Pdelay_Resp Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRP2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP processes messages.
    _1 = 1,
}
impl From<PDRP2_A> for bool {
    #[inline(always)]
    fn from(variant: PDRP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDRP2` reader - Pdelay_Resp Message Processing
pub type PDRP2_R = crate::BitReader<PDRP2_A>;
impl PDRP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDRP2_A {
        match self.bits {
            false => PDRP2_A::_0,
            true => PDRP2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRP2_A::_0
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRP2_A::_1
    }
}
///Field `PDRP2` writer - Pdelay_Resp Message Processing
pub type PDRP2_W<'a, REG> = crate::BitWriter<'a, REG, PDRP2_A>;
impl<'a, REG> PDRP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDRP2_A::_0)
    }
    ///The SYNFP processes messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDRP2_A::_1)
    }
}
/**Pdelay_Resp_Follow_Up Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDFUP0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<PDFUP0_A> for bool {
    #[inline(always)]
    fn from(variant: PDFUP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDFUP0` reader - Pdelay_Resp_Follow_Up Message Processing
pub type PDFUP0_R = crate::BitReader<PDFUP0_A>;
impl PDFUP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDFUP0_A {
        match self.bits {
            false => PDFUP0_A::_0,
            true => PDFUP0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDFUP0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDFUP0_A::_1
    }
}
///Field `PDFUP0` writer - Pdelay_Resp_Follow_Up Message Processing
pub type PDFUP0_W<'a, REG> = crate::BitWriter<'a, REG, PDFUP0_A>;
impl<'a, REG> PDFUP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDFUP0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDFUP0_A::_1)
    }
}
/**Pdelay_Resp_Follow_Up Message Processing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDFUP2_A {
    ///0: The SYNFP does not process messages.
    _0 = 0,
    ///1: The SYNFP does not process messages.
    _1 = 1,
}
impl From<PDFUP2_A> for bool {
    #[inline(always)]
    fn from(variant: PDFUP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDFUP2` reader - Pdelay_Resp_Follow_Up Message Processing
pub type PDFUP2_R = crate::BitReader<PDFUP2_A>;
impl PDFUP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDFUP2_A {
        match self.bits {
            false => PDFUP2_A::_0,
            true => PDFUP2_A::_1,
        }
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDFUP2_A::_0
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDFUP2_A::_1
    }
}
///Field `PDFUP2` writer - Pdelay_Resp_Follow_Up Message Processing
pub type PDFUP2_W<'a, REG> = crate::BitWriter<'a, REG, PDFUP2_A>;
impl<'a, REG> PDFUP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDFUP2_A::_0)
    }
    ///The SYNFP does not process messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDFUP2_A::_1)
    }
}
impl R {
    ///Bit 0 - Announce Message Processing
    #[inline(always)]
    pub fn ance0(&self) -> ANCE0_R {
        ANCE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Sync Message Processing
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Sync Message Processing
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Follow_Up Message Processing
    #[inline(always)]
    pub fn fup0(&self) -> FUP0_R {
        FUP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Follow_Up Message Processing
    #[inline(always)]
    pub fn fup2(&self) -> FUP2_R {
        FUP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Delay_Req Message Processing
    #[inline(always)]
    pub fn drq0(&self) -> DRQ0_R {
        DRQ0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Delay_Req Message Processing
    #[inline(always)]
    pub fn drq2(&self) -> DRQ2_R {
        DRQ2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Delay_Resp Message Processing
    #[inline(always)]
    pub fn drp0(&self) -> DRP0_R {
        DRP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Delay_Resp Message Processing
    #[inline(always)]
    pub fn drp2(&self) -> DRP2_R {
        DRP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Pdelay_Req Message Processing
    #[inline(always)]
    pub fn pdrq0(&self) -> PDRQ0_R {
        PDRQ0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Pdelay_Req Message Processing
    #[inline(always)]
    pub fn pdrq2(&self) -> PDRQ2_R {
        PDRQ2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Pdelay_Resp Message Processing
    #[inline(always)]
    pub fn pdrp0(&self) -> PDRP0_R {
        PDRP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Pdelay_Resp Message Processing
    #[inline(always)]
    pub fn pdrp2(&self) -> PDRP2_R {
        PDRP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Pdelay_Resp_Follow_Up Message Processing
    #[inline(always)]
    pub fn pdfup0(&self) -> PDFUP0_R {
        PDFUP0_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Pdelay_Resp_Follow_Up Message Processing
    #[inline(always)]
    pub fn pdfup2(&self) -> PDFUP2_R {
        PDFUP2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Announce Message Processing
    #[inline(always)]
    pub fn ance0(&mut self) -> ANCE0_W<SYRFL1R_SPEC> {
        ANCE0_W::new(self, 0)
    }
    ///Bit 4 - Sync Message Processing
    #[inline(always)]
    pub fn sync0(&mut self) -> SYNC0_W<SYRFL1R_SPEC> {
        SYNC0_W::new(self, 4)
    }
    ///Bit 6 - Sync Message Processing
    #[inline(always)]
    pub fn sync2(&mut self) -> SYNC2_W<SYRFL1R_SPEC> {
        SYNC2_W::new(self, 6)
    }
    ///Bit 8 - Follow_Up Message Processing
    #[inline(always)]
    pub fn fup0(&mut self) -> FUP0_W<SYRFL1R_SPEC> {
        FUP0_W::new(self, 8)
    }
    ///Bit 10 - Follow_Up Message Processing
    #[inline(always)]
    pub fn fup2(&mut self) -> FUP2_W<SYRFL1R_SPEC> {
        FUP2_W::new(self, 10)
    }
    ///Bit 12 - Delay_Req Message Processing
    #[inline(always)]
    pub fn drq0(&mut self) -> DRQ0_W<SYRFL1R_SPEC> {
        DRQ0_W::new(self, 12)
    }
    ///Bit 14 - Delay_Req Message Processing
    #[inline(always)]
    pub fn drq2(&mut self) -> DRQ2_W<SYRFL1R_SPEC> {
        DRQ2_W::new(self, 14)
    }
    ///Bit 16 - Delay_Resp Message Processing
    #[inline(always)]
    pub fn drp0(&mut self) -> DRP0_W<SYRFL1R_SPEC> {
        DRP0_W::new(self, 16)
    }
    ///Bit 18 - Delay_Resp Message Processing
    #[inline(always)]
    pub fn drp2(&mut self) -> DRP2_W<SYRFL1R_SPEC> {
        DRP2_W::new(self, 18)
    }
    ///Bit 20 - Pdelay_Req Message Processing
    #[inline(always)]
    pub fn pdrq0(&mut self) -> PDRQ0_W<SYRFL1R_SPEC> {
        PDRQ0_W::new(self, 20)
    }
    ///Bit 22 - Pdelay_Req Message Processing
    #[inline(always)]
    pub fn pdrq2(&mut self) -> PDRQ2_W<SYRFL1R_SPEC> {
        PDRQ2_W::new(self, 22)
    }
    ///Bit 24 - Pdelay_Resp Message Processing
    #[inline(always)]
    pub fn pdrp0(&mut self) -> PDRP0_W<SYRFL1R_SPEC> {
        PDRP0_W::new(self, 24)
    }
    ///Bit 26 - Pdelay_Resp Message Processing
    #[inline(always)]
    pub fn pdrp2(&mut self) -> PDRP2_W<SYRFL1R_SPEC> {
        PDRP2_W::new(self, 26)
    }
    ///Bit 28 - Pdelay_Resp_Follow_Up Message Processing
    #[inline(always)]
    pub fn pdfup0(&mut self) -> PDFUP0_W<SYRFL1R_SPEC> {
        PDFUP0_W::new(self, 28)
    }
    ///Bit 30 - Pdelay_Resp_Follow_Up Message Processing
    #[inline(always)]
    pub fn pdfup2(&mut self) -> PDFUP2_W<SYRFL1R_SPEC> {
        PDFUP2_W::new(self, 30)
    }
}
/**SYNFP Reception Filter Register 1

You can [`read`](crate::Reg::read) this register and get [`syrfl1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syrfl1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYRFL1R_SPEC;
impl crate::RegisterSpec for SYRFL1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syrfl1r::R`](R) reader structure
impl crate::Readable for SYRFL1R_SPEC {}
///`write(|w| ..)` method takes [`syrfl1r::W`](W) writer structure
impl crate::Writable for SYRFL1R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYRFL1R to value 0
impl crate::Resettable for SYRFL1R_SPEC {}
