///Register `INTSTS0` reader
pub type R = crate::R<INTSTS0_SPEC>;
///Register `INTSTS0` writer
pub type W = crate::W<INTSTS0_SPEC>;
/**Control Transfer Stage

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSQ_A {
    ///0: Idle or setup stage
    _000 = 0,
    ///1: Control read data stage
    _001 = 1,
    ///2: Control read status stage
    _010 = 2,
    ///3: Control write data stage
    _011 = 3,
    ///4: Control write status stage
    _100 = 4,
    ///5: Control write (no data) status stage
    _101 = 5,
    ///6: Control transfer sequence error
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<CTSQ_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSQ_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSQ_A {}
///Field `CTSQ` reader - Control Transfer Stage
pub type CTSQ_R = crate::FieldReader<CTSQ_A>;
impl CTSQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSQ_A {
        match self.bits {
            0 => CTSQ_A::_000,
            1 => CTSQ_A::_001,
            2 => CTSQ_A::_010,
            3 => CTSQ_A::_011,
            4 => CTSQ_A::_100,
            5 => CTSQ_A::_101,
            6 => CTSQ_A::_110,
            7 => CTSQ_A::OTHERS,
            _ => unreachable!(),
        }
    }
    ///Idle or setup stage
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CTSQ_A::_000
    }
    ///Control read data stage
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CTSQ_A::_001
    }
    ///Control read status stage
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CTSQ_A::_010
    }
    ///Control write data stage
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CTSQ_A::_011
    }
    ///Control write status stage
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CTSQ_A::_100
    }
    ///Control write (no data) status stage
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CTSQ_A::_101
    }
    ///Control transfer sequence error
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CTSQ_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == CTSQ_A::OTHERS
    }
}
/**USB Request Reception

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALID_A {
    ///0: Setup packet is not received
    _0 = 0,
    ///1: Setup packet is received
    _1 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VALID` reader - USB Request Reception
pub type VALID_R = crate::BitReader<VALID_A>;
impl VALID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::_0,
            true => VALID_A::_1,
        }
    }
    ///Setup packet is not received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VALID_A::_0
    }
    ///Setup packet is received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VALID_A::_1
    }
}
///Field `VALID` writer - USB Request Reception
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG, VALID_A>;
impl<'a, REG> VALID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setup packet is not received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VALID_A::_0)
    }
    ///Setup packet is received
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VALID_A::_1)
    }
}
/**Device State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSQ_A {
    ///0: Powered state
    _000 = 0,
    ///1: Default state
    _001 = 1,
    ///2: Address state
    _010 = 2,
    ///3: Configured state
    _011 = 3,
    ///4: Suspended state
    OTHERS = 4,
}
impl From<DVSQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DVSQ_A {
    type Ux = u8;
}
impl crate::IsEnum for DVSQ_A {}
///Field `DVSQ` reader - Device State
pub type DVSQ_R = crate::FieldReader<DVSQ_A>;
impl DVSQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVSQ_A {
        match self.bits {
            0 => DVSQ_A::_000,
            1 => DVSQ_A::_001,
            2 => DVSQ_A::_010,
            3 => DVSQ_A::_011,
            _ => DVSQ_A::OTHERS,
        }
    }
    ///Powered state
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DVSQ_A::_000
    }
    ///Default state
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DVSQ_A::_001
    }
    ///Address state
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DVSQ_A::_010
    }
    ///Configured state
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DVSQ_A::_011
    }
    ///Suspended state
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DVSQ_A::OTHERS)
    }
}
/**VBUS Input Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBSTS_A {
    ///0: USB0_VBUS pin is low.
    _0 = 0,
    ///1: USB0_VBUS pin is high.
    _1 = 1,
}
impl From<VBSTS_A> for bool {
    #[inline(always)]
    fn from(variant: VBSTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBSTS` reader - VBUS Input Status
pub type VBSTS_R = crate::BitReader<VBSTS_A>;
impl VBSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBSTS_A {
        match self.bits {
            false => VBSTS_A::_0,
            true => VBSTS_A::_1,
        }
    }
    ///USB0_VBUS pin is low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBSTS_A::_0
    }
    ///USB0_VBUS pin is high.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBSTS_A::_1
    }
}
/**Buffer Ready Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDY_A {
    ///0: BRDY interrupts are not generated.
    _0 = 0,
    ///1: BRDY interrupts are generated.
    _1 = 1,
}
impl From<BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDY` reader - Buffer Ready Interrupt Status
pub type BRDY_R = crate::BitReader<BRDY_A>;
impl BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRDY_A {
        match self.bits {
            false => BRDY_A::_0,
            true => BRDY_A::_1,
        }
    }
    ///BRDY interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRDY_A::_0
    }
    ///BRDY interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRDY_A::_1
    }
}
/**Buffer Not Ready Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRDY_A {
    ///0: NRDY interrupts are not generated.
    _0 = 0,
    ///1: NRDY interrupts are generated.
    _1 = 1,
}
impl From<NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NRDY` reader - Buffer Not Ready Interrupt Status
pub type NRDY_R = crate::BitReader<NRDY_A>;
impl NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NRDY_A {
        match self.bits {
            false => NRDY_A::_0,
            true => NRDY_A::_1,
        }
    }
    ///NRDY interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRDY_A::_0
    }
    ///NRDY interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRDY_A::_1
    }
}
/**Buffer Empty Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEMP_A {
    ///0: BEMP interrupts are not generated.
    _0 = 0,
    ///1: BEMP interrupts are generated.
    _1 = 1,
}
impl From<BEMP_A> for bool {
    #[inline(always)]
    fn from(variant: BEMP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BEMP` reader - Buffer Empty Interrupt Status
pub type BEMP_R = crate::BitReader<BEMP_A>;
impl BEMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BEMP_A {
        match self.bits {
            false => BEMP_A::_0,
            true => BEMP_A::_1,
        }
    }
    ///BEMP interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEMP_A::_0
    }
    ///BEMP interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEMP_A::_1
    }
}
/**Control Transfer Stage Transition Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRT_A {
    ///0: Control transfer stage transition interrupts are not generated.
    _0 = 0,
    ///1: Control transfer stage transition interrupts are generated.
    _1 = 1,
}
impl From<CTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CTRT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CTRT` reader - Control Transfer Stage Transition Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CTRT_R = crate::BitReader<CTRT_A>;
impl CTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTRT_A {
        match self.bits {
            false => CTRT_A::_0,
            true => CTRT_A::_1,
        }
    }
    ///Control transfer stage transition interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTRT_A::_0
    }
    ///Control transfer stage transition interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTRT_A::_1
    }
}
///Field `CTRT` writer - Control Transfer Stage Transition Interrupt Status
pub type CTRT_W<'a, REG> = crate::BitWriter0C<'a, REG, CTRT_A>;
impl<'a, REG> CTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Control transfer stage transition interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTRT_A::_0)
    }
    ///Control transfer stage transition interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTRT_A::_1)
    }
}
/**Device State Transition Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVST_A {
    ///0: Device state transition interrupts are not generated.
    _0 = 0,
    ///1: Device state transition interrupts are generated.
    _1 = 1,
}
impl From<DVST_A> for bool {
    #[inline(always)]
    fn from(variant: DVST_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DVST` reader - Device State Transition Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DVST_R = crate::BitReader<DVST_A>;
impl DVST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVST_A {
        match self.bits {
            false => DVST_A::_0,
            true => DVST_A::_1,
        }
    }
    ///Device state transition interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVST_A::_0
    }
    ///Device state transition interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVST_A::_1
    }
}
///Field `DVST` writer - Device State Transition Interrupt Status
pub type DVST_W<'a, REG> = crate::BitWriter0C<'a, REG, DVST_A>;
impl<'a, REG> DVST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Device state transition interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVST_A::_0)
    }
    ///Device state transition interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVST_A::_1)
    }
}
/**Frame Number Refresh Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFR_A {
    ///0: SOF interrupts are not generated.
    _0 = 0,
    ///1: SOF interrupts are generated.
    _1 = 1,
}
impl From<SOFR_A> for bool {
    #[inline(always)]
    fn from(variant: SOFR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SOFR` reader - Frame Number Refresh Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SOFR_R = crate::BitReader<SOFR_A>;
impl SOFR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOFR_A {
        match self.bits {
            false => SOFR_A::_0,
            true => SOFR_A::_1,
        }
    }
    ///SOF interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFR_A::_0
    }
    ///SOF interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFR_A::_1
    }
}
///Field `SOFR` writer - Frame Number Refresh Interrupt Status
pub type SOFR_W<'a, REG> = crate::BitWriter0C<'a, REG, SOFR_A>;
impl<'a, REG> SOFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SOF interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOFR_A::_0)
    }
    ///SOF interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOFR_A::_1)
    }
}
/**Resume Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESM_A {
    ///0: Resume interrupts are not generated.
    _0 = 0,
    ///1: Resume interrupts are generated.
    _1 = 1,
}
impl From<RESM_A> for bool {
    #[inline(always)]
    fn from(variant: RESM_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RESM` reader - Resume Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RESM_R = crate::BitReader<RESM_A>;
impl RESM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESM_A {
        match self.bits {
            false => RESM_A::_0,
            true => RESM_A::_1,
        }
    }
    ///Resume interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESM_A::_0
    }
    ///Resume interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESM_A::_1
    }
}
///Field `RESM` writer - Resume Interrupt Status
pub type RESM_W<'a, REG> = crate::BitWriter0C<'a, REG, RESM_A>;
impl<'a, REG> RESM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resume interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESM_A::_0)
    }
    ///Resume interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESM_A::_1)
    }
}
/**VBUS Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBINT_A {
    ///0: VBUS interrupts are not generated.
    _0 = 0,
    ///1: VBUS interrupts are generated.
    _1 = 1,
}
impl From<VBINT_A> for bool {
    #[inline(always)]
    fn from(variant: VBINT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VBINT` reader - VBUS Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VBINT_R = crate::BitReader<VBINT_A>;
impl VBINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBINT_A {
        match self.bits {
            false => VBINT_A::_0,
            true => VBINT_A::_1,
        }
    }
    ///VBUS interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBINT_A::_0
    }
    ///VBUS interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBINT_A::_1
    }
}
///Field `VBINT` writer - VBUS Interrupt Status
pub type VBINT_W<'a, REG> = crate::BitWriter0C<'a, REG, VBINT_A>;
impl<'a, REG> VBINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBUS interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBINT_A::_0)
    }
    ///VBUS interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBINT_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Control Transfer Stage
    #[inline(always)]
    pub fn ctsq(&self) -> CTSQ_R {
        CTSQ_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - USB Request Reception
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Device State
    #[inline(always)]
    pub fn dvsq(&self) -> DVSQ_R {
        DVSQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - VBUS Input Status
    #[inline(always)]
    pub fn vbsts(&self) -> VBSTS_R {
        VBSTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Buffer Ready Interrupt Status
    #[inline(always)]
    pub fn brdy(&self) -> BRDY_R {
        BRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Buffer Not Ready Interrupt Status
    #[inline(always)]
    pub fn nrdy(&self) -> NRDY_R {
        NRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer Empty Interrupt Status
    #[inline(always)]
    pub fn bemp(&self) -> BEMP_R {
        BEMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Status
    #[inline(always)]
    pub fn ctrt(&self) -> CTRT_R {
        CTRT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Device State Transition Interrupt Status
    #[inline(always)]
    pub fn dvst(&self) -> DVST_R {
        DVST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Frame Number Refresh Interrupt Status
    #[inline(always)]
    pub fn sofr(&self) -> SOFR_R {
        SOFR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Resume Interrupt Status
    #[inline(always)]
    pub fn resm(&self) -> RESM_R {
        RESM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VBUS Interrupt Status
    #[inline(always)]
    pub fn vbint(&self) -> VBINT_R {
        VBINT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - USB Request Reception
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W<INTSTS0_SPEC> {
        VALID_W::new(self, 3)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Status
    #[inline(always)]
    pub fn ctrt(&mut self) -> CTRT_W<INTSTS0_SPEC> {
        CTRT_W::new(self, 11)
    }
    ///Bit 12 - Device State Transition Interrupt Status
    #[inline(always)]
    pub fn dvst(&mut self) -> DVST_W<INTSTS0_SPEC> {
        DVST_W::new(self, 12)
    }
    ///Bit 13 - Frame Number Refresh Interrupt Status
    #[inline(always)]
    pub fn sofr(&mut self) -> SOFR_W<INTSTS0_SPEC> {
        SOFR_W::new(self, 13)
    }
    ///Bit 14 - Resume Interrupt Status
    #[inline(always)]
    pub fn resm(&mut self) -> RESM_W<INTSTS0_SPEC> {
        RESM_W::new(self, 14)
    }
    ///Bit 15 - VBUS Interrupt Status
    #[inline(always)]
    pub fn vbint(&mut self) -> VBINT_W<INTSTS0_SPEC> {
        VBINT_W::new(self, 15)
    }
}
/**Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`intsts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTSTS0_SPEC;
impl crate::RegisterSpec for INTSTS0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`intsts0::R`](R) reader structure
impl crate::Readable for INTSTS0_SPEC {}
///`write(|w| ..)` method takes [`intsts0::W`](W) writer structure
impl crate::Writable for INTSTS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xf800;
}
///`reset()` method sets INTSTS0 to value 0
impl crate::Resettable for INTSTS0_SPEC {}
