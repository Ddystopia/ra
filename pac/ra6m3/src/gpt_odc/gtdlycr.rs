///Register `GTDLYCR` reader
pub type R = crate::R<GTDLYCR_SPEC>;
///Register `GTDLYCR` writer
pub type W = crate::W<GTDLYCR_SPEC>;
/**DLL Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLEN_A {
    ///0: Disable DLL operation
    _0 = 0,
    ///1: Enable DLL operation
    _1 = 1,
}
impl From<DLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLLEN` reader - DLL Operation Enable
pub type DLLEN_R = crate::BitReader<DLLEN_A>;
impl DLLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLLEN_A {
        match self.bits {
            false => DLLEN_A::_0,
            true => DLLEN_A::_1,
        }
    }
    ///Disable DLL operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLLEN_A::_0
    }
    ///Enable DLL operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLLEN_A::_1
    }
}
///Field `DLLEN` writer - DLL Operation Enable
pub type DLLEN_W<'a, REG> = crate::BitWriter<'a, REG, DLLEN_A>;
impl<'a, REG> DLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DLL operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLLEN_A::_0)
    }
    ///Enable DLL operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLLEN_A::_1)
    }
}
/**PWM Delay Generation Circuit Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYRST_A {
    ///0: Normal operation
    _0 = 0,
    ///1: Reset
    _1 = 1,
}
impl From<DLYRST_A> for bool {
    #[inline(always)]
    fn from(variant: DLYRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYRST` reader - PWM Delay Generation Circuit Reset
pub type DLYRST_R = crate::BitReader<DLYRST_A>;
impl DLYRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYRST_A {
        match self.bits {
            false => DLYRST_A::_0,
            true => DLYRST_A::_1,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYRST_A::_0
    }
    ///Reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYRST_A::_1
    }
}
///Field `DLYRST` writer - PWM Delay Generation Circuit Reset
pub type DLYRST_W<'a, REG> = crate::BitWriter<'a, REG, DLYRST_A>;
impl<'a, REG> DLYRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYRST_A::_0)
    }
    ///Reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYRST_A::_1)
    }
}
impl R {
    ///Bit 0 - DLL Operation Enable
    #[inline(always)]
    pub fn dllen(&self) -> DLLEN_R {
        DLLEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PWM Delay Generation Circuit Reset
    #[inline(always)]
    pub fn dlyrst(&self) -> DLYRST_R {
        DLYRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DLL Operation Enable
    #[inline(always)]
    pub fn dllen(&mut self) -> DLLEN_W<GTDLYCR_SPEC> {
        DLLEN_W::new(self, 0)
    }
    ///Bit 1 - PWM Delay Generation Circuit Reset
    #[inline(always)]
    pub fn dlyrst(&mut self) -> DLYRST_W<GTDLYCR_SPEC> {
        DLYRST_W::new(self, 1)
    }
}
/**PWM Output Delay Control Register

You can [`read`](crate::Reg::read) this register and get [`gtdlycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDLYCR_SPEC;
impl crate::RegisterSpec for GTDLYCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`gtdlycr::R`](R) reader structure
impl crate::Readable for GTDLYCR_SPEC {}
///`write(|w| ..)` method takes [`gtdlycr::W`](W) writer structure
impl crate::Writable for GTDLYCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDLYCR to value 0
impl crate::Resettable for GTDLYCR_SPEC {}
