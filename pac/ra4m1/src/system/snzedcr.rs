///Register `SNZEDCR` reader
pub type R = crate::R<SNZEDCR_SPEC>;
///Register `SNZEDCR` writer
pub type W = crate::W<SNZEDCR_SPEC>;
/**AGT1 Underflow Snooze End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGTUNFED_A {
    ///0: Disable the Snooze End request
    _0 = 0,
    ///1: Enable the Snooze End request
    _1 = 1,
}
impl From<AGTUNFED_A> for bool {
    #[inline(always)]
    fn from(variant: AGTUNFED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AGTUNFED` reader - AGT1 Underflow Snooze End Enable
pub type AGTUNFED_R = crate::BitReader<AGTUNFED_A>;
impl AGTUNFED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AGTUNFED_A {
        match self.bits {
            false => AGTUNFED_A::_0,
            true => AGTUNFED_A::_1,
        }
    }
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGTUNFED_A::_0
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGTUNFED_A::_1
    }
}
///Field `AGTUNFED` writer - AGT1 Underflow Snooze End Enable
pub type AGTUNFED_W<'a, REG> = crate::BitWriter<'a, REG, AGTUNFED_A>;
impl<'a, REG> AGTUNFED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the Snooze End request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AGTUNFED_A::_0)
    }
    ///Enable the Snooze End request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AGTUNFED_A::_1)
    }
}
/**Last DTC Transmission Completion Snooze End Enable

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
///Field `DTCZRED` reader - Last DTC Transmission Completion Snooze End Enable
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
///Field `DTCZRED` writer - Last DTC Transmission Completion Snooze End Enable
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
/**Not Last DTC Transmission Completion Snooze End Enable

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
///Field `DTCNZRED` reader - Not Last DTC Transmission Completion Snooze End Enable
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
///Field `DTCNZRED` writer - Not Last DTC Transmission Completion Snooze End Enable
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
/**ADC140 Compare Match Snooze End Enable

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
///Field `AD0MATED` reader - ADC140 Compare Match Snooze End Enable
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
///Field `AD0MATED` writer - ADC140 Compare Match Snooze End Enable
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
/**ADC140 Compare Mismatch Snooze End Enable

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
///Field `AD0UMTED` reader - ADC140 Compare Mismatch Snooze End Enable
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
///Field `AD0UMTED` writer - ADC140 Compare Mismatch Snooze End Enable
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
/**SCI0 Address Mismatch Snooze End Enable

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
///Field `SCI0UMTED` reader - SCI0 Address Mismatch Snooze End Enable
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
///Field `SCI0UMTED` writer - SCI0 Address Mismatch Snooze End Enable
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
    ///Bit 0 - AGT1 Underflow Snooze End Enable
    #[inline(always)]
    pub fn agtunfed(&self) -> AGTUNFED_R {
        AGTUNFED_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtczred(&self) -> DTCZRED_R {
        DTCZRED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Not Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtcnzred(&self) -> DTCNZRED_R {
        DTCNZRED_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC140 Compare Match Snooze End Enable
    #[inline(always)]
    pub fn ad0mated(&self) -> AD0MATED_R {
        AD0MATED_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC140 Compare Mismatch Snooze End Enable
    #[inline(always)]
    pub fn ad0umted(&self) -> AD0UMTED_R {
        AD0UMTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - SCI0 Address Mismatch Snooze End Enable
    #[inline(always)]
    pub fn sci0umted(&self) -> SCI0UMTED_R {
        SCI0UMTED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AGT1 Underflow Snooze End Enable
    #[inline(always)]
    pub fn agtunfed(&mut self) -> AGTUNFED_W<SNZEDCR_SPEC> {
        AGTUNFED_W::new(self, 0)
    }
    ///Bit 1 - Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtczred(&mut self) -> DTCZRED_W<SNZEDCR_SPEC> {
        DTCZRED_W::new(self, 1)
    }
    ///Bit 2 - Not Last DTC Transmission Completion Snooze End Enable
    #[inline(always)]
    pub fn dtcnzred(&mut self) -> DTCNZRED_W<SNZEDCR_SPEC> {
        DTCNZRED_W::new(self, 2)
    }
    ///Bit 3 - ADC140 Compare Match Snooze End Enable
    #[inline(always)]
    pub fn ad0mated(&mut self) -> AD0MATED_W<SNZEDCR_SPEC> {
        AD0MATED_W::new(self, 3)
    }
    ///Bit 4 - ADC140 Compare Mismatch Snooze End Enable
    #[inline(always)]
    pub fn ad0umted(&mut self) -> AD0UMTED_W<SNZEDCR_SPEC> {
        AD0UMTED_W::new(self, 4)
    }
    ///Bit 7 - SCI0 Address Mismatch Snooze End Enable
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
