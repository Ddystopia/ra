///Register `SYSCNT_DTCTEN` reader
pub type R = crate::R<SYSCNT_DTCTEN_SPEC>;
///Register `SYSCNT_DTCTEN` writer
pub type W = crate::W<SYSCNT_DTCTEN_SPEC>;
/**Specified line detection control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOSDTC_A {
    ///1: Enables detection.
    _1 = 1,
    ///0: Disables detection.
    _0 = 0,
}
impl From<VPOSDTC_A> for bool {
    #[inline(always)]
    fn from(variant: VPOSDTC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VPOSDTC` reader - Specified line detection control
pub type VPOSDTC_R = crate::BitReader<VPOSDTC_A>;
impl VPOSDTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VPOSDTC_A {
        match self.bits {
            true => VPOSDTC_A::_1,
            false => VPOSDTC_A::_0,
        }
    }
    ///Enables detection.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOSDTC_A::_1
    }
    ///Disables detection.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOSDTC_A::_0
    }
}
///Field `VPOSDTC` writer - Specified line detection control
pub type VPOSDTC_W<'a, REG> = crate::BitWriter<'a, REG, VPOSDTC_A>;
impl<'a, REG> VPOSDTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables detection.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VPOSDTC_A::_1)
    }
    ///Disables detection.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VPOSDTC_A::_0)
    }
}
/**Graphics 1 underflow detection control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDFDTC_A {
    ///1: Enables detection.
    _1 = 1,
    ///0: Disables detection.
    _0 = 0,
}
impl From<L1UNDFDTC_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDFDTC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1UNDFDTC` reader - Graphics 1 underflow detection control
pub type L1UNDFDTC_R = crate::BitReader<L1UNDFDTC_A>;
impl L1UNDFDTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1UNDFDTC_A {
        match self.bits {
            true => L1UNDFDTC_A::_1,
            false => L1UNDFDTC_A::_0,
        }
    }
    ///Enables detection.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDFDTC_A::_1
    }
    ///Disables detection.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDFDTC_A::_0
    }
}
///Field `L1UNDFDTC` writer - Graphics 1 underflow detection control
pub type L1UNDFDTC_W<'a, REG> = crate::BitWriter<'a, REG, L1UNDFDTC_A>;
impl<'a, REG> L1UNDFDTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables detection.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1UNDFDTC_A::_1)
    }
    ///Disables detection.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1UNDFDTC_A::_0)
    }
}
/**Graphics 2 underflow detection control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDFDTC_A {
    ///1: Enables detection.
    _1 = 1,
    ///0: Disables detection.
    _0 = 0,
}
impl From<L2UNDFDTC_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDFDTC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L2UNDFDTC` reader - Graphics 2 underflow detection control
pub type L2UNDFDTC_R = crate::BitReader<L2UNDFDTC_A>;
impl L2UNDFDTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L2UNDFDTC_A {
        match self.bits {
            true => L2UNDFDTC_A::_1,
            false => L2UNDFDTC_A::_0,
        }
    }
    ///Enables detection.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDFDTC_A::_1
    }
    ///Disables detection.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDFDTC_A::_0
    }
}
///Field `L2UNDFDTC` writer - Graphics 2 underflow detection control
pub type L2UNDFDTC_W<'a, REG> = crate::BitWriter<'a, REG, L2UNDFDTC_A>;
impl<'a, REG> L2UNDFDTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables detection.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L2UNDFDTC_A::_1)
    }
    ///Disables detection.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L2UNDFDTC_A::_0)
    }
}
impl R {
    ///Bit 0 - Specified line detection control
    #[inline(always)]
    pub fn vposdtc(&self) -> VPOSDTC_R {
        VPOSDTC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Graphics 1 underflow detection control
    #[inline(always)]
    pub fn l1undfdtc(&self) -> L1UNDFDTC_R {
        L1UNDFDTC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Graphics 2 underflow detection control
    #[inline(always)]
    pub fn l2undfdtc(&self) -> L2UNDFDTC_R {
        L2UNDFDTC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Specified line detection control
    #[inline(always)]
    pub fn vposdtc(&mut self) -> VPOSDTC_W<SYSCNT_DTCTEN_SPEC> {
        VPOSDTC_W::new(self, 0)
    }
    ///Bit 1 - Graphics 1 underflow detection control
    #[inline(always)]
    pub fn l1undfdtc(&mut self) -> L1UNDFDTC_W<SYSCNT_DTCTEN_SPEC> {
        L1UNDFDTC_W::new(self, 1)
    }
    ///Bit 2 - Graphics 2 underflow detection control
    #[inline(always)]
    pub fn l2undfdtc(&mut self) -> L2UNDFDTC_W<SYSCNT_DTCTEN_SPEC> {
        L2UNDFDTC_W::new(self, 2)
    }
}
/**System Control Block State Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_dtcten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_dtcten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCNT_DTCTEN_SPEC;
impl crate::RegisterSpec for SYSCNT_DTCTEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syscnt_dtcten::R`](R) reader structure
impl crate::Readable for SYSCNT_DTCTEN_SPEC {}
///`write(|w| ..)` method takes [`syscnt_dtcten::W`](W) writer structure
impl crate::Writable for SYSCNT_DTCTEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCNT_DTCTEN to value 0
impl crate::Resettable for SYSCNT_DTCTEN_SPEC {}
