///Register `CTSUSO1` reader
pub type R = crate::R<CTSUSO1_SPEC>;
///Register `CTSUSO1` writer
pub type W = crate::W<CTSUSO1_SPEC>;
///Field `CTSURICOA` reader - CTSU Reference ICO Current AdjustmentCurrent offset amount is CTSUSO ( 0 to 255 )
pub type CTSURICOA_R = crate::FieldReader;
///Field `CTSURICOA` writer - CTSU Reference ICO Current AdjustmentCurrent offset amount is CTSUSO ( 0 to 255 )
pub type CTSURICOA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CTSUSDPA` reader - CTSU Base Clock SettingOperating clock divided by ( CTSUSDPA + 1 ) x 2
pub type CTSUSDPA_R = crate::FieldReader;
///Field `CTSUSDPA` writer - CTSU Base Clock SettingOperating clock divided by ( CTSUSDPA + 1 ) x 2
pub type CTSUSDPA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**CTSU ICO Gain Adjustment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUICOG_A {
    ///0: 100% gain
    _00 = 0,
    ///1: 66% gain
    _01 = 1,
    ///2: 50% gain
    _10 = 2,
    ///3: 40% gain
    _11 = 3,
}
impl From<CTSUICOG_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUICOG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUICOG_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUICOG_A {}
///Field `CTSUICOG` reader - CTSU ICO Gain Adjustment
pub type CTSUICOG_R = crate::FieldReader<CTSUICOG_A>;
impl CTSUICOG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUICOG_A {
        match self.bits {
            0 => CTSUICOG_A::_00,
            1 => CTSUICOG_A::_01,
            2 => CTSUICOG_A::_10,
            3 => CTSUICOG_A::_11,
            _ => unreachable!(),
        }
    }
    ///100% gain
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUICOG_A::_00
    }
    ///66% gain
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUICOG_A::_01
    }
    ///50% gain
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUICOG_A::_10
    }
    ///40% gain
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUICOG_A::_11
    }
}
///Field `CTSUICOG` writer - CTSU ICO Gain Adjustment
pub type CTSUICOG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CTSUICOG_A, crate::Safe>;
impl<'a, REG> CTSUICOG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///100% gain
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_00)
    }
    ///66% gain
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_01)
    }
    ///50% gain
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_10)
    }
    ///40% gain
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUICOG_A::_11)
    }
}
impl R {
    ///Bits 0:7 - CTSU Reference ICO Current AdjustmentCurrent offset amount is CTSUSO ( 0 to 255 )
    #[inline(always)]
    pub fn ctsuricoa(&self) -> CTSURICOA_R {
        CTSURICOA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:12 - CTSU Base Clock SettingOperating clock divided by ( CTSUSDPA + 1 ) x 2
    #[inline(always)]
    pub fn ctsusdpa(&self) -> CTSUSDPA_R {
        CTSUSDPA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:14 - CTSU ICO Gain Adjustment
    #[inline(always)]
    pub fn ctsuicog(&self) -> CTSUICOG_R {
        CTSUICOG_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    ///Bits 0:7 - CTSU Reference ICO Current AdjustmentCurrent offset amount is CTSUSO ( 0 to 255 )
    #[inline(always)]
    pub fn ctsuricoa(&mut self) -> CTSURICOA_W<CTSUSO1_SPEC> {
        CTSURICOA_W::new(self, 0)
    }
    ///Bits 8:12 - CTSU Base Clock SettingOperating clock divided by ( CTSUSDPA + 1 ) x 2
    #[inline(always)]
    pub fn ctsusdpa(&mut self) -> CTSUSDPA_W<CTSUSO1_SPEC> {
        CTSUSDPA_W::new(self, 8)
    }
    ///Bits 13:14 - CTSU ICO Gain Adjustment
    #[inline(always)]
    pub fn ctsuicog(&mut self) -> CTSUICOG_W<CTSUSO1_SPEC> {
        CTSUICOG_W::new(self, 13)
    }
}
/**CTSU Sensor Offset Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuso1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUSO1_SPEC;
impl crate::RegisterSpec for CTSUSO1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctsuso1::R`](R) reader structure
impl crate::Readable for CTSUSO1_SPEC {}
///`write(|w| ..)` method takes [`ctsuso1::W`](W) writer structure
impl crate::Writable for CTSUSO1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSO1 to value 0
impl crate::Resettable for CTSUSO1_SPEC {}
