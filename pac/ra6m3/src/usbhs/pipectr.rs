///Register `PIPE%sCTR` reader
pub type R = crate::R<PIPECTR_SPEC>;
///Register `PIPE%sCTR` writer
pub type W = crate::W<PIPECTR_SPEC>;
/**Response PIDThese bits specify the response type for the next transaction of the relevant pipe.

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
///Field `PID` reader - Response PIDThese bits specify the response type for the next transaction of the relevant pipe.
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
///Field `PID` writer - Response PIDThese bits specify the response type for the next transaction of the relevant pipe.
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
/**Pipe BusyThis bit indicates whether the relevant pipe is being used for the USB bus

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
///Field `PBUSY` reader - Pipe BusyThis bit indicates whether the relevant pipe is being used for the USB bus
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
/**Toggle Bit ConfirmationThis bit indicates the expected value of the sequence toggle bit for the next transaction of the relevant pipe

Value on reset: 0*/
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
///Field `SQMON` reader - Toggle Bit ConfirmationThis bit indicates the expected value of the sequence toggle bit for the next transaction of the relevant pipe
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
/**Toggle Bit SetThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is set for DATA1

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
///Field `SQSET` reader - Toggle Bit SetThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is set for DATA1
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
///Field `SQSET` writer - Toggle Bit SetThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is set for DATA1
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
/**Toggle Bit ClearThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is cleared to DATA0

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
///Field `SQCLR` reader - Toggle Bit ClearThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is cleared to DATA0
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
///Field `SQCLR` writer - Toggle Bit ClearThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is cleared to DATA0
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
/**Auto Buffer Clear ModeThis bit enables or disables auto buffer clear mode for the relevant pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACLRM_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled (all buffers are initialized)
    _1 = 1,
}
impl From<ACLRM_A> for bool {
    #[inline(always)]
    fn from(variant: ACLRM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACLRM` reader - Auto Buffer Clear ModeThis bit enables or disables auto buffer clear mode for the relevant pipe
pub type ACLRM_R = crate::BitReader<ACLRM_A>;
impl ACLRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACLRM_A {
        match self.bits {
            false => ACLRM_A::_0,
            true => ACLRM_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACLRM_A::_0
    }
    ///Enabled (all buffers are initialized)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACLRM_A::_1
    }
}
///Field `ACLRM` writer - Auto Buffer Clear ModeThis bit enables or disables auto buffer clear mode for the relevant pipe
pub type ACLRM_W<'a, REG> = crate::BitWriter<'a, REG, ACLRM_A>;
impl<'a, REG> ACLRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACLRM_A::_0)
    }
    ///Enabled (all buffers are initialized)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACLRM_A::_1)
    }
}
/**Auto Response ModeThis bit enables or disables auto response mode for the relevant pipe.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATREPM_A {
    ///0: Auto response mode is disabled.
    _0 = 0,
    ///1: Auto response mode is enabled (Transmission: zero-length packet response, Reception: NAK response and NRDY interrupt)
    _1 = 1,
}
impl From<ATREPM_A> for bool {
    #[inline(always)]
    fn from(variant: ATREPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ATREPM` reader - Auto Response ModeThis bit enables or disables auto response mode for the relevant pipe.
pub type ATREPM_R = crate::BitReader<ATREPM_A>;
impl ATREPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ATREPM_A {
        match self.bits {
            false => ATREPM_A::_0,
            true => ATREPM_A::_1,
        }
    }
    ///Auto response mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATREPM_A::_0
    }
    ///Auto response mode is enabled (Transmission: zero-length packet response, Reception: NAK response and NRDY interrupt)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATREPM_A::_1
    }
}
///Field `ATREPM` writer - Auto Response ModeThis bit enables or disables auto response mode for the relevant pipe.
pub type ATREPM_W<'a, REG> = crate::BitWriter<'a, REG, ATREPM_A>;
impl<'a, REG> ATREPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto response mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ATREPM_A::_0)
    }
    ///Auto response mode is enabled (Transmission: zero-length packet response, Reception: NAK response and NRDY interrupt)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ATREPM_A::_1)
    }
}
/**CSSTS StatusThis bit indicates the CSPLIT status of Split Transaction of the relevant pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSTS_A {
    ///0: SSplit Transaction processing is in progress or transfer without Split Transaction is in progress.
    _0 = 0,
    ///1: CSplit Transaction processing is in progress.
    _1 = 1,
}
impl From<CSSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CSSTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSTS` reader - CSSTS StatusThis bit indicates the CSPLIT status of Split Transaction of the relevant pipe
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
    ///SSplit Transaction processing is in progress or transfer without Split Transaction is in progress.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSSTS_A::_0
    }
    ///CSplit Transaction processing is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSSTS_A::_1
    }
}
/**CSPLIT Status ClearSet this bit to 1 when clearing the CSSTS bit of the relevant pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCLR_A {
    ///0: Writing is disabled.
    _0 = 0,
    ///1: The CSSTS bit is cleared.
    _1 = 1,
}
impl From<CSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CSCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCLR` reader - CSPLIT Status ClearSet this bit to 1 when clearing the CSSTS bit of the relevant pipe
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
    ///Writing is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCLR_A::_0
    }
    ///The CSSTS bit is cleared.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCLR_A::_1
    }
}
///Field `CSCLR` writer - CSPLIT Status ClearSet this bit to 1 when clearing the CSSTS bit of the relevant pipe
pub type CSCLR_W<'a, REG> = crate::BitWriter<'a, REG, CSCLR_A>;
impl<'a, REG> CSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCLR_A::_0)
    }
    ///The CSSTS bit is cleared.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCLR_A::_1)
    }
}
/**Transmit Buffer MonitorThis bit indicates the FIFO buffer status for the relevant pipe in the transmitting direction.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INBUFM_A {
    ///0: No transmittable data is present in the FIFO buffer.
    _0 = 0,
    ///1: Transmittable data is present in the FIFO buffer.
    _1 = 1,
}
impl From<INBUFM_A> for bool {
    #[inline(always)]
    fn from(variant: INBUFM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INBUFM` reader - Transmit Buffer MonitorThis bit indicates the FIFO buffer status for the relevant pipe in the transmitting direction.
pub type INBUFM_R = crate::BitReader<INBUFM_A>;
impl INBUFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INBUFM_A {
        match self.bits {
            false => INBUFM_A::_0,
            true => INBUFM_A::_1,
        }
    }
    ///No transmittable data is present in the FIFO buffer.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INBUFM_A::_0
    }
    ///Transmittable data is present in the FIFO buffer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INBUFM_A::_1
    }
}
/**Buffer StatusThis bit indicates the FIFO buffer status for the relevant pipe.

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
///Field `BSTS` reader - Buffer StatusThis bit indicates the FIFO buffer status for the relevant pipe.
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
    ///Bits 0:1 - Response PIDThese bits specify the response type for the next transaction of the relevant pipe.
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 3) as u8)
    }
    ///Bit 5 - Pipe BusyThis bit indicates whether the relevant pipe is being used for the USB bus
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Toggle Bit ConfirmationThis bit indicates the expected value of the sequence toggle bit for the next transaction of the relevant pipe
    #[inline(always)]
    pub fn sqmon(&self) -> SQMON_R {
        SQMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Toggle Bit SetThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is set for DATA1
    #[inline(always)]
    pub fn sqset(&self) -> SQSET_R {
        SQSET_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Toggle Bit ClearThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is cleared to DATA0
    #[inline(always)]
    pub fn sqclr(&self) -> SQCLR_R {
        SQCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Auto Buffer Clear ModeThis bit enables or disables auto buffer clear mode for the relevant pipe
    #[inline(always)]
    pub fn aclrm(&self) -> ACLRM_R {
        ACLRM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Auto Response ModeThis bit enables or disables auto response mode for the relevant pipe.
    #[inline(always)]
    pub fn atrepm(&self) -> ATREPM_R {
        ATREPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - CSSTS StatusThis bit indicates the CSPLIT status of Split Transaction of the relevant pipe
    #[inline(always)]
    pub fn cssts(&self) -> CSSTS_R {
        CSSTS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CSPLIT Status ClearSet this bit to 1 when clearing the CSSTS bit of the relevant pipe
    #[inline(always)]
    pub fn csclr(&self) -> CSCLR_R {
        CSCLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Transmit Buffer MonitorThis bit indicates the FIFO buffer status for the relevant pipe in the transmitting direction.
    #[inline(always)]
    pub fn inbufm(&self) -> INBUFM_R {
        INBUFM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Buffer StatusThis bit indicates the FIFO buffer status for the relevant pipe.
    #[inline(always)]
    pub fn bsts(&self) -> BSTS_R {
        BSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Response PIDThese bits specify the response type for the next transaction of the relevant pipe.
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W<PIPECTR_SPEC> {
        PID_W::new(self, 0)
    }
    ///Bit 7 - Toggle Bit SetThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is set for DATA1
    #[inline(always)]
    pub fn sqset(&mut self) -> SQSET_W<PIPECTR_SPEC> {
        SQSET_W::new(self, 7)
    }
    ///Bit 8 - Toggle Bit ClearThis bit is set to 1 when the expected value of the sequence toggle bit for the next transaction of the relevant pipe is cleared to DATA0
    #[inline(always)]
    pub fn sqclr(&mut self) -> SQCLR_W<PIPECTR_SPEC> {
        SQCLR_W::new(self, 8)
    }
    ///Bit 9 - Auto Buffer Clear ModeThis bit enables or disables auto buffer clear mode for the relevant pipe
    #[inline(always)]
    pub fn aclrm(&mut self) -> ACLRM_W<PIPECTR_SPEC> {
        ACLRM_W::new(self, 9)
    }
    ///Bit 10 - Auto Response ModeThis bit enables or disables auto response mode for the relevant pipe.
    #[inline(always)]
    pub fn atrepm(&mut self) -> ATREPM_W<PIPECTR_SPEC> {
        ATREPM_W::new(self, 10)
    }
    ///Bit 13 - CSPLIT Status ClearSet this bit to 1 when clearing the CSSTS bit of the relevant pipe
    #[inline(always)]
    pub fn csclr(&mut self) -> CSCLR_W<PIPECTR_SPEC> {
        CSCLR_W::new(self, 13)
    }
}
/**PIPE Control Register

You can [`read`](crate::Reg::read) this register and get [`pipectr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPECTR_SPEC;
impl crate::RegisterSpec for PIPECTR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipectr::R`](R) reader structure
impl crate::Readable for PIPECTR_SPEC {}
///`write(|w| ..)` method takes [`pipectr::W`](W) writer structure
impl crate::Writable for PIPECTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPE%sCTR to value 0
impl crate::Resettable for PIPECTR_SPEC {}
