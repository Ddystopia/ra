///Register `ADCMPLER` reader
pub type R = crate::R<ADCMPLER_SPEC>;
///Register `ADCMPLER` writer
pub type W = crate::W<ADCMPLER_SPEC>;
/**Compare Window A Temperature Sensor Output Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLTSA_A {
    ///0: ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value < ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1).
    _0 = 0,
    ///1: ADCMPDR0 register value < A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value < A/D-converted value < ADCMPDR1 register value(ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLTSA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLTSA` reader - Compare Window A Temperature Sensor Output Comparison Condition Select
pub type CMPLTSA_R = crate::BitReader<CMPLTSA_A>;
impl CMPLTSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLTSA_A {
        match self.bits {
            false => CMPLTSA_A::_0,
            true => CMPLTSA_A::_1,
        }
    }
    ///ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value < ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLTSA_A::_0
    }
    ///ADCMPDR0 register value < A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value < A/D-converted value < ADCMPDR1 register value(ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLTSA_A::_1
    }
}
///Field `CMPLTSA` writer - Compare Window A Temperature Sensor Output Comparison Condition Select
pub type CMPLTSA_W<'a, REG> = crate::BitWriter<'a, REG, CMPLTSA_A>;
impl<'a, REG> CMPLTSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value < ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLTSA_A::_0)
    }
    ///ADCMPDR0 register value < A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value < A/D-converted value < ADCMPDR1 register value(ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLTSA_A::_1)
    }
}
/**Compare Window A Internal Reference Voltage ComparisonCondition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLOCA_A {
    ///0: ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value < A/D converted value < ADCMPDR1 value(ADCMPCR.WCMPE=1)
    _1 = 1,
}
impl From<CMPLOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLOCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLOCA` reader - Compare Window A Internal Reference Voltage ComparisonCondition Select
pub type CMPLOCA_R = crate::BitReader<CMPLOCA_A>;
impl CMPLOCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLOCA_A {
        match self.bits {
            false => CMPLOCA_A::_0,
            true => CMPLOCA_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLOCA_A::_0
    }
    ///ADCMPDR0 value < A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value < A/D converted value < ADCMPDR1 value(ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLOCA_A::_1
    }
}
///Field `CMPLOCA` writer - Compare Window A Internal Reference Voltage ComparisonCondition Select
pub type CMPLOCA_W<'a, REG> = crate::BitWriter<'a, REG, CMPLOCA_A>;
impl<'a, REG> CMPLOCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLOCA_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value < A/D converted value < ADCMPDR1 value(ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLOCA_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select
    #[inline(always)]
    pub fn cmpltsa(&self) -> CMPLTSA_R {
        CMPLTSA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage ComparisonCondition Select
    #[inline(always)]
    pub fn cmploca(&self) -> CMPLOCA_R {
        CMPLOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select
    #[inline(always)]
    pub fn cmpltsa(&mut self) -> CMPLTSA_W<ADCMPLER_SPEC> {
        CMPLTSA_W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage ComparisonCondition Select
    #[inline(always)]
    pub fn cmploca(&mut self) -> CMPLOCA_W<ADCMPLER_SPEC> {
        CMPLOCA_W::new(self, 1)
    }
}
/**A/D Compare Function Window A Extended Input Comparison Condition Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPLER_SPEC;
impl crate::RegisterSpec for ADCMPLER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adcmpler::R`](R) reader structure
impl crate::Readable for ADCMPLER_SPEC {}
///`write(|w| ..)` method takes [`adcmpler::W`](W) writer structure
impl crate::Writable for ADCMPLER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPLER to value 0
impl crate::Resettable for ADCMPLER_SPEC {}
