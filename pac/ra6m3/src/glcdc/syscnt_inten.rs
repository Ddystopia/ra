///Register `SYSCNT_INTEN` reader
pub type R = crate::R<SYSCNT_INTEN_SPEC>;
///Register `SYSCNT_INTEN` writer
pub type W = crate::W<SYSCNT_INTEN_SPEC>;
/**Interrupt request signal GLCDC_VPOS enable control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOSINTEN_A {
    ///1: Enables GLCDC_VPOS output
    _1 = 1,
    ///0: Disables GLCDC_VPOS output
    _0 = 0,
}
impl From<VPOSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: VPOSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VPOSINTEN` reader - Interrupt request signal GLCDC_VPOS enable control.
pub type VPOSINTEN_R = crate::BitReader<VPOSINTEN_A>;
impl VPOSINTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VPOSINTEN_A {
        match self.bits {
            true => VPOSINTEN_A::_1,
            false => VPOSINTEN_A::_0,
        }
    }
    ///Enables GLCDC_VPOS output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOSINTEN_A::_1
    }
    ///Disables GLCDC_VPOS output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOSINTEN_A::_0
    }
}
///Field `VPOSINTEN` writer - Interrupt request signal GLCDC_VPOS enable control.
pub type VPOSINTEN_W<'a, REG> = crate::BitWriter<'a, REG, VPOSINTEN_A>;
impl<'a, REG> VPOSINTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables GLCDC_VPOS output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VPOSINTEN_A::_1)
    }
    ///Disables GLCDC_VPOS output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VPOSINTEN_A::_0)
    }
}
/**Interrupt request signal GLCDC_L1UNDF enable control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDFINTEN_A {
    ///1: Enables GLCDC_L1UNDF output
    _1 = 1,
    ///0: Disables GLCDC_L1UNDF output
    _0 = 0,
}
impl From<L1UNDFINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDFINTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1UNDFINTEN` reader - Interrupt request signal GLCDC_L1UNDF enable control.
pub type L1UNDFINTEN_R = crate::BitReader<L1UNDFINTEN_A>;
impl L1UNDFINTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1UNDFINTEN_A {
        match self.bits {
            true => L1UNDFINTEN_A::_1,
            false => L1UNDFINTEN_A::_0,
        }
    }
    ///Enables GLCDC_L1UNDF output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDFINTEN_A::_1
    }
    ///Disables GLCDC_L1UNDF output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDFINTEN_A::_0
    }
}
///Field `L1UNDFINTEN` writer - Interrupt request signal GLCDC_L1UNDF enable control.
pub type L1UNDFINTEN_W<'a, REG> = crate::BitWriter<'a, REG, L1UNDFINTEN_A>;
impl<'a, REG> L1UNDFINTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables GLCDC_L1UNDF output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1UNDFINTEN_A::_1)
    }
    ///Disables GLCDC_L1UNDF output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1UNDFINTEN_A::_0)
    }
}
/**Interrupt request signal GLCDC_L2UNDF enable control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDFINTEN_A {
    ///1: Enables GLCDC_L2UNDF output
    _1 = 1,
    ///0: Disables GLCDC_L2UNDF output
    _0 = 0,
}
impl From<L2UNDFINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDFINTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L2UNDFINTEN` reader - Interrupt request signal GLCDC_L2UNDF enable control.
pub type L2UNDFINTEN_R = crate::BitReader<L2UNDFINTEN_A>;
impl L2UNDFINTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L2UNDFINTEN_A {
        match self.bits {
            true => L2UNDFINTEN_A::_1,
            false => L2UNDFINTEN_A::_0,
        }
    }
    ///Enables GLCDC_L2UNDF output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDFINTEN_A::_1
    }
    ///Disables GLCDC_L2UNDF output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDFINTEN_A::_0
    }
}
///Field `L2UNDFINTEN` writer - Interrupt request signal GLCDC_L2UNDF enable control.
pub type L2UNDFINTEN_W<'a, REG> = crate::BitWriter<'a, REG, L2UNDFINTEN_A>;
impl<'a, REG> L2UNDFINTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables GLCDC_L2UNDF output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L2UNDFINTEN_A::_1)
    }
    ///Disables GLCDC_L2UNDF output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L2UNDFINTEN_A::_0)
    }
}
impl R {
    ///Bit 0 - Interrupt request signal GLCDC_VPOS enable control.
    #[inline(always)]
    pub fn vposinten(&self) -> VPOSINTEN_R {
        VPOSINTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt request signal GLCDC_L1UNDF enable control.
    #[inline(always)]
    pub fn l1undfinten(&self) -> L1UNDFINTEN_R {
        L1UNDFINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt request signal GLCDC_L2UNDF enable control.
    #[inline(always)]
    pub fn l2undfinten(&self) -> L2UNDFINTEN_R {
        L2UNDFINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt request signal GLCDC_VPOS enable control.
    #[inline(always)]
    pub fn vposinten(&mut self) -> VPOSINTEN_W<SYSCNT_INTEN_SPEC> {
        VPOSINTEN_W::new(self, 0)
    }
    ///Bit 1 - Interrupt request signal GLCDC_L1UNDF enable control.
    #[inline(always)]
    pub fn l1undfinten(&mut self) -> L1UNDFINTEN_W<SYSCNT_INTEN_SPEC> {
        L1UNDFINTEN_W::new(self, 1)
    }
    ///Bit 2 - Interrupt request signal GLCDC_L2UNDF enable control.
    #[inline(always)]
    pub fn l2undfinten(&mut self) -> L2UNDFINTEN_W<SYSCNT_INTEN_SPEC> {
        L2UNDFINTEN_W::new(self, 2)
    }
}
/**System Control Block Interrupt Request Enable Control Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCNT_INTEN_SPEC;
impl crate::RegisterSpec for SYSCNT_INTEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syscnt_inten::R`](R) reader structure
impl crate::Readable for SYSCNT_INTEN_SPEC {}
///`write(|w| ..)` method takes [`syscnt_inten::W`](W) writer structure
impl crate::Writable for SYSCNT_INTEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCNT_INTEN to value 0
impl crate::Resettable for SYSCNT_INTEN_SPEC {}
