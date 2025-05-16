///Register `SPMR` reader
pub type R = crate::R<SPMR_SPEC>;
///Register `SPMR` writer
pub type W = crate::W<SPMR_SPEC>;
/**SSn# Pin Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSE_A {
    ///0: SSn# pin function is disabled.
    _0 = 0,
    ///1: SSn# pin function is enabled.
    _1 = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSE` reader - SSn# Pin Function Enable
pub type SSE_R = crate::BitReader<SSE_A>;
impl SSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSE_A {
        match self.bits {
            false => SSE_A::_0,
            true => SSE_A::_1,
        }
    }
    ///SSn# pin function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSE_A::_0
    }
    ///SSn# pin function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSE_A::_1
    }
}
///Field `SSE` writer - SSn# Pin Function Enable
pub type SSE_W<'a, REG> = crate::BitWriter<'a, REG, SSE_A>;
impl<'a, REG> SSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSn# pin function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSE_A::_0)
    }
    ///SSn# pin function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSE_A::_1)
    }
}
/**CTS Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE_A {
    ///0: CTS function is disabled (RTS output function is enabled).
    _0 = 0,
    ///1: CTS function is enabled.
    _1 = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTS Enable
pub type CTSE_R = crate::BitReader<CTSE_A>;
impl CTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::_0,
            true => CTSE_A::_1,
        }
    }
    ///CTS function is disabled (RTS output function is enabled).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSE_A::_0
    }
    ///CTS function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSE_A::_1
    }
}
///Field `CTSE` writer - CTS Enable
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE_A>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CTS function is disabled (RTS output function is enabled).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE_A::_0)
    }
    ///CTS function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE_A::_1)
    }
}
/**Master or slave mode selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSS_A {
    ///0: Transmission is through the TXDn pin and reception is through the RXDn pin (master mode).
    _0 = 0,
    ///1: Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode).
    _1 = 1,
}
impl From<MSS_A> for bool {
    #[inline(always)]
    fn from(variant: MSS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSS` reader - Master or slave mode selection
pub type MSS_R = crate::BitReader<MSS_A>;
impl MSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSS_A {
        match self.bits {
            false => MSS_A::_0,
            true => MSS_A::_1,
        }
    }
    ///Transmission is through the TXDn pin and reception is through the RXDn pin (master mode).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSS_A::_0
    }
    ///Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSS_A::_1
    }
}
///Field `MSS` writer - Master or slave mode selection
pub type MSS_W<'a, REG> = crate::BitWriter<'a, REG, MSS_A>;
impl<'a, REG> MSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission is through the TXDn pin and reception is through the RXDn pin (master mode).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSS_A::_0)
    }
    ///Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSS_A::_1)
    }
}
/**Mode Fault Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MFF_A {
    ///0: No mode fault error
    _0 = 0,
    ///1: Mode fault error
    _1 = 1,
}
impl From<MFF_A> for bool {
    #[inline(always)]
    fn from(variant: MFF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `MFF` reader - Mode Fault Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type MFF_R = crate::BitReader<MFF_A>;
impl MFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MFF_A {
        match self.bits {
            false => MFF_A::_0,
            true => MFF_A::_1,
        }
    }
    ///No mode fault error
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MFF_A::_0
    }
    ///Mode fault error
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MFF_A::_1
    }
}
///Field `MFF` writer - Mode Fault Flag
pub type MFF_W<'a, REG> = crate::BitWriter0C<'a, REG, MFF_A>;
impl<'a, REG> MFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No mode fault error
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MFF_A::_0)
    }
    ///Mode fault error
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MFF_A::_1)
    }
}
/**Clock Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    ///0: Clock polarity is not inverted.
    _0 = 0,
    ///1: Clock polarity is inverted
    _1 = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - Clock Polarity Select
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::_0,
            true => CKPOL_A::_1,
        }
    }
    ///Clock polarity is not inverted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKPOL_A::_0
    }
    ///Clock polarity is inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKPOL_A::_1
    }
}
///Field `CKPOL` writer - Clock Polarity Select
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL_A>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock polarity is not inverted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::_0)
    }
    ///Clock polarity is inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::_1)
    }
}
/**Clock Phase Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPH_A {
    ///0: Clock is not delayed.
    _0 = 0,
    ///1: Clock is delayed.
    _1 = 1,
}
impl From<CKPH_A> for bool {
    #[inline(always)]
    fn from(variant: CKPH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPH` reader - Clock Phase Select
pub type CKPH_R = crate::BitReader<CKPH_A>;
impl CKPH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKPH_A {
        match self.bits {
            false => CKPH_A::_0,
            true => CKPH_A::_1,
        }
    }
    ///Clock is not delayed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKPH_A::_0
    }
    ///Clock is delayed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKPH_A::_1
    }
}
///Field `CKPH` writer - Clock Phase Select
pub type CKPH_W<'a, REG> = crate::BitWriter<'a, REG, CKPH_A>;
impl<'a, REG> CKPH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock is not delayed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPH_A::_0)
    }
    ///Clock is delayed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPH_A::_1)
    }
}
impl R {
    ///Bit 0 - SSn# Pin Function Enable
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTS Enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master or slave mode selection
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Mode Fault Flag
    #[inline(always)]
    pub fn mff(&self) -> MFF_R {
        MFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Clock Polarity Select
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clock Phase Select
    #[inline(always)]
    pub fn ckph(&self) -> CKPH_R {
        CKPH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SSn# Pin Function Enable
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W<SPMR_SPEC> {
        SSE_W::new(self, 0)
    }
    ///Bit 1 - CTS Enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<SPMR_SPEC> {
        CTSE_W::new(self, 1)
    }
    ///Bit 2 - Master or slave mode selection
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W<SPMR_SPEC> {
        MSS_W::new(self, 2)
    }
    ///Bit 4 - Mode Fault Flag
    #[inline(always)]
    pub fn mff(&mut self) -> MFF_W<SPMR_SPEC> {
        MFF_W::new(self, 4)
    }
    ///Bit 6 - Clock Polarity Select
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<SPMR_SPEC> {
        CKPOL_W::new(self, 6)
    }
    ///Bit 7 - Clock Phase Select
    #[inline(always)]
    pub fn ckph(&mut self) -> CKPH_W<SPMR_SPEC> {
        CKPH_W::new(self, 7)
    }
}
/**SPI Mode Register

You can [`read`](crate::Reg::read) this register and get [`spmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPMR_SPEC;
impl crate::RegisterSpec for SPMR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spmr::R`](R) reader structure
impl crate::Readable for SPMR_SPEC {}
///`write(|w| ..)` method takes [`spmr::W`](W) writer structure
impl crate::Writable for SPMR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x10;
}
///`reset()` method sets SPMR to value 0
impl crate::Resettable for SPMR_SPEC {}
