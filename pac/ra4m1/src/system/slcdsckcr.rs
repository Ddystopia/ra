///Register `SLCDSCKCR` reader
pub type R = crate::R<SLCDSCKCR_SPEC>;
///Register `SLCDSCKCR` writer
pub type W = crate::W<SLCDSCKCR_SPEC>;
/**LCD Source Clock (LCDSRCCLK) Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDSCKSEL_A {
    ///0: LOCO
    _000 = 0,
    ///1: SOSC
    _001 = 1,
    ///2: MOSC
    _010 = 2,
    ///4: HOCO
    _100 = 4,
    ///3: Settings other than above are prohibited.
    OTHERS = 3,
}
impl From<LCDSCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDSCKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LCDSCKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for LCDSCKSEL_A {}
///Field `LCDSCKSEL` reader - LCD Source Clock (LCDSRCCLK) Select
pub type LCDSCKSEL_R = crate::FieldReader<LCDSCKSEL_A>;
impl LCDSCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCDSCKSEL_A {
        match self.bits {
            0 => LCDSCKSEL_A::_000,
            1 => LCDSCKSEL_A::_001,
            2 => LCDSCKSEL_A::_010,
            4 => LCDSCKSEL_A::_100,
            _ => LCDSCKSEL_A::OTHERS,
        }
    }
    ///LOCO
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LCDSCKSEL_A::_000
    }
    ///SOSC
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LCDSCKSEL_A::_001
    }
    ///MOSC
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LCDSCKSEL_A::_010
    }
    ///HOCO
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == LCDSCKSEL_A::_100
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LCDSCKSEL_A::OTHERS)
    }
}
///Field `LCDSCKSEL` writer - LCD Source Clock (LCDSRCCLK) Select
pub type LCDSCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LCDSCKSEL_A, crate::Safe>;
impl<'a, REG> LCDSCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LOCO
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_000)
    }
    ///SOSC
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_001)
    }
    ///MOSC
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_010)
    }
    ///HOCO
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::_100)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKSEL_A::OTHERS)
    }
}
/**LCD Source Clock Out Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDSCKEN_A {
    ///0: LCD source clock out disabled
    _0 = 0,
    ///1: LCD source clock out enabled.
    _1 = 1,
}
impl From<LCDSCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCDSCKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCDSCKEN` reader - LCD Source Clock Out Enable
pub type LCDSCKEN_R = crate::BitReader<LCDSCKEN_A>;
impl LCDSCKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCDSCKEN_A {
        match self.bits {
            false => LCDSCKEN_A::_0,
            true => LCDSCKEN_A::_1,
        }
    }
    ///LCD source clock out disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDSCKEN_A::_0
    }
    ///LCD source clock out enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDSCKEN_A::_1
    }
}
///Field `LCDSCKEN` writer - LCD Source Clock Out Enable
pub type LCDSCKEN_W<'a, REG> = crate::BitWriter<'a, REG, LCDSCKEN_A>;
impl<'a, REG> LCDSCKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LCD source clock out disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKEN_A::_0)
    }
    ///LCD source clock out enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSCKEN_A::_1)
    }
}
impl R {
    ///Bits 0:2 - LCD Source Clock (LCDSRCCLK) Select
    #[inline(always)]
    pub fn lcdscksel(&self) -> LCDSCKSEL_R {
        LCDSCKSEL_R::new(self.bits & 7)
    }
    ///Bit 7 - LCD Source Clock Out Enable
    #[inline(always)]
    pub fn lcdscken(&self) -> LCDSCKEN_R {
        LCDSCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - LCD Source Clock (LCDSRCCLK) Select
    #[inline(always)]
    pub fn lcdscksel(&mut self) -> LCDSCKSEL_W<'_, SLCDSCKCR_SPEC> {
        LCDSCKSEL_W::new(self, 0)
    }
    ///Bit 7 - LCD Source Clock Out Enable
    #[inline(always)]
    pub fn lcdscken(&mut self) -> LCDSCKEN_W<'_, SLCDSCKCR_SPEC> {
        LCDSCKEN_W::new(self, 7)
    }
}
/**Segment LCD Source Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`slcdsckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slcdsckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLCDSCKCR_SPEC;
impl crate::RegisterSpec for SLCDSCKCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`slcdsckcr::R`](R) reader structure
impl crate::Readable for SLCDSCKCR_SPEC {}
///`write(|w| ..)` method takes [`slcdsckcr::W`](W) writer structure
impl crate::Writable for SLCDSCKCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SLCDSCKCR to value 0
impl crate::Resettable for SLCDSCKCR_SPEC {}
