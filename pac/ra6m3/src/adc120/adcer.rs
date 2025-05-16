///Register `ADCER` reader
pub type R = crate::R<ADCER_SPEC>;
///Register `ADCER` writer
pub type W = crate::W<ADCER_SPEC>;
/**A/D Conversion Accuracy Specify

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADPRC_A {
    ///0: A/D conversion is performed with 12-bit accuracy.
    _00 = 0,
    ///1: A/D conversion is performed with 10-bit accuracy.
    _01 = 1,
    ///2: A/D conversion is performed with 8-bit accuracy.
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<ADPRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADPRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADPRC_A {
    type Ux = u8;
}
impl crate::IsEnum for ADPRC_A {}
///Field `ADPRC` reader - A/D Conversion Accuracy Specify
pub type ADPRC_R = crate::FieldReader<ADPRC_A>;
impl ADPRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADPRC_A {
        match self.bits {
            0 => ADPRC_A::_00,
            1 => ADPRC_A::_01,
            2 => ADPRC_A::_10,
            3 => ADPRC_A::_11,
            _ => unreachable!(),
        }
    }
    ///A/D conversion is performed with 12-bit accuracy.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADPRC_A::_00
    }
    ///A/D conversion is performed with 10-bit accuracy.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADPRC_A::_01
    }
    ///A/D conversion is performed with 8-bit accuracy.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADPRC_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADPRC_A::_11
    }
}
///Field `ADPRC` writer - A/D Conversion Accuracy Specify
pub type ADPRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADPRC_A, crate::Safe>;
impl<'a, REG> ADPRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A/D conversion is performed with 12-bit accuracy.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ADPRC_A::_00)
    }
    ///A/D conversion is performed with 10-bit accuracy.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ADPRC_A::_01)
    }
    ///A/D conversion is performed with 8-bit accuracy.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(ADPRC_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ADPRC_A::_11)
    }
}
/**A/D Data Register Automatic Clearing Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACE_A {
    ///0: Disables automatic clearing.
    _0 = 0,
    ///1: Enables automatic clearing.
    _1 = 1,
}
impl From<ACE_A> for bool {
    #[inline(always)]
    fn from(variant: ACE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACE` reader - A/D Data Register Automatic Clearing Enable
pub type ACE_R = crate::BitReader<ACE_A>;
impl ACE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACE_A {
        match self.bits {
            false => ACE_A::_0,
            true => ACE_A::_1,
        }
    }
    ///Disables automatic clearing.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACE_A::_0
    }
    ///Enables automatic clearing.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACE_A::_1
    }
}
///Field `ACE` writer - A/D Data Register Automatic Clearing Enable
pub type ACE_W<'a, REG> = crate::BitWriter<'a, REG, ACE_A>;
impl<'a, REG> ACE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables automatic clearing.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACE_A::_0)
    }
    ///Enables automatic clearing.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACE_A::_1)
    }
}
/**Self-Diagnosis Conversion Voltage Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGVAL_A {
    ///0: When the self-diagnosis fixation mode is selected, it set prohibits it.
    _00 = 0,
    ///1: The self-diagnosis by using the voltage of 0V.
    _01 = 1,
    ///2: The self-diagnosis by using the voltage of reference supply x 1/2.
    _10 = 2,
    ///3: The self-diagnosis by using the voltage of the reference supply.
    _11 = 3,
}
impl From<DIAGVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGVAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIAGVAL_A {
    type Ux = u8;
}
impl crate::IsEnum for DIAGVAL_A {}
///Field `DIAGVAL` reader - Self-Diagnosis Conversion Voltage Select
pub type DIAGVAL_R = crate::FieldReader<DIAGVAL_A>;
impl DIAGVAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIAGVAL_A {
        match self.bits {
            0 => DIAGVAL_A::_00,
            1 => DIAGVAL_A::_01,
            2 => DIAGVAL_A::_10,
            3 => DIAGVAL_A::_11,
            _ => unreachable!(),
        }
    }
    ///When the self-diagnosis fixation mode is selected, it set prohibits it.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGVAL_A::_00
    }
    ///The self-diagnosis by using the voltage of 0V.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGVAL_A::_01
    }
    ///The self-diagnosis by using the voltage of reference supply x 1/2.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGVAL_A::_10
    }
    ///The self-diagnosis by using the voltage of the reference supply.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGVAL_A::_11
    }
}
///Field `DIAGVAL` writer - Self-Diagnosis Conversion Voltage Select
pub type DIAGVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DIAGVAL_A, crate::Safe>;
impl<'a, REG> DIAGVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///When the self-diagnosis fixation mode is selected, it set prohibits it.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_00)
    }
    ///The self-diagnosis by using the voltage of 0V.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_01)
    }
    ///The self-diagnosis by using the voltage of reference supply x 1/2.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_10)
    }
    ///The self-diagnosis by using the voltage of the reference supply.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGVAL_A::_11)
    }
}
/**Self-Diagnosis Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIAGLD_A {
    ///0: Rotation mode for self-diagnosis voltage
    _0 = 0,
    ///1: Fixed mode for self-diagnosis voltage
    _1 = 1,
}
impl From<DIAGLD_A> for bool {
    #[inline(always)]
    fn from(variant: DIAGLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIAGLD` reader - Self-Diagnosis Mode Select
pub type DIAGLD_R = crate::BitReader<DIAGLD_A>;
impl DIAGLD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIAGLD_A {
        match self.bits {
            false => DIAGLD_A::_0,
            true => DIAGLD_A::_1,
        }
    }
    ///Rotation mode for self-diagnosis voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIAGLD_A::_0
    }
    ///Fixed mode for self-diagnosis voltage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIAGLD_A::_1
    }
}
///Field `DIAGLD` writer - Self-Diagnosis Mode Select
pub type DIAGLD_W<'a, REG> = crate::BitWriter<'a, REG, DIAGLD_A>;
impl<'a, REG> DIAGLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rotation mode for self-diagnosis voltage
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGLD_A::_0)
    }
    ///Fixed mode for self-diagnosis voltage
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGLD_A::_1)
    }
}
/**Self-Diagnosis Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIAGM_A {
    ///0: Disables self-diagnosis of ADC12.
    _0 = 0,
    ///1: Enables self-diagnosis of ADC12.
    _1 = 1,
}
impl From<DIAGM_A> for bool {
    #[inline(always)]
    fn from(variant: DIAGM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIAGM` reader - Self-Diagnosis Enable
pub type DIAGM_R = crate::BitReader<DIAGM_A>;
impl DIAGM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIAGM_A {
        match self.bits {
            false => DIAGM_A::_0,
            true => DIAGM_A::_1,
        }
    }
    ///Disables self-diagnosis of ADC12.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIAGM_A::_0
    }
    ///Enables self-diagnosis of ADC12.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIAGM_A::_1
    }
}
///Field `DIAGM` writer - Self-Diagnosis Enable
pub type DIAGM_W<'a, REG> = crate::BitWriter<'a, REG, DIAGM_A>;
impl<'a, REG> DIAGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables self-diagnosis of ADC12.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGM_A::_0)
    }
    ///Enables self-diagnosis of ADC12.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIAGM_A::_1)
    }
}
/**A/D Data Register Format Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRFMT_A {
    ///0: Flush-right is selected for the A/D data register format.
    _0 = 0,
    ///1: Flush-left is selected for the A/D data register format.
    _1 = 1,
}
impl From<ADRFMT_A> for bool {
    #[inline(always)]
    fn from(variant: ADRFMT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRFMT` reader - A/D Data Register Format Select
pub type ADRFMT_R = crate::BitReader<ADRFMT_A>;
impl ADRFMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRFMT_A {
        match self.bits {
            false => ADRFMT_A::_0,
            true => ADRFMT_A::_1,
        }
    }
    ///Flush-right is selected for the A/D data register format.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADRFMT_A::_0
    }
    ///Flush-left is selected for the A/D data register format.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADRFMT_A::_1
    }
}
///Field `ADRFMT` writer - A/D Data Register Format Select
pub type ADRFMT_W<'a, REG> = crate::BitWriter<'a, REG, ADRFMT_A>;
impl<'a, REG> ADRFMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flush-right is selected for the A/D data register format.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRFMT_A::_0)
    }
    ///Flush-left is selected for the A/D data register format.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRFMT_A::_1)
    }
}
impl R {
    ///Bits 1:2 - A/D Conversion Accuracy Specify
    #[inline(always)]
    pub fn adprc(&self) -> ADPRC_R {
        ADPRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 5 - A/D Data Register Automatic Clearing Enable
    #[inline(always)]
    pub fn ace(&self) -> ACE_R {
        ACE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:9 - Self-Diagnosis Conversion Voltage Select
    #[inline(always)]
    pub fn diagval(&self) -> DIAGVAL_R {
        DIAGVAL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Self-Diagnosis Mode Select
    #[inline(always)]
    pub fn diagld(&self) -> DIAGLD_R {
        DIAGLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Self-Diagnosis Enable
    #[inline(always)]
    pub fn diagm(&self) -> DIAGM_R {
        DIAGM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - A/D Data Register Format Select
    #[inline(always)]
    pub fn adrfmt(&self) -> ADRFMT_R {
        ADRFMT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 1:2 - A/D Conversion Accuracy Specify
    #[inline(always)]
    pub fn adprc(&mut self) -> ADPRC_W<ADCER_SPEC> {
        ADPRC_W::new(self, 1)
    }
    ///Bit 5 - A/D Data Register Automatic Clearing Enable
    #[inline(always)]
    pub fn ace(&mut self) -> ACE_W<ADCER_SPEC> {
        ACE_W::new(self, 5)
    }
    ///Bits 8:9 - Self-Diagnosis Conversion Voltage Select
    #[inline(always)]
    pub fn diagval(&mut self) -> DIAGVAL_W<ADCER_SPEC> {
        DIAGVAL_W::new(self, 8)
    }
    ///Bit 10 - Self-Diagnosis Mode Select
    #[inline(always)]
    pub fn diagld(&mut self) -> DIAGLD_W<ADCER_SPEC> {
        DIAGLD_W::new(self, 10)
    }
    ///Bit 11 - Self-Diagnosis Enable
    #[inline(always)]
    pub fn diagm(&mut self) -> DIAGM_W<ADCER_SPEC> {
        DIAGM_W::new(self, 11)
    }
    ///Bit 15 - A/D Data Register Format Select
    #[inline(always)]
    pub fn adrfmt(&mut self) -> ADRFMT_W<ADCER_SPEC> {
        ADRFMT_W::new(self, 15)
    }
}
/**A/D Control Extended Register

You can [`read`](crate::Reg::read) this register and get [`adcer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCER_SPEC;
impl crate::RegisterSpec for ADCER_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcer::R`](R) reader structure
impl crate::Readable for ADCER_SPEC {}
///`write(|w| ..)` method takes [`adcer::W`](W) writer structure
impl crate::Writable for ADCER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCER to value 0
impl crate::Resettable for ADCER_SPEC {}
