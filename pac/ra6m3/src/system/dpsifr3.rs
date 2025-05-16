///Register `DPSIFR3` reader
pub type R = crate::R<DPSIFR3_SPEC>;
///Register `DPSIFR3` writer
pub type W = crate::W<DPSIFR3_SPEC>;
/**USBFS Suspend/Resume Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBFSIF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DUSBFSIF_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBFSIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DUSBFSIF` reader - USBFS Suspend/Resume Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DUSBFSIF_R = crate::BitReader<DUSBFSIF_A>;
impl DUSBFSIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DUSBFSIF_A {
        match self.bits {
            false => DUSBFSIF_A::_0,
            true => DUSBFSIF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBFSIF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBFSIF_A::_1
    }
}
///Field `DUSBFSIF` writer - USBFS Suspend/Resume Deep Standby Cancel Flag
pub type DUSBFSIF_W<'a, REG> = crate::BitWriter0C<'a, REG, DUSBFSIF_A>;
impl<'a, REG> DUSBFSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBFSIF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBFSIF_A::_1)
    }
}
/**USBHS Suspend/Resume Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBHSIF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DUSBHSIF_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBHSIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DUSBHSIF` reader - USBHS Suspend/Resume Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DUSBHSIF_R = crate::BitReader<DUSBHSIF_A>;
impl DUSBHSIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DUSBHSIF_A {
        match self.bits {
            false => DUSBHSIF_A::_0,
            true => DUSBHSIF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBHSIF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBHSIF_A::_1
    }
}
///Field `DUSBHSIF` writer - USBHS Suspend/Resume Deep Standby Cancel Flag
pub type DUSBHSIF_W<'a, REG> = crate::BitWriter0C<'a, REG, DUSBHSIF_A>;
impl<'a, REG> DUSBHSIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBHSIF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBHSIF_A::_1)
    }
}
/**AGT1 Underflow Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAGT1IF_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DAGT1IF_A> for bool {
    #[inline(always)]
    fn from(variant: DAGT1IF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DAGT1IF` reader - AGT1 Underflow Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DAGT1IF_R = crate::BitReader<DAGT1IF_A>;
impl DAGT1IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAGT1IF_A {
        match self.bits {
            false => DAGT1IF_A::_0,
            true => DAGT1IF_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAGT1IF_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAGT1IF_A::_1
    }
}
///Field `DAGT1IF` writer - AGT1 Underflow Deep Standby Cancel Flag
pub type DAGT1IF_W<'a, REG> = crate::BitWriter0C<'a, REG, DAGT1IF_A>;
impl<'a, REG> DAGT1IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAGT1IF_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAGT1IF_A::_1)
    }
}
impl R {
    ///Bit 0 - USBFS Suspend/Resume Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dusbfsif(&self) -> DUSBFSIF_R {
        DUSBFSIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dusbhsif(&self) -> DUSBHSIF_R {
        DUSBHSIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGT1 Underflow Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dagt1if(&self) -> DAGT1IF_R {
        DAGT1IF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USBFS Suspend/Resume Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dusbfsif(&mut self) -> DUSBFSIF_W<DPSIFR3_SPEC> {
        DUSBFSIF_W::new(self, 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dusbhsif(&mut self) -> DUSBHSIF_W<DPSIFR3_SPEC> {
        DUSBHSIF_W::new(self, 1)
    }
    ///Bit 2 - AGT1 Underflow Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dagt1if(&mut self) -> DAGT1IF_W<DPSIFR3_SPEC> {
        DAGT1IF_W::new(self, 2)
    }
}
/**Deep Standby Interrupt Flag Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsifr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIFR3_SPEC;
impl crate::RegisterSpec for DPSIFR3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsifr3::R`](R) reader structure
impl crate::Readable for DPSIFR3_SPEC {}
///`write(|w| ..)` method takes [`dpsifr3::W`](W) writer structure
impl crate::Writable for DPSIFR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x07;
}
///`reset()` method sets DPSIFR3 to value 0
impl crate::Resettable for DPSIFR3_SPEC {}
