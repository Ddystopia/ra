///Register `LPCTRL` reader
pub type R = crate::R<LPCTRL_SPEC>;
///Register `LPCTRL` writer
pub type W = crate::W<LPCTRL_SPEC>;
/**Resume Return Mode Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWUPM_A {
    ///0: Hardware does not recover while CPU clock inactive
    _0 = 0,
    ///1: Hardware recovers while CPU clock inactive.
    _1 = 1,
}
impl From<HWUPM_A> for bool {
    #[inline(always)]
    fn from(variant: HWUPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HWUPM` reader - Resume Return Mode Setting
pub type HWUPM_R = crate::BitReader<HWUPM_A>;
impl HWUPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HWUPM_A {
        match self.bits {
            false => HWUPM_A::_0,
            true => HWUPM_A::_1,
        }
    }
    ///Hardware does not recover while CPU clock inactive
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HWUPM_A::_0
    }
    ///Hardware recovers while CPU clock inactive.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HWUPM_A::_1
    }
}
///Field `HWUPM` writer - Resume Return Mode Setting
pub type HWUPM_W<'a, REG> = crate::BitWriter<'a, REG, HWUPM_A>;
impl<'a, REG> HWUPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware does not recover while CPU clock inactive
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HWUPM_A::_0)
    }
    ///Hardware recovers while CPU clock inactive.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HWUPM_A::_1)
    }
}
impl R {
    ///Bit 7 - Resume Return Mode Setting
    #[inline(always)]
    pub fn hwupm(&self) -> HWUPM_R {
        HWUPM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - Resume Return Mode Setting
    #[inline(always)]
    pub fn hwupm(&mut self) -> HWUPM_W<LPCTRL_SPEC> {
        HWUPM_W::new(self, 7)
    }
}
/**Low Power Control Register

You can [`read`](crate::Reg::read) this register and get [`lpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LPCTRL_SPEC;
impl crate::RegisterSpec for LPCTRL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`lpctrl::R`](R) reader structure
impl crate::Readable for LPCTRL_SPEC {}
///`write(|w| ..)` method takes [`lpctrl::W`](W) writer structure
impl crate::Writable for LPCTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPCTRL to value 0
impl crate::Resettable for LPCTRL_SPEC {}
