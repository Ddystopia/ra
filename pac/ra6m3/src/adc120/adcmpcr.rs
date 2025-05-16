///Register `ADCMPCR` reader
pub type R = crate::R<ADCMPCR_SPEC>;
///Register `ADCMPCR` writer
pub type W = crate::W<ADCMPCR_SPEC>;
/**Window A/B Composite Conditions SettingNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPAB_A {
    ///0: S12ADWMELC is output when window A comparison conditions are met OR window B comparison conditions are met. S12ADWUMELC is output in other cases.
    _00 = 0,
    ///1: S12ADWMELC is output when window A comparison conditions are met EXOR window B comparison conditions are met. S12ADWUMELC is output in other cases.
    _01 = 1,
    ///2: S12ADWMELC is output when window A comparison conditions are met and window B comparison conditions are met. S12ADWUMELC is output in other cases.
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<CMPAB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPAB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPAB_A {
    type Ux = u8;
}
impl crate::IsEnum for CMPAB_A {}
///Field `CMPAB` reader - Window A/B Composite Conditions SettingNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).
pub type CMPAB_R = crate::FieldReader<CMPAB_A>;
impl CMPAB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPAB_A {
        match self.bits {
            0 => CMPAB_A::_00,
            1 => CMPAB_A::_01,
            2 => CMPAB_A::_10,
            3 => CMPAB_A::_11,
            _ => unreachable!(),
        }
    }
    ///S12ADWMELC is output when window A comparison conditions are met OR window B comparison conditions are met. S12ADWUMELC is output in other cases.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMPAB_A::_00
    }
    ///S12ADWMELC is output when window A comparison conditions are met EXOR window B comparison conditions are met. S12ADWUMELC is output in other cases.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMPAB_A::_01
    }
    ///S12ADWMELC is output when window A comparison conditions are met and window B comparison conditions are met. S12ADWUMELC is output in other cases.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMPAB_A::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMPAB_A::_11
    }
}
///Field `CMPAB` writer - Window A/B Composite Conditions SettingNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).
pub type CMPAB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CMPAB_A, crate::Safe>;
impl<'a, REG> CMPAB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///S12ADWMELC is output when window A comparison conditions are met OR window B comparison conditions are met. S12ADWUMELC is output in other cases.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAB_A::_00)
    }
    ///S12ADWMELC is output when window A comparison conditions are met EXOR window B comparison conditions are met. S12ADWUMELC is output in other cases.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAB_A::_01)
    }
    ///S12ADWMELC is output when window A comparison conditions are met and window B comparison conditions are met. S12ADWUMELC is output in other cases.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAB_A::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAB_A::_11)
    }
}
/**Compare Window B Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPBE_A {
    ///0: Compare window B operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled.
    _0 = 0,
    ///1: Compare window B operation is enabled.
    _1 = 1,
}
impl From<CMPBE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPBE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPBE` reader - Compare Window B Operation Enable
pub type CMPBE_R = crate::BitReader<CMPBE_A>;
impl CMPBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPBE_A {
        match self.bits {
            false => CMPBE_A::_0,
            true => CMPBE_A::_1,
        }
    }
    ///Compare window B operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPBE_A::_0
    }
    ///Compare window B operation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPBE_A::_1
    }
}
///Field `CMPBE` writer - Compare Window B Operation Enable
pub type CMPBE_W<'a, REG> = crate::BitWriter<'a, REG, CMPBE_A>;
impl<'a, REG> CMPBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare window B operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPBE_A::_0)
    }
    ///Compare window B operation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPBE_A::_1)
    }
}
/**Compare Window A Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPAE_A {
    ///0: Compare window A operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled.
    _0 = 0,
    ///1: Compare window A operation is enabled.
    _1 = 1,
}
impl From<CMPAE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPAE` reader - Compare Window A Operation Enable
pub type CMPAE_R = crate::BitReader<CMPAE_A>;
impl CMPAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPAE_A {
        match self.bits {
            false => CMPAE_A::_0,
            true => CMPAE_A::_1,
        }
    }
    ///Compare window A operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPAE_A::_0
    }
    ///Compare window A operation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPAE_A::_1
    }
}
///Field `CMPAE` writer - Compare Window A Operation Enable
pub type CMPAE_W<'a, REG> = crate::BitWriter<'a, REG, CMPAE_A>;
impl<'a, REG> CMPAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare window A operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAE_A::_0)
    }
    ///Compare window A operation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAE_A::_1)
    }
}
/**Compare B Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPBIE_A {
    ///0: S12ADCMPBIi interrupt is disabled when comparison conditions (window B) are met.
    _0 = 0,
    ///1: S12ADCMPBIi interrupt is enabled when comparison conditions (window B) are met.
    _1 = 1,
}
impl From<CMPBIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPBIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPBIE` reader - Compare B Interrupt Enable
pub type CMPBIE_R = crate::BitReader<CMPBIE_A>;
impl CMPBIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPBIE_A {
        match self.bits {
            false => CMPBIE_A::_0,
            true => CMPBIE_A::_1,
        }
    }
    ///S12ADCMPBIi interrupt is disabled when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPBIE_A::_0
    }
    ///S12ADCMPBIi interrupt is enabled when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPBIE_A::_1
    }
}
///Field `CMPBIE` writer - Compare B Interrupt Enable
pub type CMPBIE_W<'a, REG> = crate::BitWriter<'a, REG, CMPBIE_A>;
impl<'a, REG> CMPBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S12ADCMPBIi interrupt is disabled when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPBIE_A::_0)
    }
    ///S12ADCMPBIi interrupt is enabled when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPBIE_A::_1)
    }
}
/**Window Function Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCMPE_A {
    ///0: Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result.
    _0 = 0,
    ///1: Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result.
    _1 = 1,
}
impl From<WCMPE_A> for bool {
    #[inline(always)]
    fn from(variant: WCMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WCMPE` reader - Window Function Setting
pub type WCMPE_R = crate::BitReader<WCMPE_A>;
impl WCMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WCMPE_A {
        match self.bits {
            false => WCMPE_A::_0,
            true => WCMPE_A::_1,
        }
    }
    ///Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WCMPE_A::_0
    }
    ///Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WCMPE_A::_1
    }
}
///Field `WCMPE` writer - Window Function Setting
pub type WCMPE_W<'a, REG> = crate::BitWriter<'a, REG, WCMPE_A>;
impl<'a, REG> WCMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WCMPE_A::_0)
    }
    ///Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WCMPE_A::_1)
    }
}
/**Compare A Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPAIE_A {
    ///0: S12ADCMPAIi interrupt is disabled when comparison conditions (window A) are met.
    _0 = 0,
    ///1: S12ADCMPAIi interrupt is enabled when comparison conditions (window A) are met.
    _1 = 1,
}
impl From<CMPAIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPAIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPAIE` reader - Compare A Interrupt Enable
pub type CMPAIE_R = crate::BitReader<CMPAIE_A>;
impl CMPAIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPAIE_A {
        match self.bits {
            false => CMPAIE_A::_0,
            true => CMPAIE_A::_1,
        }
    }
    ///S12ADCMPAIi interrupt is disabled when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPAIE_A::_0
    }
    ///S12ADCMPAIi interrupt is enabled when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPAIE_A::_1
    }
}
///Field `CMPAIE` writer - Compare A Interrupt Enable
pub type CMPAIE_W<'a, REG> = crate::BitWriter<'a, REG, CMPAIE_A>;
impl<'a, REG> CMPAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S12ADCMPAIi interrupt is disabled when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAIE_A::_0)
    }
    ///S12ADCMPAIi interrupt is enabled when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPAIE_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Window A/B Composite Conditions SettingNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).
    #[inline(always)]
    pub fn cmpab(&self) -> CMPAB_R {
        CMPAB_R::new((self.bits & 3) as u8)
    }
    ///Bit 9 - Compare Window B Operation Enable
    #[inline(always)]
    pub fn cmpbe(&self) -> CMPBE_R {
        CMPBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Compare Window A Operation Enable
    #[inline(always)]
    pub fn cmpae(&self) -> CMPAE_R {
        CMPAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Compare B Interrupt Enable
    #[inline(always)]
    pub fn cmpbie(&self) -> CMPBIE_R {
        CMPBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Window Function Setting
    #[inline(always)]
    pub fn wcmpe(&self) -> WCMPE_R {
        WCMPE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Compare A Interrupt Enable
    #[inline(always)]
    pub fn cmpaie(&self) -> CMPAIE_R {
        CMPAIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Window A/B Composite Conditions SettingNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1).
    #[inline(always)]
    pub fn cmpab(&mut self) -> CMPAB_W<ADCMPCR_SPEC> {
        CMPAB_W::new(self, 0)
    }
    ///Bit 9 - Compare Window B Operation Enable
    #[inline(always)]
    pub fn cmpbe(&mut self) -> CMPBE_W<ADCMPCR_SPEC> {
        CMPBE_W::new(self, 9)
    }
    ///Bit 11 - Compare Window A Operation Enable
    #[inline(always)]
    pub fn cmpae(&mut self) -> CMPAE_W<ADCMPCR_SPEC> {
        CMPAE_W::new(self, 11)
    }
    ///Bit 13 - Compare B Interrupt Enable
    #[inline(always)]
    pub fn cmpbie(&mut self) -> CMPBIE_W<ADCMPCR_SPEC> {
        CMPBIE_W::new(self, 13)
    }
    ///Bit 14 - Window Function Setting
    #[inline(always)]
    pub fn wcmpe(&mut self) -> WCMPE_W<ADCMPCR_SPEC> {
        WCMPE_W::new(self, 14)
    }
    ///Bit 15 - Compare A Interrupt Enable
    #[inline(always)]
    pub fn cmpaie(&mut self) -> CMPAIE_W<ADCMPCR_SPEC> {
        CMPAIE_W::new(self, 15)
    }
}
/**A/D Compare Function Control Register

You can [`read`](crate::Reg::read) this register and get [`adcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPCR_SPEC;
impl crate::RegisterSpec for ADCMPCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpcr::R`](R) reader structure
impl crate::Readable for ADCMPCR_SPEC {}
///`write(|w| ..)` method takes [`adcmpcr::W`](W) writer structure
impl crate::Writable for ADCMPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPCR to value 0
impl crate::Resettable for ADCMPCR_SPEC {}
