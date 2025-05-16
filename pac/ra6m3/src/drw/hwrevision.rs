///Register `HWREVISION` reader
pub type R = crate::R<HWREVISION_SPEC>;
///Field `REV` reader - Revision number
pub type REV_R = crate::FieldReader<u16>;
/**Display list reader feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLR_A {
    ///0: Display list reader unavailable
    _0 = 0,
    ///1: Display list reader available
    _1 = 1,
}
impl From<DLR_A> for bool {
    #[inline(always)]
    fn from(variant: DLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLR` reader - Display list reader feature
pub type DLR_R = crate::BitReader<DLR_A>;
impl DLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLR_A {
        match self.bits {
            false => DLR_A::_0,
            true => DLR_A::_1,
        }
    }
    ///Display list reader unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLR_A::_0
    }
    ///Display list reader available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLR_A::_1
    }
}
/**Framebuffer cache feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBCACHE_A {
    ///0: Framebuffer cache unavailable
    _0 = 0,
    ///1: Framebuffer cache available
    _1 = 1,
}
impl From<FBCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: FBCACHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FBCACHE` reader - Framebuffer cache feature
pub type FBCACHE_R = crate::BitReader<FBCACHE_A>;
impl FBCACHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FBCACHE_A {
        match self.bits {
            false => FBCACHE_A::_0,
            true => FBCACHE_A::_1,
        }
    }
    ///Framebuffer cache unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FBCACHE_A::_0
    }
    ///Framebuffer cache available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FBCACHE_A::_1
    }
}
/**Texture cache feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCACHE_A {
    ///0: Texture cache unavailable
    _0 = 0,
    ///1: Texture cache available
    _1 = 1,
}
impl From<TXCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: TXCACHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXCACHE` reader - Texture cache feature
pub type TXCACHE_R = crate::BitReader<TXCACHE_A>;
impl TXCACHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXCACHE_A {
        match self.bits {
            false => TXCACHE_A::_0,
            true => TXCACHE_A::_1,
        }
    }
    ///Texture cache unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXCACHE_A::_0
    }
    ///Texture cache available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXCACHE_A::_1
    }
}
/**Two performance counter feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERFCOUNT_A {
    ///0: Two performance counter unavailable
    _0 = 0,
    ///1: Two performance counter available
    _1 = 1,
}
impl From<PERFCOUNT_A> for bool {
    #[inline(always)]
    fn from(variant: PERFCOUNT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PERFCOUNT` reader - Two performance counter feature
pub type PERFCOUNT_R = crate::BitReader<PERFCOUNT_A>;
impl PERFCOUNT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PERFCOUNT_A {
        match self.bits {
            false => PERFCOUNT_A::_0,
            true => PERFCOUNT_A::_1,
        }
    }
    ///Two performance counter unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PERFCOUNT_A::_0
    }
    ///Two performance counter available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PERFCOUNT_A::_1
    }
}
/**Texture CLUT with 16 or 256 entries feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXCLU_A {
    ///0: Texture CLUT with 16 or 256 entries unavailable
    _0 = 0,
    ///1: Texture CLUT with 16 or 256 entries available
    _1 = 1,
}
impl From<TEXCLU_A> for bool {
    #[inline(always)]
    fn from(variant: TEXCLU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXCLU` reader - Texture CLUT with 16 or 256 entries feature
pub type TEXCLU_R = crate::BitReader<TEXCLU_A>;
impl TEXCLU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEXCLU_A {
        match self.bits {
            false => TEXCLU_A::_0,
            true => TEXCLU_A::_1,
        }
    }
    ///Texture CLUT with 16 or 256 entries unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEXCLU_A::_0
    }
    ///Texture CLUT with 16 or 256 entries available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEXCLU_A::_1
    }
}
/**RLE unit feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLEUNIT_A {
    ///0: RLE unit unavailable
    _0 = 0,
    ///1: RLE unit available
    _1 = 1,
}
impl From<RLEUNIT_A> for bool {
    #[inline(always)]
    fn from(variant: RLEUNIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RLEUNIT` reader - RLE unit feature
pub type RLEUNIT_R = crate::BitReader<RLEUNIT_A>;
impl RLEUNIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RLEUNIT_A {
        match self.bits {
            false => RLEUNIT_A::_0,
            true => RLEUNIT_A::_1,
        }
    }
    ///RLE unit unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RLEUNIT_A::_0
    }
    ///RLE unit available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RLEUNIT_A::_1
    }
}
/**Texture CLUT feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXCLUT256_A {
    ///0: Texture CLUT unavailable
    _0 = 0,
    ///1: Texture CLUT available
    _1 = 1,
}
impl From<TEXCLUT256_A> for bool {
    #[inline(always)]
    fn from(variant: TEXCLUT256_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXCLUT256` reader - Texture CLUT feature
pub type TEXCLUT256_R = crate::BitReader<TEXCLUT256_A>;
impl TEXCLUT256_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEXCLUT256_A {
        match self.bits {
            false => TEXCLUT256_A::_0,
            true => TEXCLUT256_A::_1,
        }
    }
    ///Texture CLUT unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEXCLUT256_A::_0
    }
    ///Texture CLUT available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEXCLUT256_A::_1
    }
}
/**Colorkey feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLORKEY_A {
    ///0: Colorkey unavailable
    _0 = 0,
    ///1: Colorkey available
    _1 = 1,
}
impl From<COLORKEY_A> for bool {
    #[inline(always)]
    fn from(variant: COLORKEY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COLORKEY` reader - Colorkey feature
pub type COLORKEY_R = crate::BitReader<COLORKEY_A>;
impl COLORKEY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COLORKEY_A {
        match self.bits {
            false => COLORKEY_A::_0,
            true => COLORKEY_A::_1,
        }
    }
    ///Colorkey unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COLORKEY_A::_0
    }
    ///Colorkey available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COLORKEY_A::_1
    }
}
/**Alpha channel blending feature

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACBLEND_A {
    ///0: Alpha channel blending unavailable
    _0 = 0,
    ///1: Alpha channel blending available
    _1 = 1,
}
impl From<ACBLEND_A> for bool {
    #[inline(always)]
    fn from(variant: ACBLEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACBLEND` reader - Alpha channel blending feature
pub type ACBLEND_R = crate::BitReader<ACBLEND_A>;
impl ACBLEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACBLEND_A {
        match self.bits {
            false => ACBLEND_A::_0,
            true => ACBLEND_A::_1,
        }
    }
    ///Alpha channel blending unavailable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACBLEND_A::_0
    }
    ///Alpha channel blending available
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACBLEND_A::_1
    }
}
impl R {
    ///Bits 0:11 - Revision number
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 17 - Display list reader feature
    #[inline(always)]
    pub fn dlr(&self) -> DLR_R {
        DLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Framebuffer cache feature
    #[inline(always)]
    pub fn fbcache(&self) -> FBCACHE_R {
        FBCACHE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Texture cache feature
    #[inline(always)]
    pub fn txcache(&self) -> TXCACHE_R {
        TXCACHE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Two performance counter feature
    #[inline(always)]
    pub fn perfcount(&self) -> PERFCOUNT_R {
        PERFCOUNT_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Texture CLUT with 16 or 256 entries feature
    #[inline(always)]
    pub fn texclu(&self) -> TEXCLU_R {
        TEXCLU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - RLE unit feature
    #[inline(always)]
    pub fn rleunit(&self) -> RLEUNIT_R {
        RLEUNIT_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Texture CLUT feature
    #[inline(always)]
    pub fn texclut256(&self) -> TEXCLUT256_R {
        TEXCLUT256_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Colorkey feature
    #[inline(always)]
    pub fn colorkey(&self) -> COLORKEY_R {
        COLORKEY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Alpha channel blending feature
    #[inline(always)]
    pub fn acblend(&self) -> ACBLEND_R {
        ACBLEND_R::new(((self.bits >> 27) & 1) != 0)
    }
}
/**Hardware Version and Feature Set ID Register

You can [`read`](crate::Reg::read) this register and get [`hwrevision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HWREVISION_SPEC;
impl crate::RegisterSpec for HWREVISION_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hwrevision::R`](R) reader structure
impl crate::Readable for HWREVISION_SPEC {}
///`reset()` method sets HWREVISION to value 0x0fbe_0107
impl crate::Resettable for HWREVISION_SPEC {
    const RESET_VALUE: u32 = 0x0fbe_0107;
}
