///Register `SOMCR` reader
pub type R = crate::R<SOMCR_SPEC>;
///Register `SOMCR` writer
pub type W = crate::W<SOMCR_SPEC>;
/**Sub-Clock Oscillator Drive Capability Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SODRV_A {
    ///0: Normal mode
    _00 = 0,
    ///1: Low power mode 1
    _01 = 1,
    ///2: Low power mode 2
    _10 = 2,
    ///3: Low power mode 3.
    _11 = 3,
}
impl From<SODRV_A> for u8 {
    #[inline(always)]
    fn from(variant: SODRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SODRV_A {
    type Ux = u8;
}
impl crate::IsEnum for SODRV_A {}
///Field `SODRV` reader - Sub-Clock Oscillator Drive Capability Switching
pub type SODRV_R = crate::FieldReader<SODRV_A>;
impl SODRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SODRV_A {
        match self.bits {
            0 => SODRV_A::_00,
            1 => SODRV_A::_01,
            2 => SODRV_A::_10,
            3 => SODRV_A::_11,
            _ => unreachable!(),
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SODRV_A::_00
    }
    ///Low power mode 1
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SODRV_A::_01
    }
    ///Low power mode 2
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SODRV_A::_10
    }
    ///Low power mode 3.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SODRV_A::_11
    }
}
///Field `SODRV` writer - Sub-Clock Oscillator Drive Capability Switching
pub type SODRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SODRV_A, crate::Safe>;
impl<'a, REG> SODRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_00)
    }
    ///Low power mode 1
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_01)
    }
    ///Low power mode 2
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_10)
    }
    ///Low power mode 3.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SODRV_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching
    #[inline(always)]
    pub fn sodrv(&self) -> SODRV_R {
        SODRV_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - Sub-Clock Oscillator Drive Capability Switching
    #[inline(always)]
    pub fn sodrv(&mut self) -> SODRV_W<'_, SOMCR_SPEC> {
        SODRV_W::new(self, 0)
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
