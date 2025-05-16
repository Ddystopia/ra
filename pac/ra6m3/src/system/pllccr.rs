///Register `PLLCCR` reader
pub type R = crate::R<PLLCCR_SPEC>;
///Register `PLLCCR` writer
pub type W = crate::W<PLLCCR_SPEC>;
/**PLL Input Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLIDIV_A {
    ///0: /1
    _00 = 0,
    ///1: /2
    _01 = 1,
    ///2: /3
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<PLIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLIDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLIDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for PLIDIV_A {}
///Field `PLIDIV` reader - PLL Input Frequency Division Ratio Select
pub type PLIDIV_R = crate::FieldReader<PLIDIV_A>;
impl PLIDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLIDIV_A {
        match self.bits {
            0 => PLIDIV_A::_00,
            1 => PLIDIV_A::_01,
            2 => PLIDIV_A::_10,
            3 => PLIDIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLIDIV_A::_00
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLIDIV_A::_01
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLIDIV_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLIDIV_A::_11
    }
}
///Field `PLIDIV` writer - PLL Input Frequency Division Ratio Select
pub type PLIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLIDIV_A, crate::Safe>;
impl<'a, REG> PLIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PLIDIV_A::_00)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PLIDIV_A::_01)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PLIDIV_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PLIDIV_A::_11)
    }
}
/**PLL Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSRCSEL_A {
    ///0: Main clock oscillator
    _0 = 0,
    ///1: HOCO
    _1 = 1,
}
impl From<PLSRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLSRCSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLSRCSEL` reader - PLL Clock Source Select
pub type PLSRCSEL_R = crate::BitReader<PLSRCSEL_A>;
impl PLSRCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLSRCSEL_A {
        match self.bits {
            false => PLSRCSEL_A::_0,
            true => PLSRCSEL_A::_1,
        }
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLSRCSEL_A::_0
    }
    ///HOCO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLSRCSEL_A::_1
    }
}
///Field `PLSRCSEL` writer - PLL Clock Source Select
pub type PLSRCSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLSRCSEL_A>;
impl<'a, REG> PLSRCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main clock oscillator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PLSRCSEL_A::_0)
    }
    ///HOCO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PLSRCSEL_A::_1)
    }
}
/**PLL Frequency Multiplication Factor Select \[PLL Frequency Multiplication Factor\] = (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0

Value on reset: 19*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL_A {
    ///0: Setting prohibited
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
///Field `PLLMUL` reader - PLL Frequency Multiplication Factor Select \[PLL Frequency Multiplication Factor\] = (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0
pub type PLLMUL_R = crate::FieldReader<PLLMUL_A>;
impl PLLMUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLMUL_A {
        match self.bits {
            _ => PLLMUL_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PLLMUL_A::OTHERS)
    }
}
///Field `PLLMUL` writer - PLL Frequency Multiplication Factor Select \[PLL Frequency Multiplication Factor\] = (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0
pub type PLLMUL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, PLLMUL_A, crate::Safe>;
impl<'a, REG> PLLMUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PLLMUL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:1 - PLL Input Frequency Division Ratio Select
    #[inline(always)]
    pub fn plidiv(&self) -> PLIDIV_R {
        PLIDIV_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - PLL Clock Source Select
    #[inline(always)]
    pub fn plsrcsel(&self) -> PLSRCSEL_R {
        PLSRCSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:13 - PLL Frequency Multiplication Factor Select \[PLL Frequency Multiplication Factor\] = (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:1 - PLL Input Frequency Division Ratio Select
    #[inline(always)]
    pub fn plidiv(&mut self) -> PLIDIV_W<PLLCCR_SPEC> {
        PLIDIV_W::new(self, 0)
    }
    ///Bit 4 - PLL Clock Source Select
    #[inline(always)]
    pub fn plsrcsel(&mut self) -> PLSRCSEL_W<PLLCCR_SPEC> {
        PLSRCSEL_W::new(self, 4)
    }
    ///Bits 8:13 - PLL Frequency Multiplication Factor Select \[PLL Frequency Multiplication Factor\] = (PLLUMUL+1) / 2 Range: 0x23 - 0x3B for example 010011: x10.0 010100: x10.5 010101: x11.0 : 011100: x14.5 011101: x15.0 011110: x15.5 : 111010: x29.5 111011: x30.0
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W<PLLCCR_SPEC> {
        PLLMUL_W::new(self, 8)
    }
}
/**PLL Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`pllccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLLCCR_SPEC;
impl crate::RegisterSpec for PLLCCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pllccr::R`](R) reader structure
impl crate::Readable for PLLCCR_SPEC {}
///`write(|w| ..)` method takes [`pllccr::W`](W) writer structure
impl crate::Writable for PLLCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCCR to value 0x1300
impl crate::Resettable for PLLCCR_SPEC {
    const RESET_VALUE: u16 = 0x1300;
}
