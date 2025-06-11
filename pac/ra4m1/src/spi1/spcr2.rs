///Register `SPCR2` reader
pub type R = crate::R<SPCR2_SPEC>;
///Register `SPCR2` writer
pub type W = crate::W<SPCR2_SPEC>;
/**Parity Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPPE_A {
    ///0: Does not add the parity bit to transmit data and does not check the parity bit of receive data
    _0 = 0,
    ///1: Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)
    _1 = 1,
}
impl From<SPPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPPE` reader - Parity Enable
pub type SPPE_R = crate::BitReader<SPPE_A>;
impl SPPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPPE_A {
        match self.bits {
            false => SPPE_A::_0,
            true => SPPE_A::_1,
        }
    }
    ///Does not add the parity bit to transmit data and does not check the parity bit of receive data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPPE_A::_0
    }
    ///Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPPE_A::_1
    }
}
///Field `SPPE` writer - Parity Enable
pub type SPPE_W<'a, REG> = crate::BitWriter<'a, REG, SPPE_A>;
impl<'a, REG> SPPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not add the parity bit to transmit data and does not check the parity bit of receive data
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPPE_A::_0)
    }
    ///Adds the parity bit to transmit data and checks the parity bit of receive data (when SPCR.TXMD = 0) / Adds the parity bit to transmit data but does not check the parity bit of receive data (when SPCR.TXMD = 1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPPE_A::_1)
    }
}
/**Parity Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOE_A {
    ///0: Selects even parity for use in transmission and reception
    _0 = 0,
    ///1: Selects odd parity for use in transmission and reception
    _1 = 1,
}
impl From<SPOE_A> for bool {
    #[inline(always)]
    fn from(variant: SPOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPOE` reader - Parity Mode
pub type SPOE_R = crate::BitReader<SPOE_A>;
impl SPOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPOE_A {
        match self.bits {
            false => SPOE_A::_0,
            true => SPOE_A::_1,
        }
    }
    ///Selects even parity for use in transmission and reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPOE_A::_0
    }
    ///Selects odd parity for use in transmission and reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPOE_A::_1
    }
}
///Field `SPOE` writer - Parity Mode
pub type SPOE_W<'a, REG> = crate::BitWriter<'a, REG, SPOE_A>;
impl<'a, REG> SPOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selects even parity for use in transmission and reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPOE_A::_0)
    }
    ///Selects odd parity for use in transmission and reception
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPOE_A::_1)
    }
}
/**SPI Idle Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIIE_A {
    ///0: Disables the generation of idle interrupt requests
    _0 = 0,
    ///1: Enables the generation of idle interrupt requests
    _1 = 1,
}
impl From<SPIIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPIIE` reader - SPI Idle Interrupt Enable
pub type SPIIE_R = crate::BitReader<SPIIE_A>;
impl SPIIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPIIE_A {
        match self.bits {
            false => SPIIE_A::_0,
            true => SPIIE_A::_1,
        }
    }
    ///Disables the generation of idle interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIIE_A::_0
    }
    ///Enables the generation of idle interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIIE_A::_1
    }
}
///Field `SPIIE` writer - SPI Idle Interrupt Enable
pub type SPIIE_W<'a, REG> = crate::BitWriter<'a, REG, SPIIE_A>;
impl<'a, REG> SPIIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the generation of idle interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPIIE_A::_0)
    }
    ///Enables the generation of idle interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPIIE_A::_1)
    }
}
/**Parity Self-Testing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTE_A {
    ///0: Disables the self-diagnosis function of the parity circuit
    _0 = 0,
    ///1: Enables the self-diagnosis function of the parity circuit
    _1 = 1,
}
impl From<PTE_A> for bool {
    #[inline(always)]
    fn from(variant: PTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PTE` reader - Parity Self-Testing
pub type PTE_R = crate::BitReader<PTE_A>;
impl PTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PTE_A {
        match self.bits {
            false => PTE_A::_0,
            true => PTE_A::_1,
        }
    }
    ///Disables the self-diagnosis function of the parity circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTE_A::_0
    }
    ///Enables the self-diagnosis function of the parity circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTE_A::_1
    }
}
///Field `PTE` writer - Parity Self-Testing
pub type PTE_W<'a, REG> = crate::BitWriter<'a, REG, PTE_A>;
impl<'a, REG> PTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the self-diagnosis function of the parity circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PTE_A::_0)
    }
    ///Enables the self-diagnosis function of the parity circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PTE_A::_1)
    }
}
/**RSPCK Auto-Stop Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKASE_A {
    ///0: Disables the RSPCK auto-stop function
    _0 = 0,
    ///1: Enables the RSPCK auto-stop function
    _1 = 1,
}
impl From<SCKASE_A> for bool {
    #[inline(always)]
    fn from(variant: SCKASE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCKASE` reader - RSPCK Auto-Stop Function Enable
pub type SCKASE_R = crate::BitReader<SCKASE_A>;
impl SCKASE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCKASE_A {
        match self.bits {
            false => SCKASE_A::_0,
            true => SCKASE_A::_1,
        }
    }
    ///Disables the RSPCK auto-stop function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKASE_A::_0
    }
    ///Enables the RSPCK auto-stop function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKASE_A::_1
    }
}
///Field `SCKASE` writer - RSPCK Auto-Stop Function Enable
pub type SCKASE_W<'a, REG> = crate::BitWriter<'a, REG, SCKASE_A>;
impl<'a, REG> SCKASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the RSPCK auto-stop function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCKASE_A::_0)
    }
    ///Enables the RSPCK auto-stop function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCKASE_A::_1)
    }
}
impl R {
    ///Bit 0 - Parity Enable
    #[inline(always)]
    pub fn sppe(&self) -> SPPE_R {
        SPPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Parity Mode
    #[inline(always)]
    pub fn spoe(&self) -> SPOE_R {
        SPOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPI Idle Interrupt Enable
    #[inline(always)]
    pub fn spiie(&self) -> SPIIE_R {
        SPIIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity Self-Testing
    #[inline(always)]
    pub fn pte(&self) -> PTE_R {
        PTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RSPCK Auto-Stop Function Enable
    #[inline(always)]
    pub fn sckase(&self) -> SCKASE_R {
        SCKASE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Parity Enable
    #[inline(always)]
    pub fn sppe(&mut self) -> SPPE_W<SPCR2_SPEC> {
        SPPE_W::new(self, 0)
    }
    ///Bit 1 - Parity Mode
    #[inline(always)]
    pub fn spoe(&mut self) -> SPOE_W<SPCR2_SPEC> {
        SPOE_W::new(self, 1)
    }
    ///Bit 2 - SPI Idle Interrupt Enable
    #[inline(always)]
    pub fn spiie(&mut self) -> SPIIE_W<SPCR2_SPEC> {
        SPIIE_W::new(self, 2)
    }
    ///Bit 3 - Parity Self-Testing
    #[inline(always)]
    pub fn pte(&mut self) -> PTE_W<SPCR2_SPEC> {
        PTE_W::new(self, 3)
    }
    ///Bit 4 - RSPCK Auto-Stop Function Enable
    #[inline(always)]
    pub fn sckase(&mut self) -> SCKASE_W<SPCR2_SPEC> {
        SCKASE_W::new(self, 4)
    }
}
/**SPI Control Register 2

You can [`read`](crate::Reg::read) this register and get [`spcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPCR2_SPEC;
impl crate::RegisterSpec for SPCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spcr2::R`](R) reader structure
impl crate::Readable for SPCR2_SPEC {}
///`write(|w| ..)` method takes [`spcr2::W`](W) writer structure
impl crate::Writable for SPCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCR2 to value 0
impl crate::Resettable for SPCR2_SPEC {}
