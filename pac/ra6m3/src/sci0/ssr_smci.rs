///Register `SSR_SMCI` reader
pub type R = crate::R<SSR_SMCI_SPEC>;
///Register `SSR_SMCI` writer
pub type W = crate::W<SSR_SMCI_SPEC>;
///Field `MPBT` reader - This bit should be 0 in smart card interface mode.
pub type MPBT_R = crate::BitReader;
///Field `MPBT` writer - This bit should be 0 in smart card interface mode.
pub type MPBT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPB` reader - This bit should be 0 in smart card interface mode.
pub type MPB_R = crate::BitReader;
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
/**Error Signal Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERS_A {
    ///0: Low error signal not responded
    _0 = 0,
    ///1: Low error signal responded
    _1 = 1,
}
impl From<ERS_A> for bool {
    #[inline(always)]
    fn from(variant: ERS_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ERS` reader - Error Signal Status Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ERS_R = crate::BitReader<ERS_A>;
impl ERS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERS_A {
        match self.bits {
            false => ERS_A::_0,
            true => ERS_A::_1,
        }
    }
    ///Low error signal not responded
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERS_A::_0
    }
    ///Low error signal responded
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERS_A::_1
    }
}
///Field `ERS` writer - Error Signal Status Flag
pub type ERS_W<'a, REG> = crate::BitWriter0C<'a, REG, ERS_A>;
impl<'a, REG> ERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low error signal not responded
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ERS_A::_0)
    }
    ///Low error signal responded
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ERS_A::_1)
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
    ///Bit 0 - This bit should be 0 in smart card interface mode.
    #[inline(always)]
    pub fn mpbt(&self) -> MPBT_R {
        MPBT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This bit should be 0 in smart card interface mode.
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
    ///Bit 4 - Error Signal Status Flag
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 4) & 1) != 0)
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
    ///Bit 0 - This bit should be 0 in smart card interface mode.
    #[inline(always)]
    pub fn mpbt(&mut self) -> MPBT_W<SSR_SMCI_SPEC> {
        MPBT_W::new(self, 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<SSR_SMCI_SPEC> {
        PER_W::new(self, 3)
    }
    ///Bit 4 - Error Signal Status Flag
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W<SSR_SMCI_SPEC> {
        ERS_W::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> ORER_W<SSR_SMCI_SPEC> {
        ORER_W::new(self, 5)
    }
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&mut self) -> RDRF_W<SSR_SMCI_SPEC> {
        RDRF_W::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W<SSR_SMCI_SPEC> {
        TDRE_W::new(self, 7)
    }
}
/**Serial Status Register(SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`ssr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSR_SMCI_SPEC;
impl crate::RegisterSpec for SSR_SMCI_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ssr_smci::R`](R) reader structure
impl crate::Readable for SSR_SMCI_SPEC {}
///`write(|w| ..)` method takes [`ssr_smci::W`](W) writer structure
impl crate::Writable for SSR_SMCI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xf8;
}
///`reset()` method sets SSR_SMCI to value 0x84
impl crate::Resettable for SSR_SMCI_SPEC {
    const RESET_VALUE: u8 = 0x84;
}
