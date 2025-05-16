///Register `DCPCTR` reader
pub type R = crate::R<DCPCTR_SPEC>;
///Register `DCPCTR` writer
pub type W = crate::W<DCPCTR_SPEC>;
/**Response PID

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    ///0: NAK response
    _00 = 0,
    ///1: BUF response (depending on buffer state)
    _01 = 1,
    ///2: STALL response
    _10 = 2,
    ///3: STALL response
    _11 = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PID_A {
    type Ux = u8;
}
impl crate::IsEnum for PID_A {}
///Field `PID` reader - Response PID
pub type PID_R = crate::FieldReader<PID_A>;
impl PID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::_00,
            1 => PID_A::_01,
            2 => PID_A::_10,
            3 => PID_A::_11,
            _ => unreachable!(),
        }
    }
    ///NAK response
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PID_A::_00
    }
    ///BUF response (depending on buffer state)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PID_A::_01
    }
    ///STALL response
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PID_A::_10
    }
    ///STALL response
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PID_A::_11
    }
}
///Field `PID` writer - Response PID
pub type PID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PID_A, crate::Safe>;
impl<'a, REG> PID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///NAK response
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::_00)
    }
    ///BUF response (depending on buffer state)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::_01)
    }
    ///STALL response
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::_10)
    }
    ///STALL response
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::_11)
    }
}
/**Control Transfer End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPL_A {
    ///0: Completion of control transfer is disabled.
    _0 = 0,
    ///1: Completion of control transfer is enabled.
    _1 = 1,
}
impl From<CCPL_A> for bool {
    #[inline(always)]
    fn from(variant: CCPL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCPL` reader - Control Transfer End Enable
pub type CCPL_R = crate::BitReader<CCPL_A>;
impl CCPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCPL_A {
        match self.bits {
            false => CCPL_A::_0,
            true => CCPL_A::_1,
        }
    }
    ///Completion of control transfer is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCPL_A::_0
    }
    ///Completion of control transfer is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCPL_A::_1
    }
}
///Field `CCPL` writer - Control Transfer End Enable
pub type CCPL_W<'a, REG> = crate::BitWriter<'a, REG, CCPL_A>;
impl<'a, REG> CCPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Completion of control transfer is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPL_A::_0)
    }
    ///Completion of control transfer is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCPL_A::_1)
    }
}
/**PING Token Issue Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINGE_A {
    ///0: Issuing PING token is disabled.
    _0 = 0,
    ///1: Normal PING operation
    _1 = 1,
}
impl From<PINGE_A> for bool {
    #[inline(always)]
    fn from(variant: PINGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PINGE` reader - PING Token Issue Enable
pub type PINGE_R = crate::BitReader<PINGE_A>;
impl PINGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINGE_A {
        match self.bits {
            false => PINGE_A::_0,
            true => PINGE_A::_1,
        }
    }
    ///Issuing PING token is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PINGE_A::_0
    }
    ///Normal PING operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PINGE_A::_1
    }
}
///Field `PINGE` writer - PING Token Issue Enable
pub type PINGE_W<'a, REG> = crate::BitWriter<'a, REG, PINGE_A>;
impl<'a, REG> PINGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Issuing PING token is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PINGE_A::_0)
    }
    ///Normal PING operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PINGE_A::_1)
    }
}
/**Pipe Busy

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBUSY_A {
    ///0: The relevant pipe is not used for the USB bus.
    _0 = 0,
    ///1: The relevant pipe is in use for the USB bus.
    _1 = 1,
}
impl From<PBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PBUSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PBUSY` reader - Pipe Busy
pub type PBUSY_R = crate::BitReader<PBUSY_A>;
impl PBUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PBUSY_A {
        match self.bits {
            false => PBUSY_A::_0,
            true => PBUSY_A::_1,
        }
    }
    ///The relevant pipe is not used for the USB bus.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PBUSY_A::_0
    }
    ///The relevant pipe is in use for the USB bus.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PBUSY_A::_1
    }
}
/**Sequence Toggle Bit Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQMON_A {
    ///0: DATA0
    _0 = 0,
    ///1: DATA1
    _1 = 1,
}
impl From<SQMON_A> for bool {
    #[inline(always)]
    fn from(variant: SQMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SQMON` reader - Sequence Toggle Bit Monitor
pub type SQMON_R = crate::BitReader<SQMON_A>;
impl SQMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SQMON_A {
        match self.bits {
            false => SQMON_A::_0,
            true => SQMON_A::_1,
        }
    }
    ///DATA0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SQMON_A::_0
    }
    ///DATA1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SQMON_A::_1
    }
}
/**Toggle Bit Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQSET_A {
    ///0: Writing is ignored.
    _0 = 0,
    ///1: Specifies DATA1.
    _1 = 1,
}
impl From<SQSET_A> for bool {
    #[inline(always)]
    fn from(variant: SQSET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SQSET` reader - Toggle Bit Set
pub type SQSET_R = crate::BitReader<SQSET_A>;
impl SQSET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SQSET_A {
        match self.bits {
            false => SQSET_A::_0,
            true => SQSET_A::_1,
        }
    }
    ///Writing is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SQSET_A::_0
    }
    ///Specifies DATA1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SQSET_A::_1
    }
}
///Field `SQSET` writer - Toggle Bit Set
pub type SQSET_W<'a, REG> = crate::BitWriter<'a, REG, SQSET_A>;
impl<'a, REG> SQSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SQSET_A::_0)
    }
    ///Specifies DATA1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SQSET_A::_1)
    }
}
/**Toggle Bit Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQCLR_A {
    ///0: Writing is ignored.
    _0 = 0,
    ///1: Specifies DATA0.
    _1 = 1,
}
impl From<SQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SQCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SQCLR` reader - Toggle Bit Clear
pub type SQCLR_R = crate::BitReader<SQCLR_A>;
impl SQCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SQCLR_A {
        match self.bits {
            false => SQCLR_A::_0,
            true => SQCLR_A::_1,
        }
    }
    ///Writing is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SQCLR_A::_0
    }
    ///Specifies DATA0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SQCLR_A::_1
    }
}
///Field `SQCLR` writer - Toggle Bit Clear
pub type SQCLR_W<'a, REG> = crate::BitWriter<'a, REG, SQCLR_A>;
impl<'a, REG> SQCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SQCLR_A::_0)
    }
    ///Specifies DATA0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SQCLR_A::_1)
    }
}
/**SUREQ Bit Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUREQCLR_A {
    ///0: Writing is ignored.
    _0 = 0,
    ///1: Clears the SUREQ bit to 0.
    _1 = 1,
}
impl From<SUREQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SUREQCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SUREQCLR` reader - SUREQ Bit Clear
pub type SUREQCLR_R = crate::BitReader<SUREQCLR_A>;
impl SUREQCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUREQCLR_A {
        match self.bits {
            false => SUREQCLR_A::_0,
            true => SUREQCLR_A::_1,
        }
    }
    ///Writing is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUREQCLR_A::_0
    }
    ///Clears the SUREQ bit to 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUREQCLR_A::_1
    }
}
///Field `SUREQCLR` writer - SUREQ Bit Clear
pub type SUREQCLR_W<'a, REG> = crate::BitWriter<'a, REG, SUREQCLR_A>;
impl<'a, REG> SUREQCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SUREQCLR_A::_0)
    }
    ///Clears the SUREQ bit to 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SUREQCLR_A::_1)
    }
}
/**Split Transaction COMPLETE SPLIT(CSPLIT) Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSTS_A {
    ///0: START-SPLIT(SSPLIT) transaction processing is in progress or processing for devices that do not use Split Transaction is in progress.
    _0 = 0,
    ///1: The CSPLIT transaction processing is in progress.
    _1 = 1,
}
impl From<CSSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CSSTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSTS` reader - Split Transaction COMPLETE SPLIT(CSPLIT) Status
pub type CSSTS_R = crate::BitReader<CSSTS_A>;
impl CSSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSTS_A {
        match self.bits {
            false => CSSTS_A::_0,
            true => CSSTS_A::_1,
        }
    }
    ///START-SPLIT(SSPLIT) transaction processing is in progress or processing for devices that do not use Split Transaction is in progress.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSSTS_A::_0
    }
    ///The CSPLIT transaction processing is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSSTS_A::_1
    }
}
/**Split Transaction CSPLIT Status Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCLR_A {
    ///0: Writing is ignored.
    _0 = 0,
    ///1: Clears the CSSTS bit to 0.
    _1 = 1,
}
impl From<CSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CSCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCLR` reader - Split Transaction CSPLIT Status Clear
pub type CSCLR_R = crate::BitReader<CSCLR_A>;
impl CSCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCLR_A {
        match self.bits {
            false => CSCLR_A::_0,
            true => CSCLR_A::_1,
        }
    }
    ///Writing is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCLR_A::_0
    }
    ///Clears the CSSTS bit to 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCLR_A::_1
    }
}
///Field `CSCLR` writer - Split Transaction CSPLIT Status Clear
pub type CSCLR_W<'a, REG> = crate::BitWriter<'a, REG, CSCLR_A>;
impl<'a, REG> CSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCLR_A::_0)
    }
    ///Clears the CSSTS bit to 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCLR_A::_1)
    }
}
/**SETUP Token Transmission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUREQ_A {
    ///0: Writing is ignored.
    _0 = 0,
    ///1: Transmits the setup packet.
    _1 = 1,
}
impl From<SUREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SUREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SUREQ` reader - SETUP Token Transmission
pub type SUREQ_R = crate::BitReader<SUREQ_A>;
impl SUREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUREQ_A {
        match self.bits {
            false => SUREQ_A::_0,
            true => SUREQ_A::_1,
        }
    }
    ///Writing is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUREQ_A::_0
    }
    ///Transmits the setup packet.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUREQ_A::_1
    }
}
///Field `SUREQ` writer - SETUP Token Transmission
pub type SUREQ_W<'a, REG> = crate::BitWriter<'a, REG, SUREQ_A>;
impl<'a, REG> SUREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SUREQ_A::_0)
    }
    ///Transmits the setup packet.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SUREQ_A::_1)
    }
}
/**Buffer Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSTS_A {
    ///0: Buffer access is disabled.
    _0 = 0,
    ///1: Buffer access is enabled.
    _1 = 1,
}
impl From<BSTS_A> for bool {
    #[inline(always)]
    fn from(variant: BSTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSTS` reader - Buffer Status
pub type BSTS_R = crate::BitReader<BSTS_A>;
impl BSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSTS_A {
        match self.bits {
            false => BSTS_A::_0,
            true => BSTS_A::_1,
        }
    }
    ///Buffer access is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSTS_A::_0
    }
    ///Buffer access is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSTS_A::_1
    }
}
impl R {
    ///Bits 0:1 - Response PID
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Control Transfer End Enable
    #[inline(always)]
    pub fn ccpl(&self) -> CCPL_R {
        CCPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - PING Token Issue Enable
    #[inline(always)]
    pub fn pinge(&self) -> PINGE_R {
        PINGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pipe Busy
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Sequence Toggle Bit Monitor
    #[inline(always)]
    pub fn sqmon(&self) -> SQMON_R {
        SQMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Toggle Bit Set
    #[inline(always)]
    pub fn sqset(&self) -> SQSET_R {
        SQSET_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Toggle Bit Clear
    #[inline(always)]
    pub fn sqclr(&self) -> SQCLR_R {
        SQCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SUREQ Bit Clear
    #[inline(always)]
    pub fn sureqclr(&self) -> SUREQCLR_R {
        SUREQCLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Split Transaction COMPLETE SPLIT(CSPLIT) Status
    #[inline(always)]
    pub fn cssts(&self) -> CSSTS_R {
        CSSTS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Split Transaction CSPLIT Status Clear
    #[inline(always)]
    pub fn csclr(&self) -> CSCLR_R {
        CSCLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SETUP Token Transmission
    #[inline(always)]
    pub fn sureq(&self) -> SUREQ_R {
        SUREQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Buffer Status
    #[inline(always)]
    pub fn bsts(&self) -> BSTS_R {
        BSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Response PID
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W<DCPCTR_SPEC> {
        PID_W::new(self, 0)
    }
    ///Bit 2 - Control Transfer End Enable
    #[inline(always)]
    pub fn ccpl(&mut self) -> CCPL_W<DCPCTR_SPEC> {
        CCPL_W::new(self, 2)
    }
    ///Bit 4 - PING Token Issue Enable
    #[inline(always)]
    pub fn pinge(&mut self) -> PINGE_W<DCPCTR_SPEC> {
        PINGE_W::new(self, 4)
    }
    ///Bit 7 - Toggle Bit Set
    #[inline(always)]
    pub fn sqset(&mut self) -> SQSET_W<DCPCTR_SPEC> {
        SQSET_W::new(self, 7)
    }
    ///Bit 8 - Toggle Bit Clear
    #[inline(always)]
    pub fn sqclr(&mut self) -> SQCLR_W<DCPCTR_SPEC> {
        SQCLR_W::new(self, 8)
    }
    ///Bit 11 - SUREQ Bit Clear
    #[inline(always)]
    pub fn sureqclr(&mut self) -> SUREQCLR_W<DCPCTR_SPEC> {
        SUREQCLR_W::new(self, 11)
    }
    ///Bit 13 - Split Transaction CSPLIT Status Clear
    #[inline(always)]
    pub fn csclr(&mut self) -> CSCLR_W<DCPCTR_SPEC> {
        CSCLR_W::new(self, 13)
    }
    ///Bit 14 - SETUP Token Transmission
    #[inline(always)]
    pub fn sureq(&mut self) -> SUREQ_W<DCPCTR_SPEC> {
        SUREQ_W::new(self, 14)
    }
}
/**DCP Control Register

You can [`read`](crate::Reg::read) this register and get [`dcpctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCPCTR_SPEC;
impl crate::RegisterSpec for DCPCTR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dcpctr::R`](R) reader structure
impl crate::Readable for DCPCTR_SPEC {}
///`write(|w| ..)` method takes [`dcpctr::W`](W) writer structure
impl crate::Writable for DCPCTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCPCTR to value 0x40
impl crate::Resettable for DCPCTR_SPEC {
    const RESET_VALUE: u16 = 0x40;
}
