///Register `SD_INFO2_MASK` reader
pub type R = crate::R<SD_INFO2_MASK_SPEC>;
///Register `SD_INFO2_MASK` writer
pub type W = crate::W<SD_INFO2_MASK_SPEC>;
/**Command Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDEM_A {
    ///0: Command error interrupt request not masked
    _0 = 0,
    ///1: Command error interrupt request masked
    _1 = 1,
}
impl From<CMDEM_A> for bool {
    #[inline(always)]
    fn from(variant: CMDEM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDEM` reader - Command Error Interrupt Request Mask
pub type CMDEM_R = crate::BitReader<CMDEM_A>;
impl CMDEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDEM_A {
        match self.bits {
            false => CMDEM_A::_0,
            true => CMDEM_A::_1,
        }
    }
    ///Command error interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDEM_A::_0
    }
    ///Command error interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDEM_A::_1
    }
}
///Field `CMDEM` writer - Command Error Interrupt Request Mask
pub type CMDEM_W<'a, REG> = crate::BitWriter<'a, REG, CMDEM_A>;
impl<'a, REG> CMDEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command error interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMDEM_A::_0)
    }
    ///Command error interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDEM_A::_1)
    }
}
/**CRC Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEM_A {
    ///0: CRC error interrupt request not masked
    _0 = 0,
    ///1: CRC error interrupt request masked
    _1 = 1,
}
impl From<CRCEM_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEM` reader - CRC Error Interrupt Request Mask
pub type CRCEM_R = crate::BitReader<CRCEM_A>;
impl CRCEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEM_A {
        match self.bits {
            false => CRCEM_A::_0,
            true => CRCEM_A::_1,
        }
    }
    ///CRC error interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCEM_A::_0
    }
    ///CRC error interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCEM_A::_1
    }
}
///Field `CRCEM` writer - CRC Error Interrupt Request Mask
pub type CRCEM_W<'a, REG> = crate::BitWriter<'a, REG, CRCEM_A>;
impl<'a, REG> CRCEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEM_A::_0)
    }
    ///CRC error interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEM_A::_1)
    }
}
/**End Bit Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDEM_A {
    ///0: End bit detection error interrupt request not masked
    _0 = 0,
    ///1: End bit detection error interrupt request masked
    _1 = 1,
}
impl From<ENDEM_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENDEM` reader - End Bit Error Interrupt Request Mask
pub type ENDEM_R = crate::BitReader<ENDEM_A>;
impl ENDEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENDEM_A {
        match self.bits {
            false => ENDEM_A::_0,
            true => ENDEM_A::_1,
        }
    }
    ///End bit detection error interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDEM_A::_0
    }
    ///End bit detection error interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDEM_A::_1
    }
}
///Field `ENDEM` writer - End Bit Error Interrupt Request Mask
pub type ENDEM_W<'a, REG> = crate::BitWriter<'a, REG, ENDEM_A>;
impl<'a, REG> ENDEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End bit detection error interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENDEM_A::_0)
    }
    ///End bit detection error interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDEM_A::_1)
    }
}
/**Data Timeout Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOM_A {
    ///0: Data timeout interrupt request not masked
    _0 = 0,
    ///1: Data timeout interrupt request masked
    _1 = 1,
}
impl From<DTOM_A> for bool {
    #[inline(always)]
    fn from(variant: DTOM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTOM` reader - Data Timeout Interrupt Request Mask
pub type DTOM_R = crate::BitReader<DTOM_A>;
impl DTOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTOM_A {
        match self.bits {
            false => DTOM_A::_0,
            true => DTOM_A::_1,
        }
    }
    ///Data timeout interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOM_A::_0
    }
    ///Data timeout interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOM_A::_1
    }
}
///Field `DTOM` writer - Data Timeout Interrupt Request Mask
pub type DTOM_W<'a, REG> = crate::BitWriter<'a, REG, DTOM_A>;
impl<'a, REG> DTOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data timeout interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTOM_A::_0)
    }
    ///Data timeout interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTOM_A::_1)
    }
}
/**SD_BUF Register Illegal Write Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILWM_A {
    ///0: Illegal write detection interrupt request for the SD_BUF register not masked
    _0 = 0,
    ///1: Illegal write detection interrupt request for the SD_BUF register masked
    _1 = 1,
}
impl From<ILWM_A> for bool {
    #[inline(always)]
    fn from(variant: ILWM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ILWM` reader - SD_BUF Register Illegal Write Interrupt Request Mask
pub type ILWM_R = crate::BitReader<ILWM_A>;
impl ILWM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILWM_A {
        match self.bits {
            false => ILWM_A::_0,
            true => ILWM_A::_1,
        }
    }
    ///Illegal write detection interrupt request for the SD_BUF register not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILWM_A::_0
    }
    ///Illegal write detection interrupt request for the SD_BUF register masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILWM_A::_1
    }
}
///Field `ILWM` writer - SD_BUF Register Illegal Write Interrupt Request Mask
pub type ILWM_W<'a, REG> = crate::BitWriter<'a, REG, ILWM_A>;
impl<'a, REG> ILWM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal write detection interrupt request for the SD_BUF register not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILWM_A::_0)
    }
    ///Illegal write detection interrupt request for the SD_BUF register masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILWM_A::_1)
    }
}
/**SD_BUF Register Illegal Read Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILRM_A {
    ///0: Illegal read detection interrupt request for the SD_BUF register not masked
    _0 = 0,
    ///1: Illegal read detection interrupt request for the SD_BUF register masked
    _1 = 1,
}
impl From<ILRM_A> for bool {
    #[inline(always)]
    fn from(variant: ILRM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ILRM` reader - SD_BUF Register Illegal Read Interrupt Request Mask
pub type ILRM_R = crate::BitReader<ILRM_A>;
impl ILRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILRM_A {
        match self.bits {
            false => ILRM_A::_0,
            true => ILRM_A::_1,
        }
    }
    ///Illegal read detection interrupt request for the SD_BUF register not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILRM_A::_0
    }
    ///Illegal read detection interrupt request for the SD_BUF register masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILRM_A::_1
    }
}
///Field `ILRM` writer - SD_BUF Register Illegal Read Interrupt Request Mask
pub type ILRM_W<'a, REG> = crate::BitWriter<'a, REG, ILRM_A>;
impl<'a, REG> ILRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal read detection interrupt request for the SD_BUF register not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILRM_A::_0)
    }
    ///Illegal read detection interrupt request for the SD_BUF register masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILRM_A::_1)
    }
}
/**Response Timeout Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTOM_A {
    ///0: Response timeout interrupt request not masked
    _0 = 0,
    ///1: Response timeout interrupt request masked
    _1 = 1,
}
impl From<RSPTOM_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTOM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTOM` reader - Response Timeout Interrupt Request Mask
pub type RSPTOM_R = crate::BitReader<RSPTOM_A>;
impl RSPTOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPTOM_A {
        match self.bits {
            false => RSPTOM_A::_0,
            true => RSPTOM_A::_1,
        }
    }
    ///Response timeout interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTOM_A::_0
    }
    ///Response timeout interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTOM_A::_1
    }
}
///Field `RSPTOM` writer - Response Timeout Interrupt Request Mask
pub type RSPTOM_W<'a, REG> = crate::BitWriter<'a, REG, RSPTOM_A>;
impl<'a, REG> RSPTOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Response timeout interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTOM_A::_0)
    }
    ///Response timeout interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTOM_A::_1)
    }
}
/**BRE Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREM_A {
    ///0: Read enable interrupt request for the SD buffer not masked
    _0 = 0,
    ///1: Read enable interrupt request for the SD buffer masked
    _1 = 1,
}
impl From<BREM_A> for bool {
    #[inline(always)]
    fn from(variant: BREM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BREM` reader - BRE Interrupt Request Mask
pub type BREM_R = crate::BitReader<BREM_A>;
impl BREM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BREM_A {
        match self.bits {
            false => BREM_A::_0,
            true => BREM_A::_1,
        }
    }
    ///Read enable interrupt request for the SD buffer not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BREM_A::_0
    }
    ///Read enable interrupt request for the SD buffer masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BREM_A::_1
    }
}
///Field `BREM` writer - BRE Interrupt Request Mask
pub type BREM_W<'a, REG> = crate::BitWriter<'a, REG, BREM_A>;
impl<'a, REG> BREM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read enable interrupt request for the SD buffer not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BREM_A::_0)
    }
    ///Read enable interrupt request for the SD buffer masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BREM_A::_1)
    }
}
/**BWE Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWEM_A {
    ///0: Write enable interrupt request for the SD_BUF register not masked
    _0 = 0,
    ///1: Write enable interrupt request for the SD_BUF register masked
    _1 = 1,
}
impl From<BWEM_A> for bool {
    #[inline(always)]
    fn from(variant: BWEM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BWEM` reader - BWE Interrupt Request Mask
pub type BWEM_R = crate::BitReader<BWEM_A>;
impl BWEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BWEM_A {
        match self.bits {
            false => BWEM_A::_0,
            true => BWEM_A::_1,
        }
    }
    ///Write enable interrupt request for the SD_BUF register not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWEM_A::_0
    }
    ///Write enable interrupt request for the SD_BUF register masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWEM_A::_1
    }
}
///Field `BWEM` writer - BWE Interrupt Request Mask
pub type BWEM_W<'a, REG> = crate::BitWriter<'a, REG, BWEM_A>;
impl<'a, REG> BWEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write enable interrupt request for the SD_BUF register not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BWEM_A::_0)
    }
    ///Write enable interrupt request for the SD_BUF register masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BWEM_A::_1)
    }
}
/**Illegal Access Error Interrupt Request Mask

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILAM_A {
    ///0: Illegal access error interrupt request not masked
    _0 = 0,
    ///1: Illegal access error interrupt request masked
    _1 = 1,
}
impl From<ILAM_A> for bool {
    #[inline(always)]
    fn from(variant: ILAM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ILAM` reader - Illegal Access Error Interrupt Request Mask
pub type ILAM_R = crate::BitReader<ILAM_A>;
impl ILAM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILAM_A {
        match self.bits {
            false => ILAM_A::_0,
            true => ILAM_A::_1,
        }
    }
    ///Illegal access error interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILAM_A::_0
    }
    ///Illegal access error interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILAM_A::_1
    }
}
///Field `ILAM` writer - Illegal Access Error Interrupt Request Mask
pub type ILAM_W<'a, REG> = crate::BitWriter<'a, REG, ILAM_A>;
impl<'a, REG> ILAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal access error interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILAM_A::_0)
    }
    ///Illegal access error interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILAM_A::_1)
    }
}
impl R {
    ///Bit 0 - Command Error Interrupt Request Mask
    #[inline(always)]
    pub fn cmdem(&self) -> CMDEM_R {
        CMDEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRC Error Interrupt Request Mask
    #[inline(always)]
    pub fn crcem(&self) -> CRCEM_R {
        CRCEM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End Bit Error Interrupt Request Mask
    #[inline(always)]
    pub fn endem(&self) -> ENDEM_R {
        ENDEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn dtom(&self) -> DTOM_R {
        DTOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SD_BUF Register Illegal Write Interrupt Request Mask
    #[inline(always)]
    pub fn ilwm(&self) -> ILWM_R {
        ILWM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SD_BUF Register Illegal Read Interrupt Request Mask
    #[inline(always)]
    pub fn ilrm(&self) -> ILRM_R {
        ILRM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Response Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn rsptom(&self) -> RSPTOM_R {
        RSPTOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BRE Interrupt Request Mask
    #[inline(always)]
    pub fn brem(&self) -> BREM_R {
        BREM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BWE Interrupt Request Mask
    #[inline(always)]
    pub fn bwem(&self) -> BWEM_R {
        BWEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Illegal Access Error Interrupt Request Mask
    #[inline(always)]
    pub fn ilam(&self) -> ILAM_R {
        ILAM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Command Error Interrupt Request Mask
    #[inline(always)]
    pub fn cmdem(&mut self) -> CMDEM_W<SD_INFO2_MASK_SPEC> {
        CMDEM_W::new(self, 0)
    }
    ///Bit 1 - CRC Error Interrupt Request Mask
    #[inline(always)]
    pub fn crcem(&mut self) -> CRCEM_W<SD_INFO2_MASK_SPEC> {
        CRCEM_W::new(self, 1)
    }
    ///Bit 2 - End Bit Error Interrupt Request Mask
    #[inline(always)]
    pub fn endem(&mut self) -> ENDEM_W<SD_INFO2_MASK_SPEC> {
        ENDEM_W::new(self, 2)
    }
    ///Bit 3 - Data Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn dtom(&mut self) -> DTOM_W<SD_INFO2_MASK_SPEC> {
        DTOM_W::new(self, 3)
    }
    ///Bit 4 - SD_BUF Register Illegal Write Interrupt Request Mask
    #[inline(always)]
    pub fn ilwm(&mut self) -> ILWM_W<SD_INFO2_MASK_SPEC> {
        ILWM_W::new(self, 4)
    }
    ///Bit 5 - SD_BUF Register Illegal Read Interrupt Request Mask
    #[inline(always)]
    pub fn ilrm(&mut self) -> ILRM_W<SD_INFO2_MASK_SPEC> {
        ILRM_W::new(self, 5)
    }
    ///Bit 6 - Response Timeout Interrupt Request Mask
    #[inline(always)]
    pub fn rsptom(&mut self) -> RSPTOM_W<SD_INFO2_MASK_SPEC> {
        RSPTOM_W::new(self, 6)
    }
    ///Bit 8 - BRE Interrupt Request Mask
    #[inline(always)]
    pub fn brem(&mut self) -> BREM_W<SD_INFO2_MASK_SPEC> {
        BREM_W::new(self, 8)
    }
    ///Bit 9 - BWE Interrupt Request Mask
    #[inline(always)]
    pub fn bwem(&mut self) -> BWEM_W<SD_INFO2_MASK_SPEC> {
        BWEM_W::new(self, 9)
    }
    ///Bit 15 - Illegal Access Error Interrupt Request Mask
    #[inline(always)]
    pub fn ilam(&mut self) -> ILAM_W<SD_INFO2_MASK_SPEC> {
        ILAM_W::new(self, 15)
    }
}
/**SD_INFO2 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info2_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_INFO2_MASK_SPEC;
impl crate::RegisterSpec for SD_INFO2_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_info2_mask::R`](R) reader structure
impl crate::Readable for SD_INFO2_MASK_SPEC {}
///`write(|w| ..)` method takes [`sd_info2_mask::W`](W) writer structure
impl crate::Writable for SD_INFO2_MASK_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_INFO2_MASK to value 0x8b7f
impl crate::Resettable for SD_INFO2_MASK_SPEC {
    const RESET_VALUE: u32 = 0x8b7f;
}
