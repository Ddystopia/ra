///Register `PIPE%sCTR` reader
pub type R = crate::R<PIPECTR_SPEC>;
///Register `PIPE%sCTR` writer
pub type W = crate::W<PIPECTR_SPEC>;
/**Response PID

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    ///0: NAK response
    _00 = 0,
    ///1: BUF response (depending on the buffer state)
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
    ///BUF response (depending on the buffer state)
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
    ///BUF response (depending on the buffer state)
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
/**Pipe Busy

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBUSY_A {
    ///0: The relevant pipe is not used for the transaction.
    _0 = 0,
    ///1: The relevant pipe is used for the transaction.
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
    ///The relevant pipe is not used for the transaction.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PBUSY_A::_0
    }
    ///The relevant pipe is used for the transaction.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PBUSY_A::_1
    }
}
/**Sequence Toggle Bit Confirmation

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
///Field `SQMON` reader - Sequence Toggle Bit Confirmation
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
/**Sequence Toggle Bit Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQSET_A {
    ///0: Write disabled
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
///Field `SQSET` writer - Sequence Toggle Bit Set
pub type SQSET_W<'a, REG> = crate::BitWriter<'a, REG, SQSET_A>;
impl<'a, REG> SQSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write disabled
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
/**Sequence Toggle Bit Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQCLR_A {
    ///0: Write disabled
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
///Field `SQCLR` writer - Sequence Toggle Bit Clear
pub type SQCLR_W<'a, REG> = crate::BitWriter<'a, REG, SQCLR_A>;
impl<'a, REG> SQCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write disabled
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
/**Auto Buffer Clear Mode

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
///Field `ACLRM` reader - Auto Buffer Clear Mode
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
///Field `ACLRM` writer - Auto Buffer Clear Mode
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
/**Auto Response Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATREPM_A {
    ///0: Auto response is disabled.
    _0 = 0,
    ///1: Auto response is enabled.
    _1 = 1,
}
impl From<ATREPM_A> for bool {
    #[inline(always)]
    fn from(variant: ATREPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ATREPM` reader - Auto Response Mode
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
    ///Auto response is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATREPM_A::_0
    }
    ///Auto response is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATREPM_A::_1
    }
}
///Field `ATREPM` writer - Auto Response Mode
pub type ATREPM_W<'a, REG> = crate::BitWriter<'a, REG, ATREPM_A>;
impl<'a, REG> ATREPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto response is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ATREPM_A::_0)
    }
    ///Auto response is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ATREPM_A::_1)
    }
}
/**Transmit Buffer Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INBUFM_A {
    ///0: No data to be transmitted is in the FIFO buffer
    _0 = 0,
    ///1: Data to be transmitted is in the FIFO buffer
    _1 = 1,
}
impl From<INBUFM_A> for bool {
    #[inline(always)]
    fn from(variant: INBUFM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INBUFM` reader - Transmit Buffer Monitor
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
    ///No data to be transmitted is in the FIFO buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INBUFM_A::_0
    }
    ///Data to be transmitted is in the FIFO buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INBUFM_A::_1
    }
}
/**Buffer Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSTS_A {
    ///0: Buffer access by the CPU is disabled.
    _0 = 0,
    ///1: Buffer access by the CPU is enabled.
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
    ///Buffer access by the CPU is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSTS_A::_0
    }
    ///Buffer access by the CPU is enabled.
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
    ///Bit 5 - Pipe Busy
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Sequence Toggle Bit Confirmation
    #[inline(always)]
    pub fn sqmon(&self) -> SQMON_R {
        SQMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Auto Buffer Clear Mode
    #[inline(always)]
    pub fn aclrm(&self) -> ACLRM_R {
        ACLRM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Auto Response Mode
    #[inline(always)]
    pub fn atrepm(&self) -> ATREPM_R {
        ATREPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Transmit Buffer Monitor
    #[inline(always)]
    pub fn inbufm(&self) -> INBUFM_R {
        INBUFM_R::new(((self.bits >> 14) & 1) != 0)
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
    pub fn pid(&mut self) -> PID_W<PIPECTR_SPEC> {
        PID_W::new(self, 0)
    }
    ///Bit 7 - Sequence Toggle Bit Set
    #[inline(always)]
    pub fn sqset(&mut self) -> SQSET_W<PIPECTR_SPEC> {
        SQSET_W::new(self, 7)
    }
    ///Bit 8 - Sequence Toggle Bit Clear
    #[inline(always)]
    pub fn sqclr(&mut self) -> SQCLR_W<PIPECTR_SPEC> {
        SQCLR_W::new(self, 8)
    }
    ///Bit 9 - Auto Buffer Clear Mode
    #[inline(always)]
    pub fn aclrm(&mut self) -> ACLRM_W<PIPECTR_SPEC> {
        ACLRM_W::new(self, 9)
    }
    ///Bit 10 - Auto Response Mode
    #[inline(always)]
    pub fn atrepm(&mut self) -> ATREPM_W<PIPECTR_SPEC> {
        ATREPM_W::new(self, 10)
    }
}
/**Pipe %s Control Register

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
