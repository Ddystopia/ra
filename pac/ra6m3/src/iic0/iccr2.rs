///Register `ICCR2` reader
pub type R = crate::R<ICCR2_SPEC>;
///Register `ICCR2` writer
pub type W = crate::W<ICCR2_SPEC>;
/**Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    ///0: Does not request to issue a start condition.
    _0 = 0,
    ///1: Requests to issue a start condition.
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ST` reader - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).
pub type ST_R = crate::BitReader<ST_A>;
impl ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    ///Does not request to issue a start condition.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    ///Requests to issue a start condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
///Field `ST` writer - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG, ST_A>;
impl<'a, REG> ST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not request to issue a start condition.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ST_A::_0)
    }
    ///Requests to issue a start condition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ST_A::_1)
    }
}
/**Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    ///0: Does not request to issue a restart condition.
    _0 = 0,
    ///1: Requests to issue a restart condition.
    _1 = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RS` reader - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition.
pub type RS_R = crate::BitReader<RS_A>;
impl RS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::_0,
            true => RS_A::_1,
        }
    }
    ///Does not request to issue a restart condition.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RS_A::_0
    }
    ///Requests to issue a restart condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RS_A::_1
    }
}
///Field `RS` writer - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition.
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG, RS_A>;
impl<'a, REG> RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not request to issue a restart condition.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RS_A::_0)
    }
    ///Requests to issue a restart condition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RS_A::_1)
    }
}
/**Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP_A {
    ///0: Does not request to issue a stop condition.
    _0 = 0,
    ///1: Requests to issue a stop condition.
    _1 = 1,
}
impl From<SP_A> for bool {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SP` reader - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued.
pub type SP_R = crate::BitReader<SP_A>;
impl SP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SP_A {
        match self.bits {
            false => SP_A::_0,
            true => SP_A::_1,
        }
    }
    ///Does not request to issue a stop condition.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP_A::_0
    }
    ///Requests to issue a stop condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP_A::_1
    }
}
///Field `SP` writer - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued.
pub type SP_W<'a, REG> = crate::BitWriter<'a, REG, SP_A>;
impl<'a, REG> SP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not request to issue a stop condition.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SP_A::_0)
    }
    ///Requests to issue a stop condition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SP_A::_1)
    }
}
/**Transmit/Receive Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRS_A {
    ///0: Receive mode
    _0 = 0,
    ///1: Transmit mode
    _1 = 1,
}
impl From<TRS_A> for bool {
    #[inline(always)]
    fn from(variant: TRS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRS` reader - Transmit/Receive Mode
pub type TRS_R = crate::BitReader<TRS_A>;
impl TRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRS_A {
        match self.bits {
            false => TRS_A::_0,
            true => TRS_A::_1,
        }
    }
    ///Receive mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRS_A::_0
    }
    ///Transmit mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRS_A::_1
    }
}
///Field `TRS` writer - Transmit/Receive Mode
pub type TRS_W<'a, REG> = crate::BitWriter<'a, REG, TRS_A>;
impl<'a, REG> TRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRS_A::_0)
    }
    ///Transmit mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRS_A::_1)
    }
}
/**Master/Slave Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_A {
    ///0: Slave mode
    _0 = 0,
    ///1: Master mode
    _1 = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MST` reader - Master/Slave Mode
pub type MST_R = crate::BitReader<MST_A>;
impl MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::_0,
            true => MST_A::_1,
        }
    }
    ///Slave mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MST_A::_0
    }
    ///Master mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MST_A::_1
    }
}
///Field `MST` writer - Master/Slave Mode
pub type MST_W<'a, REG> = crate::BitWriter<'a, REG, MST_A>;
impl<'a, REG> MST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MST_A::_0)
    }
    ///Master mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MST_A::_1)
    }
}
/**Bus Busy Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BBSY_A {
    ///0: The I2C bus is released (bus free state).
    _0 = 0,
    ///1: The I2C bus is occupied (bus busy state).
    _1 = 1,
}
impl From<BBSY_A> for bool {
    #[inline(always)]
    fn from(variant: BBSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BBSY` reader - Bus Busy Detection Flag
pub type BBSY_R = crate::BitReader<BBSY_A>;
impl BBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BBSY_A {
        match self.bits {
            false => BBSY_A::_0,
            true => BBSY_A::_1,
        }
    }
    ///The I2C bus is released (bus free state).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BBSY_A::_0
    }
    ///The I2C bus is occupied (bus busy state).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BBSY_A::_1
    }
}
impl R {
    ///Bit 1 - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition.
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued.
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Transmit/Receive Mode
    #[inline(always)]
    pub fn trs(&self) -> TRS_R {
        TRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master/Slave Mode
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bus Busy Detection Flag
    #[inline(always)]
    pub fn bbsy(&self) -> BBSY_R {
        BBSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<ICCR2_SPEC> {
        ST_W::new(self, 1)
    }
    ///Bit 2 - Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition.
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W<ICCR2_SPEC> {
        RS_W::new(self, 2)
    }
    ///Bit 3 - Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued.
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W<ICCR2_SPEC> {
        SP_W::new(self, 3)
    }
    ///Bit 5 - Transmit/Receive Mode
    #[inline(always)]
    pub fn trs(&mut self) -> TRS_W<ICCR2_SPEC> {
        TRS_W::new(self, 5)
    }
    ///Bit 6 - Master/Slave Mode
    #[inline(always)]
    pub fn mst(&mut self) -> MST_W<ICCR2_SPEC> {
        MST_W::new(self, 6)
    }
}
/**I2C Bus Control Register 2

You can [`read`](crate::Reg::read) this register and get [`iccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICCR2_SPEC;
impl crate::RegisterSpec for ICCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`iccr2::R`](R) reader structure
impl crate::Readable for ICCR2_SPEC {}
///`write(|w| ..)` method takes [`iccr2::W`](W) writer structure
impl crate::Writable for ICCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICCR2 to value 0
impl crate::Resettable for ICCR2_SPEC {}
