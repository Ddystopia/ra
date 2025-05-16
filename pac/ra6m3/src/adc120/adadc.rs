///Register `ADADC` reader
pub type R = crate::R<ADADC_SPEC>;
///Register `ADADC` writer
pub type W = crate::W<ADADC_SPEC>;
/**Addition frequency selection bit.NOTE: AVEE bit is valid at the only setting of ADC\[2:0\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\[2:0\] = 010b)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_A {
    ///0: 1-time conversion (no addition; same as normal conversion)
    _000 = 0,
    ///1: 2-time conversion (addition once)
    _001 = 1,
    ///2: 3-time conversion (addition twice)
    _010 = 2,
    ///3: 4-time conversion (addition three times)
    _011 = 3,
    ///5: 16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy.
    _101 = 5,
    ///4: Setting prohibited
    OTHERS = 4,
}
impl From<ADC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_A {
    type Ux = u8;
}
impl crate::IsEnum for ADC_A {}
///Field `ADC` reader - Addition frequency selection bit.NOTE: AVEE bit is valid at the only setting of ADC\[2:0\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\[2:0\] = 010b)
pub type ADC_R = crate::FieldReader<ADC_A>;
impl ADC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC_A {
        match self.bits {
            0 => ADC_A::_000,
            1 => ADC_A::_001,
            2 => ADC_A::_010,
            3 => ADC_A::_011,
            5 => ADC_A::_101,
            _ => ADC_A::OTHERS,
        }
    }
    ///1-time conversion (no addition; same as normal conversion)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADC_A::_000
    }
    ///2-time conversion (addition once)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ADC_A::_001
    }
    ///3-time conversion (addition twice)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ADC_A::_010
    }
    ///4-time conversion (addition three times)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ADC_A::_011
    }
    ///16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy.
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ADC_A::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), ADC_A::OTHERS)
    }
}
///Field `ADC` writer - Addition frequency selection bit.NOTE: AVEE bit is valid at the only setting of ADC\[2:0\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\[2:0\] = 010b)
pub type ADC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADC_A, crate::Safe>;
impl<'a, REG> ADC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1-time conversion (no addition; same as normal conversion)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_A::_000)
    }
    ///2-time conversion (addition once)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_A::_001)
    }
    ///3-time conversion (addition twice)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_A::_010)
    }
    ///4-time conversion (addition three times)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_A::_011)
    }
    ///16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy.
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_A::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_A::OTHERS)
    }
}
/**Average mode enable bit.Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVEE_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<AVEE_A> for bool {
    #[inline(always)]
    fn from(variant: AVEE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AVEE` reader - Average mode enable bit.Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode.
pub type AVEE_R = crate::BitReader<AVEE_A>;
impl AVEE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AVEE_A {
        match self.bits {
            false => AVEE_A::_0,
            true => AVEE_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVEE_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVEE_A::_1
    }
}
///Field `AVEE` writer - Average mode enable bit.Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode.
pub type AVEE_W<'a, REG> = crate::BitWriter<'a, REG, AVEE_A>;
impl<'a, REG> AVEE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AVEE_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AVEE_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Addition frequency selection bit.NOTE: AVEE bit is valid at the only setting of ADC\[2:0\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\[2:0\] = 010b)
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(self.bits & 7)
    }
    ///Bit 7 - Average mode enable bit.Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode.
    #[inline(always)]
    pub fn avee(&self) -> AVEE_R {
        AVEE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Addition frequency selection bit.NOTE: AVEE bit is valid at the only setting of ADC\[2:0\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\[2:0\] = 010b)
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W<ADADC_SPEC> {
        ADC_W::new(self, 0)
    }
    ///Bit 7 - Average mode enable bit.Note: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode.
    #[inline(always)]
    pub fn avee(&mut self) -> AVEE_W<ADADC_SPEC> {
        AVEE_W::new(self, 7)
    }
}
/**A/D-Converted Value Addition/Average Count Select Register

You can [`read`](crate::Reg::read) this register and get [`adadc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adadc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADADC_SPEC;
impl crate::RegisterSpec for ADADC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adadc::R`](R) reader structure
impl crate::Readable for ADADC_SPEC {}
///`write(|w| ..)` method takes [`adadc::W`](W) writer structure
impl crate::Writable for ADADC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADADC to value 0
impl crate::Resettable for ADADC_SPEC {}
