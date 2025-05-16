///Register `GTINTAD` reader
pub type R = crate::R<GTINTAD_SPEC>;
///Register `GTINTAD` writer
pub type W = crate::W<GTINTAD_SPEC>;
/**GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRAUEN_A {
    ///0: Disable A/D converter start request
    _0 = 0,
    ///1: Enable A/D converter start request.
    _1 = 1,
}
impl From<ADTRAUEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRAUEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTRAUEN` reader - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRAUEN_R = crate::BitReader<ADTRAUEN_A>;
impl ADTRAUEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTRAUEN_A {
        match self.bits {
            false => ADTRAUEN_A::_0,
            true => ADTRAUEN_A::_1,
        }
    }
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRAUEN_A::_0
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRAUEN_A::_1
    }
}
///Field `ADTRAUEN` writer - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRAUEN_W<'a, REG> = crate::BitWriter<'a, REG, ADTRAUEN_A>;
impl<'a, REG> ADTRAUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRAUEN_A::_0)
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRAUEN_A::_1)
    }
}
/**GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRADEN_A {
    ///0: Disable A/D converter start request
    _0 = 0,
    ///1: Enable A/D converter start request.
    _1 = 1,
}
impl From<ADTRADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRADEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTRADEN` reader - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRADEN_R = crate::BitReader<ADTRADEN_A>;
impl ADTRADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTRADEN_A {
        match self.bits {
            false => ADTRADEN_A::_0,
            true => ADTRADEN_A::_1,
        }
    }
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRADEN_A::_0
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRADEN_A::_1
    }
}
///Field `ADTRADEN` writer - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRADEN_W<'a, REG> = crate::BitWriter<'a, REG, ADTRADEN_A>;
impl<'a, REG> ADTRADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRADEN_A::_0)
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRADEN_A::_1)
    }
}
/**GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBUEN_A {
    ///0: Disable A/D converter start request
    _0 = 0,
    ///1: Enable A/D converter start request.
    _1 = 1,
}
impl From<ADTRBUEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBUEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTRBUEN` reader - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRBUEN_R = crate::BitReader<ADTRBUEN_A>;
impl ADTRBUEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTRBUEN_A {
        match self.bits {
            false => ADTRBUEN_A::_0,
            true => ADTRBUEN_A::_1,
        }
    }
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBUEN_A::_0
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBUEN_A::_1
    }
}
///Field `ADTRBUEN` writer - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRBUEN_W<'a, REG> = crate::BitWriter<'a, REG, ADTRBUEN_A>;
impl<'a, REG> ADTRBUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRBUEN_A::_0)
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRBUEN_A::_1)
    }
}
/**GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBDEN_A {
    ///0: Disable A/D converter start request
    _0 = 0,
    ///1: Enable A/D converter start request.
    _1 = 1,
}
impl From<ADTRBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTRBDEN` reader - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRBDEN_R = crate::BitReader<ADTRBDEN_A>;
impl ADTRBDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTRBDEN_A {
        match self.bits {
            false => ADTRBDEN_A::_0,
            true => ADTRBDEN_A::_1,
        }
    }
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBDEN_A::_0
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBDEN_A::_1
    }
}
///Field `ADTRBDEN` writer - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
pub type ADTRBDEN_W<'a, REG> = crate::BitWriter<'a, REG, ADTRBDEN_A>;
impl<'a, REG> ADTRBDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D converter start request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRBDEN_A::_0)
    }
    ///Enable A/D converter start request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTRBDEN_A::_1)
    }
}
/**Output Disable Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    ///0: Select Group A output disable request
    _00 = 0,
    ///1: Select Group B output disable request
    _01 = 1,
    ///2: Select Group C output disable request
    _10 = 2,
    ///3: Select Group D output disable request.
    _11 = 3,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRP_A {
    type Ux = u8;
}
impl crate::IsEnum for GRP_A {}
///Field `GRP` reader - Output Disable Source Select
pub type GRP_R = crate::FieldReader<GRP_A>;
impl GRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRP_A {
        match self.bits {
            0 => GRP_A::_00,
            1 => GRP_A::_01,
            2 => GRP_A::_10,
            3 => GRP_A::_11,
            _ => unreachable!(),
        }
    }
    ///Select Group A output disable request
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
    ///Select Group B output disable request
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GRP_A::_01
    }
    ///Select Group C output disable request
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GRP_A::_10
    }
    ///Select Group D output disable request.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GRP_A::_11
    }
}
///Field `GRP` writer - Output Disable Source Select
pub type GRP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GRP_A, crate::Safe>;
impl<'a, REG> GRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select Group A output disable request
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_00)
    }
    ///Select Group B output disable request
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_01)
    }
    ///Select Group C output disable request
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_10)
    }
    ///Select Group D output disable request.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_11)
    }
}
/**Dead Time Error Output Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPDTE_A {
    ///0: Disable dead time error output disable request
    _0 = 0,
    ///1: Enable dead time error output disable request
    _1 = 1,
}
impl From<GRPDTE_A> for bool {
    #[inline(always)]
    fn from(variant: GRPDTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GRPDTE` reader - Dead Time Error Output Disable Request Enable
pub type GRPDTE_R = crate::BitReader<GRPDTE_A>;
impl GRPDTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRPDTE_A {
        match self.bits {
            false => GRPDTE_A::_0,
            true => GRPDTE_A::_1,
        }
    }
    ///Disable dead time error output disable request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPDTE_A::_0
    }
    ///Enable dead time error output disable request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPDTE_A::_1
    }
}
///Field `GRPDTE` writer - Dead Time Error Output Disable Request Enable
pub type GRPDTE_W<'a, REG> = crate::BitWriter<'a, REG, GRPDTE_A>;
impl<'a, REG> GRPDTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable dead time error output disable request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GRPDTE_A::_0)
    }
    ///Enable dead time error output disable request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GRPDTE_A::_1)
    }
}
/**Same Time Output Level High Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABH_A {
    ///0: Disable same time output level high disable request
    _0 = 0,
    ///1: Enable same time output level high disable request
    _1 = 1,
}
impl From<GRPABH_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GRPABH` reader - Same Time Output Level High Disable Request Enable
pub type GRPABH_R = crate::BitReader<GRPABH_A>;
impl GRPABH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRPABH_A {
        match self.bits {
            false => GRPABH_A::_0,
            true => GRPABH_A::_1,
        }
    }
    ///Disable same time output level high disable request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABH_A::_0
    }
    ///Enable same time output level high disable request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABH_A::_1
    }
}
///Field `GRPABH` writer - Same Time Output Level High Disable Request Enable
pub type GRPABH_W<'a, REG> = crate::BitWriter<'a, REG, GRPABH_A>;
impl<'a, REG> GRPABH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable same time output level high disable request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABH_A::_0)
    }
    ///Enable same time output level high disable request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABH_A::_1)
    }
}
/**Same Time Output Level Low Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABL_A {
    ///0: Disable same time output level low disable request
    _0 = 0,
    ///1: Enable same time output level low disable request
    _1 = 1,
}
impl From<GRPABL_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable
pub type GRPABL_R = crate::BitReader<GRPABL_A>;
impl GRPABL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRPABL_A {
        match self.bits {
            false => GRPABL_A::_0,
            true => GRPABL_A::_1,
        }
    }
    ///Disable same time output level low disable request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABL_A::_0
    }
    ///Enable same time output level low disable request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABL_A::_1
    }
}
///Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable
pub type GRPABL_W<'a, REG> = crate::BitWriter<'a, REG, GRPABL_A>;
impl<'a, REG> GRPABL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable same time output level low disable request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABL_A::_0)
    }
    ///Enable same time output level low disable request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GRPABL_A::_1)
    }
}
impl R {
    ///Bit 16 - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtrauen(&self) -> ADTRAUEN_R {
        ADTRAUEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtraden(&self) -> ADTRADEN_R {
        ADTRADEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtrbuen(&self) -> ADTRBUEN_R {
        ADTRBUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtrbden(&self) -> ADTRBDEN_R {
        ADTRBDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:25 - Output Disable Source Select
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 28 - Dead Time Error Output Disable Request Enable
    #[inline(always)]
    pub fn grpdte(&self) -> GRPDTE_R {
        GRPDTE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Same Time Output Level High Disable Request Enable
    #[inline(always)]
    pub fn grpabh(&self) -> GRPABH_R {
        GRPABH_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Same Time Output Level Low Disable Request Enable
    #[inline(always)]
    pub fn grpabl(&self) -> GRPABL_R {
        GRPABL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtrauen(&mut self) -> ADTRAUEN_W<GTINTAD_SPEC> {
        ADTRAUEN_W::new(self, 16)
    }
    ///Bit 17 - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtraden(&mut self) -> ADTRADEN_W<GTINTAD_SPEC> {
        ADTRADEN_W::new(self, 17)
    }
    ///Bit 18 - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtrbuen(&mut self) -> ADTRBUEN_W<GTINTAD_SPEC> {
        ADTRBUEN_W::new(self, 18)
    }
    ///Bit 19 - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable
    #[inline(always)]
    pub fn adtrbden(&mut self) -> ADTRBDEN_W<GTINTAD_SPEC> {
        ADTRBDEN_W::new(self, 19)
    }
    ///Bits 24:25 - Output Disable Source Select
    #[inline(always)]
    pub fn grp(&mut self) -> GRP_W<GTINTAD_SPEC> {
        GRP_W::new(self, 24)
    }
    ///Bit 28 - Dead Time Error Output Disable Request Enable
    #[inline(always)]
    pub fn grpdte(&mut self) -> GRPDTE_W<GTINTAD_SPEC> {
        GRPDTE_W::new(self, 28)
    }
    ///Bit 29 - Same Time Output Level High Disable Request Enable
    #[inline(always)]
    pub fn grpabh(&mut self) -> GRPABH_W<GTINTAD_SPEC> {
        GRPABH_W::new(self, 29)
    }
    ///Bit 30 - Same Time Output Level Low Disable Request Enable
    #[inline(always)]
    pub fn grpabl(&mut self) -> GRPABL_W<GTINTAD_SPEC> {
        GRPABL_W::new(self, 30)
    }
}
/**General PWM Timer Interrupt Output Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtintad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtintad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTINTAD_SPEC;
impl crate::RegisterSpec for GTINTAD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtintad::R`](R) reader structure
impl crate::Readable for GTINTAD_SPEC {}
///`write(|w| ..)` method takes [`gtintad::W`](W) writer structure
impl crate::Writable for GTINTAD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTINTAD to value 0
impl crate::Resettable for GTINTAD_SPEC {}
