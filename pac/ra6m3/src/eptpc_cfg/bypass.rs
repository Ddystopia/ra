///Register `BYPASS` reader
pub type R = crate::R<BYPASS_SPEC>;
///Register `BYPASS` writer
pub type W = crate::W<BYPASS_SPEC>;
/**Bypass 1588 module for Ether 0ch

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS0_A {
    ///0: to use 1588 module for Ether 0ch
    _0 = 0,
    ///1: to bypass 1588 module for Ether 0ch
    _1 = 1,
}
impl From<BYPASS0_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPASS0` reader - Bypass 1588 module for Ether 0ch
pub type BYPASS0_R = crate::BitReader<BYPASS0_A>;
impl BYPASS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS0_A {
        match self.bits {
            false => BYPASS0_A::_0,
            true => BYPASS0_A::_1,
        }
    }
    ///to use 1588 module for Ether 0ch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYPASS0_A::_0
    }
    ///to bypass 1588 module for Ether 0ch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYPASS0_A::_1
    }
}
///Field `BYPASS0` writer - Bypass 1588 module for Ether 0ch
pub type BYPASS0_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS0_A>;
impl<'a, REG> BYPASS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///to use 1588 module for Ether 0ch
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS0_A::_0)
    }
    ///to bypass 1588 module for Ether 0ch
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS0_A::_1)
    }
}
impl R {
    ///Bit 0 - Bypass 1588 module for Ether 0ch
    #[inline(always)]
    pub fn bypass0(&self) -> BYPASS0_R {
        BYPASS0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Bypass 1588 module for Ether 0ch
    #[inline(always)]
    pub fn bypass0(&mut self) -> BYPASS0_W<BYPASS_SPEC> {
        BYPASS0_W::new(self, 0)
    }
}
/**Bypass 1588 module Register

You can [`read`](crate::Reg::read) this register and get [`bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BYPASS_SPEC;
impl crate::RegisterSpec for BYPASS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bypass::R`](R) reader structure
impl crate::Readable for BYPASS_SPEC {}
///`write(|w| ..)` method takes [`bypass::W`](W) writer structure
impl crate::Writable for BYPASS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BYPASS to value 0
impl crate::Resettable for BYPASS_SPEC {}
