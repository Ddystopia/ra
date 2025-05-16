///Register `SNZEDCR` reader
pub type R = crate::R<SNZEDCR_SPEC>;
///Register `SNZEDCR` writer
pub type W = crate::W<SNZEDCR_SPEC>;
/**AGT1 underflow Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1UNFED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<AGT1UNFED_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1UNFED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1UNFED` reader - AGT1 underflow Snooze End Enable
pub type AGT1UNFED_R = crate::BitReader<AGT1UNFED_A>;
impl AGT1UNFED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AGT1UNFED_A {
        match self.bits {
            false => AGT1UNFED_A::_0,
            true => AGT1UNFED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1UNFED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1UNFED_A::_1
    }
}
///Field `AGT1UNFED` writer - AGT1 underflow Snooze End Enable
pub type AGT1UNFED_W<'a, REG> = crate::BitWriter<'a, REG, AGT1UNFED_A>;
impl<'a, REG> AGT1UNFED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1UNFED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1UNFED_A::_1)
    }
}
/**Last DTC transmission completion Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCZRED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<DTCZRED_A> for bool {
    #[inline(always)]
    fn from(variant: DTCZRED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCZRED` reader - Last DTC transmission completion Snooze End Enable
pub type DTCZRED_R = crate::BitReader<DTCZRED_A>;
impl DTCZRED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCZRED_A {
        match self.bits {
            false => DTCZRED_A::_0,
            true => DTCZRED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCZRED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCZRED_A::_1
    }
}
///Field `DTCZRED` writer - Last DTC transmission completion Snooze End Enable
pub type DTCZRED_W<'a, REG> = crate::BitWriter<'a, REG, DTCZRED_A>;
impl<'a, REG> DTCZRED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCZRED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCZRED_A::_1)
    }
}
/**Not Last DTC transmission completion Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCNZRED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<DTCNZRED_A> for bool {
    #[inline(always)]
    fn from(variant: DTCNZRED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCNZRED` reader - Not Last DTC transmission completion Snooze End Enable
pub type DTCNZRED_R = crate::BitReader<DTCNZRED_A>;
impl DTCNZRED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCNZRED_A {
        match self.bits {
            false => DTCNZRED_A::_0,
            true => DTCNZRED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCNZRED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCNZRED_A::_1
    }
}
///Field `DTCNZRED` writer - Not Last DTC transmission completion Snooze End Enable
pub type DTCNZRED_W<'a, REG> = crate::BitWriter<'a, REG, DTCNZRED_A>;
impl<'a, REG> DTCNZRED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCNZRED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCNZRED_A::_1)
    }
}
/**AD compare match 0 Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD0MATED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<AD0MATED_A> for bool {
    #[inline(always)]
    fn from(variant: AD0MATED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AD0MATED` reader - AD compare match 0 Snooze End Enable
pub type AD0MATED_R = crate::BitReader<AD0MATED_A>;
impl AD0MATED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD0MATED_A {
        match self.bits {
            false => AD0MATED_A::_0,
            true => AD0MATED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD0MATED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD0MATED_A::_1
    }
}
///Field `AD0MATED` writer - AD compare match 0 Snooze End Enable
pub type AD0MATED_W<'a, REG> = crate::BitWriter<'a, REG, AD0MATED_A>;
impl<'a, REG> AD0MATED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AD0MATED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AD0MATED_A::_1)
    }
}
/**AD compare mismatch 0 Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD0UMTED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<AD0UMTED_A> for bool {
    #[inline(always)]
    fn from(variant: AD0UMTED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AD0UMTED` reader - AD compare mismatch 0 Snooze End Enable
pub type AD0UMTED_R = crate::BitReader<AD0UMTED_A>;
impl AD0UMTED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD0UMTED_A {
        match self.bits {
            false => AD0UMTED_A::_0,
            true => AD0UMTED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD0UMTED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD0UMTED_A::_1
    }
}
///Field `AD0UMTED` writer - AD compare mismatch 0 Snooze End Enable
pub type AD0UMTED_W<'a, REG> = crate::BitWriter<'a, REG, AD0UMTED_A>;
impl<'a, REG> AD0UMTED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AD0UMTED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AD0UMTED_A::_1)
    }
}
/**AD compare match 1 Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MATED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<AD1MATED_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MATED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AD1MATED` reader - AD compare match 1 Snooze End Enable
pub type AD1MATED_R = crate::BitReader<AD1MATED_A>;
impl AD1MATED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD1MATED_A {
        match self.bits {
            false => AD1MATED_A::_0,
            true => AD1MATED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD1MATED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD1MATED_A::_1
    }
}
///Field `AD1MATED` writer - AD compare match 1 Snooze End Enable
pub type AD1MATED_W<'a, REG> = crate::BitWriter<'a, REG, AD1MATED_A>;
impl<'a, REG> AD1MATED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AD1MATED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AD1MATED_A::_1)
    }
}
/**AD compare mismatch 1 Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1UMTED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<AD1UMTED_A> for bool {
    #[inline(always)]
    fn from(variant: AD1UMTED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AD1UMTED` reader - AD compare mismatch 1 Snooze End Enable
pub type AD1UMTED_R = crate::BitReader<AD1UMTED_A>;
impl AD1UMTED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD1UMTED_A {
        match self.bits {
            false => AD1UMTED_A::_0,
            true => AD1UMTED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AD1UMTED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AD1UMTED_A::_1
    }
}
///Field `AD1UMTED` writer - AD compare mismatch 1 Snooze End Enable
pub type AD1UMTED_W<'a, REG> = crate::BitWriter<'a, REG, AD1UMTED_A>;
impl<'a, REG> AD1UMTED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AD1UMTED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AD1UMTED_A::_1)
    }
}
/**SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCI0UMTED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<SCI0UMTED_A> for bool {
    #[inline(always)]
    fn from(variant: SCI0UMTED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCI0UMTED` reader - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode.
pub type SCI0UMTED_R = crate::BitReader<SCI0UMTED_A>;
impl SCI0UMTED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCI0UMTED_A {
        match self.bits {
            false => SCI0UMTED_A::_0,
            true => SCI0UMTED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCI0UMTED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCI0UMTED_A::_1
    }
}
///Field `SCI0UMTED` writer - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode.
pub type SCI0UMTED_W<'a, REG> = crate::BitWriter<'a, REG, SCI0UMTED_A>;
impl<'a, REG> SCI0UMTED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCI0UMTED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCI0UMTED_A::_1)
    }
}
impl R {
    ///Bit 0 - AGT1 underflow Snooze End Enable
    #[inline(always)]
    pub fn agt1unfed(&self) -> AGT1UNFED_R {
        AGT1UNFED_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Last DTC transmission completion Snooze End Enable
    #[inline(always)]
    pub fn dtczred(&self) -> DTCZRED_R {
        DTCZRED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Not Last DTC transmission completion Snooze End Enable
    #[inline(always)]
    pub fn dtcnzred(&self) -> DTCNZRED_R {
        DTCNZRED_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AD compare match 0 Snooze End Enable
    #[inline(always)]
    pub fn ad0mated(&self) -> AD0MATED_R {
        AD0MATED_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AD compare mismatch 0 Snooze End Enable
    #[inline(always)]
    pub fn ad0umted(&self) -> AD0UMTED_R {
        AD0UMTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AD compare match 1 Snooze End Enable
    #[inline(always)]
    pub fn ad1mated(&self) -> AD1MATED_R {
        AD1MATED_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AD compare mismatch 1 Snooze End Enable
    #[inline(always)]
    pub fn ad1umted(&self) -> AD1UMTED_R {
        AD1UMTED_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode.
    #[inline(always)]
    pub fn sci0umted(&self) -> SCI0UMTED_R {
        SCI0UMTED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AGT1 underflow Snooze End Enable
    #[inline(always)]
    pub fn agt1unfed(&mut self) -> AGT1UNFED_W<SNZEDCR_SPEC> {
        AGT1UNFED_W::new(self, 0)
    }
    ///Bit 1 - Last DTC transmission completion Snooze End Enable
    #[inline(always)]
    pub fn dtczred(&mut self) -> DTCZRED_W<SNZEDCR_SPEC> {
        DTCZRED_W::new(self, 1)
    }
    ///Bit 2 - Not Last DTC transmission completion Snooze End Enable
    #[inline(always)]
    pub fn dtcnzred(&mut self) -> DTCNZRED_W<SNZEDCR_SPEC> {
        DTCNZRED_W::new(self, 2)
    }
    ///Bit 3 - AD compare match 0 Snooze End Enable
    #[inline(always)]
    pub fn ad0mated(&mut self) -> AD0MATED_W<SNZEDCR_SPEC> {
        AD0MATED_W::new(self, 3)
    }
    ///Bit 4 - AD compare mismatch 0 Snooze End Enable
    #[inline(always)]
    pub fn ad0umted(&mut self) -> AD0UMTED_W<SNZEDCR_SPEC> {
        AD0UMTED_W::new(self, 4)
    }
    ///Bit 5 - AD compare match 1 Snooze End Enable
    #[inline(always)]
    pub fn ad1mated(&mut self) -> AD1MATED_W<SNZEDCR_SPEC> {
        AD1MATED_W::new(self, 5)
    }
    ///Bit 6 - AD compare mismatch 1 Snooze End Enable
    #[inline(always)]
    pub fn ad1umted(&mut self) -> AD1UMTED_W<SNZEDCR_SPEC> {
        AD1UMTED_W::new(self, 6)
    }
    ///Bit 7 - SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode.
    #[inline(always)]
    pub fn sci0umted(&mut self) -> SCI0UMTED_W<SNZEDCR_SPEC> {
        SCI0UMTED_W::new(self, 7)
    }
}
/**Snooze End Control Register

You can [`read`](crate::Reg::read) this register and get [`snzedcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SNZEDCR_SPEC;
impl crate::RegisterSpec for SNZEDCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`snzedcr::R`](R) reader structure
impl crate::Readable for SNZEDCR_SPEC {}
///`write(|w| ..)` method takes [`snzedcr::W`](W) writer structure
impl crate::Writable for SNZEDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZEDCR to value 0
impl crate::Resettable for SNZEDCR_SPEC {}
