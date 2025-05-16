///Register `SD_INFO1` reader
pub type R = crate::R<SD_INFO1_SPEC>;
///Register `SD_INFO1` writer
pub type W = crate::W<SD_INFO1_SPEC>;
/**Response End Detection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPEND_A {
    ///0: Response end is not detected
    _0 = 0,
    ///1: Response end is detected
    _1 = 1,
}
impl From<RSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: RSPEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPEND` reader - Response End Detection
pub type RSPEND_R = crate::BitReader<RSPEND_A>;
impl RSPEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPEND_A {
        match self.bits {
            false => RSPEND_A::_0,
            true => RSPEND_A::_1,
        }
    }
    ///Response end is not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPEND_A::_0
    }
    ///Response end is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPEND_A::_1
    }
}
///Field `RSPEND` writer - Response End Detection
pub type RSPEND_W<'a, REG> = crate::BitWriter<'a, REG, RSPEND_A>;
impl<'a, REG> RSPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Response end is not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSPEND_A::_0)
    }
    ///Response end is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSPEND_A::_1)
    }
}
/**Access End

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACEND_A {
    ///0: Access end is not detected
    _0 = 0,
    ///1: Access end is detected
    _1 = 1,
}
impl From<ACEND_A> for bool {
    #[inline(always)]
    fn from(variant: ACEND_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ACEND` reader - Access End

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ACEND_R = crate::BitReader<ACEND_A>;
impl ACEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACEND_A {
        match self.bits {
            false => ACEND_A::_0,
            true => ACEND_A::_1,
        }
    }
    ///Access end is not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACEND_A::_0
    }
    ///Access end is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACEND_A::_1
    }
}
///Field `ACEND` writer - Access End
pub type ACEND_W<'a, REG> = crate::BitWriter0C<'a, REG, ACEND_A>;
impl<'a, REG> ACEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access end is not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACEND_A::_0)
    }
    ///Access end is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACEND_A::_1)
    }
}
/**SDnCD Card Removal

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDRM_A {
    ///0: Card removal not detected
    _0 = 0,
    ///1: Card removal detected
    _1 = 1,
}
impl From<SDCDRM_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDRM_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SDCDRM` reader - SDnCD Card Removal

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SDCDRM_R = crate::BitReader<SDCDRM_A>;
impl SDCDRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDCDRM_A {
        match self.bits {
            false => SDCDRM_A::_0,
            true => SDCDRM_A::_1,
        }
    }
    ///Card removal not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDRM_A::_0
    }
    ///Card removal detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDRM_A::_1
    }
}
///Field `SDCDRM` writer - SDnCD Card Removal
pub type SDCDRM_W<'a, REG> = crate::BitWriter0C<'a, REG, SDCDRM_A>;
impl<'a, REG> SDCDRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Card removal not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDRM_A::_0)
    }
    ///Card removal detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDRM_A::_1)
    }
}
/**SDnCD Card Insertion

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDIN_A {
    ///0: Card insertion not detected
    _0 = 0,
    ///1: Card insertion detected
    _1 = 1,
}
impl From<SDCDIN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDIN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SDCDIN` reader - SDnCD Card Insertion

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SDCDIN_R = crate::BitReader<SDCDIN_A>;
impl SDCDIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDCDIN_A {
        match self.bits {
            false => SDCDIN_A::_0,
            true => SDCDIN_A::_1,
        }
    }
    ///Card insertion not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDIN_A::_0
    }
    ///Card insertion detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDIN_A::_1
    }
}
///Field `SDCDIN` writer - SDnCD Card Insertion
pub type SDCDIN_W<'a, REG> = crate::BitWriter0C<'a, REG, SDCDIN_A>;
impl<'a, REG> SDCDIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Card insertion not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDIN_A::_0)
    }
    ///Card insertion detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDIN_A::_1)
    }
}
/**Indicates the SDnCD state

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDMON_A {
    ///0: Indicates that Mcycle has elapsed with SDnCD held 1.(Mcycle is set by bits 3 to 0 in SD_OPTION.)
    _0 = 0,
    ///1: Indicates that Mcycle has elapsed with SDnCD held 0. (Mcycle is set by bits 3 to 0 in SD_OPTION.)
    _1 = 1,
}
impl From<SDCDMON_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDMON` reader - Indicates the SDnCD state
pub type SDCDMON_R = crate::BitReader<SDCDMON_A>;
impl SDCDMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDCDMON_A {
        match self.bits {
            false => SDCDMON_A::_0,
            true => SDCDMON_A::_1,
        }
    }
    ///Indicates that Mcycle has elapsed with SDnCD held 1.(Mcycle is set by bits 3 to 0 in SD_OPTION.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDMON_A::_0
    }
    ///Indicates that Mcycle has elapsed with SDnCD held 0. (Mcycle is set by bits 3 to 0 in SD_OPTION.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDMON_A::_1
    }
}
/**Indicates the SDnWP state

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDWPMON_A {
    ///0: SDnWP is set to 1.
    _0 = 0,
    ///1: SDnWP is set to 0.
    _1 = 1,
}
impl From<SDWPMON_A> for bool {
    #[inline(always)]
    fn from(variant: SDWPMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDWPMON` reader - Indicates the SDnWP state
pub type SDWPMON_R = crate::BitReader<SDWPMON_A>;
impl SDWPMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDWPMON_A {
        match self.bits {
            false => SDWPMON_A::_0,
            true => SDWPMON_A::_1,
        }
    }
    ///SDnWP is set to 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDWPMON_A::_0
    }
    ///SDnWP is set to 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDWPMON_A::_1
    }
}
/**SDnDAT3 Card Removal

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3RM_A {
    ///0: SD card removal not detected
    _0 = 0,
    ///1: SD card removal detected
    _1 = 1,
}
impl From<SDD3RM_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3RM_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SDD3RM` reader - SDnDAT3 Card Removal

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SDD3RM_R = crate::BitReader<SDD3RM_A>;
impl SDD3RM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDD3RM_A {
        match self.bits {
            false => SDD3RM_A::_0,
            true => SDD3RM_A::_1,
        }
    }
    ///SD card removal not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3RM_A::_0
    }
    ///SD card removal detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3RM_A::_1
    }
}
///Field `SDD3RM` writer - SDnDAT3 Card Removal
pub type SDD3RM_W<'a, REG> = crate::BitWriter0C<'a, REG, SDD3RM_A>;
impl<'a, REG> SDD3RM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card removal not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3RM_A::_0)
    }
    ///SD card removal detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3RM_A::_1)
    }
}
/**SDnDAT3 Card Insertion

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3IN_A {
    ///0: SD card insertion not detected
    _0 = 0,
    ///1: SD card insertion detected
    _1 = 1,
}
impl From<SDD3IN_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3IN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SDD3IN` reader - SDnDAT3 Card Insertion

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SDD3IN_R = crate::BitReader<SDD3IN_A>;
impl SDD3IN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDD3IN_A {
        match self.bits {
            false => SDD3IN_A::_0,
            true => SDD3IN_A::_1,
        }
    }
    ///SD card insertion not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3IN_A::_0
    }
    ///SD card insertion detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3IN_A::_1
    }
}
///Field `SDD3IN` writer - SDnDAT3 Card Insertion
pub type SDD3IN_W<'a, REG> = crate::BitWriter0C<'a, REG, SDD3IN_A>;
impl<'a, REG> SDD3IN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card insertion not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3IN_A::_0)
    }
    ///SD card insertion detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3IN_A::_1)
    }
}
/**Inticates the SDnDAT3 State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3MON_A {
    ///0: SDnDAT3 is set to 0.
    _0 = 0,
    ///1: SDnDAT3 is set to 1.
    _1 = 1,
}
impl From<SDD3MON_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3MON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3MON` reader - Inticates the SDnDAT3 State
pub type SDD3MON_R = crate::BitReader<SDD3MON_A>;
impl SDD3MON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDD3MON_A {
        match self.bits {
            false => SDD3MON_A::_0,
            true => SDD3MON_A::_1,
        }
    }
    ///SDnDAT3 is set to 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3MON_A::_0
    }
    ///SDnDAT3 is set to 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3MON_A::_1
    }
}
impl R {
    ///Bit 0 - Response End Detection
    #[inline(always)]
    pub fn rspend(&self) -> RSPEND_R {
        RSPEND_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Access End
    #[inline(always)]
    pub fn acend(&self) -> ACEND_R {
        ACEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SDnCD Card Removal
    #[inline(always)]
    pub fn sdcdrm(&self) -> SDCDRM_R {
        SDCDRM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SDnCD Card Insertion
    #[inline(always)]
    pub fn sdcdin(&self) -> SDCDIN_R {
        SDCDIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Indicates the SDnCD state
    #[inline(always)]
    pub fn sdcdmon(&self) -> SDCDMON_R {
        SDCDMON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Indicates the SDnWP state
    #[inline(always)]
    pub fn sdwpmon(&self) -> SDWPMON_R {
        SDWPMON_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SDnDAT3 Card Removal
    #[inline(always)]
    pub fn sdd3rm(&self) -> SDD3RM_R {
        SDD3RM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SDnDAT3 Card Insertion
    #[inline(always)]
    pub fn sdd3in(&self) -> SDD3IN_R {
        SDD3IN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Inticates the SDnDAT3 State
    #[inline(always)]
    pub fn sdd3mon(&self) -> SDD3MON_R {
        SDD3MON_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Response End Detection
    #[inline(always)]
    pub fn rspend(&mut self) -> RSPEND_W<SD_INFO1_SPEC> {
        RSPEND_W::new(self, 0)
    }
    ///Bit 2 - Access End
    #[inline(always)]
    pub fn acend(&mut self) -> ACEND_W<SD_INFO1_SPEC> {
        ACEND_W::new(self, 2)
    }
    ///Bit 3 - SDnCD Card Removal
    #[inline(always)]
    pub fn sdcdrm(&mut self) -> SDCDRM_W<SD_INFO1_SPEC> {
        SDCDRM_W::new(self, 3)
    }
    ///Bit 4 - SDnCD Card Insertion
    #[inline(always)]
    pub fn sdcdin(&mut self) -> SDCDIN_W<SD_INFO1_SPEC> {
        SDCDIN_W::new(self, 4)
    }
    ///Bit 8 - SDnDAT3 Card Removal
    #[inline(always)]
    pub fn sdd3rm(&mut self) -> SDD3RM_W<SD_INFO1_SPEC> {
        SDD3RM_W::new(self, 8)
    }
    ///Bit 9 - SDnDAT3 Card Insertion
    #[inline(always)]
    pub fn sdd3in(&mut self) -> SDD3IN_W<SD_INFO1_SPEC> {
        SDD3IN_W::new(self, 9)
    }
}
/**SD Card Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_info1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_INFO1_SPEC;
impl crate::RegisterSpec for SD_INFO1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_info1::R`](R) reader structure
impl crate::Readable for SD_INFO1_SPEC {}
///`write(|w| ..)` method takes [`sd_info1::W`](W) writer structure
impl crate::Writable for SD_INFO1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x031c;
}
///`reset()` method sets SD_INFO1 to value 0
impl crate::Resettable for SD_INFO1_SPEC {}
