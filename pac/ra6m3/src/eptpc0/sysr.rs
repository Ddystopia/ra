///Register `SYSR` reader
pub type R = crate::R<SYSR_SPEC>;
///Register `SYSR` writer
pub type W = crate::W<SYSR_SPEC>;
/**offsetFromMaster Value Update Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFMUD_A {
    ///0: The offsetFromMaster value has not been updated.
    _0 = 0,
    ///1: The offsetFromMaster value has been updated.
    _1 = 1,
}
impl From<OFMUD_A> for bool {
    #[inline(always)]
    fn from(variant: OFMUD_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OFMUD` reader - offsetFromMaster Value Update Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OFMUD_R = crate::BitReader<OFMUD_A>;
impl OFMUD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFMUD_A {
        match self.bits {
            false => OFMUD_A::_0,
            true => OFMUD_A::_1,
        }
    }
    ///The offsetFromMaster value has not been updated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFMUD_A::_0
    }
    ///The offsetFromMaster value has been updated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFMUD_A::_1
    }
}
///Field `OFMUD` writer - offsetFromMaster Value Update Flag
pub type OFMUD_W<'a, REG> = crate::BitWriter1C<'a, REG, OFMUD_A>;
impl<'a, REG> OFMUD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The offsetFromMaster value has not been updated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OFMUD_A::_0)
    }
    ///The offsetFromMaster value has been updated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OFMUD_A::_1)
    }
}
/**Receive logMessageInterval Value Change Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTCHG_A {
    ///0: No change in the received logMessageInterval value.
    _0 = 0,
    ///1: A change in the received logMessageInterval value.
    _1 = 1,
}
impl From<INTCHG_A> for bool {
    #[inline(always)]
    fn from(variant: INTCHG_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `INTCHG` reader - Receive logMessageInterval Value Change Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type INTCHG_R = crate::BitReader<INTCHG_A>;
impl INTCHG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTCHG_A {
        match self.bits {
            false => INTCHG_A::_0,
            true => INTCHG_A::_1,
        }
    }
    ///No change in the received logMessageInterval value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTCHG_A::_0
    }
    ///A change in the received logMessageInterval value.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTCHG_A::_1
    }
}
///Field `INTCHG` writer - Receive logMessageInterval Value Change Detection Flag
pub type INTCHG_W<'a, REG> = crate::BitWriter1C<'a, REG, INTCHG_A>;
impl<'a, REG> INTCHG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No change in the received logMessageInterval value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INTCHG_A::_0)
    }
    ///A change in the received logMessageInterval value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INTCHG_A::_1)
    }
}
/**meanPathDelay Value Update Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDUD_A {
    ///0: The meanPathDelay value has not been updated.
    _0 = 0,
    ///1: The meanPathDelay value has been updated.
    _1 = 1,
}
impl From<MPDUD_A> for bool {
    #[inline(always)]
    fn from(variant: MPDUD_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `MPDUD` reader - meanPathDelay Value Update Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type MPDUD_R = crate::BitReader<MPDUD_A>;
impl MPDUD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPDUD_A {
        match self.bits {
            false => MPDUD_A::_0,
            true => MPDUD_A::_1,
        }
    }
    ///The meanPathDelay value has not been updated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDUD_A::_0
    }
    ///The meanPathDelay value has been updated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDUD_A::_1
    }
}
///Field `MPDUD` writer - meanPathDelay Value Update Flag
pub type MPDUD_W<'a, REG> = crate::BitWriter1C<'a, REG, MPDUD_A>;
impl<'a, REG> MPDUD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The meanPathDelay value has not been updated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPDUD_A::_0)
    }
    ///The meanPathDelay value has been updated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPDUD_A::_1)
    }
}
/**Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPTO_A {
    ///0: A Delay_Resp/Pdelay_Resp timeout has not occurred.
    _0 = 0,
    ///1: A Delay_Resp/Pdelay_Resp timeout has occurred.
    _1 = 1,
}
impl From<DRPTO_A> for bool {
    #[inline(always)]
    fn from(variant: DRPTO_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DRPTO` reader - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DRPTO_R = crate::BitReader<DRPTO_A>;
impl DRPTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRPTO_A {
        match self.bits {
            false => DRPTO_A::_0,
            true => DRPTO_A::_1,
        }
    }
    ///A Delay_Resp/Pdelay_Resp timeout has not occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPTO_A::_0
    }
    ///A Delay_Resp/Pdelay_Resp timeout has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPTO_A::_1
    }
}
///Field `DRPTO` writer - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag
pub type DRPTO_W<'a, REG> = crate::BitWriter1C<'a, REG, DRPTO_A>;
impl<'a, REG> DRPTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A Delay_Resp/Pdelay_Resp timeout has not occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRPTO_A::_0)
    }
    ///A Delay_Resp/Pdelay_Resp timeout has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRPTO_A::_1)
    }
}
/**Receive logMessageInterval Value Out-of-Range Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTDEV_A {
    ///0: The received logMessageInterval value is within the range.
    _0 = 0,
    ///1: The received logMessageInterval value is out of the range.
    _1 = 1,
}
impl From<INTDEV_A> for bool {
    #[inline(always)]
    fn from(variant: INTDEV_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `INTDEV` reader - Receive logMessageInterval Value Out-of-Range Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type INTDEV_R = crate::BitReader<INTDEV_A>;
impl INTDEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTDEV_A {
        match self.bits {
            false => INTDEV_A::_0,
            true => INTDEV_A::_1,
        }
    }
    ///The received logMessageInterval value is within the range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTDEV_A::_0
    }
    ///The received logMessageInterval value is out of the range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTDEV_A::_1
    }
}
///Field `INTDEV` writer - Receive logMessageInterval Value Out-of-Range Flag
pub type INTDEV_W<'a, REG> = crate::BitWriter1C<'a, REG, INTDEV_A>;
impl<'a, REG> INTDEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The received logMessageInterval value is within the range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INTDEV_A::_0)
    }
    ///The received logMessageInterval value is out of the range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INTDEV_A::_1)
    }
}
/**Delay_Req Reception FIFO Overflow Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQOVR_A {
    ///0: The received Delay_Req has not caused the reception FIFO to overflow.
    _0 = 0,
    ///1: The received Delay_Req has caused the reception FIFO to overflow.
    _1 = 1,
}
impl From<DRQOVR_A> for bool {
    #[inline(always)]
    fn from(variant: DRQOVR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DRQOVR` reader - Delay_Req Reception FIFO Overflow Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DRQOVR_R = crate::BitReader<DRQOVR_A>;
impl DRQOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRQOVR_A {
        match self.bits {
            false => DRQOVR_A::_0,
            true => DRQOVR_A::_1,
        }
    }
    ///The received Delay_Req has not caused the reception FIFO to overflow.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQOVR_A::_0
    }
    ///The received Delay_Req has caused the reception FIFO to overflow.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQOVR_A::_1
    }
}
///Field `DRQOVR` writer - Delay_Req Reception FIFO Overflow Detection Flag
pub type DRQOVR_W<'a, REG> = crate::BitWriter1C<'a, REG, DRQOVR_A>;
impl<'a, REG> DRQOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The received Delay_Req has not caused the reception FIFO to overflow.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRQOVR_A::_0)
    }
    ///The received Delay_Req has caused the reception FIFO to overflow.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRQOVR_A::_1)
    }
}
/**Loop Reception Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECLP_A {
    ///0: A received message has not returned through a loop.
    _0 = 0,
    ///1: A received message has returned through a loop.
    _1 = 1,
}
impl From<RECLP_A> for bool {
    #[inline(always)]
    fn from(variant: RECLP_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RECLP` reader - Loop Reception Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RECLP_R = crate::BitReader<RECLP_A>;
impl RECLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RECLP_A {
        match self.bits {
            false => RECLP_A::_0,
            true => RECLP_A::_1,
        }
    }
    ///A received message has not returned through a loop.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECLP_A::_0
    }
    ///A received message has returned through a loop.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECLP_A::_1
    }
}
///Field `RECLP` writer - Loop Reception Detection Flag
pub type RECLP_W<'a, REG> = crate::BitWriter1C<'a, REG, RECLP_A>;
impl<'a, REG> RECLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A received message has not returned through a loop.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RECLP_A::_0)
    }
    ///A received message has returned through a loop.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RECLP_A::_1)
    }
}
/**Control Information Abnormality Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFABT_A {
    ///0: No abnormality in control information
    _0 = 0,
    ///1: Abnormality in control information
    _1 = 1,
}
impl From<INFABT_A> for bool {
    #[inline(always)]
    fn from(variant: INFABT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `INFABT` reader - Control Information Abnormality Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type INFABT_R = crate::BitReader<INFABT_A>;
impl INFABT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INFABT_A {
        match self.bits {
            false => INFABT_A::_0,
            true => INFABT_A::_1,
        }
    }
    ///No abnormality in control information
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INFABT_A::_0
    }
    ///Abnormality in control information
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INFABT_A::_1
    }
}
///Field `INFABT` writer - Control Information Abnormality Detection Flag
pub type INFABT_W<'a, REG> = crate::BitWriter1C<'a, REG, INFABT_A>;
impl<'a, REG> INFABT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No abnormality in control information
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INFABT_A::_0)
    }
    ///Abnormality in control information
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INFABT_A::_1)
    }
}
/**Response Stop Completion Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESDN_A {
    ///0: Stopping responses has not been completed.
    _0 = 0,
    ///1: Stopping responses has been completed.
    _1 = 1,
}
impl From<RESDN_A> for bool {
    #[inline(always)]
    fn from(variant: RESDN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RESDN` reader - Response Stop Completion Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RESDN_R = crate::BitReader<RESDN_A>;
impl RESDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESDN_A {
        match self.bits {
            false => RESDN_A::_0,
            true => RESDN_A::_1,
        }
    }
    ///Stopping responses has not been completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESDN_A::_0
    }
    ///Stopping responses has been completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESDN_A::_1
    }
}
///Field `RESDN` writer - Response Stop Completion Detection Flag
pub type RESDN_W<'a, REG> = crate::BitWriter1C<'a, REG, RESDN_A>;
impl<'a, REG> RESDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stopping responses has not been completed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESDN_A::_0)
    }
    ///Stopping responses has been completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESDN_A::_1)
    }
}
/**Generation Stop Completion Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENDN_A {
    ///0: Stopping generation has not been completed.
    _0 = 0,
    ///1: Stopping generation has been completed.
    _1 = 1,
}
impl From<GENDN_A> for bool {
    #[inline(always)]
    fn from(variant: GENDN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `GENDN` reader - Generation Stop Completion Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type GENDN_R = crate::BitReader<GENDN_A>;
impl GENDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GENDN_A {
        match self.bits {
            false => GENDN_A::_0,
            true => GENDN_A::_1,
        }
    }
    ///Stopping generation has not been completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GENDN_A::_0
    }
    ///Stopping generation has been completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GENDN_A::_1
    }
}
///Field `GENDN` writer - Generation Stop Completion Detection Flag
pub type GENDN_W<'a, REG> = crate::BitWriter1C<'a, REG, GENDN_A>;
impl<'a, REG> GENDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stopping generation has not been completed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GENDN_A::_0)
    }
    ///Stopping generation has been completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GENDN_A::_1)
    }
}
impl R {
    ///Bit 0 - offsetFromMaster Value Update Flag
    #[inline(always)]
    pub fn ofmud(&self) -> OFMUD_R {
        OFMUD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive logMessageInterval Value Change Detection Flag
    #[inline(always)]
    pub fn intchg(&self) -> INTCHG_R {
        INTCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - meanPathDelay Value Update Flag
    #[inline(always)]
    pub fn mpdud(&self) -> MPDUD_R {
        MPDUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag
    #[inline(always)]
    pub fn drpto(&self) -> DRPTO_R {
        DRPTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive logMessageInterval Value Out-of-Range Flag
    #[inline(always)]
    pub fn intdev(&self) -> INTDEV_R {
        INTDEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Delay_Req Reception FIFO Overflow Detection Flag
    #[inline(always)]
    pub fn drqovr(&self) -> DRQOVR_R {
        DRQOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - Loop Reception Detection Flag
    #[inline(always)]
    pub fn reclp(&self) -> RECLP_R {
        RECLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Control Information Abnormality Detection Flag
    #[inline(always)]
    pub fn infabt(&self) -> INFABT_R {
        INFABT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Response Stop Completion Detection Flag
    #[inline(always)]
    pub fn resdn(&self) -> RESDN_R {
        RESDN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Generation Stop Completion Detection Flag
    #[inline(always)]
    pub fn gendn(&self) -> GENDN_R {
        GENDN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - offsetFromMaster Value Update Flag
    #[inline(always)]
    pub fn ofmud(&mut self) -> OFMUD_W<SYSR_SPEC> {
        OFMUD_W::new(self, 0)
    }
    ///Bit 1 - Receive logMessageInterval Value Change Detection Flag
    #[inline(always)]
    pub fn intchg(&mut self) -> INTCHG_W<SYSR_SPEC> {
        INTCHG_W::new(self, 1)
    }
    ///Bit 2 - meanPathDelay Value Update Flag
    #[inline(always)]
    pub fn mpdud(&mut self) -> MPDUD_W<SYSR_SPEC> {
        MPDUD_W::new(self, 2)
    }
    ///Bit 4 - Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag
    #[inline(always)]
    pub fn drpto(&mut self) -> DRPTO_W<SYSR_SPEC> {
        DRPTO_W::new(self, 4)
    }
    ///Bit 5 - Receive logMessageInterval Value Out-of-Range Flag
    #[inline(always)]
    pub fn intdev(&mut self) -> INTDEV_W<SYSR_SPEC> {
        INTDEV_W::new(self, 5)
    }
    ///Bit 6 - Delay_Req Reception FIFO Overflow Detection Flag
    #[inline(always)]
    pub fn drqovr(&mut self) -> DRQOVR_W<SYSR_SPEC> {
        DRQOVR_W::new(self, 6)
    }
    ///Bit 12 - Loop Reception Detection Flag
    #[inline(always)]
    pub fn reclp(&mut self) -> RECLP_W<SYSR_SPEC> {
        RECLP_W::new(self, 12)
    }
    ///Bit 14 - Control Information Abnormality Detection Flag
    #[inline(always)]
    pub fn infabt(&mut self) -> INFABT_W<SYSR_SPEC> {
        INFABT_W::new(self, 14)
    }
    ///Bit 16 - Response Stop Completion Detection Flag
    #[inline(always)]
    pub fn resdn(&mut self) -> RESDN_W<SYSR_SPEC> {
        RESDN_W::new(self, 16)
    }
    ///Bit 17 - Generation Stop Completion Detection Flag
    #[inline(always)]
    pub fn gendn(&mut self) -> GENDN_W<SYSR_SPEC> {
        GENDN_W::new(self, 17)
    }
}
/**SYNFP Status Register

You can [`read`](crate::Reg::read) this register and get [`sysr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSR_SPEC;
impl crate::RegisterSpec for SYSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sysr::R`](R) reader structure
impl crate::Readable for SYSR_SPEC {}
///`write(|w| ..)` method takes [`sysr::W`](W) writer structure
impl crate::Writable for SYSR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_5077;
}
///`reset()` method sets SYSR to value 0
impl crate::Resettable for SYSR_SPEC {}
