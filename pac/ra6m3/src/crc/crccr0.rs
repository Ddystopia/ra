///Register `CRCCR0` reader
pub type R = crate::R<CRCCR0_SPEC>;
///Register `CRCCR0` writer
pub type W = crate::W<CRCCR0_SPEC>;
/**CRC Generating Polynomial Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPS_A {
    ///0: No calculation is executed.
    _000 = 0,
    ///1: 8-bit CRC-8 (X8 + X2 + X + 1)
    _001 = 1,
    ///2: 16-bit CRC-16 (X16 + X15 + X2 + 1)
    _010 = 2,
    ///3: 16-bit CRC-CCITT (X16 + X12 + X5 + 1)
    _011 = 3,
    ///4: 32-bit CRC-32 (X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)
    _100 = 4,
    ///5: 32-bit CRC-32C (X32+X28+X27+X26+ X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)
    _101 = 5,
    ///6: No calculation is executed.
    OTHERS = 6,
}
impl From<GPS_A> for u8 {
    #[inline(always)]
    fn from(variant: GPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPS_A {
    type Ux = u8;
}
impl crate::IsEnum for GPS_A {}
///Field `GPS` reader - CRC Generating Polynomial Switching
pub type GPS_R = crate::FieldReader<GPS_A>;
impl GPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPS_A {
        match self.bits {
            0 => GPS_A::_000,
            1 => GPS_A::_001,
            2 => GPS_A::_010,
            3 => GPS_A::_011,
            4 => GPS_A::_100,
            5 => GPS_A::_101,
            _ => GPS_A::OTHERS,
        }
    }
    ///No calculation is executed.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == GPS_A::_000
    }
    ///8-bit CRC-8 (X8 + X2 + X + 1)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == GPS_A::_001
    }
    ///16-bit CRC-16 (X16 + X15 + X2 + 1)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == GPS_A::_010
    }
    ///16-bit CRC-CCITT (X16 + X12 + X5 + 1)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == GPS_A::_011
    }
    ///32-bit CRC-32 (X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == GPS_A::_100
    }
    ///32-bit CRC-32C (X32+X28+X27+X26+ X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == GPS_A::_101
    }
    ///No calculation is executed.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), GPS_A::OTHERS)
    }
}
///Field `GPS` writer - CRC Generating Polynomial Switching
pub type GPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, GPS_A, crate::Safe>;
impl<'a, REG> GPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No calculation is executed.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::_000)
    }
    ///8-bit CRC-8 (X8 + X2 + X + 1)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::_001)
    }
    ///16-bit CRC-16 (X16 + X15 + X2 + 1)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::_010)
    }
    ///16-bit CRC-CCITT (X16 + X12 + X5 + 1)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::_011)
    }
    ///32-bit CRC-32 (X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::_100)
    }
    ///32-bit CRC-32C (X32+X28+X27+X26+ X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::_101)
    }
    ///No calculation is executed.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(GPS_A::OTHERS)
    }
}
/**CRC Calculation Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMS_A {
    ///0: Generates CRC for LSB first communication.
    _0 = 0,
    ///1: Generates CRC for MSB first communication.
    _1 = 1,
}
impl From<LMS_A> for bool {
    #[inline(always)]
    fn from(variant: LMS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LMS` reader - CRC Calculation Switching
pub type LMS_R = crate::BitReader<LMS_A>;
impl LMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LMS_A {
        match self.bits {
            false => LMS_A::_0,
            true => LMS_A::_1,
        }
    }
    ///Generates CRC for LSB first communication.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LMS_A::_0
    }
    ///Generates CRC for MSB first communication.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LMS_A::_1
    }
}
///Field `LMS` writer - CRC Calculation Switching
pub type LMS_W<'a, REG> = crate::BitWriter<'a, REG, LMS_A>;
impl<'a, REG> LMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates CRC for LSB first communication.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LMS_A::_0)
    }
    ///Generates CRC for MSB first communication.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LMS_A::_1)
    }
}
/**CRCDOR Register Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clears the CRCDOR register.
    _1 = 1,
}
impl From<DORCLR_A> for bool {
    #[inline(always)]
    fn from(variant: DORCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DORCLR` writer - CRCDOR Register Clear
pub type DORCLR_W<'a, REG> = crate::BitWriter<'a, REG, DORCLR_A>;
impl<'a, REG> DORCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DORCLR_A::_0)
    }
    ///Clears the CRCDOR register.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DORCLR_A::_1)
    }
}
impl R {
    ///Bits 0:2 - CRC Generating Polynomial Switching
    #[inline(always)]
    pub fn gps(&self) -> GPS_R {
        GPS_R::new(self.bits & 7)
    }
    ///Bit 6 - CRC Calculation Switching
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - CRC Generating Polynomial Switching
    #[inline(always)]
    pub fn gps(&mut self) -> GPS_W<CRCCR0_SPEC> {
        GPS_W::new(self, 0)
    }
    ///Bit 6 - CRC Calculation Switching
    #[inline(always)]
    pub fn lms(&mut self) -> LMS_W<CRCCR0_SPEC> {
        LMS_W::new(self, 6)
    }
    ///Bit 7 - CRCDOR Register Clear
    #[inline(always)]
    pub fn dorclr(&mut self) -> DORCLR_W<CRCCR0_SPEC> {
        DORCLR_W::new(self, 7)
    }
}
/**CRC Control Register0

You can [`read`](crate::Reg::read) this register and get [`crccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCCR0_SPEC;
impl crate::RegisterSpec for CRCCR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`crccr0::R`](R) reader structure
impl crate::Readable for CRCCR0_SPEC {}
///`write(|w| ..)` method takes [`crccr0::W`](W) writer structure
impl crate::Writable for CRCCR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCCR0 to value 0
impl crate::Resettable for CRCCR0_SPEC {}
