///Register `ADCSR` reader
pub type R = crate::R<ADCSR_SPEC>;
///Register `ADCSR` writer
pub type W = crate::W<ADCSR_SPEC>;
///Field `DBLANS` reader - Double Trigger Channel SelectThese bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected.
pub type DBLANS_R = crate::FieldReader;
///Field `DBLANS` writer - Double Trigger Channel SelectThese bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected.
pub type DBLANS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Group B Scan End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GBADIE_A {
    ///0: Disables S12GBADI1 interrupt generation upon group B scan completion.
    _0 = 0,
    ///1: Enables S12GBADI1 interrupt generation upon group B scan completion.
    _1 = 1,
}
impl From<GBADIE_A> for bool {
    #[inline(always)]
    fn from(variant: GBADIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GBADIE` reader - Group B Scan End Interrupt Enable
pub type GBADIE_R = crate::BitReader<GBADIE_A>;
impl GBADIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GBADIE_A {
        match self.bits {
            false => GBADIE_A::_0,
            true => GBADIE_A::_1,
        }
    }
    ///Disables S12GBADI1 interrupt generation upon group B scan completion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBADIE_A::_0
    }
    ///Enables S12GBADI1 interrupt generation upon group B scan completion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBADIE_A::_1
    }
}
///Field `GBADIE` writer - Group B Scan End Interrupt Enable
pub type GBADIE_W<'a, REG> = crate::BitWriter<'a, REG, GBADIE_A>;
impl<'a, REG> GBADIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables S12GBADI1 interrupt generation upon group B scan completion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GBADIE_A::_0)
    }
    ///Enables S12GBADI1 interrupt generation upon group B scan completion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GBADIE_A::_1)
    }
}
/**Double Trigger Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLE_A {
    ///0: Double trigger mode non-selection
    _0 = 0,
    ///1: Double trigger mode selection
    _1 = 1,
}
impl From<DBLE_A> for bool {
    #[inline(always)]
    fn from(variant: DBLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBLE` reader - Double Trigger Mode Select
pub type DBLE_R = crate::BitReader<DBLE_A>;
impl DBLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBLE_A {
        match self.bits {
            false => DBLE_A::_0,
            true => DBLE_A::_1,
        }
    }
    ///Double trigger mode non-selection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBLE_A::_0
    }
    ///Double trigger mode selection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBLE_A::_1
    }
}
///Field `DBLE` writer - Double Trigger Mode Select
pub type DBLE_W<'a, REG> = crate::BitWriter<'a, REG, DBLE_A>;
impl<'a, REG> DBLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Double trigger mode non-selection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBLE_A::_0)
    }
    ///Double trigger mode selection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBLE_A::_1)
    }
}
/**Trigger Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTRG_A {
    ///0: A/D conversion is started by the synchronous trigger (ELCTRG1).
    _0 = 0,
    ///1: A/D conversion is started by the asynchronous trigger (ADTRG1#).
    _1 = 1,
}
impl From<EXTRG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTRG` reader - Trigger Select
pub type EXTRG_R = crate::BitReader<EXTRG_A>;
impl EXTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTRG_A {
        match self.bits {
            false => EXTRG_A::_0,
            true => EXTRG_A::_1,
        }
    }
    ///A/D conversion is started by the synchronous trigger (ELCTRG1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTRG_A::_0
    }
    ///A/D conversion is started by the asynchronous trigger (ADTRG1#).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTRG_A::_1
    }
}
///Field `EXTRG` writer - Trigger Select
pub type EXTRG_W<'a, REG> = crate::BitWriter<'a, REG, EXTRG_A>;
impl<'a, REG> EXTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A/D conversion is started by the synchronous trigger (ELCTRG1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTRG_A::_0)
    }
    ///A/D conversion is started by the asynchronous trigger (ADTRG1#).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTRG_A::_1)
    }
}
/**Trigger Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGE_A {
    ///0: Disables A/D conversion to be started by the synchronous or asynchronous trigger.
    _0 = 0,
    ///1: Enables A/D conversion to be started by the synchronous or asynchronous trigger.
    _1 = 1,
}
impl From<TRGE_A> for bool {
    #[inline(always)]
    fn from(variant: TRGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRGE` reader - Trigger Start Enable
pub type TRGE_R = crate::BitReader<TRGE_A>;
impl TRGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRGE_A {
        match self.bits {
            false => TRGE_A::_0,
            true => TRGE_A::_1,
        }
    }
    ///Disables A/D conversion to be started by the synchronous or asynchronous trigger.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGE_A::_0
    }
    ///Enables A/D conversion to be started by the synchronous or asynchronous trigger.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGE_A::_1
    }
}
///Field `TRGE` writer - Trigger Start Enable
pub type TRGE_W<'a, REG> = crate::BitWriter<'a, REG, TRGE_A>;
impl<'a, REG> TRGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables A/D conversion to be started by the synchronous or asynchronous trigger.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGE_A::_0)
    }
    ///Enables A/D conversion to be started by the synchronous or asynchronous trigger.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGE_A::_1)
    }
}
/**Scan Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCS_A {
    ///0: Single scan mode
    _00 = 0,
    ///1: Group scan mode
    _01 = 1,
    ///2: Continuous scan mode
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<ADCS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCS_A {
    type Ux = u8;
}
impl crate::IsEnum for ADCS_A {}
///Field `ADCS` reader - Scan Mode Select
pub type ADCS_R = crate::FieldReader<ADCS_A>;
impl ADCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCS_A {
        match self.bits {
            0 => ADCS_A::_00,
            1 => ADCS_A::_01,
            2 => ADCS_A::_10,
            3 => ADCS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Single scan mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADCS_A::_00
    }
    ///Group scan mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADCS_A::_01
    }
    ///Continuous scan mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADCS_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADCS_A::_11
    }
}
///Field `ADCS` writer - Scan Mode Select
pub type ADCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCS_A, crate::Safe>;
impl<'a, REG> ADCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Single scan mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ADCS_A::_00)
    }
    ///Group scan mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ADCS_A::_01)
    }
    ///Continuous scan mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(ADCS_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ADCS_A::_11)
    }
}
/**A/D Conversion Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADST_A {
    ///0: Stops A/D conversion process.
    _0 = 0,
    ///1: Starts A/D conversion process.
    _1 = 1,
}
impl From<ADST_A> for bool {
    #[inline(always)]
    fn from(variant: ADST_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ADST` reader - A/D Conversion Start

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ADST_R = crate::BitReader<ADST_A>;
impl ADST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADST_A {
        match self.bits {
            false => ADST_A::_0,
            true => ADST_A::_1,
        }
    }
    ///Stops A/D conversion process.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADST_A::_0
    }
    ///Starts A/D conversion process.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADST_A::_1
    }
}
///Field `ADST` writer - A/D Conversion Start
pub type ADST_W<'a, REG> = crate::BitWriter<'a, REG, ADST_A>;
impl<'a, REG> ADST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops A/D conversion process.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADST_A::_0)
    }
    ///Starts A/D conversion process.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADST_A::_1)
    }
}
impl R {
    ///Bits 0:4 - Double Trigger Channel SelectThese bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected.
    #[inline(always)]
    pub fn dblans(&self) -> DBLANS_R {
        DBLANS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - Group B Scan End Interrupt Enable
    #[inline(always)]
    pub fn gbadie(&self) -> GBADIE_R {
        GBADIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Double Trigger Mode Select
    #[inline(always)]
    pub fn dble(&self) -> DBLE_R {
        DBLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Trigger Select
    #[inline(always)]
    pub fn extrg(&self) -> EXTRG_R {
        EXTRG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Trigger Start Enable
    #[inline(always)]
    pub fn trge(&self) -> TRGE_R {
        TRGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 13:14 - Scan Mode Select
    #[inline(always)]
    pub fn adcs(&self) -> ADCS_R {
        ADCS_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - A/D Conversion Start
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Double Trigger Channel SelectThese bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected.
    #[inline(always)]
    pub fn dblans(&mut self) -> DBLANS_W<ADCSR_SPEC> {
        DBLANS_W::new(self, 0)
    }
    ///Bit 6 - Group B Scan End Interrupt Enable
    #[inline(always)]
    pub fn gbadie(&mut self) -> GBADIE_W<ADCSR_SPEC> {
        GBADIE_W::new(self, 6)
    }
    ///Bit 7 - Double Trigger Mode Select
    #[inline(always)]
    pub fn dble(&mut self) -> DBLE_W<ADCSR_SPEC> {
        DBLE_W::new(self, 7)
    }
    ///Bit 8 - Trigger Select
    #[inline(always)]
    pub fn extrg(&mut self) -> EXTRG_W<ADCSR_SPEC> {
        EXTRG_W::new(self, 8)
    }
    ///Bit 9 - Trigger Start Enable
    #[inline(always)]
    pub fn trge(&mut self) -> TRGE_W<ADCSR_SPEC> {
        TRGE_W::new(self, 9)
    }
    ///Bits 13:14 - Scan Mode Select
    #[inline(always)]
    pub fn adcs(&mut self) -> ADCS_W<ADCSR_SPEC> {
        ADCS_W::new(self, 13)
    }
    ///Bit 15 - A/D Conversion Start
    #[inline(always)]
    pub fn adst(&mut self) -> ADST_W<ADCSR_SPEC> {
        ADST_W::new(self, 15)
    }
}
/**A/D Control Register

You can [`read`](crate::Reg::read) this register and get [`adcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCSR_SPEC;
impl crate::RegisterSpec for ADCSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcsr::R`](R) reader structure
impl crate::Readable for ADCSR_SPEC {}
///`write(|w| ..)` method takes [`adcsr::W`](W) writer structure
impl crate::Writable for ADCSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCSR to value 0
impl crate::Resettable for ADCSR_SPEC {}
