///Register `GTINTAD` reader
pub type R = crate::R<GTINTAD_SPEC>;
///Register `GTINTAD` writer
pub type W = crate::W<GTINTAD_SPEC>;
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
    ///Bits 24:25 - Output Disable Source Select
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
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
    ///Bits 24:25 - Output Disable Source Select
    #[inline(always)]
    pub fn grp(&mut self) -> GRP_W<GTINTAD_SPEC> {
        GRP_W::new(self, 24)
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
