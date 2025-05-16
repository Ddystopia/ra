///Register `DYRQFR` reader
pub type R = crate::R<DYRQFR_SPEC>;
///Register `DYRQFR` writer
pub type W = crate::W<DYRQFR_SPEC>;
/**unicastFlag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG10_A {
    ///0: unicastFlag is set to FALSE.
    _0 = 0,
    ///1: unicastFlag is set to TRULE.
    _1 = 1,
}
impl From<FLAG10_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG10` reader - unicastFlag
pub type FLAG10_R = crate::BitReader<FLAG10_A>;
impl FLAG10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG10_A {
        match self.bits {
            false => FLAG10_A::_0,
            true => FLAG10_A::_1,
        }
    }
    ///unicastFlag is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG10_A::_0
    }
    ///unicastFlag is set to TRULE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG10_A::_1
    }
}
///Field `FLAG10` writer - unicastFlag
pub type FLAG10_W<'a, REG> = crate::BitWriter<'a, REG, FLAG10_A>;
impl<'a, REG> FLAG10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///unicastFlag is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG10_A::_0)
    }
    ///unicastFlag is set to TRULE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG10_A::_1)
    }
}
/**PTP profile Specific 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG13_A {
    ///0: PTP profile Specific 1 is set to FALSE.
    _0 = 0,
    ///1: PTP profile Specific 1 is set to TRULE.
    _1 = 1,
}
impl From<FLAG13_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG13` reader - PTP profile Specific 1
pub type FLAG13_R = crate::BitReader<FLAG13_A>;
impl FLAG13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG13_A {
        match self.bits {
            false => FLAG13_A::_0,
            true => FLAG13_A::_1,
        }
    }
    ///PTP profile Specific 1 is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG13_A::_0
    }
    ///PTP profile Specific 1 is set to TRULE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG13_A::_1
    }
}
///Field `FLAG13` writer - PTP profile Specific 1
pub type FLAG13_W<'a, REG> = crate::BitWriter<'a, REG, FLAG13_A>;
impl<'a, REG> FLAG13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PTP profile Specific 1 is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG13_A::_0)
    }
    ///PTP profile Specific 1 is set to TRULE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG13_A::_1)
    }
}
/**PTP profile Specific 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG14_A {
    ///0: PTP profile Specific 2 is set to FALSE.
    _0 = 0,
    ///1: PTP profile Specific 2 is set to TRULE.
    _1 = 1,
}
impl From<FLAG14_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG14` reader - PTP profile Specific 2
pub type FLAG14_R = crate::BitReader<FLAG14_A>;
impl FLAG14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG14_A {
        match self.bits {
            false => FLAG14_A::_0,
            true => FLAG14_A::_1,
        }
    }
    ///PTP profile Specific 2 is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG14_A::_0
    }
    ///PTP profile Specific 2 is set to TRULE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG14_A::_1
    }
}
///Field `FLAG14` writer - PTP profile Specific 2
pub type FLAG14_W<'a, REG> = crate::BitWriter<'a, REG, FLAG14_A>;
impl<'a, REG> FLAG14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PTP profile Specific 2 is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG14_A::_0)
    }
    ///PTP profile Specific 2 is set to TRULE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG14_A::_1)
    }
}
impl R {
    ///Bit 10 - unicastFlag
    #[inline(always)]
    pub fn flag10(&self) -> FLAG10_R {
        FLAG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - PTP profile Specific 1
    #[inline(always)]
    pub fn flag13(&self) -> FLAG13_R {
        FLAG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PTP profile Specific 2
    #[inline(always)]
    pub fn flag14(&self) -> FLAG14_R {
        FLAG14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 10 - unicastFlag
    #[inline(always)]
    pub fn flag10(&mut self) -> FLAG10_W<DYRQFR_SPEC> {
        FLAG10_W::new(self, 10)
    }
    ///Bit 13 - PTP profile Specific 1
    #[inline(always)]
    pub fn flag13(&mut self) -> FLAG13_W<DYRQFR_SPEC> {
        FLAG13_W::new(self, 13)
    }
    ///Bit 14 - PTP profile Specific 2
    #[inline(always)]
    pub fn flag14(&mut self) -> FLAG14_W<DYRQFR_SPEC> {
        FLAG14_W::new(self, 14)
    }
}
/**Delay_Req Message Flag Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`dyrqfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dyrqfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DYRQFR_SPEC;
impl crate::RegisterSpec for DYRQFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dyrqfr::R`](R) reader structure
impl crate::Readable for DYRQFR_SPEC {}
///`write(|w| ..)` method takes [`dyrqfr::W`](W) writer structure
impl crate::Writable for DYRQFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DYRQFR to value 0
impl crate::Resettable for DYRQFR_SPEC {}
