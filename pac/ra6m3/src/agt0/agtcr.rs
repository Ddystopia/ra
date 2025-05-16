///Register `AGTCR` reader
pub type R = crate::R<AGTCR_SPEC>;
///Register `AGTCR` writer
pub type W = crate::W<AGTCR_SPEC>;
/**AGT count start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTART_A {
    ///0: Count stops
    _0 = 0,
    ///1: Count starts
    _1 = 1,
}
impl From<TSTART_A> for bool {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTART` reader - AGT count start
pub type TSTART_R = crate::BitReader<TSTART_A>;
impl TSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSTART_A {
        match self.bits {
            false => TSTART_A::_0,
            true => TSTART_A::_1,
        }
    }
    ///Count stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTART_A::_0
    }
    ///Count starts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTART_A::_1
    }
}
///Field `TSTART` writer - AGT count start
pub type TSTART_W<'a, REG> = crate::BitWriter<'a, REG, TSTART_A>;
impl<'a, REG> TSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count stops
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::_0)
    }
    ///Count starts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::_1)
    }
}
/**AGT count status flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSTF_A {
    ///0: Count stops
    _0 = 0,
    ///1: Count starts
    _1 = 1,
}
impl From<TCSTF_A> for bool {
    #[inline(always)]
    fn from(variant: TCSTF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCSTF` reader - AGT count status flag
pub type TCSTF_R = crate::BitReader<TCSTF_A>;
impl TCSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCSTF_A {
        match self.bits {
            false => TCSTF_A::_0,
            true => TCSTF_A::_1,
        }
    }
    ///Count stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCSTF_A::_0
    }
    ///Count starts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCSTF_A::_1
    }
}
/**AGT count forced stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTOP_A {
    ///0: no effect
    _0 = 0,
    ///1: The count is forcibly stopped.
    _1 = 1,
}
impl From<TSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTOP` writer - AGT count forced stop
pub type TSTOP_W<'a, REG> = crate::BitWriter<'a, REG, TSTOP_A>;
impl<'a, REG> TSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::_0)
    }
    ///The count is forcibly stopped.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::_1)
    }
}
/**Active edge judgement flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEDGF_A {
    ///0: No active edge received
    _0 = 0,
    ///1: Active edge received
    _1 = 1,
}
impl From<TEDGF_A> for bool {
    #[inline(always)]
    fn from(variant: TEDGF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TEDGF` reader - Active edge judgement flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TEDGF_R = crate::BitReader<TEDGF_A>;
impl TEDGF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEDGF_A {
        match self.bits {
            false => TEDGF_A::_0,
            true => TEDGF_A::_1,
        }
    }
    ///No active edge received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEDGF_A::_0
    }
    ///Active edge received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEDGF_A::_1
    }
}
///Field `TEDGF` writer - Active edge judgement flag
pub type TEDGF_W<'a, REG> = crate::BitWriter0C<'a, REG, TEDGF_A>;
impl<'a, REG> TEDGF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No active edge received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEDGF_A::_0)
    }
    ///Active edge received
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEDGF_A::_1)
    }
}
/**AGT underflow flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUNDF_A {
    ///0: No underflow
    _0 = 0,
    ///1: Underflow
    _1 = 1,
}
impl From<TUNDF_A> for bool {
    #[inline(always)]
    fn from(variant: TUNDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TUNDF` reader - AGT underflow flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TUNDF_R = crate::BitReader<TUNDF_A>;
impl TUNDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TUNDF_A {
        match self.bits {
            false => TUNDF_A::_0,
            true => TUNDF_A::_1,
        }
    }
    ///No underflow
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUNDF_A::_0
    }
    ///Underflow
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUNDF_A::_1
    }
}
///Field `TUNDF` writer - AGT underflow flag
pub type TUNDF_W<'a, REG> = crate::BitWriter0C<'a, REG, TUNDF_A>;
impl<'a, REG> TUNDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No underflow
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TUNDF_A::_0)
    }
    ///Underflow
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TUNDF_A::_1)
    }
}
/**AGT compare match A flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMAF_A {
    ///0: No Match
    _0 = 0,
    ///1: Match
    _1 = 1,
}
impl From<TCMAF_A> for bool {
    #[inline(always)]
    fn from(variant: TCMAF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TCMAF` reader - AGT compare match A flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TCMAF_R = crate::BitReader<TCMAF_A>;
impl TCMAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCMAF_A {
        match self.bits {
            false => TCMAF_A::_0,
            true => TCMAF_A::_1,
        }
    }
    ///No Match
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMAF_A::_0
    }
    ///Match
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMAF_A::_1
    }
}
///Field `TCMAF` writer - AGT compare match A flag
pub type TCMAF_W<'a, REG> = crate::BitWriter0C<'a, REG, TCMAF_A>;
impl<'a, REG> TCMAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Match
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCMAF_A::_0)
    }
    ///Match
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCMAF_A::_1)
    }
}
/**AGT compare match B flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMBF_A {
    ///0: No Match
    _0 = 0,
    ///1: Match
    _1 = 1,
}
impl From<TCMBF_A> for bool {
    #[inline(always)]
    fn from(variant: TCMBF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TCMBF` reader - AGT compare match B flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TCMBF_R = crate::BitReader<TCMBF_A>;
impl TCMBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCMBF_A {
        match self.bits {
            false => TCMBF_A::_0,
            true => TCMBF_A::_1,
        }
    }
    ///No Match
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMBF_A::_0
    }
    ///Match
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMBF_A::_1
    }
}
///Field `TCMBF` writer - AGT compare match B flag
pub type TCMBF_W<'a, REG> = crate::BitWriter0C<'a, REG, TCMBF_A>;
impl<'a, REG> TCMBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Match
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCMBF_A::_0)
    }
    ///Match
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCMBF_A::_1)
    }
}
impl R {
    ///Bit 0 - AGT count start
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AGT count status flag
    #[inline(always)]
    pub fn tcstf(&self) -> TCSTF_R {
        TCSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Active edge judgement flag
    #[inline(always)]
    pub fn tedgf(&self) -> TEDGF_R {
        TEDGF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AGT underflow flag
    #[inline(always)]
    pub fn tundf(&self) -> TUNDF_R {
        TUNDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AGT compare match A flag
    #[inline(always)]
    pub fn tcmaf(&self) -> TCMAF_R {
        TCMAF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AGT compare match B flag
    #[inline(always)]
    pub fn tcmbf(&self) -> TCMBF_R {
        TCMBF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AGT count start
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W<AGTCR_SPEC> {
        TSTART_W::new(self, 0)
    }
    ///Bit 2 - AGT count forced stop
    #[inline(always)]
    pub fn tstop(&mut self) -> TSTOP_W<AGTCR_SPEC> {
        TSTOP_W::new(self, 2)
    }
    ///Bit 4 - Active edge judgement flag
    #[inline(always)]
    pub fn tedgf(&mut self) -> TEDGF_W<AGTCR_SPEC> {
        TEDGF_W::new(self, 4)
    }
    ///Bit 5 - AGT underflow flag
    #[inline(always)]
    pub fn tundf(&mut self) -> TUNDF_W<AGTCR_SPEC> {
        TUNDF_W::new(self, 5)
    }
    ///Bit 6 - AGT compare match A flag
    #[inline(always)]
    pub fn tcmaf(&mut self) -> TCMAF_W<AGTCR_SPEC> {
        TCMAF_W::new(self, 6)
    }
    ///Bit 7 - AGT compare match B flag
    #[inline(always)]
    pub fn tcmbf(&mut self) -> TCMBF_W<AGTCR_SPEC> {
        TCMBF_W::new(self, 7)
    }
}
/**AGT Control Register

You can [`read`](crate::Reg::read) this register and get [`agtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTCR_SPEC;
impl crate::RegisterSpec for AGTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtcr::R`](R) reader structure
impl crate::Readable for AGTCR_SPEC {}
///`write(|w| ..)` method takes [`agtcr::W`](W) writer structure
impl crate::Writable for AGTCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xf0;
}
///`reset()` method sets AGTCR to value 0
impl crate::Resettable for AGTCR_SPEC {}
