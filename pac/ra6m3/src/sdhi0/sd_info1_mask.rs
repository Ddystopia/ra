///Register `SD_INFO1_MASK` reader
pub type R = crate::R<SD_INFO1_MASK_SPEC>;
///Register `SD_INFO1_MASK` writer
pub type W = crate::W<SD_INFO1_MASK_SPEC>;
/**Response End Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPENDM_A {
    ///0: Response end interrupt request is not masked
    _0 = 0,
    ///1: Response end interrupt request is masked
    _1 = 1,
}
impl From<RSPENDM_A> for bool {
    #[inline(always)]
    fn from(variant: RSPENDM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPENDM` reader - Response End Interrupt Request Mask
pub type RSPENDM_R = crate::BitReader<RSPENDM_A>;
impl RSPENDM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPENDM_A {
        match self.bits {
            false => RSPENDM_A::_0,
            true => RSPENDM_A::_1,
        }
    }
    ///Response end interrupt request is not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPENDM_A::_0
    }
    ///Response end interrupt request is masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPENDM_A::_1
    }
}
///Field `RSPENDM` writer - Response End Interrupt Request Mask
pub type RSPENDM_W<'a, REG> = crate::BitWriter<'a, REG, RSPENDM_A>;
impl<'a, REG> RSPENDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Response end interrupt request is not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSPENDM_A::_0)
    }
    ///Response end interrupt request is masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSPENDM_A::_1)
    }
}
/**Access End Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACENDM_A {
    ///0: Access end interrupt request is not masked
    _0 = 0,
    ///1: Access end interrupt request is masked
    _1 = 1,
}
impl From<ACENDM_A> for bool {
    #[inline(always)]
    fn from(variant: ACENDM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACENDM` reader - Access End Interrupt Request Mask
pub type ACENDM_R = crate::BitReader<ACENDM_A>;
impl ACENDM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACENDM_A {
        match self.bits {
            false => ACENDM_A::_0,
            true => ACENDM_A::_1,
        }
    }
    ///Access end interrupt request is not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACENDM_A::_0
    }
    ///Access end interrupt request is masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACENDM_A::_1
    }
}
///Field `ACENDM` writer - Access End Interrupt Request Mask
pub type ACENDM_W<'a, REG> = crate::BitWriter<'a, REG, ACENDM_A>;
impl<'a, REG> ACENDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access end interrupt request is not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACENDM_A::_0)
    }
    ///Access end interrupt request is masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACENDM_A::_1)
    }
}
/**SDnCD card Removal Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDRMM_A {
    ///0: Card removal interrupt request by the by the SDnCD is not masked
    _0 = 0,
    ///1: Card removal interrupt request by the by the SDnCD is masked
    _1 = 1,
}
impl From<SDCDRMM_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDRMM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDRMM` reader - SDnCD card Removal Interrupt Request Mask
pub type SDCDRMM_R = crate::BitReader<SDCDRMM_A>;
impl SDCDRMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDCDRMM_A {
        match self.bits {
            false => SDCDRMM_A::_0,
            true => SDCDRMM_A::_1,
        }
    }
    ///Card removal interrupt request by the by the SDnCD is not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDRMM_A::_0
    }
    ///Card removal interrupt request by the by the SDnCD is masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDRMM_A::_1
    }
}
///Field `SDCDRMM` writer - SDnCD card Removal Interrupt Request Mask
pub type SDCDRMM_W<'a, REG> = crate::BitWriter<'a, REG, SDCDRMM_A>;
impl<'a, REG> SDCDRMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Card removal interrupt request by the by the SDnCD is not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDRMM_A::_0)
    }
    ///Card removal interrupt request by the by the SDnCD is masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDRMM_A::_1)
    }
}
/**SDnCD card Insertion Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCDINM_A {
    ///0: Card insertion interrupt request by the SDnCD is not masked
    _0 = 0,
    ///1: Card insertion interrupt request by the SDnCD is masked
    _1 = 1,
}
impl From<SDCDINM_A> for bool {
    #[inline(always)]
    fn from(variant: SDCDINM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDCDINM` reader - SDnCD card Insertion Interrupt Request Mask
pub type SDCDINM_R = crate::BitReader<SDCDINM_A>;
impl SDCDINM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDCDINM_A {
        match self.bits {
            false => SDCDINM_A::_0,
            true => SDCDINM_A::_1,
        }
    }
    ///Card insertion interrupt request by the SDnCD is not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDCDINM_A::_0
    }
    ///Card insertion interrupt request by the SDnCD is masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCDINM_A::_1
    }
}
///Field `SDCDINM` writer - SDnCD card Insertion Interrupt Request Mask
pub type SDCDINM_W<'a, REG> = crate::BitWriter<'a, REG, SDCDINM_A>;
impl<'a, REG> SDCDINM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Card insertion interrupt request by the SDnCD is not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDINM_A::_0)
    }
    ///Card insertion interrupt request by the SDnCD is masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCDINM_A::_1)
    }
}
/**SDnDAT3 Card Removal Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3RMM_A {
    ///0: SD card removal interrupt request by the SDnDAT3 is not masked
    _0 = 0,
    ///1: SD card removal interrupt request by the SDnDAT3 is masked
    _1 = 1,
}
impl From<SDD3RMM_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3RMM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3RMM` reader - SDnDAT3 Card Removal Interrupt Request Mask
pub type SDD3RMM_R = crate::BitReader<SDD3RMM_A>;
impl SDD3RMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDD3RMM_A {
        match self.bits {
            false => SDD3RMM_A::_0,
            true => SDD3RMM_A::_1,
        }
    }
    ///SD card removal interrupt request by the SDnDAT3 is not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3RMM_A::_0
    }
    ///SD card removal interrupt request by the SDnDAT3 is masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3RMM_A::_1
    }
}
///Field `SDD3RMM` writer - SDnDAT3 Card Removal Interrupt Request Mask
pub type SDD3RMM_W<'a, REG> = crate::BitWriter<'a, REG, SDD3RMM_A>;
impl<'a, REG> SDD3RMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card removal interrupt request by the SDnDAT3 is not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3RMM_A::_0)
    }
    ///SD card removal interrupt request by the SDnDAT3 is masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3RMM_A::_1)
    }
}
/**SDnDAT3 Card Insertion Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD3INM_A {
    ///0: SD card insertion interrupt request by the SDnDAT3 is not masked
    _0 = 0,
    ///1: SD card insertion interrupt request by the SDnDAT3 is masked
    _1 = 1,
}
impl From<SDD3INM_A> for bool {
    #[inline(always)]
    fn from(variant: SDD3INM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD3INM` reader - SDnDAT3 Card Insertion Interrupt Request Mask
pub type SDD3INM_R = crate::BitReader<SDD3INM_A>;
impl SDD3INM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDD3INM_A {
        match self.bits {
            false => SDD3INM_A::_0,
            true => SDD3INM_A::_1,
        }
    }
    ///SD card insertion interrupt request by the SDnDAT3 is not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD3INM_A::_0
    }
    ///SD card insertion interrupt request by the SDnDAT3 is masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD3INM_A::_1
    }
}
///Field `SDD3INM` writer - SDnDAT3 Card Insertion Interrupt Request Mask
pub type SDD3INM_W<'a, REG> = crate::BitWriter<'a, REG, SDD3INM_A>;
impl<'a, REG> SDD3INM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD card insertion interrupt request by the SDnDAT3 is not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3INM_A::_0)
    }
    ///SD card insertion interrupt request by the SDnDAT3 is masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDD3INM_A::_1)
    }
}
impl R {
    ///Bit 0 - Response End Interrupt Request Mask
    #[inline(always)]
    pub fn rspendm(&self) -> RSPENDM_R {
        RSPENDM_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Access End Interrupt Request Mask
    #[inline(always)]
    pub fn acendm(&self) -> ACENDM_R {
        ACENDM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SDnCD card Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdrmm(&self) -> SDCDRMM_R {
        SDCDRMM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SDnCD card Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdinm(&self) -> SDCDINM_R {
        SDCDINM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SDnDAT3 Card Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3rmm(&self) -> SDD3RMM_R {
        SDD3RMM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SDnDAT3 Card Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3inm(&self) -> SDD3INM_R {
        SDD3INM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Response End Interrupt Request Mask
    #[inline(always)]
    pub fn rspendm(&mut self) -> RSPENDM_W<SD_INFO1_MASK_SPEC> {
        RSPENDM_W::new(self, 0)
    }
    ///Bit 2 - Access End Interrupt Request Mask
    #[inline(always)]
    pub fn acendm(&mut self) -> ACENDM_W<SD_INFO1_MASK_SPEC> {
        ACENDM_W::new(self, 2)
    }
    ///Bit 3 - SDnCD card Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdrmm(&mut self) -> SDCDRMM_W<SD_INFO1_MASK_SPEC> {
        SDCDRMM_W::new(self, 3)
    }
    ///Bit 4 - SDnCD card Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdcdinm(&mut self) -> SDCDINM_W<SD_INFO1_MASK_SPEC> {
        SDCDINM_W::new(self, 4)
    }
    ///Bit 8 - SDnDAT3 Card Removal Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3rmm(&mut self) -> SDD3RMM_W<SD_INFO1_MASK_SPEC> {
        SDD3RMM_W::new(self, 8)
    }
    ///Bit 9 - SDnDAT3 Card Insertion Interrupt Request Mask
    #[inline(always)]
    pub fn sdd3inm(&mut self) -> SDD3INM_W<SD_INFO1_MASK_SPEC> {
        SDD3INM_W::new(self, 9)
    }
}
/**SD_INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info1_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_INFO1_MASK_SPEC;
impl crate::RegisterSpec for SD_INFO1_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_info1_mask::R`](R) reader structure
impl crate::Readable for SD_INFO1_MASK_SPEC {}
///`write(|w| ..)` method takes [`sd_info1_mask::W`](W) writer structure
impl crate::Writable for SD_INFO1_MASK_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_INFO1_MASK to value 0x031d
impl crate::Resettable for SD_INFO1_MASK_SPEC {
    const RESET_VALUE: u32 = 0x031d;
}
