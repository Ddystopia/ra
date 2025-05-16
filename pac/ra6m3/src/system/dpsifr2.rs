///Register `DPSIFR2` reader
pub type R = crate::R<DPSIFR2_SPEC>;
///Register `DPSIFR2` writer
pub type W = crate::W<DPSIFR2_SPEC>;
/**LVD1 Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD1IF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DLVD1IF_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD1IF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DLVD1IF` reader - LVD1 Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DLVD1IF_R = crate::BitReader<DLVD1IF_A>;
impl DLVD1IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLVD1IF_A {
        match self.bits {
            false => DLVD1IF_A::_0,
            true => DLVD1IF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD1IF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD1IF_A::_1
    }
}
///Field `DLVD1IF` writer - LVD1 Deep Standby Cancel Flag
pub type DLVD1IF_W<'a, REG> = crate::BitWriter0C<'a, REG, DLVD1IF_A>;
impl<'a, REG> DLVD1IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD1IF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD1IF_A::_1)
    }
}
/**LVD2 Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD2IF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DLVD2IF_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD2IF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DLVD2IF` reader - LVD2 Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DLVD2IF_R = crate::BitReader<DLVD2IF_A>;
impl DLVD2IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLVD2IF_A {
        match self.bits {
            false => DLVD2IF_A::_0,
            true => DLVD2IF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD2IF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD2IF_A::_1
    }
}
///Field `DLVD2IF` writer - LVD2 Deep Standby Cancel Flag
pub type DLVD2IF_W<'a, REG> = crate::BitWriter0C<'a, REG, DLVD2IF_A>;
impl<'a, REG> DLVD2IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD2IF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD2IF_A::_1)
    }
}
/**RTC Interval interrupt Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRTCIIF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DTRTCIIF_A> for bool {
    #[inline(always)]
    fn from(variant: DTRTCIIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DTRTCIIF` reader - RTC Interval interrupt Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DTRTCIIF_R = crate::BitReader<DTRTCIIF_A>;
impl DTRTCIIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTRTCIIF_A {
        match self.bits {
            false => DTRTCIIF_A::_0,
            true => DTRTCIIF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTRTCIIF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTRTCIIF_A::_1
    }
}
///Field `DTRTCIIF` writer - RTC Interval interrupt Deep Standby Cancel Flag
pub type DTRTCIIF_W<'a, REG> = crate::BitWriter0C<'a, REG, DTRTCIIF_A>;
impl<'a, REG> DTRTCIIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTRTCIIF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTRTCIIF_A::_1)
    }
}
/**RTC Alarm interrupt Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRTCAIF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DRTCAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DRTCAIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DRTCAIF` reader - RTC Alarm interrupt Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DRTCAIF_R = crate::BitReader<DRTCAIF_A>;
impl DRTCAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRTCAIF_A {
        match self.bits {
            false => DRTCAIF_A::_0,
            true => DRTCAIF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRTCAIF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRTCAIF_A::_1
    }
}
///Field `DRTCAIF` writer - RTC Alarm interrupt Deep Standby Cancel Flag
pub type DRTCAIF_W<'a, REG> = crate::BitWriter0C<'a, REG, DRTCAIF_A>;
impl<'a, REG> DRTCAIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRTCAIF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRTCAIF_A::_1)
    }
}
/**NMI Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNMIF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DNMIF_A> for bool {
    #[inline(always)]
    fn from(variant: DNMIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DNMIF` reader - NMI Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DNMIF_R = crate::BitReader<DNMIF_A>;
impl DNMIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DNMIF_A {
        match self.bits {
            false => DNMIF_A::_0,
            true => DNMIF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNMIF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNMIF_A::_1
    }
}
///Field `DNMIF` writer - NMI Pin Deep Standby Cancel Flag
pub type DNMIF_W<'a, REG> = crate::BitWriter0C<'a, REG, DNMIF_A>;
impl<'a, REG> DNMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DNMIF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DNMIF_A::_1)
    }
}
impl R {
    ///Bit 0 - LVD1 Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd1if(&self) -> DLVD1IF_R {
        DLVD1IF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LVD2 Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd2if(&self) -> DLVD2IF_R {
        DLVD2IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTC Interval interrupt Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dtrtciif(&self) -> DTRTCIIF_R {
        DTRTCIIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTC Alarm interrupt Deep Standby Cancel Flag
    #[inline(always)]
    pub fn drtcaif(&self) -> DRTCAIF_R {
        DRTCAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NMI Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dnmif(&self) -> DNMIF_R {
        DNMIF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LVD1 Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd1if(&mut self) -> DLVD1IF_W<DPSIFR2_SPEC> {
        DLVD1IF_W::new(self, 0)
    }
    ///Bit 1 - LVD2 Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd2if(&mut self) -> DLVD2IF_W<DPSIFR2_SPEC> {
        DLVD2IF_W::new(self, 1)
    }
    ///Bit 2 - RTC Interval interrupt Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dtrtciif(&mut self) -> DTRTCIIF_W<DPSIFR2_SPEC> {
        DTRTCIIF_W::new(self, 2)
    }
    ///Bit 3 - RTC Alarm interrupt Deep Standby Cancel Flag
    #[inline(always)]
    pub fn drtcaif(&mut self) -> DRTCAIF_W<DPSIFR2_SPEC> {
        DRTCAIF_W::new(self, 3)
    }
    ///Bit 4 - NMI Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dnmif(&mut self) -> DNMIF_W<DPSIFR2_SPEC> {
        DNMIF_W::new(self, 4)
    }
}
/**Deep Standby Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsifr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIFR2_SPEC;
impl crate::RegisterSpec for DPSIFR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsifr2::R`](R) reader structure
impl crate::Readable for DPSIFR2_SPEC {}
///`write(|w| ..)` method takes [`dpsifr2::W`](W) writer structure
impl crate::Writable for DPSIFR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x1f;
}
///`reset()` method sets DPSIFR2 to value 0
impl crate::Resettable for DPSIFR2_SPEC {}
