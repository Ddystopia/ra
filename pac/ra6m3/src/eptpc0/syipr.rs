///Register `SYIPR` reader
pub type R = crate::R<SYIPR_SPEC>;
///Register `SYIPR` writer
pub type W = crate::W<SYIPR_SPEC>;
/**SYSR.OFMUD Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFMUD_A {
    ///0: Prohibits notification of the state of SYSR.OFMUD.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.OFMUD.
    _1 = 1,
}
impl From<OFMUD_A> for bool {
    #[inline(always)]
    fn from(variant: OFMUD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OFMUD` reader - SYSR.OFMUD Status Notification Permission
pub type OFMUD_R = crate::BitReader<OFMUD_A>;
impl OFMUD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFMUD_A {
        match self.bits {
            false => OFMUD_A::_0,
            true => OFMUD_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.OFMUD.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFMUD_A::_0
    }
    ///Permits notification of the state of SYSR.OFMUD.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFMUD_A::_1
    }
}
///Field `OFMUD` writer - SYSR.OFMUD Status Notification Permission
pub type OFMUD_W<'a, REG> = crate::BitWriter<'a, REG, OFMUD_A>;
impl<'a, REG> OFMUD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.OFMUD.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OFMUD_A::_0)
    }
    ///Permits notification of the state of SYSR.OFMUD.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OFMUD_A::_1)
    }
}
/**SYSR.INTCHG Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTCHG_A {
    ///0: Prohibits notification of the state of SYSR.INTCHG.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.INTCHG.
    _1 = 1,
}
impl From<INTCHG_A> for bool {
    #[inline(always)]
    fn from(variant: INTCHG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INTCHG` reader - SYSR.INTCHG Status Notification Permission
pub type INTCHG_R = crate::BitReader<INTCHG_A>;
impl INTCHG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTCHG_A {
        match self.bits {
            false => INTCHG_A::_0,
            true => INTCHG_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.INTCHG.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTCHG_A::_0
    }
    ///Permits notification of the state of SYSR.INTCHG.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTCHG_A::_1
    }
}
///Field `INTCHG` writer - SYSR.INTCHG Status Notification Permission
pub type INTCHG_W<'a, REG> = crate::BitWriter<'a, REG, INTCHG_A>;
impl<'a, REG> INTCHG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.INTCHG.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INTCHG_A::_0)
    }
    ///Permits notification of the state of SYSR.INTCHG.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INTCHG_A::_1)
    }
}
/**SYSR.MPDUD Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDUD_A {
    ///0: Prohibits notification of the state of SYSR.MPDUD.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.MPDUD.
    _1 = 1,
}
impl From<MPDUD_A> for bool {
    #[inline(always)]
    fn from(variant: MPDUD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPDUD` reader - SYSR.MPDUD Status Notification Permission
pub type MPDUD_R = crate::BitReader<MPDUD_A>;
impl MPDUD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPDUD_A {
        match self.bits {
            false => MPDUD_A::_0,
            true => MPDUD_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.MPDUD.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDUD_A::_0
    }
    ///Permits notification of the state of SYSR.MPDUD.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDUD_A::_1
    }
}
///Field `MPDUD` writer - SYSR.MPDUD Status Notification Permission
pub type MPDUD_W<'a, REG> = crate::BitWriter<'a, REG, MPDUD_A>;
impl<'a, REG> MPDUD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.MPDUD.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPDUD_A::_0)
    }
    ///Permits notification of the state of SYSR.MPDUD.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPDUD_A::_1)
    }
}
/**SYSR.DRPTO Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPTO_A {
    ///0: Prohibits notification of the state of SYSR.DRPTO.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.DRPTO.
    _1 = 1,
}
impl From<DRPTO_A> for bool {
    #[inline(always)]
    fn from(variant: DRPTO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRPTO` reader - SYSR.DRPTO Status Notification Permission
pub type DRPTO_R = crate::BitReader<DRPTO_A>;
impl DRPTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRPTO_A {
        match self.bits {
            false => DRPTO_A::_0,
            true => DRPTO_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.DRPTO.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPTO_A::_0
    }
    ///Permits notification of the state of SYSR.DRPTO.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPTO_A::_1
    }
}
///Field `DRPTO` writer - SYSR.DRPTO Status Notification Permission
pub type DRPTO_W<'a, REG> = crate::BitWriter<'a, REG, DRPTO_A>;
impl<'a, REG> DRPTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.DRPTO.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRPTO_A::_0)
    }
    ///Permits notification of the state of SYSR.DRPTO.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRPTO_A::_1)
    }
}
/**SYSR.INTDEV Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTDEV_A {
    ///0: Prohibits notification of the state of SYSR.INTDEV.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.INTDEV.
    _1 = 1,
}
impl From<INTDEV_A> for bool {
    #[inline(always)]
    fn from(variant: INTDEV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INTDEV` reader - SYSR.INTDEV Status Notification Permission
pub type INTDEV_R = crate::BitReader<INTDEV_A>;
impl INTDEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTDEV_A {
        match self.bits {
            false => INTDEV_A::_0,
            true => INTDEV_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.INTDEV.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTDEV_A::_0
    }
    ///Permits notification of the state of SYSR.INTDEV.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTDEV_A::_1
    }
}
///Field `INTDEV` writer - SYSR.INTDEV Status Notification Permission
pub type INTDEV_W<'a, REG> = crate::BitWriter<'a, REG, INTDEV_A>;
impl<'a, REG> INTDEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.INTDEV.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INTDEV_A::_0)
    }
    ///Permits notification of the state of SYSR.INTDEV.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INTDEV_A::_1)
    }
}
/**SYSR.DRQOVR Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQOVR_A {
    ///0: Prohibits notification of the state of SYSR.DRQOVR.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.DRQOVR.
    _1 = 1,
}
impl From<DRQOVR_A> for bool {
    #[inline(always)]
    fn from(variant: DRQOVR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRQOVR` reader - SYSR.DRQOVR Status Notification Permission
pub type DRQOVR_R = crate::BitReader<DRQOVR_A>;
impl DRQOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRQOVR_A {
        match self.bits {
            false => DRQOVR_A::_0,
            true => DRQOVR_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.DRQOVR.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQOVR_A::_0
    }
    ///Permits notification of the state of SYSR.DRQOVR.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQOVR_A::_1
    }
}
///Field `DRQOVR` writer - SYSR.DRQOVR Status Notification Permission
pub type DRQOVR_W<'a, REG> = crate::BitWriter<'a, REG, DRQOVR_A>;
impl<'a, REG> DRQOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.DRQOVR.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRQOVR_A::_0)
    }
    ///Permits notification of the state of SYSR.DRQOVR.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRQOVR_A::_1)
    }
}
/**SYSR.RECLP Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECLP_A {
    ///0: Prohibits notification of the state of SYSR.RECLP.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.RECLP.
    _1 = 1,
}
impl From<RECLP_A> for bool {
    #[inline(always)]
    fn from(variant: RECLP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RECLP` reader - SYSR.RECLP Status Notification Permission
pub type RECLP_R = crate::BitReader<RECLP_A>;
impl RECLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RECLP_A {
        match self.bits {
            false => RECLP_A::_0,
            true => RECLP_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.RECLP.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECLP_A::_0
    }
    ///Permits notification of the state of SYSR.RECLP.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECLP_A::_1
    }
}
///Field `RECLP` writer - SYSR.RECLP Status Notification Permission
pub type RECLP_W<'a, REG> = crate::BitWriter<'a, REG, RECLP_A>;
impl<'a, REG> RECLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.RECLP.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RECLP_A::_0)
    }
    ///Permits notification of the state of SYSR.RECLP.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RECLP_A::_1)
    }
}
/**SYSR.INFABT Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFABT_A {
    ///0: Prohibits notification of the state of SYSR.INFABT.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.INFABT.
    _1 = 1,
}
impl From<INFABT_A> for bool {
    #[inline(always)]
    fn from(variant: INFABT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INFABT` reader - SYSR.INFABT Status Notification Permission
pub type INFABT_R = crate::BitReader<INFABT_A>;
impl INFABT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INFABT_A {
        match self.bits {
            false => INFABT_A::_0,
            true => INFABT_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.INFABT.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INFABT_A::_0
    }
    ///Permits notification of the state of SYSR.INFABT.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INFABT_A::_1
    }
}
///Field `INFABT` writer - SYSR.INFABT Status Notification Permission
pub type INFABT_W<'a, REG> = crate::BitWriter<'a, REG, INFABT_A>;
impl<'a, REG> INFABT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.INFABT.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INFABT_A::_0)
    }
    ///Permits notification of the state of SYSR.INFABT.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INFABT_A::_1)
    }
}
/**SYSR.RESDN Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESDN_A {
    ///0: Prohibits notification of the state of SYSR.RESDN.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.RESDN.
    _1 = 1,
}
impl From<RESDN_A> for bool {
    #[inline(always)]
    fn from(variant: RESDN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RESDN` reader - SYSR.RESDN Status Notification Permission
pub type RESDN_R = crate::BitReader<RESDN_A>;
impl RESDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESDN_A {
        match self.bits {
            false => RESDN_A::_0,
            true => RESDN_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.RESDN.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESDN_A::_0
    }
    ///Permits notification of the state of SYSR.RESDN.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESDN_A::_1
    }
}
///Field `RESDN` writer - SYSR.RESDN Status Notification Permission
pub type RESDN_W<'a, REG> = crate::BitWriter<'a, REG, RESDN_A>;
impl<'a, REG> RESDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.RESDN.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESDN_A::_0)
    }
    ///Permits notification of the state of SYSR.RESDN.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESDN_A::_1)
    }
}
/**SYSR.GENDN Status Notification Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENDN_A {
    ///0: Prohibits notification of the state of SYSR.GENDN.
    _0 = 0,
    ///1: Permits notification of the state of SYSR.GENDN.
    _1 = 1,
}
impl From<GENDN_A> for bool {
    #[inline(always)]
    fn from(variant: GENDN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GENDN` reader - SYSR.GENDN Status Notification Permission
pub type GENDN_R = crate::BitReader<GENDN_A>;
impl GENDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GENDN_A {
        match self.bits {
            false => GENDN_A::_0,
            true => GENDN_A::_1,
        }
    }
    ///Prohibits notification of the state of SYSR.GENDN.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GENDN_A::_0
    }
    ///Permits notification of the state of SYSR.GENDN.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GENDN_A::_1
    }
}
///Field `GENDN` writer - SYSR.GENDN Status Notification Permission
pub type GENDN_W<'a, REG> = crate::BitWriter<'a, REG, GENDN_A>;
impl<'a, REG> GENDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits notification of the state of SYSR.GENDN.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GENDN_A::_0)
    }
    ///Permits notification of the state of SYSR.GENDN.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GENDN_A::_1)
    }
}
impl R {
    ///Bit 0 - SYSR.OFMUD Status Notification Permission
    #[inline(always)]
    pub fn ofmud(&self) -> OFMUD_R {
        OFMUD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYSR.INTCHG Status Notification Permission
    #[inline(always)]
    pub fn intchg(&self) -> INTCHG_R {
        INTCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SYSR.MPDUD Status Notification Permission
    #[inline(always)]
    pub fn mpdud(&self) -> MPDUD_R {
        MPDUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SYSR.DRPTO Status Notification Permission
    #[inline(always)]
    pub fn drpto(&self) -> DRPTO_R {
        DRPTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSR.INTDEV Status Notification Permission
    #[inline(always)]
    pub fn intdev(&self) -> INTDEV_R {
        INTDEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SYSR.DRQOVR Status Notification Permission
    #[inline(always)]
    pub fn drqovr(&self) -> DRQOVR_R {
        DRQOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - SYSR.RECLP Status Notification Permission
    #[inline(always)]
    pub fn reclp(&self) -> RECLP_R {
        RECLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - SYSR.INFABT Status Notification Permission
    #[inline(always)]
    pub fn infabt(&self) -> INFABT_R {
        INFABT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SYSR.RESDN Status Notification Permission
    #[inline(always)]
    pub fn resdn(&self) -> RESDN_R {
        RESDN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SYSR.GENDN Status Notification Permission
    #[inline(always)]
    pub fn gendn(&self) -> GENDN_R {
        GENDN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYSR.OFMUD Status Notification Permission
    #[inline(always)]
    pub fn ofmud(&mut self) -> OFMUD_W<SYIPR_SPEC> {
        OFMUD_W::new(self, 0)
    }
    ///Bit 1 - SYSR.INTCHG Status Notification Permission
    #[inline(always)]
    pub fn intchg(&mut self) -> INTCHG_W<SYIPR_SPEC> {
        INTCHG_W::new(self, 1)
    }
    ///Bit 2 - SYSR.MPDUD Status Notification Permission
    #[inline(always)]
    pub fn mpdud(&mut self) -> MPDUD_W<SYIPR_SPEC> {
        MPDUD_W::new(self, 2)
    }
    ///Bit 4 - SYSR.DRPTO Status Notification Permission
    #[inline(always)]
    pub fn drpto(&mut self) -> DRPTO_W<SYIPR_SPEC> {
        DRPTO_W::new(self, 4)
    }
    ///Bit 5 - SYSR.INTDEV Status Notification Permission
    #[inline(always)]
    pub fn intdev(&mut self) -> INTDEV_W<SYIPR_SPEC> {
        INTDEV_W::new(self, 5)
    }
    ///Bit 6 - SYSR.DRQOVR Status Notification Permission
    #[inline(always)]
    pub fn drqovr(&mut self) -> DRQOVR_W<SYIPR_SPEC> {
        DRQOVR_W::new(self, 6)
    }
    ///Bit 12 - SYSR.RECLP Status Notification Permission
    #[inline(always)]
    pub fn reclp(&mut self) -> RECLP_W<SYIPR_SPEC> {
        RECLP_W::new(self, 12)
    }
    ///Bit 14 - SYSR.INFABT Status Notification Permission
    #[inline(always)]
    pub fn infabt(&mut self) -> INFABT_W<SYIPR_SPEC> {
        INFABT_W::new(self, 14)
    }
    ///Bit 16 - SYSR.RESDN Status Notification Permission
    #[inline(always)]
    pub fn resdn(&mut self) -> RESDN_W<SYIPR_SPEC> {
        RESDN_W::new(self, 16)
    }
    ///Bit 17 - SYSR.GENDN Status Notification Permission
    #[inline(always)]
    pub fn gendn(&mut self) -> GENDN_W<SYIPR_SPEC> {
        GENDN_W::new(self, 17)
    }
}
/**SYNFP Status Notification Permission Register

You can [`read`](crate::Reg::read) this register and get [`syipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYIPR_SPEC;
impl crate::RegisterSpec for SYIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syipr::R`](R) reader structure
impl crate::Readable for SYIPR_SPEC {}
///`write(|w| ..)` method takes [`syipr::W`](W) writer structure
impl crate::Writable for SYIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYIPR to value 0
impl crate::Resettable for SYIPR_SPEC {}
