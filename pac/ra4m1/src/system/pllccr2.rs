///Register `PLLCCR2` reader
pub type R = crate::R<PLLCCR2_SPEC>;
///Register `PLLCCR2` writer
pub type W = crate::W<PLLCCR2_SPEC>;
/**PLL Frequency Multiplication Factor Select

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL_A {
    ///15: Settings prohibited.
    _1111 = 15,
    ///0: x PLLMUL\[4:0\] +1
    OTHERS = 0,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLMUL_A {
    type Ux = u8;
}
impl crate::IsEnum for PLLMUL_A {}
///Field `PLLMUL` reader - PLL Frequency Multiplication Factor Select
pub type PLLMUL_R = crate::FieldReader<PLLMUL_A>;
impl PLLMUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLMUL_A {
        match self.bits {
            15 => PLLMUL_A::_1111,
            _ => PLLMUL_A::OTHERS,
        }
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PLLMUL_A::_1111
    }
    ///x PLLMUL\[4:0\] +1
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PLLMUL_A::OTHERS)
    }
}
///Field `PLLMUL` writer - PLL Frequency Multiplication Factor Select
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLMUL_A, crate::Safe>;
impl<'a, REG> PLLMUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Settings prohibited.
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL_A::_1111)
    }
    ///x PLLMUL\[4:0\] +1
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL_A::OTHERS)
    }
}
/**PLL Output Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLODIV_A {
    ///0: /1.
    _00 = 0,
    ///1: /2.
    _01 = 1,
    ///2: /4.
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<PLODIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLODIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLODIV_A {
    type Ux = u8;
}
impl crate::IsEnum for PLODIV_A {}
///Field `PLODIV` reader - PLL Output Frequency Division Ratio Select
pub type PLODIV_R = crate::FieldReader<PLODIV_A>;
impl PLODIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLODIV_A {
        match self.bits {
            0 => PLODIV_A::_00,
            1 => PLODIV_A::_01,
            2 => PLODIV_A::_10,
            3 => PLODIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "/1."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLODIV_A::_00
    }
    #[doc = "/2."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLODIV_A::_01
    }
    #[doc = "/4."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLODIV_A::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLODIV_A::_11
    }
}
///Field `PLODIV` writer - PLL Output Frequency Division Ratio Select
pub type PLODIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLODIV_A, crate::Safe>;
impl<'a, REG> PLODIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PLODIV_A::_00)
    }
    #[doc = "/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PLODIV_A::_01)
    }
    #[doc = "/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PLODIV_A::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PLODIV_A::_11)
    }
}
impl R {
    ///Bits 0:4 - PLL Frequency Multiplication Factor Select
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(self.bits & 0x1f)
    }
    ///Bits 6:7 - PLL Output Frequency Division Ratio Select
    #[inline(always)]
    pub fn plodiv(&self) -> PLODIV_R {
        PLODIV_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bits 0:4 - PLL Frequency Multiplication Factor Select
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W<'_, PLLCCR2_SPEC> {
        PLLMUL_W::new(self, 0)
    }
    ///Bits 6:7 - PLL Output Frequency Division Ratio Select
    #[inline(always)]
    pub fn plodiv(&mut self) -> PLODIV_W<'_, PLLCCR2_SPEC> {
        PLODIV_W::new(self, 6)
    }
}
/**PLL Clock Control Register2

You can [`read`](crate::Reg::read) this register and get [`pllccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLLCCR2_SPEC;
impl crate::RegisterSpec for PLLCCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`pllccr2::R`](R) reader structure
impl crate::Readable for PLLCCR2_SPEC {}
///`write(|w| ..)` method takes [`pllccr2::W`](W) writer structure
impl crate::Writable for PLLCCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCCR2 to value 0x07
impl crate::Resettable for PLLCCR2_SPEC {
    const RESET_VALUE: u8 = 0x07;
}
