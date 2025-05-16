///Register `ICMR3` reader
pub type R = crate::R<ICMR3_SPEC>;
///Register `ICMR3` writer
pub type W = crate::W<ICMR3_SPEC>;
/**Noise Filter Stage Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NF_A {
    ///0: Noise of up to one fIIC cycle is filtered out (single-stage filter).
    _00 = 0,
    ///1: Noise of up to two fIIC cycles is filtered out (2-stage filter).
    _01 = 1,
    ///2: Noise of up to three fIIC cycles is filtered out (3-stage filter).
    _10 = 2,
    ///3: Noise of up to four fIIC cycles is filtered out (4-stage filter)
    _11 = 3,
}
impl From<NF_A> for u8 {
    #[inline(always)]
    fn from(variant: NF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NF_A {
    type Ux = u8;
}
impl crate::IsEnum for NF_A {}
///Field `NF` reader - Noise Filter Stage Selection
pub type NF_R = crate::FieldReader<NF_A>;
impl NF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NF_A {
        match self.bits {
            0 => NF_A::_00,
            1 => NF_A::_01,
            2 => NF_A::_10,
            3 => NF_A::_11,
            _ => unreachable!(),
        }
    }
    ///Noise of up to one fIIC cycle is filtered out (single-stage filter).
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NF_A::_00
    }
    ///Noise of up to two fIIC cycles is filtered out (2-stage filter).
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NF_A::_01
    }
    ///Noise of up to three fIIC cycles is filtered out (3-stage filter).
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NF_A::_10
    }
    ///Noise of up to four fIIC cycles is filtered out (4-stage filter)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NF_A::_11
    }
}
///Field `NF` writer - Noise Filter Stage Selection
pub type NF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NF_A, crate::Safe>;
impl<'a, REG> NF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Noise of up to one fIIC cycle is filtered out (single-stage filter).
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NF_A::_00)
    }
    ///Noise of up to two fIIC cycles is filtered out (2-stage filter).
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NF_A::_01)
    }
    ///Noise of up to three fIIC cycles is filtered out (3-stage filter).
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NF_A::_10)
    }
    ///Noise of up to four fIIC cycles is filtered out (4-stage filter)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NF_A::_11)
    }
}
/**Receive Acknowledge

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKBR_A {
    ///0: A 0 is received as the acknowledge bit (ACK reception).
    _0 = 0,
    ///1: A 1 is received as the acknowledge bit (NACK reception).
    _1 = 1,
}
impl From<ACKBR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKBR` reader - Receive Acknowledge
pub type ACKBR_R = crate::BitReader<ACKBR_A>;
impl ACKBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACKBR_A {
        match self.bits {
            false => ACKBR_A::_0,
            true => ACKBR_A::_1,
        }
    }
    ///A 0 is received as the acknowledge bit (ACK reception).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKBR_A::_0
    }
    ///A 1 is received as the acknowledge bit (NACK reception).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKBR_A::_1
    }
}
/**Transmit Acknowledge

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKBT_A {
    ///0: A 0 is sent as the acknowledge bit (ACK transmission).
    _0 = 0,
    ///1: A 1 is sent as the acknowledge bit (NACK transmission).
    _1 = 1,
}
impl From<ACKBT_A> for bool {
    #[inline(always)]
    fn from(variant: ACKBT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKBT` reader - Transmit Acknowledge
pub type ACKBT_R = crate::BitReader<ACKBT_A>;
impl ACKBT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACKBT_A {
        match self.bits {
            false => ACKBT_A::_0,
            true => ACKBT_A::_1,
        }
    }
    ///A 0 is sent as the acknowledge bit (ACK transmission).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKBT_A::_0
    }
    ///A 1 is sent as the acknowledge bit (NACK transmission).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKBT_A::_1
    }
}
///Field `ACKBT` writer - Transmit Acknowledge
pub type ACKBT_W<'a, REG> = crate::BitWriter<'a, REG, ACKBT_A>;
impl<'a, REG> ACKBT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A 0 is sent as the acknowledge bit (ACK transmission).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACKBT_A::_0)
    }
    ///A 1 is sent as the acknowledge bit (NACK transmission).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACKBT_A::_1)
    }
}
/**ACKBT Write Protect

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKWP_A {
    ///0: Modification of the ACKBT bit is disabled.
    _0 = 0,
    ///1: Modification of the ACKBT bit is enabled.
    _1 = 1,
}
impl From<ACKWP_A> for bool {
    #[inline(always)]
    fn from(variant: ACKWP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACKWP` reader - ACKBT Write Protect
pub type ACKWP_R = crate::BitReader<ACKWP_A>;
impl ACKWP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACKWP_A {
        match self.bits {
            false => ACKWP_A::_0,
            true => ACKWP_A::_1,
        }
    }
    ///Modification of the ACKBT bit is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKWP_A::_0
    }
    ///Modification of the ACKBT bit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKWP_A::_1
    }
}
///Field `ACKWP` writer - ACKBT Write Protect
pub type ACKWP_W<'a, REG> = crate::BitWriter<'a, REG, ACKWP_A>;
impl<'a, REG> ACKWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Modification of the ACKBT bit is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACKWP_A::_0)
    }
    ///Modification of the ACKBT bit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACKWP_A::_1)
    }
}
/**RDRF Flag Set Timing Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRFS_A {
    ///0: The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)
    _0 = 0,
    ///1: The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)
    _1 = 1,
}
impl From<RDRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RDRFS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDRFS` reader - RDRF Flag Set Timing Selection
pub type RDRFS_R = crate::BitReader<RDRFS_A>;
impl RDRFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDRFS_A {
        match self.bits {
            false => RDRFS_A::_0,
            true => RDRFS_A::_1,
        }
    }
    ///The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRFS_A::_0
    }
    ///The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRFS_A::_1
    }
}
///Field `RDRFS` writer - RDRF Flag Set Timing Selection
pub type RDRFS_W<'a, REG> = crate::BitWriter<'a, REG, RDRFS_A>;
impl<'a, REG> RDRFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDRFS_A::_0)
    }
    ///The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDRFS_A::_1)
    }
}
/**WAITNote: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_A {
    ///0: No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)
    _0 = 0,
    ///1: WAIT (The period between ninth clock cycle and first clock cycle is held low.)
    _1 = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WAIT` reader - WAITNote: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand.
pub type WAIT_R = crate::BitReader<WAIT_A>;
impl WAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::_0,
            true => WAIT_A::_1,
        }
    }
    ///No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAIT_A::_0
    }
    ///WAIT (The period between ninth clock cycle and first clock cycle is held low.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAIT_A::_1
    }
}
///Field `WAIT` writer - WAITNote: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand.
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG, WAIT_A>;
impl<'a, REG> WAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_A::_0)
    }
    ///WAIT (The period between ninth clock cycle and first clock cycle is held low.)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_A::_1)
    }
}
/**SMBus/I2C Bus Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBS_A {
    ///0: The I2C bus is selected.
    _0 = 0,
    ///1: The SMBus is selected.
    _1 = 1,
}
impl From<SMBS_A> for bool {
    #[inline(always)]
    fn from(variant: SMBS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBS` reader - SMBus/I2C Bus Selection
pub type SMBS_R = crate::BitReader<SMBS_A>;
impl SMBS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMBS_A {
        match self.bits {
            false => SMBS_A::_0,
            true => SMBS_A::_1,
        }
    }
    ///The I2C bus is selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMBS_A::_0
    }
    ///The SMBus is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMBS_A::_1
    }
}
///Field `SMBS` writer - SMBus/I2C Bus Selection
pub type SMBS_W<'a, REG> = crate::BitWriter<'a, REG, SMBS_A>;
impl<'a, REG> SMBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The I2C bus is selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SMBS_A::_0)
    }
    ///The SMBus is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SMBS_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Noise Filter Stage Selection
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(self.bits & 3)
    }
    ///Bit 2 - Receive Acknowledge
    #[inline(always)]
    pub fn ackbr(&self) -> ACKBR_R {
        ACKBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit Acknowledge
    #[inline(always)]
    pub fn ackbt(&self) -> ACKBT_R {
        ACKBT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ACKBT Write Protect
    #[inline(always)]
    pub fn ackwp(&self) -> ACKWP_R {
        ACKWP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RDRF Flag Set Timing Selection
    #[inline(always)]
    pub fn rdrfs(&self) -> RDRFS_R {
        RDRFS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - WAITNote: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand.
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SMBus/I2C Bus Selection
    #[inline(always)]
    pub fn smbs(&self) -> SMBS_R {
        SMBS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Noise Filter Stage Selection
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W<ICMR3_SPEC> {
        NF_W::new(self, 0)
    }
    ///Bit 3 - Transmit Acknowledge
    #[inline(always)]
    pub fn ackbt(&mut self) -> ACKBT_W<ICMR3_SPEC> {
        ACKBT_W::new(self, 3)
    }
    ///Bit 4 - ACKBT Write Protect
    #[inline(always)]
    pub fn ackwp(&mut self) -> ACKWP_W<ICMR3_SPEC> {
        ACKWP_W::new(self, 4)
    }
    ///Bit 5 - RDRF Flag Set Timing Selection
    #[inline(always)]
    pub fn rdrfs(&mut self) -> RDRFS_W<ICMR3_SPEC> {
        RDRFS_W::new(self, 5)
    }
    ///Bit 6 - WAITNote: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand.
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<ICMR3_SPEC> {
        WAIT_W::new(self, 6)
    }
    ///Bit 7 - SMBus/I2C Bus Selection
    #[inline(always)]
    pub fn smbs(&mut self) -> SMBS_W<ICMR3_SPEC> {
        SMBS_W::new(self, 7)
    }
}
/**I2C Bus Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`icmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICMR3_SPEC;
impl crate::RegisterSpec for ICMR3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icmr3::R`](R) reader structure
impl crate::Readable for ICMR3_SPEC {}
///`write(|w| ..)` method takes [`icmr3::W`](W) writer structure
impl crate::Writable for ICMR3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICMR3 to value 0
impl crate::Resettable for ICMR3_SPEC {}
