///Register `SSR_FIFO` reader
pub type R = crate::R<SSR_FIFO_SPEC>;
///Register `SSR_FIFO` writer
pub type W = crate::W<SSR_FIFO_SPEC>;
/**Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DR_A {
    ///0: Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)
    _0 = 0,
    ///1: Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number.
    _1 = 1,
}
impl From<DR_A> for bool {
    #[inline(always)]
    fn from(variant: DR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DR` reader - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DR_R = crate::BitReader<DR_A>;
impl DR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DR_A {
        match self.bits {
            false => DR_A::_0,
            true => DR_A::_1,
        }
    }
    ///Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DR_A::_0
    }
    ///Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DR_A::_1
    }
}
///Field `DR` writer - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)
pub type DR_W<'a, REG> = crate::BitWriter0C<'a, REG, DR_A>;
impl<'a, REG> DR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DR_A::_0)
    }
    ///Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DR_A::_1)
    }
}
/**Transmit End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    ///0: A character is being transmitted.
    _0 = 0,
    ///1: Character transfer has been completed.
    _1 = 1,
}
impl From<TEND_A> for bool {
    #[inline(always)]
    fn from(variant: TEND_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TEND` reader - Transmit End Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TEND_R = crate::BitReader<TEND_A>;
impl TEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEND_A {
        match self.bits {
            false => TEND_A::_0,
            true => TEND_A::_1,
        }
    }
    ///A character is being transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEND_A::_0
    }
    ///Character transfer has been completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEND_A::_1
    }
}
///Field `TEND` writer - Transmit End Flag
pub type TEND_W<'a, REG> = crate::BitWriter0C<'a, REG, TEND_A>;
impl<'a, REG> TEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A character is being transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEND_A::_0)
    }
    ///Character transfer has been completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEND_A::_1)
    }
}
/**Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    ///0: No parity error occurred.
    _0 = 0,
    ///1: A parity error has occurred.
    _1 = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PER` reader - Parity Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PER_R = crate::BitReader<PER_A>;
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::_0,
            true => PER_A::_1,
        }
    }
    ///No parity error occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    ///A parity error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER_A::_1
    }
}
///Field `PER` writer - Parity Error Flag
pub type PER_W<'a, REG> = crate::BitWriter0C<'a, REG, PER_A>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No parity error occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::_0)
    }
    ///A parity error has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::_1)
    }
}
/**Framing Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FER_A {
    ///0: No framing error occurred.
    _0 = 0,
    ///1: A framing error has occurred.
    _1 = 1,
}
impl From<FER_A> for bool {
    #[inline(always)]
    fn from(variant: FER_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `FER` reader - Framing Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type FER_R = crate::BitReader<FER_A>;
impl FER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FER_A {
        match self.bits {
            false => FER_A::_0,
            true => FER_A::_1,
        }
    }
    ///No framing error occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FER_A::_0
    }
    ///A framing error has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FER_A::_1
    }
}
///Field `FER` writer - Framing Error Flag
pub type FER_W<'a, REG> = crate::BitWriter0C<'a, REG, FER_A>;
impl<'a, REG> FER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No framing error occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FER_A::_0)
    }
    ///A framing error has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FER_A::_1)
    }
}
/**Overrun Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORER_A {
    ///0: No overrun error occurred
    _0 = 0,
    ///1: An overrun error has occurred
    _1 = 1,
}
impl From<ORER_A> for bool {
    #[inline(always)]
    fn from(variant: ORER_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ORER` reader - Overrun Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ORER_R = crate::BitReader<ORER_A>;
impl ORER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORER_A {
        match self.bits {
            false => ORER_A::_0,
            true => ORER_A::_1,
        }
    }
    ///No overrun error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORER_A::_0
    }
    ///An overrun error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORER_A::_1
    }
}
///Field `ORER` writer - Overrun Error Flag
pub type ORER_W<'a, REG> = crate::BitWriter0C<'a, REG, ORER_A>;
impl<'a, REG> ORER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overrun error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ORER_A::_0)
    }
    ///An overrun error has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ORER_A::_1)
    }
}
/**Receive FIFO data full flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    ///0: The quantity of receive data written in FRDR falls below the specified receive triggering number.
    _0 = 0,
    ///1: The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number.
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RDF` reader - Receive FIFO data full flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RDF_R = crate::BitReader<RDF_A>;
impl RDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    ///The quantity of receive data written in FRDR falls below the specified receive triggering number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    ///The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
///Field `RDF` writer - Receive FIFO data full flag
pub type RDF_W<'a, REG> = crate::BitWriter0C<'a, REG, RDF_A>;
impl<'a, REG> RDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The quantity of receive data written in FRDR falls below the specified receive triggering number.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_0)
    }
    ///The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_1)
    }
}
/**Transmit FIFO data empty flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDFE_A {
    ///0: The quantity of transmit data written in FTDR exceeds the specified transmit triggering number.
    _0 = 0,
    ///1: The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number
    _1 = 1,
}
impl From<TDFE_A> for bool {
    #[inline(always)]
    fn from(variant: TDFE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TDFE` reader - Transmit FIFO data empty flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TDFE_R = crate::BitReader<TDFE_A>;
impl TDFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDFE_A {
        match self.bits {
            false => TDFE_A::_0,
            true => TDFE_A::_1,
        }
    }
    ///The quantity of transmit data written in FTDR exceeds the specified transmit triggering number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDFE_A::_0
    }
    ///The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDFE_A::_1
    }
}
///Field `TDFE` writer - Transmit FIFO data empty flag
pub type TDFE_W<'a, REG> = crate::BitWriter0C<'a, REG, TDFE_A>;
impl<'a, REG> TDFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The quantity of transmit data written in FTDR exceeds the specified transmit triggering number.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDFE_A::_0)
    }
    ///The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDFE_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Transmit End Flag
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive FIFO data full flag
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit FIFO data empty flag
    #[inline(always)]
    pub fn tdfe(&self) -> TDFE_R {
        TDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<SSR_FIFO_SPEC> {
        DR_W::new(self, 0)
    }
    ///Bit 2 - Transmit End Flag
    #[inline(always)]
    pub fn tend(&mut self) -> TEND_W<SSR_FIFO_SPEC> {
        TEND_W::new(self, 2)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<SSR_FIFO_SPEC> {
        PER_W::new(self, 3)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&mut self) -> FER_W<SSR_FIFO_SPEC> {
        FER_W::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> ORER_W<SSR_FIFO_SPEC> {
        ORER_W::new(self, 5)
    }
    ///Bit 6 - Receive FIFO data full flag
    #[inline(always)]
    pub fn rdf(&mut self) -> RDF_W<SSR_FIFO_SPEC> {
        RDF_W::new(self, 6)
    }
    ///Bit 7 - Transmit FIFO data empty flag
    #[inline(always)]
    pub fn tdfe(&mut self) -> TDFE_W<SSR_FIFO_SPEC> {
        TDFE_W::new(self, 7)
    }
}
/**Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)

You can [`read`](crate::Reg::read) this register and get [`ssr_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSR_FIFO_SPEC;
impl crate::RegisterSpec for SSR_FIFO_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ssr_fifo::R`](R) reader structure
impl crate::Readable for SSR_FIFO_SPEC {}
///`write(|w| ..)` method takes [`ssr_fifo::W`](W) writer structure
impl crate::Writable for SSR_FIFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xfd;
}
///`reset()` method sets SSR_FIFO to value 0x80
impl crate::Resettable for SSR_FIFO_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
