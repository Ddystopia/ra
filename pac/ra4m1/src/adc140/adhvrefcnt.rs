///Register `ADHVREFCNT` reader
pub type R = crate::R<ADHVREFCNT_SPEC>;
///Register `ADHVREFCNT` writer
pub type W = crate::W<ADHVREFCNT_SPEC>;
/**High-Potential Reference Voltage Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HVSEL_A {
    ///0: AVCC0 is selected as the high-potential reference voltage
    _00 = 0,
    ///1: VREFH0 is selected as the high-potential reference voltage
    _01 = 1,
    ///2: Internal reference voltage is selected as the high-potential reference voltage
    _10 = 2,
    ///3: Internal node discharge. No reference voltage pin is selected.
    _11 = 3,
}
impl From<HVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HVSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for HVSEL_A {}
///Field `HVSEL` reader - High-Potential Reference Voltage Select
pub type HVSEL_R = crate::FieldReader<HVSEL_A>;
impl HVSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HVSEL_A {
        match self.bits {
            0 => HVSEL_A::_00,
            1 => HVSEL_A::_01,
            2 => HVSEL_A::_10,
            3 => HVSEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///AVCC0 is selected as the high-potential reference voltage
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HVSEL_A::_00
    }
    ///VREFH0 is selected as the high-potential reference voltage
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HVSEL_A::_01
    }
    ///Internal reference voltage is selected as the high-potential reference voltage
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HVSEL_A::_10
    }
    ///Internal node discharge. No reference voltage pin is selected.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HVSEL_A::_11
    }
}
///Field `HVSEL` writer - High-Potential Reference Voltage Select
pub type HVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HVSEL_A, crate::Safe>;
impl<'a, REG> HVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AVCC0 is selected as the high-potential reference voltage
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_00)
    }
    ///VREFH0 is selected as the high-potential reference voltage
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_01)
    }
    ///Internal reference voltage is selected as the high-potential reference voltage
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_10)
    }
    ///Internal node discharge. No reference voltage pin is selected.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(HVSEL_A::_11)
    }
}
/**Low-Potential Reference Voltage Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVSEL_A {
    ///0: AVSS0 is selected as the low-potential reference voltage
    _0 = 0,
    ///1: VREFL0 is selected as the low-potential reference voltage.
    _1 = 1,
}
impl From<LVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for LVSEL_A {}
///Field `LVSEL` reader - Low-Potential Reference Voltage Select
pub type LVSEL_R = crate::FieldReader<LVSEL_A>;
impl LVSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LVSEL_A> {
        match self.bits {
            0 => Some(LVSEL_A::_0),
            1 => Some(LVSEL_A::_1),
            _ => None,
        }
    }
    ///AVSS0 is selected as the low-potential reference voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVSEL_A::_0
    }
    ///VREFL0 is selected as the low-potential reference voltage.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVSEL_A::_1
    }
}
///Field `LVSEL` writer - Low-Potential Reference Voltage Select
pub type LVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LVSEL_A>;
impl<'a, REG> LVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AVSS0 is selected as the low-potential reference voltage
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVSEL_A::_0)
    }
    ///VREFL0 is selected as the low-potential reference voltage.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVSEL_A::_1)
    }
}
/**Sleep

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSLP_A {
    ///0: Normal operation
    _0 = 0,
    ///1: Standby state.
    _1 = 1,
}
impl From<ADSLP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSLP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSLP` reader - Sleep
pub type ADSLP_R = crate::BitReader<ADSLP_A>;
impl ADSLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSLP_A {
        match self.bits {
            false => ADSLP_A::_0,
            true => ADSLP_A::_1,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSLP_A::_0
    }
    ///Standby state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSLP_A::_1
    }
}
///Field `ADSLP` writer - Sleep
pub type ADSLP_W<'a, REG> = crate::BitWriter<'a, REG, ADSLP_A>;
impl<'a, REG> ADSLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADSLP_A::_0)
    }
    ///Standby state.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADSLP_A::_1)
    }
}
impl R {
    ///Bits 0:1 - High-Potential Reference Voltage Select
    #[inline(always)]
    pub fn hvsel(&self) -> HVSEL_R {
        HVSEL_R::new(self.bits & 3)
    }
    ///Bits 2:4 - Low-Potential Reference Voltage Select
    #[inline(always)]
    pub fn lvsel(&self) -> LVSEL_R {
        LVSEL_R::new((self.bits >> 2) & 7)
    }
    ///Bit 7 - Sleep
    #[inline(always)]
    pub fn adslp(&self) -> ADSLP_R {
        ADSLP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - High-Potential Reference Voltage Select
    #[inline(always)]
    pub fn hvsel(&mut self) -> HVSEL_W<'_, ADHVREFCNT_SPEC> {
        HVSEL_W::new(self, 0)
    }
    ///Bits 2:4 - Low-Potential Reference Voltage Select
    #[inline(always)]
    pub fn lvsel(&mut self) -> LVSEL_W<'_, ADHVREFCNT_SPEC> {
        LVSEL_W::new(self, 2)
    }
    ///Bit 7 - Sleep
    #[inline(always)]
    pub fn adslp(&mut self) -> ADSLP_W<'_, ADHVREFCNT_SPEC> {
        ADSLP_W::new(self, 7)
    }
}
/**A/D High-Potential/Low-Potential Reference Voltage Control Register

You can [`read`](crate::Reg::read) this register and get [`adhvrefcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adhvrefcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADHVREFCNT_SPEC;
impl crate::RegisterSpec for ADHVREFCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adhvrefcnt::R`](R) reader structure
impl crate::Readable for ADHVREFCNT_SPEC {}
///`write(|w| ..)` method takes [`adhvrefcnt::W`](W) writer structure
impl crate::Writable for ADHVREFCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADHVREFCNT to value 0
impl crate::Resettable for ADHVREFCNT_SPEC {}
