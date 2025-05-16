///Register `PIPE%sTRE` reader
pub type R = crate::R<PIPETRE_SPEC>;
///Register `PIPE%sTRE` writer
pub type W = crate::W<PIPETRE_SPEC>;
/**Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRCLR_A {
    ///0: Invalid
    _0 = 0,
    ///1: The current counter value is cleared.
    _1 = 1,
}
impl From<TRCLR_A> for bool {
    #[inline(always)]
    fn from(variant: TRCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRCLR` reader - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0.
pub type TRCLR_R = crate::BitReader<TRCLR_A>;
impl TRCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRCLR_A {
        match self.bits {
            false => TRCLR_A::_0,
            true => TRCLR_A::_1,
        }
    }
    ///Invalid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRCLR_A::_0
    }
    ///The current counter value is cleared.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRCLR_A::_1
    }
}
///Field `TRCLR` writer - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0.
pub type TRCLR_W<'a, REG> = crate::BitWriter<'a, REG, TRCLR_A>;
impl<'a, REG> TRCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRCLR_A::_0)
    }
    ///The current counter value is cleared.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRCLR_A::_1)
    }
}
/**Transaction Counter EnableEnables or disables the transaction counter function.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRENB_A {
    ///0: The transaction counter function is disabled.
    _0 = 0,
    ///1: The transaction counter function is enabled.
    _1 = 1,
}
impl From<TRENB_A> for bool {
    #[inline(always)]
    fn from(variant: TRENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRENB` reader - Transaction Counter EnableEnables or disables the transaction counter function.
pub type TRENB_R = crate::BitReader<TRENB_A>;
impl TRENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRENB_A {
        match self.bits {
            false => TRENB_A::_0,
            true => TRENB_A::_1,
        }
    }
    ///The transaction counter function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRENB_A::_0
    }
    ///The transaction counter function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRENB_A::_1
    }
}
///Field `TRENB` writer - Transaction Counter EnableEnables or disables the transaction counter function.
pub type TRENB_W<'a, REG> = crate::BitWriter<'a, REG, TRENB_A>;
impl<'a, REG> TRENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The transaction counter function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRENB_A::_0)
    }
    ///The transaction counter function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRENB_A::_1)
    }
}
impl R {
    ///Bit 8 - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0.
    #[inline(always)]
    pub fn trclr(&self) -> TRCLR_R {
        TRCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transaction Counter EnableEnables or disables the transaction counter function.
    #[inline(always)]
    pub fn trenb(&self) -> TRENB_R {
        TRENB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - Transaction Counter ClearSetting this bit to 1 allows clearing the transaction counter to 0.
    #[inline(always)]
    pub fn trclr(&mut self) -> TRCLR_W<PIPETRE_SPEC> {
        TRCLR_W::new(self, 8)
    }
    ///Bit 9 - Transaction Counter EnableEnables or disables the transaction counter function.
    #[inline(always)]
    pub fn trenb(&mut self) -> TRENB_W<PIPETRE_SPEC> {
        TRENB_W::new(self, 9)
    }
}
/**PIPE Transaction Counter Enable Register

You can [`read`](crate::Reg::read) this register and get [`pipetre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPETRE_SPEC;
impl crate::RegisterSpec for PIPETRE_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipetre::R`](R) reader structure
impl crate::Readable for PIPETRE_SPEC {}
///`write(|w| ..)` method takes [`pipetre::W`](W) writer structure
impl crate::Writable for PIPETRE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPE%sTRE to value 0
impl crate::Resettable for PIPETRE_SPEC {}
