///Register `OPCCR` reader
pub type R = crate::R<OPCCR_SPEC>;
///Register `OPCCR` writer
pub type W = crate::W<OPCCR_SPEC>;
/**Operating Power Control Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPCM_A {
    ///0: High-speed mode
    _00 = 0,
    ///1: Prohibited
    _01 = 1,
    ///2: Prohibited
    _10 = 2,
    ///3: Low-speed mode
    _11 = 3,
}
impl From<OPCM_A> for u8 {
    #[inline(always)]
    fn from(variant: OPCM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPCM_A {
    type Ux = u8;
}
impl crate::IsEnum for OPCM_A {}
///Field `OPCM` reader - Operating Power Control Mode Select
pub type OPCM_R = crate::FieldReader<OPCM_A>;
impl OPCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPCM_A> {
        match self.bits {
            0 => Some(OPCM_A::_00),
            1 => Some(OPCM_A::_01),
            2 => Some(OPCM_A::_10),
            3 => Some(OPCM_A::_11),
            _ => None,
        }
    }
    ///High-speed mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OPCM_A::_00
    }
    ///Prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OPCM_A::_01
    }
    ///Prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OPCM_A::_10
    }
    ///Low-speed mode
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OPCM_A::_11
    }
}
///Field `OPCM` writer - Operating Power Control Mode Select
pub type OPCM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OPCM_A, crate::Safe>;
impl<'a, REG> OPCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///High-speed mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_00)
    }
    ///Prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_01)
    }
    ///Prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_10)
    }
    ///Low-speed mode
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OPCM_A::_11)
    }
}
/**Operating Power Control Mode Transition Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPCMTSF_A {
    ///0: Transition completed
    _0 = 0,
    ///1: During transition
    _1 = 1,
}
impl From<OPCMTSF_A> for bool {
    #[inline(always)]
    fn from(variant: OPCMTSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPCMTSF` reader - Operating Power Control Mode Transition Status Flag
pub type OPCMTSF_R = crate::BitReader<OPCMTSF_A>;
impl OPCMTSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPCMTSF_A {
        match self.bits {
            false => OPCMTSF_A::_0,
            true => OPCMTSF_A::_1,
        }
    }
    ///Transition completed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OPCMTSF_A::_0
    }
    ///During transition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OPCMTSF_A::_1
    }
}
impl R {
    ///Bits 0:1 - Operating Power Control Mode Select
    #[inline(always)]
    pub fn opcm(&self) -> OPCM_R {
        OPCM_R::new(self.bits & 3)
    }
    ///Bit 4 - Operating Power Control Mode Transition Status Flag
    #[inline(always)]
    pub fn opcmtsf(&self) -> OPCMTSF_R {
        OPCMTSF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Operating Power Control Mode Select
    #[inline(always)]
    pub fn opcm(&mut self) -> OPCM_W<OPCCR_SPEC> {
        OPCM_W::new(self, 0)
    }
}
/**Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`opccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OPCCR_SPEC;
impl crate::RegisterSpec for OPCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`opccr::R`](R) reader structure
impl crate::Readable for OPCCR_SPEC {}
///`write(|w| ..)` method takes [`opccr::W`](W) writer structure
impl crate::Writable for OPCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPCCR to value 0
impl crate::Resettable for OPCCR_SPEC {}
