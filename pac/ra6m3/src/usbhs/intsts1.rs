///Register `INTSTS1` reader
pub type R = crate::R<INTSTS1_SPEC>;
///Register `INTSTS1` writer
pub type W = crate::W<INTSTS1_SPEC>;
/**PDDET Detection Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETINT_A {
    ///0: PDDET interrupts are not generated
    _0 = 0,
    ///1: PDDET interrupts are generated
    _1 = 1,
}
impl From<PDDETINT_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETINT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PDDETINT` reader - PDDET Detection Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PDDETINT_R = crate::BitReader<PDDETINT_A>;
impl PDDETINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDETINT_A {
        match self.bits {
            false => PDDETINT_A::_0,
            true => PDDETINT_A::_1,
        }
    }
    ///PDDET interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETINT_A::_0
    }
    ///PDDET interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETINT_A::_1
    }
}
///Field `PDDETINT` writer - PDDET Detection Interrupt Status
pub type PDDETINT_W<'a, REG> = crate::BitWriter0C<'a, REG, PDDETINT_A>;
impl<'a, REG> PDDETINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PDDET interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDDETINT_A::_0)
    }
    ///PDDET interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDDETINT_A::_1)
    }
}
/**Setup Transaction Normal Response Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SACK_A {
    ///0: SACK interrupts are not generated
    _0 = 0,
    ///1: SACK interrupts are generated
    _1 = 1,
}
impl From<SACK_A> for bool {
    #[inline(always)]
    fn from(variant: SACK_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SACK` reader - Setup Transaction Normal Response Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SACK_R = crate::BitReader<SACK_A>;
impl SACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SACK_A {
        match self.bits {
            false => SACK_A::_0,
            true => SACK_A::_1,
        }
    }
    ///SACK interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACK_A::_0
    }
    ///SACK interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACK_A::_1
    }
}
///Field `SACK` writer - Setup Transaction Normal Response Interrupt Status
pub type SACK_W<'a, REG> = crate::BitWriter0C<'a, REG, SACK_A>;
impl<'a, REG> SACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SACK interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SACK_A::_0)
    }
    ///SACK interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SACK_A::_1)
    }
}
/**Setup Transaction Error Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGN_A {
    ///0: SIGN interrupts are not generated
    _0 = 0,
    ///1: SIGN interrupts are generated
    _1 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SIGN` reader - Setup Transaction Error Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SIGN_R = crate::BitReader<SIGN_A>;
impl SIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::_0,
            true => SIGN_A::_1,
        }
    }
    ///SIGN interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIGN_A::_0
    }
    ///SIGN interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIGN_A::_1
    }
}
///Field `SIGN` writer - Setup Transaction Error Interrupt Status
pub type SIGN_W<'a, REG> = crate::BitWriter0C<'a, REG, SIGN_A>;
impl<'a, REG> SIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SIGN interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SIGN_A::_0)
    }
    ///SIGN interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SIGN_A::_1)
    }
}
/**EOF Error Detection Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOFERR_A {
    ///0: EOFERR interrupts are not generated
    _0 = 0,
    ///1: EOFERR interrupts are generated
    _1 = 1,
}
impl From<EOFERR_A> for bool {
    #[inline(always)]
    fn from(variant: EOFERR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `EOFERR` reader - EOF Error Detection Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type EOFERR_R = crate::BitReader<EOFERR_A>;
impl EOFERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOFERR_A {
        match self.bits {
            false => EOFERR_A::_0,
            true => EOFERR_A::_1,
        }
    }
    ///EOFERR interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOFERR_A::_0
    }
    ///EOFERR interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOFERR_A::_1
    }
}
///Field `EOFERR` writer - EOF Error Detection Interrupt Status
pub type EOFERR_W<'a, REG> = crate::BitWriter0C<'a, REG, EOFERR_A>;
impl<'a, REG> EOFERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOFERR interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOFERR_A::_0)
    }
    ///EOFERR interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOFERR_A::_1)
    }
}
/**LPM Transaction End Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMEND_A {
    ///0: LPMEND interrupts are not generated
    _0 = 0,
    ///1: LPMEND interrupts are generated
    _1 = 1,
}
impl From<LPMEND_A> for bool {
    #[inline(always)]
    fn from(variant: LPMEND_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `LPMEND` reader - LPM Transaction End Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type LPMEND_R = crate::BitReader<LPMEND_A>;
impl LPMEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMEND_A {
        match self.bits {
            false => LPMEND_A::_0,
            true => LPMEND_A::_1,
        }
    }
    ///LPMEND interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPMEND_A::_0
    }
    ///LPMEND interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPMEND_A::_1
    }
}
///Field `LPMEND` writer - LPM Transaction End Interrupt Status
pub type LPMEND_W<'a, REG> = crate::BitWriter0C<'a, REG, LPMEND_A>;
impl<'a, REG> LPMEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPMEND interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMEND_A::_0)
    }
    ///LPMEND interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMEND_A::_1)
    }
}
/**L1 Resume End Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RSMEND_A {
    ///0: L1RSMEND interrupts are not generated
    _0 = 0,
    ///1: L1RSMEND interrupts are generated
    _1 = 1,
}
impl From<L1RSMEND_A> for bool {
    #[inline(always)]
    fn from(variant: L1RSMEND_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `L1RSMEND` reader - L1 Resume End Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type L1RSMEND_R = crate::BitReader<L1RSMEND_A>;
impl L1RSMEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1RSMEND_A {
        match self.bits {
            false => L1RSMEND_A::_0,
            true => L1RSMEND_A::_1,
        }
    }
    ///L1RSMEND interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1RSMEND_A::_0
    }
    ///L1RSMEND interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1RSMEND_A::_1
    }
}
///Field `L1RSMEND` writer - L1 Resume End Interrupt Status
pub type L1RSMEND_W<'a, REG> = crate::BitWriter0C<'a, REG, L1RSMEND_A>;
impl<'a, REG> L1RSMEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///L1RSMEND interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1RSMEND_A::_0)
    }
    ///L1RSMEND interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1RSMEND_A::_1)
    }
}
/**USB Connection Detection Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATTCH_A {
    ///0: ATTCH interrupts are not generated
    _0 = 0,
    ///1: ATTCH interrupts are generated
    _1 = 1,
}
impl From<ATTCH_A> for bool {
    #[inline(always)]
    fn from(variant: ATTCH_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ATTCH` reader - USB Connection Detection Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ATTCH_R = crate::BitReader<ATTCH_A>;
impl ATTCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ATTCH_A {
        match self.bits {
            false => ATTCH_A::_0,
            true => ATTCH_A::_1,
        }
    }
    ///ATTCH interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTCH_A::_0
    }
    ///ATTCH interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTCH_A::_1
    }
}
///Field `ATTCH` writer - USB Connection Detection Interrupt Status
pub type ATTCH_W<'a, REG> = crate::BitWriter0C<'a, REG, ATTCH_A>;
impl<'a, REG> ATTCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ATTCH interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ATTCH_A::_0)
    }
    ///ATTCH interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ATTCH_A::_1)
    }
}
/**USB Disconnection Detection Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCH_A {
    ///0: DTCH interrupts are not generated
    _0 = 0,
    ///1: DTCH interrupts are generated
    _1 = 1,
}
impl From<DTCH_A> for bool {
    #[inline(always)]
    fn from(variant: DTCH_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DTCH` reader - USB Disconnection Detection Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DTCH_R = crate::BitReader<DTCH_A>;
impl DTCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCH_A {
        match self.bits {
            false => DTCH_A::_0,
            true => DTCH_A::_1,
        }
    }
    ///DTCH interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCH_A::_0
    }
    ///DTCH interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCH_A::_1
    }
}
///Field `DTCH` writer - USB Disconnection Detection Interrupt Status
pub type DTCH_W<'a, REG> = crate::BitWriter0C<'a, REG, DTCH_A>;
impl<'a, REG> DTCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTCH interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCH_A::_0)
    }
    ///DTCH interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCH_A::_1)
    }
}
/**USB Bus Change Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCHG_A {
    ///0: BCHG interrupts are not generated
    _0 = 0,
    ///1: BCHG interrupts are generated
    _1 = 1,
}
impl From<BCHG_A> for bool {
    #[inline(always)]
    fn from(variant: BCHG_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `BCHG` reader - USB Bus Change Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type BCHG_R = crate::BitReader<BCHG_A>;
impl BCHG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCHG_A {
        match self.bits {
            false => BCHG_A::_0,
            true => BCHG_A::_1,
        }
    }
    ///BCHG interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCHG_A::_0
    }
    ///BCHG interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCHG_A::_1
    }
}
///Field `BCHG` writer - USB Bus Change Interrupt Status
pub type BCHG_W<'a, REG> = crate::BitWriter0C<'a, REG, BCHG_A>;
impl<'a, REG> BCHG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BCHG interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCHG_A::_0)
    }
    ///BCHG interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCHG_A::_1)
    }
}
/**Overcurrent Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCR_A {
    ///0: OVRCR interrupts are not generated
    _0 = 0,
    ///1: OVRCR interrupts are generated
    _1 = 1,
}
impl From<OVRCR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRCR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OVRCR` reader - Overcurrent Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OVRCR_R = crate::BitReader<OVRCR_A>;
impl OVRCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRCR_A {
        match self.bits {
            false => OVRCR_A::_0,
            true => OVRCR_A::_1,
        }
    }
    ///OVRCR interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRCR_A::_0
    }
    ///OVRCR interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRCR_A::_1
    }
}
///Field `OVRCR` writer - Overcurrent Interrupt Status
pub type OVRCR_W<'a, REG> = crate::BitWriter0C<'a, REG, OVRCR_A>;
impl<'a, REG> OVRCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OVRCR interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCR_A::_0)
    }
    ///OVRCR interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCR_A::_1)
    }
}
impl R {
    ///Bit 0 - PDDET Detection Interrupt Status
    #[inline(always)]
    pub fn pddetint(&self) -> PDDETINT_R {
        PDDETINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Status
    #[inline(always)]
    pub fn sack(&self) -> SACK_R {
        SACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Status
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EOF Error Detection Interrupt Status
    #[inline(always)]
    pub fn eoferr(&self) -> EOFERR_R {
        EOFERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LPM Transaction End Interrupt Status
    #[inline(always)]
    pub fn lpmend(&self) -> LPMEND_R {
        LPMEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - L1 Resume End Interrupt Status
    #[inline(always)]
    pub fn l1rsmend(&self) -> L1RSMEND_R {
        L1RSMEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - USB Connection Detection Interrupt Status
    #[inline(always)]
    pub fn attch(&self) -> ATTCH_R {
        ATTCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB Disconnection Detection Interrupt Status
    #[inline(always)]
    pub fn dtch(&self) -> DTCH_R {
        DTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USB Bus Change Interrupt Status
    #[inline(always)]
    pub fn bchg(&self) -> BCHG_R {
        BCHG_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overcurrent Interrupt Status
    #[inline(always)]
    pub fn ovrcr(&self) -> OVRCR_R {
        OVRCR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PDDET Detection Interrupt Status
    #[inline(always)]
    pub fn pddetint(&mut self) -> PDDETINT_W<INTSTS1_SPEC> {
        PDDETINT_W::new(self, 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Status
    #[inline(always)]
    pub fn sack(&mut self) -> SACK_W<INTSTS1_SPEC> {
        SACK_W::new(self, 4)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Status
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W<INTSTS1_SPEC> {
        SIGN_W::new(self, 5)
    }
    ///Bit 6 - EOF Error Detection Interrupt Status
    #[inline(always)]
    pub fn eoferr(&mut self) -> EOFERR_W<INTSTS1_SPEC> {
        EOFERR_W::new(self, 6)
    }
    ///Bit 8 - LPM Transaction End Interrupt Status
    #[inline(always)]
    pub fn lpmend(&mut self) -> LPMEND_W<INTSTS1_SPEC> {
        LPMEND_W::new(self, 8)
    }
    ///Bit 9 - L1 Resume End Interrupt Status
    #[inline(always)]
    pub fn l1rsmend(&mut self) -> L1RSMEND_W<INTSTS1_SPEC> {
        L1RSMEND_W::new(self, 9)
    }
    ///Bit 11 - USB Connection Detection Interrupt Status
    #[inline(always)]
    pub fn attch(&mut self) -> ATTCH_W<INTSTS1_SPEC> {
        ATTCH_W::new(self, 11)
    }
    ///Bit 12 - USB Disconnection Detection Interrupt Status
    #[inline(always)]
    pub fn dtch(&mut self) -> DTCH_W<INTSTS1_SPEC> {
        DTCH_W::new(self, 12)
    }
    ///Bit 14 - USB Bus Change Interrupt Status
    #[inline(always)]
    pub fn bchg(&mut self) -> BCHG_W<INTSTS1_SPEC> {
        BCHG_W::new(self, 14)
    }
    ///Bit 15 - Overcurrent Interrupt Status
    #[inline(always)]
    pub fn ovrcr(&mut self) -> OVRCR_W<INTSTS1_SPEC> {
        OVRCR_W::new(self, 15)
    }
}
/**Interrupt Status Register 1

You can [`read`](crate::Reg::read) this register and get [`intsts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTSTS1_SPEC;
impl crate::RegisterSpec for INTSTS1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`intsts1::R`](R) reader structure
impl crate::Readable for INTSTS1_SPEC {}
///`write(|w| ..)` method takes [`intsts1::W`](W) writer structure
impl crate::Writable for INTSTS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xdb71;
}
///`reset()` method sets INTSTS1 to value 0
impl crate::Resettable for INTSTS1_SPEC {}
