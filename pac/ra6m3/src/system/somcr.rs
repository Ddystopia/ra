///Register `SOMCR` reader
pub type R = crate::R<SOMCR_SPEC>;
///Register `SOMCR` writer
pub type W = crate::W<SOMCR_SPEC>;
/**Sub Clock Oscillator Drive Capability Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SODRV1_A {
    ///0: Standard
    _0 = 0,
    ///1: Middle
    _1 = 1,
}
impl From<SODRV1_A> for bool {
    #[inline(always)]
    fn from(variant: SODRV1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SODRV1` reader - Sub Clock Oscillator Drive Capability Switching
pub type SODRV1_R = crate::BitReader<SODRV1_A>;
impl SODRV1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SODRV1_A {
        match self.bits {
            false => SODRV1_A::_0,
            true => SODRV1_A::_1,
        }
    }
    ///Standard
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SODRV1_A::_0
    }
    ///Middle
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SODRV1_A::_1
    }
}
///Field `SODRV1` writer - Sub Clock Oscillator Drive Capability Switching
pub type SODRV1_W<'a, REG> = crate::BitWriter<'a, REG, SODRV1_A>;
impl<'a, REG> SODRV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Standard
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV1_A::_0)
    }
    ///Middle
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV1_A::_1)
    }
}
impl R {
    ///Bit 1 - Sub Clock Oscillator Drive Capability Switching
    #[inline(always)]
    pub fn sodrv1(&self) -> SODRV1_R {
        SODRV1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Sub Clock Oscillator Drive Capability Switching
    #[inline(always)]
    pub fn sodrv1(&mut self) -> SODRV1_W<SOMCR_SPEC> {
        SODRV1_W::new(self, 1)
    }
}
/**Sub Clock Oscillator Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`somcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SOMCR_SPEC;
impl crate::RegisterSpec for SOMCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`somcr::R`](R) reader structure
impl crate::Readable for SOMCR_SPEC {}
///`write(|w| ..)` method takes [`somcr::W`](W) writer structure
impl crate::Writable for SOMCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOMCR to value 0
impl crate::Resettable for SOMCR_SPEC {}
