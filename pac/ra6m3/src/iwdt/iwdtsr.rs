///Register `IWDTSR` reader
pub type R = crate::R<IWDTSR_SPEC>;
///Register `IWDTSR` writer
pub type W = crate::W<IWDTSR_SPEC>;
///Field `CNTVAL` reader - Counter ValueValue counted by the counter
pub type CNTVAL_R = crate::FieldReader<u16>;
/**Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDFF_A {
    ///0: Underflow not occurred
    _0 = 0,
    ///1: Underflow occurred
    _1 = 1,
}
impl From<UNDFF_A> for bool {
    #[inline(always)]
    fn from(variant: UNDFF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `UNDFF` reader - Underflow Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type UNDFF_R = crate::BitReader<UNDFF_A>;
impl UNDFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UNDFF_A {
        match self.bits {
            false => UNDFF_A::_0,
            true => UNDFF_A::_1,
        }
    }
    ///Underflow not occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNDFF_A::_0
    }
    ///Underflow occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNDFF_A::_1
    }
}
///Field `UNDFF` writer - Underflow Flag
pub type UNDFF_W<'a, REG> = crate::BitWriter0C<'a, REG, UNDFF_A>;
impl<'a, REG> UNDFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Underflow not occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UNDFF_A::_0)
    }
    ///Underflow occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UNDFF_A::_1)
    }
}
/**Refresh Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFEF_A {
    ///0: Refresh error not occurred
    _0 = 0,
    ///1: Refresh error occurred
    _1 = 1,
}
impl From<REFEF_A> for bool {
    #[inline(always)]
    fn from(variant: REFEF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `REFEF` reader - Refresh Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type REFEF_R = crate::BitReader<REFEF_A>;
impl REFEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REFEF_A {
        match self.bits {
            false => REFEF_A::_0,
            true => REFEF_A::_1,
        }
    }
    ///Refresh error not occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REFEF_A::_0
    }
    ///Refresh error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REFEF_A::_1
    }
}
///Field `REFEF` writer - Refresh Error Flag
pub type REFEF_W<'a, REG> = crate::BitWriter0C<'a, REG, REFEF_A>;
impl<'a, REG> REFEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Refresh error not occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REFEF_A::_0)
    }
    ///Refresh error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REFEF_A::_1)
    }
}
impl R {
    ///Bits 0:13 - Counter ValueValue counted by the counter
    #[inline(always)]
    pub fn cntval(&self) -> CNTVAL_R {
        CNTVAL_R::new(self.bits & 0x3fff)
    }
    ///Bit 14 - Underflow Flag
    #[inline(always)]
    pub fn undff(&self) -> UNDFF_R {
        UNDFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Refresh Error Flag
    #[inline(always)]
    pub fn refef(&self) -> REFEF_R {
        REFEF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - Underflow Flag
    #[inline(always)]
    pub fn undff(&mut self) -> UNDFF_W<IWDTSR_SPEC> {
        UNDFF_W::new(self, 14)
    }
    ///Bit 15 - Refresh Error Flag
    #[inline(always)]
    pub fn refef(&mut self) -> REFEF_W<IWDTSR_SPEC> {
        REFEF_W::new(self, 15)
    }
}
/**IWDT Status Register

You can [`read`](crate::Reg::read) this register and get [`iwdtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IWDTSR_SPEC;
impl crate::RegisterSpec for IWDTSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`iwdtsr::R`](R) reader structure
impl crate::Readable for IWDTSR_SPEC {}
///`write(|w| ..)` method takes [`iwdtsr::W`](W) writer structure
impl crate::Writable for IWDTSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xc000;
}
///`reset()` method sets IWDTSR to value 0
impl crate::Resettable for IWDTSR_SPEC {}
