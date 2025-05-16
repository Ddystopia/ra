///Register `SYSCNT_STCLR` reader
pub type R = crate::R<SYSCNT_STCLR_SPEC>;
///Register `SYSCNT_STCLR` writer
pub type W = crate::W<SYSCNT_STCLR_SPEC>;
/**Graphics 2 specified line detection flag clear field

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOSCLR_A {
    ///1: Clears the specified line detection flag.
    _1 = 1,
    ///0: No operation
    _0 = 0,
}
impl From<VPOSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: VPOSCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VPOSCLR` reader - Graphics 2 specified line detection flag clear field
pub type VPOSCLR_R = crate::BitReader<VPOSCLR_A>;
impl VPOSCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VPOSCLR_A {
        match self.bits {
            true => VPOSCLR_A::_1,
            false => VPOSCLR_A::_0,
        }
    }
    ///Clears the specified line detection flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOSCLR_A::_1
    }
    ///No operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOSCLR_A::_0
    }
}
///Field `VPOSCLR` writer - Graphics 2 specified line detection flag clear field
pub type VPOSCLR_W<'a, REG> = crate::BitWriter<'a, REG, VPOSCLR_A>;
impl<'a, REG> VPOSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the specified line detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VPOSCLR_A::_1)
    }
    ///No operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VPOSCLR_A::_0)
    }
}
/**Graphics 1 underflow detection flag clear field

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDFCLR_A {
    ///1: Clears the graphics 1 underflow detection flag.
    _1 = 1,
    ///0: No operation
    _0 = 0,
}
impl From<L1UNDFCLR_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDFCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1UNDFCLR` reader - Graphics 1 underflow detection flag clear field
pub type L1UNDFCLR_R = crate::BitReader<L1UNDFCLR_A>;
impl L1UNDFCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1UNDFCLR_A {
        match self.bits {
            true => L1UNDFCLR_A::_1,
            false => L1UNDFCLR_A::_0,
        }
    }
    ///Clears the graphics 1 underflow detection flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDFCLR_A::_1
    }
    ///No operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDFCLR_A::_0
    }
}
///Field `L1UNDFCLR` writer - Graphics 1 underflow detection flag clear field
pub type L1UNDFCLR_W<'a, REG> = crate::BitWriter<'a, REG, L1UNDFCLR_A>;
impl<'a, REG> L1UNDFCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the graphics 1 underflow detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1UNDFCLR_A::_1)
    }
    ///No operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1UNDFCLR_A::_0)
    }
}
/**Graphics 2 underflow detection flag clear field

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDFCLR_A {
    ///1: Clears the graphics 2 underflow detection flag.
    _1 = 1,
    ///0: No operation
    _0 = 0,
}
impl From<L2UNDFCLR_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDFCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L2UNDFCLR` reader - Graphics 2 underflow detection flag clear field
pub type L2UNDFCLR_R = crate::BitReader<L2UNDFCLR_A>;
impl L2UNDFCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L2UNDFCLR_A {
        match self.bits {
            true => L2UNDFCLR_A::_1,
            false => L2UNDFCLR_A::_0,
        }
    }
    ///Clears the graphics 2 underflow detection flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDFCLR_A::_1
    }
    ///No operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDFCLR_A::_0
    }
}
///Field `L2UNDFCLR` writer - Graphics 2 underflow detection flag clear field
pub type L2UNDFCLR_W<'a, REG> = crate::BitWriter<'a, REG, L2UNDFCLR_A>;
impl<'a, REG> L2UNDFCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the graphics 2 underflow detection flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L2UNDFCLR_A::_1)
    }
    ///No operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L2UNDFCLR_A::_0)
    }
}
impl R {
    ///Bit 0 - Graphics 2 specified line detection flag clear field
    #[inline(always)]
    pub fn vposclr(&self) -> VPOSCLR_R {
        VPOSCLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Graphics 1 underflow detection flag clear field
    #[inline(always)]
    pub fn l1undfclr(&self) -> L1UNDFCLR_R {
        L1UNDFCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Graphics 2 underflow detection flag clear field
    #[inline(always)]
    pub fn l2undfclr(&self) -> L2UNDFCLR_R {
        L2UNDFCLR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Graphics 2 specified line detection flag clear field
    #[inline(always)]
    pub fn vposclr(&mut self) -> VPOSCLR_W<SYSCNT_STCLR_SPEC> {
        VPOSCLR_W::new(self, 0)
    }
    ///Bit 1 - Graphics 1 underflow detection flag clear field
    #[inline(always)]
    pub fn l1undfclr(&mut self) -> L1UNDFCLR_W<SYSCNT_STCLR_SPEC> {
        L1UNDFCLR_W::new(self, 1)
    }
    ///Bit 2 - Graphics 2 underflow detection flag clear field
    #[inline(always)]
    pub fn l2undfclr(&mut self) -> L2UNDFCLR_W<SYSCNT_STCLR_SPEC> {
        L2UNDFCLR_W::new(self, 2)
    }
}
/**System Control Block Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_stclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_stclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCNT_STCLR_SPEC;
impl crate::RegisterSpec for SYSCNT_STCLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syscnt_stclr::R`](R) reader structure
impl crate::Readable for SYSCNT_STCLR_SPEC {}
///`write(|w| ..)` method takes [`syscnt_stclr::W`](W) writer structure
impl crate::Writable for SYSCNT_STCLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCNT_STCLR to value 0
impl crate::Resettable for SYSCNT_STCLR_SPEC {}
