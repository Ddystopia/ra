///Register `SD_OPTION` reader
pub type R = crate::R<SD_OPTION_SPEC>;
///Register `SD_OPTION` writer
pub type W = crate::W<SD_OPTION_SPEC>;
/**Card Detect Time Counter

Value on reset: 14*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTOP_A {
    ///15: Setting prohibited
    _1111 = 15,
    ///0: IMCLK x 2^(CTOP+10)
    OTHERS = 0,
}
impl From<CTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: CTOP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTOP_A {
    type Ux = u8;
}
impl crate::IsEnum for CTOP_A {}
///Field `CTOP` reader - Card Detect Time Counter
pub type CTOP_R = crate::FieldReader<CTOP_A>;
impl CTOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTOP_A {
        match self.bits {
            15 => CTOP_A::_1111,
            _ => CTOP_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CTOP_A::_1111
    }
    ///IMCLK x 2^(CTOP+10)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CTOP_A::OTHERS)
    }
}
///Field `CTOP` writer - Card Detect Time Counter
pub type CTOP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CTOP_A, crate::Safe>;
impl<'a, REG> CTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(CTOP_A::_1111)
    }
    ///IMCLK x 2^(CTOP+10)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CTOP_A::OTHERS)
    }
}
/**Timeout Counter

Value on reset: 14*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOP_A {
    ///15: Setting prohibited
    _1111 = 15,
    ///0: SDHI clock x 2^(TOP+13)
    OTHERS = 0,
}
impl From<TOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TOP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOP_A {
    type Ux = u8;
}
impl crate::IsEnum for TOP_A {}
///Field `TOP` reader - Timeout Counter
pub type TOP_R = crate::FieldReader<TOP_A>;
impl TOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOP_A {
        match self.bits {
            15 => TOP_A::_1111,
            _ => TOP_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TOP_A::_1111
    }
    ///SDHI clock x 2^(TOP+13)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TOP_A::OTHERS)
    }
}
///Field `TOP` writer - Timeout Counter
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TOP_A, crate::Safe>;
impl<'a, REG> TOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(TOP_A::_1111)
    }
    ///SDHI clock x 2^(TOP+13)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TOP_A::OTHERS)
    }
}
/**Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOUTMASK_A {
    ///0: Activate Timeout
    _0 = 0,
    ///1: Inactivate Timeout(RSPTO bit and DTO bit of SD_INFO2 and SD_ERR_STS2 won't be set)
    _1 = 1,
}
impl From<TOUTMASK_A> for bool {
    #[inline(always)]
    fn from(variant: TOUTMASK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOUTMASK` reader - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence.
pub type TOUTMASK_R = crate::BitReader<TOUTMASK_A>;
impl TOUTMASK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOUTMASK_A {
        match self.bits {
            false => TOUTMASK_A::_0,
            true => TOUTMASK_A::_1,
        }
    }
    ///Activate Timeout
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOUTMASK_A::_0
    }
    ///Inactivate Timeout(RSPTO bit and DTO bit of SD_INFO2 and SD_ERR_STS2 won't be set)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOUTMASK_A::_1
    }
}
///Field `TOUTMASK` writer - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence.
pub type TOUTMASK_W<'a, REG> = crate::BitWriter<'a, REG, TOUTMASK_A>;
impl<'a, REG> TOUTMASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Activate Timeout
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOUTMASK_A::_0)
    }
    ///Inactivate Timeout(RSPTO bit and DTO bit of SD_INFO2 and SD_ERR_STS2 won't be set)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOUTMASK_A::_1)
    }
}
///Field `WIDTH8` reader - Bus Widthsee b15, WIDTH bit
pub type WIDTH8_R = crate::BitReader;
///Field `WIDTH8` writer - Bus Widthsee b15, WIDTH bit
pub type WIDTH8_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIDTH_A {
    ///0: 4-bit width (WIDTH8=0) / 8-bit width (WIDTH8=1)
    _0 = 0,
    ///1: 1-bit width (WIDTH8=0 or 1 )
    _1 = 1,
}
impl From<WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WIDTH` reader - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0.
pub type WIDTH_R = crate::BitReader<WIDTH_A>;
impl WIDTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WIDTH_A {
        match self.bits {
            false => WIDTH_A::_0,
            true => WIDTH_A::_1,
        }
    }
    ///4-bit width (WIDTH8=0) / 8-bit width (WIDTH8=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WIDTH_A::_0
    }
    ///1-bit width (WIDTH8=0 or 1 )
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WIDTH_A::_1
    }
}
///Field `WIDTH` writer - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0.
pub type WIDTH_W<'a, REG> = crate::BitWriter<'a, REG, WIDTH_A>;
impl<'a, REG> WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///4-bit width (WIDTH8=0) / 8-bit width (WIDTH8=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTH_A::_0)
    }
    ///1-bit width (WIDTH8=0 or 1 )
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTH_A::_1)
    }
}
impl R {
    ///Bits 0:3 - Card Detect Time Counter
    #[inline(always)]
    pub fn ctop(&self) -> CTOP_R {
        CTOP_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Timeout Counter
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence.
    #[inline(always)]
    pub fn toutmask(&self) -> TOUTMASK_R {
        TOUTMASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 13 - Bus Widthsee b15, WIDTH bit
    #[inline(always)]
    pub fn width8(&self) -> WIDTH8_R {
        WIDTH8_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0.
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Card Detect Time Counter
    #[inline(always)]
    pub fn ctop(&mut self) -> CTOP_W<SD_OPTION_SPEC> {
        CTOP_W::new(self, 0)
    }
    ///Bits 4:7 - Timeout Counter
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W<SD_OPTION_SPEC> {
        TOP_W::new(self, 4)
    }
    ///Bit 8 - Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence.
    #[inline(always)]
    pub fn toutmask(&mut self) -> TOUTMASK_W<SD_OPTION_SPEC> {
        TOUTMASK_W::new(self, 8)
    }
    ///Bit 13 - Bus Widthsee b15, WIDTH bit
    #[inline(always)]
    pub fn width8(&mut self) -> WIDTH8_W<SD_OPTION_SPEC> {
        WIDTH8_W::new(self, 13)
    }
    ///Bit 15 - Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0.
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W<SD_OPTION_SPEC> {
        WIDTH_W::new(self, 15)
    }
}
/**SD Card Access Control Option Register

You can [`read`](crate::Reg::read) this register and get [`sd_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_OPTION_SPEC;
impl crate::RegisterSpec for SD_OPTION_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_option::R`](R) reader structure
impl crate::Readable for SD_OPTION_SPEC {}
///`write(|w| ..)` method takes [`sd_option::W`](W) writer structure
impl crate::Writable for SD_OPTION_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_OPTION to value 0x40ee
impl crate::Resettable for SD_OPTION_SPEC {
    const RESET_VALUE: u32 = 0x40ee;
}
