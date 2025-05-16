///Register `ECCMODE` reader
pub type R = crate::R<ECCMODE_SPEC>;
///Register `ECCMODE` writer
pub type W = crate::W<ECCMODE_SPEC>;
/**ECC Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCMOD_A {
    ///0: Disable ECC function
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: Enable ECC function without error checking
    _10 = 2,
    ///3: Enable ECC function with error checking.
    _11 = 3,
}
impl From<ECCMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ECCMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for ECCMOD_A {}
///Field `ECCMOD` reader - ECC Operating Mode Select
pub type ECCMOD_R = crate::FieldReader<ECCMOD_A>;
impl ECCMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCMOD_A {
        match self.bits {
            0 => ECCMOD_A::_00,
            1 => ECCMOD_A::_01,
            2 => ECCMOD_A::_10,
            3 => ECCMOD_A::_11,
            _ => unreachable!(),
        }
    }
    ///Disable ECC function
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ECCMOD_A::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ECCMOD_A::_01
    }
    ///Enable ECC function without error checking
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ECCMOD_A::_10
    }
    ///Enable ECC function with error checking.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ECCMOD_A::_11
    }
}
///Field `ECCMOD` writer - ECC Operating Mode Select
pub type ECCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ECCMOD_A, crate::Safe>;
impl<'a, REG> ECCMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disable ECC function
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_01)
    }
    ///Enable ECC function without error checking
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_10)
    }
    ///Enable ECC function with error checking.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMOD_A::_11)
    }
}
impl R {
    ///Bits 0:1 - ECC Operating Mode Select
    #[inline(always)]
    pub fn eccmod(&self) -> ECCMOD_R {
        ECCMOD_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - ECC Operating Mode Select
    #[inline(always)]
    pub fn eccmod(&mut self) -> ECCMOD_W<ECCMODE_SPEC> {
        ECCMOD_W::new(self, 0)
    }
}
/**ECCRAM Operating Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`eccmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECCMODE_SPEC;
impl crate::RegisterSpec for ECCMODE_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eccmode::R`](R) reader structure
impl crate::Readable for ECCMODE_SPEC {}
///`write(|w| ..)` method takes [`eccmode::W`](W) writer structure
impl crate::Writable for ECCMODE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCMODE to value 0
impl crate::Resettable for ECCMODE_SPEC {}
