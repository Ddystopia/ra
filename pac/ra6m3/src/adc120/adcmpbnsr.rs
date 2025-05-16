///Register `ADCMPBNSR` reader
pub type R = crate::R<ADCMPBNSR_SPEC>;
///Register `ADCMPBNSR` writer
pub type W = crate::W<ADCMPBNSR_SPEC>;
/**Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPCHB_A {
    ///0: AN000
    _0X00 = 0,
    ///1: AN001
    _0X01 = 1,
    ///2: AN002
    _0X02 = 2,
    ///3: AN003
    _0X03 = 3,
    ///4: AN004
    _0X04 = 4,
    ///5: AN005
    _0X05 = 5,
    ///6: AN006
    _0X06 = 6,
    ///7: AN007
    _0X07 = 7,
    ///16: AN016
    _0X10 = 16,
    ///17: AN017
    _0X11 = 17,
    ///18: AN018
    _0X12 = 18,
    ///19: AN019
    _0X13 = 19,
    ///20: AN020
    _0X14 = 20,
    ///32: Temperature sensor
    _0X20 = 32,
    ///33: Internal reference voltage
    _0X21 = 33,
    ///63: No channel is selected
    _0X3F = 63,
    ///8: Setting prohibited
    OTHERS = 8,
}
impl From<CMPCHB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPCHB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPCHB_A {
    type Ux = u8;
}
impl crate::IsEnum for CMPCHB_A {}
///Field `CMPCHB` reader - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected.
pub type CMPCHB_R = crate::FieldReader<CMPCHB_A>;
impl CMPCHB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHB_A {
        match self.bits {
            0 => CMPCHB_A::_0X00,
            1 => CMPCHB_A::_0X01,
            2 => CMPCHB_A::_0X02,
            3 => CMPCHB_A::_0X03,
            4 => CMPCHB_A::_0X04,
            5 => CMPCHB_A::_0X05,
            6 => CMPCHB_A::_0X06,
            7 => CMPCHB_A::_0X07,
            16 => CMPCHB_A::_0X10,
            17 => CMPCHB_A::_0X11,
            18 => CMPCHB_A::_0X12,
            19 => CMPCHB_A::_0X13,
            20 => CMPCHB_A::_0X14,
            32 => CMPCHB_A::_0X20,
            33 => CMPCHB_A::_0X21,
            63 => CMPCHB_A::_0X3F,
            _ => CMPCHB_A::OTHERS,
        }
    }
    ///AN000
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CMPCHB_A::_0X00
    }
    ///AN001
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == CMPCHB_A::_0X01
    }
    ///AN002
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == CMPCHB_A::_0X02
    }
    ///AN003
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == CMPCHB_A::_0X03
    }
    ///AN004
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == CMPCHB_A::_0X04
    }
    ///AN005
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == CMPCHB_A::_0X05
    }
    ///AN006
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == CMPCHB_A::_0X06
    }
    ///AN007
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == CMPCHB_A::_0X07
    }
    ///AN016
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == CMPCHB_A::_0X10
    }
    ///AN017
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == CMPCHB_A::_0X11
    }
    ///AN018
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == CMPCHB_A::_0X12
    }
    ///AN019
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == CMPCHB_A::_0X13
    }
    ///AN020
    #[inline(always)]
    pub fn is_0x14(&self) -> bool {
        *self == CMPCHB_A::_0X14
    }
    ///Temperature sensor
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == CMPCHB_A::_0X20
    }
    ///Internal reference voltage
    #[inline(always)]
    pub fn is_0x21(&self) -> bool {
        *self == CMPCHB_A::_0X21
    }
    ///No channel is selected
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == CMPCHB_A::_0X3F
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CMPCHB_A::OTHERS)
    }
}
///Field `CMPCHB` writer - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected.
pub type CMPCHB_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CMPCHB_A, crate::Safe>;
impl<'a, REG> CMPCHB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AN000
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X00)
    }
    ///AN001
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X01)
    }
    ///AN002
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X02)
    }
    ///AN003
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X03)
    }
    ///AN004
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X04)
    }
    ///AN005
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X05)
    }
    ///AN006
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X06)
    }
    ///AN007
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X07)
    }
    ///AN016
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X10)
    }
    ///AN017
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X11)
    }
    ///AN018
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X12)
    }
    ///AN019
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X13)
    }
    ///AN020
    #[inline(always)]
    pub fn _0x14(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X14)
    }
    ///Temperature sensor
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X20)
    }
    ///Internal reference voltage
    #[inline(always)]
    pub fn _0x21(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X21)
    }
    ///No channel is selected
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::_0X3F)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHB_A::OTHERS)
    }
}
/**Compare window B Compare condition setting bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLB_A {
    ///0: CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)
    _1 = 1,
}
impl From<CMPLB_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLB` reader - Compare window B Compare condition setting bit.
pub type CMPLB_R = crate::BitReader<CMPLB_A>;
impl CMPLB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLB_A {
        match self.bits {
            false => CMPLB_A::_0,
            true => CMPLB_A::_1,
        }
    }
    ///CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLB_A::_0
    }
    ///CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLB_A::_1
    }
}
///Field `CMPLB` writer - Compare window B Compare condition setting bit.
pub type CMPLB_W<'a, REG> = crate::BitWriter<'a, REG, CMPLB_A>;
impl<'a, REG> CMPLB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLB_A::_0)
    }
    ///CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLB_A::_1)
    }
}
impl R {
    ///Bits 0:5 - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected.
    #[inline(always)]
    pub fn cmpchb(&self) -> CMPCHB_R {
        CMPCHB_R::new(self.bits & 0x3f)
    }
    ///Bit 7 - Compare window B Compare condition setting bit.
    #[inline(always)]
    pub fn cmplb(&self) -> CMPLB_R {
        CMPLB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected.
    #[inline(always)]
    pub fn cmpchb(&mut self) -> CMPCHB_W<ADCMPBNSR_SPEC> {
        CMPCHB_W::new(self, 0)
    }
    ///Bit 7 - Compare window B Compare condition setting bit.
    #[inline(always)]
    pub fn cmplb(&mut self) -> CMPLB_W<ADCMPBNSR_SPEC> {
        CMPLB_W::new(self, 7)
    }
}
/**A/D Compare Function Window B Channel Selection Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbnsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbnsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPBNSR_SPEC;
impl crate::RegisterSpec for ADCMPBNSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adcmpbnsr::R`](R) reader structure
impl crate::Readable for ADCMPBNSR_SPEC {}
///`write(|w| ..)` method takes [`adcmpbnsr::W`](W) writer structure
impl crate::Writable for ADCMPBNSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPBNSR to value 0
impl crate::Resettable for ADCMPBNSR_SPEC {}
