///Register `SSR` reader
pub type R = crate::R<SSR_SPEC>;
///Register `SSR` writer
pub type W = crate::W<SSR_SPEC>;
/**Multi-Processor Bit Transfer. Sets the multi-processor bit for adding to the transmission frame

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBT_A {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
    _1 = 1,
}
impl From<MPBT_A> for bool {
    #[inline(always)]
    fn from(variant: MPBT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPBT` reader - Multi-Processor Bit Transfer. Sets the multi-processor bit for adding to the transmission frame
pub type MPBT_R = crate::BitReader<MPBT_A>;
impl MPBT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPBT_A {
        match self.bits {
            false => MPBT_A::_0,
            true => MPBT_A::_1,
        }
    }
    ///Data transmission cycles
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPBT_A::_0
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPBT_A::_1
    }
}
///Field `MPBT` writer - Multi-Processor Bit Transfer. Sets the multi-processor bit for adding to the transmission frame
pub type MPBT_W<'a, REG> = crate::BitWriter<'a, REG, MPBT_A>;
impl<'a, REG> MPBT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data transmission cycles
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPBT_A::_0)
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPBT_A::_1)
    }
}
/**Multi-Processor Bit. Value of the multi-processor bit in the reception frame

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPB_A {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
    _1 = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPB` reader - Multi-Processor Bit. Value of the multi-processor bit in the reception frame
pub type MPB_R = crate::BitReader<MPB_A>;
impl MPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::_0,
            true => MPB_A::_1,
        }
    }
    ///Data transmission cycles
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPB_A::_0
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPB_A::_1
    }
}
/**Transmit End Flag

Value on reset: 1*/
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
///Field `TEND` reader - Transmit End Flag
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
/**Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    ///0: No parity error occurred
    _0 = 0,
    ///1: A parity error has occurred
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
    ///No parity error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    ///A parity error has occurred
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
    ///No parity error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::_0)
    }
    ///A parity error has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::_1)
    }
}
/**Framing Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FER_A {
    ///0: No framing error occurred
    _0 = 0,
    ///1: A framing error has occurred
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
    ///No framing error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FER_A::_0
    }
    ///A framing error has occurred
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
    ///No framing error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FER_A::_0)
    }
    ///A framing error has occurred
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
/**Receive Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    ///0: No received data is in RDR register
    _0 = 0,
    ///1: Received data is in RDR register
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RDRF` reader - Receive Data Full Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RDRF_R = crate::BitReader<RDRF_A>;
impl RDRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    ///No received data is in RDR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    ///Received data is in RDR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
///Field `RDRF` writer - Receive Data Full Flag
pub type RDRF_W<'a, REG> = crate::BitWriter0C<'a, REG, RDRF_A>;
impl<'a, REG> RDRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No received data is in RDR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_0)
    }
    ///Received data is in RDR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_1)
    }
}
/**Transmit Data Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    ///0: Transmit data is in TDR register
    _0 = 0,
    ///1: No transmit data is in TDR register
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TDRE` reader - Transmit Data Empty Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TDRE_R = crate::BitReader<TDRE_A>;
impl TDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    ///Transmit data is in TDR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    ///No transmit data is in TDR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
///Field `TDRE` writer - Transmit Data Empty Flag
pub type TDRE_W<'a, REG> = crate::BitWriter0C<'a, REG, TDRE_A>;
impl<'a, REG> TDRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit data is in TDR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDRE_A::_0)
    }
    ///No transmit data is in TDR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDRE_A::_1)
    }
}
impl R {
    ///Bit 0 - Multi-Processor Bit Transfer. Sets the multi-processor bit for adding to the transmission frame
    #[inline(always)]
    pub fn mpbt(&self) -> MPBT_R {
        MPBT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Multi-Processor Bit. Value of the multi-processor bit in the reception frame
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Multi-Processor Bit Transfer. Sets the multi-processor bit for adding to the transmission frame
    #[inline(always)]
    pub fn mpbt(&mut self) -> MPBT_W<SSR_SPEC> {
        MPBT_W::new(self, 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<SSR_SPEC> {
        PER_W::new(self, 3)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&mut self) -> FER_W<SSR_SPEC> {
        FER_W::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> ORER_W<SSR_SPEC> {
        ORER_W::new(self, 5)
    }
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&mut self) -> RDRF_W<SSR_SPEC> {
        RDRF_W::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W<SSR_SPEC> {
        TDRE_W::new(self, 7)
    }
}
/**Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)

You can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ssr::R`](R) reader structure
impl crate::Readable for SSR_SPEC {}
///`write(|w| ..)` method takes [`ssr::W`](W) writer structure
impl crate::Writable for SSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xf8;
}
///`reset()` method sets SSR to value 0x84
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: u8 = 0x84;
}
