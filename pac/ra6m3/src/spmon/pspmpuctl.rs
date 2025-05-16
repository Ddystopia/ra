///Register `PSPMPUCTL` reader
pub type R = crate::R<PSPMPUCTL_SPEC>;
///Register `PSPMPUCTL` writer
pub type W = crate::W<PSPMPUCTL_SPEC>;
/**SP_process monitor enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    ///0: SP_process monitor is disabled.
    _0 = 0,
    ///1: SP_process monitor is enabled.
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - SP_process monitor enable
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    ///SP_process monitor is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    ///SP_process monitor is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
///Field `ENABLE` writer - SP_process monitor enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SP_process monitor is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    ///SP_process monitor is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_1)
    }
}
/**SP_process monitor error flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    ///0: SP_process has not overflowed or underflowed.
    _0 = 0,
    ///1: SP_process has overflowed or underflowed.
    _1 = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ERROR` reader - SP_process monitor error flag
pub type ERROR_R = crate::BitReader<ERROR_A>;
impl ERROR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::_0,
            true => ERROR_A::_1,
        }
    }
    ///SP_process has not overflowed or underflowed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERROR_A::_0
    }
    ///SP_process has overflowed or underflowed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERROR_A::_1
    }
}
impl R {
    ///Bit 0 - SP_process monitor enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - SP_process monitor error flag
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SP_process monitor enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<PSPMPUCTL_SPEC> {
        ENABLE_W::new(self, 0)
    }
}
/**Stack Pointer Monitor Access Control Register

You can [`read`](crate::Reg::read) this register and get [`pspmpuctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PSPMPUCTL_SPEC;
impl crate::RegisterSpec for PSPMPUCTL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pspmpuctl::R`](R) reader structure
impl crate::Readable for PSPMPUCTL_SPEC {}
///`write(|w| ..)` method takes [`pspmpuctl::W`](W) writer structure
impl crate::Writable for PSPMPUCTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSPMPUCTL to value 0
impl crate::Resettable for PSPMPUCTL_SPEC {}
