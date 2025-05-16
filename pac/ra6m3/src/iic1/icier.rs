///Register `ICIER` reader
pub type R = crate::R<ICIER_SPEC>;
///Register `ICIER` writer
pub type W = crate::W<ICIER_SPEC>;
/**Timeout Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOIE_A {
    ///0: Timeout interrupt request (TMOI) is disabled.
    _0 = 0,
    ///1: Timeout interrupt request (TMOI) is enabled.
    _1 = 1,
}
impl From<TMOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TMOIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TMOIE` reader - Timeout Interrupt Request Enable
pub type TMOIE_R = crate::BitReader<TMOIE_A>;
impl TMOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TMOIE_A {
        match self.bits {
            false => TMOIE_A::_0,
            true => TMOIE_A::_1,
        }
    }
    ///Timeout interrupt request (TMOI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOIE_A::_0
    }
    ///Timeout interrupt request (TMOI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOIE_A::_1
    }
}
///Field `TMOIE` writer - Timeout Interrupt Request Enable
pub type TMOIE_W<'a, REG> = crate::BitWriter<'a, REG, TMOIE_A>;
impl<'a, REG> TMOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timeout interrupt request (TMOI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOIE_A::_0)
    }
    ///Timeout interrupt request (TMOI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOIE_A::_1)
    }
}
/**Arbitration-Lost Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    ///0: Arbitration-lost interrupt request (ALI) is disabled.
    _0 = 0,
    ///1: Arbitration-lost interrupt request (ALI) is enabled.
    _1 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIE` reader - Arbitration-Lost Interrupt Request Enable
pub type ALIE_R = crate::BitReader<ALIE_A>;
impl ALIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::_0,
            true => ALIE_A::_1,
        }
    }
    ///Arbitration-lost interrupt request (ALI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIE_A::_0
    }
    ///Arbitration-lost interrupt request (ALI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIE_A::_1
    }
}
///Field `ALIE` writer - Arbitration-Lost Interrupt Request Enable
pub type ALIE_W<'a, REG> = crate::BitWriter<'a, REG, ALIE_A>;
impl<'a, REG> ALIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Arbitration-lost interrupt request (ALI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ALIE_A::_0)
    }
    ///Arbitration-lost interrupt request (ALI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIE_A::_1)
    }
}
/**Start Condition Detection Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STIE_A {
    ///0: Start condition detection interrupt request (STI) is disabled.
    _0 = 0,
    ///1: Start condition detection interrupt request (STI) is enabled.
    _1 = 1,
}
impl From<STIE_A> for bool {
    #[inline(always)]
    fn from(variant: STIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STIE` reader - Start Condition Detection Interrupt Request Enable
pub type STIE_R = crate::BitReader<STIE_A>;
impl STIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STIE_A {
        match self.bits {
            false => STIE_A::_0,
            true => STIE_A::_1,
        }
    }
    ///Start condition detection interrupt request (STI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STIE_A::_0
    }
    ///Start condition detection interrupt request (STI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STIE_A::_1
    }
}
///Field `STIE` writer - Start Condition Detection Interrupt Request Enable
pub type STIE_W<'a, REG> = crate::BitWriter<'a, REG, STIE_A>;
impl<'a, REG> STIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start condition detection interrupt request (STI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STIE_A::_0)
    }
    ///Start condition detection interrupt request (STI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STIE_A::_1)
    }
}
/**Stop Condition Detection Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIE_A {
    ///0: Stop condition detection interrupt request (SPI) is disabled.
    _0 = 0,
    ///1: Stop condition detection interrupt request (SPI) is enabled.
    _1 = 1,
}
impl From<SPIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPIE` reader - Stop Condition Detection Interrupt Request Enable
pub type SPIE_R = crate::BitReader<SPIE_A>;
impl SPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPIE_A {
        match self.bits {
            false => SPIE_A::_0,
            true => SPIE_A::_1,
        }
    }
    ///Stop condition detection interrupt request (SPI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIE_A::_0
    }
    ///Stop condition detection interrupt request (SPI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIE_A::_1
    }
}
///Field `SPIE` writer - Stop Condition Detection Interrupt Request Enable
pub type SPIE_W<'a, REG> = crate::BitWriter<'a, REG, SPIE_A>;
impl<'a, REG> SPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop condition detection interrupt request (SPI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPIE_A::_0)
    }
    ///Stop condition detection interrupt request (SPI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPIE_A::_1)
    }
}
/**NACK Reception Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAKIE_A {
    ///0: NACK reception interrupt request (NAKI) is disabled.
    _0 = 0,
    ///1: NACK reception interrupt request (NAKI) is enabled.
    _1 = 1,
}
impl From<NAKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NAKIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NAKIE` reader - NACK Reception Interrupt Request Enable
pub type NAKIE_R = crate::BitReader<NAKIE_A>;
impl NAKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NAKIE_A {
        match self.bits {
            false => NAKIE_A::_0,
            true => NAKIE_A::_1,
        }
    }
    ///NACK reception interrupt request (NAKI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NAKIE_A::_0
    }
    ///NACK reception interrupt request (NAKI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NAKIE_A::_1
    }
}
///Field `NAKIE` writer - NACK Reception Interrupt Request Enable
pub type NAKIE_W<'a, REG> = crate::BitWriter<'a, REG, NAKIE_A>;
impl<'a, REG> NAKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NACK reception interrupt request (NAKI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NAKIE_A::_0)
    }
    ///NACK reception interrupt request (NAKI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NAKIE_A::_1)
    }
}
/**Receive Data Full Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    ///0: Receive data full interrupt request (RXI) is disabled.
    _0 = 0,
    ///1: Receive data full interrupt request (RXI) is enabled.
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Data Full Interrupt Request Enable
pub type RIE_R = crate::BitReader<RIE_A>;
impl RIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    ///Receive data full interrupt request (RXI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    ///Receive data full interrupt request (RXI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
///Field `RIE` writer - Receive Data Full Interrupt Request Enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG, RIE_A>;
impl<'a, REG> RIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive data full interrupt request (RXI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    ///Receive data full interrupt request (RXI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
/**Transmit End Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    ///0: Transmit end interrupt request (TEI) is disabled.
    _0 = 0,
    ///1: Transmit end interrupt request (TEI) is enabled.
    _1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transmit End Interrupt Request Enable
pub type TEIE_R = crate::BitReader<TEIE_A>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::_0,
            true => TEIE_A::_1,
        }
    }
    ///Transmit end interrupt request (TEI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIE_A::_0
    }
    ///Transmit end interrupt request (TEI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIE_A::_1
    }
}
///Field `TEIE` writer - Transmit End Interrupt Request Enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE_A>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit end interrupt request (TEI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::_0)
    }
    ///Transmit end interrupt request (TEI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::_1)
    }
}
/**Transmit Data Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    ///0: Transmit data empty interrupt request (TXI) is disabled.
    _0 = 0,
    ///1: Transmit data empty interrupt request (TXI) is enabled.
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Data Empty Interrupt Request Enable
pub type TIE_R = crate::BitReader<TIE_A>;
impl TIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    ///Transmit data empty interrupt request (TXI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    ///Transmit data empty interrupt request (TXI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
///Field `TIE` writer - Transmit Data Empty Interrupt Request Enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit data empty interrupt request (TXI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_0)
    }
    ///Transmit data empty interrupt request (TXI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_1)
    }
}
impl R {
    ///Bit 0 - Timeout Interrupt Request Enable
    #[inline(always)]
    pub fn tmoie(&self) -> TMOIE_R {
        TMOIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Arbitration-Lost Interrupt Request Enable
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Start Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACK Reception Interrupt Request Enable
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Data Full Interrupt Request Enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit End Interrupt Request Enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Data Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timeout Interrupt Request Enable
    #[inline(always)]
    pub fn tmoie(&mut self) -> TMOIE_W<ICIER_SPEC> {
        TMOIE_W::new(self, 0)
    }
    ///Bit 1 - Arbitration-Lost Interrupt Request Enable
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W<ICIER_SPEC> {
        ALIE_W::new(self, 1)
    }
    ///Bit 2 - Start Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn stie(&mut self) -> STIE_W<ICIER_SPEC> {
        STIE_W::new(self, 2)
    }
    ///Bit 3 - Stop Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn spie(&mut self) -> SPIE_W<ICIER_SPEC> {
        SPIE_W::new(self, 3)
    }
    ///Bit 4 - NACK Reception Interrupt Request Enable
    #[inline(always)]
    pub fn nakie(&mut self) -> NAKIE_W<ICIER_SPEC> {
        NAKIE_W::new(self, 4)
    }
    ///Bit 5 - Receive Data Full Interrupt Request Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<ICIER_SPEC> {
        RIE_W::new(self, 5)
    }
    ///Bit 6 - Transmit End Interrupt Request Enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<ICIER_SPEC> {
        TEIE_W::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<ICIER_SPEC> {
        TIE_W::new(self, 7)
    }
}
/**I2C Bus Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`icier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICIER_SPEC;
impl crate::RegisterSpec for ICIER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icier::R`](R) reader structure
impl crate::Readable for ICIER_SPEC {}
///`write(|w| ..)` method takes [`icier::W`](W) writer structure
impl crate::Writable for ICIER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICIER to value 0
impl crate::Resettable for ICIER_SPEC {}
