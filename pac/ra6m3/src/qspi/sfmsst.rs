///Register `SFMSST` reader
pub type R = crate::R<SFMSST_SPEC>;
/**Number of bytes of prefetched dataRange: 00000 - 10010 (No combination other than the above is available.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PFCNT_A {
    ///0: Nodata has been prefetched.
    _00000 = 0,
    ///1: Data of (PFCNT) bytes hs been prefetched.
    OTHERS = 1,
}
impl From<PFCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: PFCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PFCNT_A {
    type Ux = u8;
}
impl crate::IsEnum for PFCNT_A {}
///Field `PFCNT` reader - Number of bytes of prefetched dataRange: 00000 - 10010 (No combination other than the above is available.)
pub type PFCNT_R = crate::FieldReader<PFCNT_A>;
impl PFCNT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFCNT_A {
        match self.bits {
            0 => PFCNT_A::_00000,
            _ => PFCNT_A::OTHERS,
        }
    }
    ///Nodata has been prefetched.
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == PFCNT_A::_00000
    }
    ///Data of (PFCNT) bytes hs been prefetched.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PFCNT_A::OTHERS)
    }
}
/**Prefetch buffer state

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFFUL_A {
    ///0: The prefetch buffer has a free space.
    _0 = 0,
    ///1: The prefetch buffer is full.
    _1 = 1,
}
impl From<PFFUL_A> for bool {
    #[inline(always)]
    fn from(variant: PFFUL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PFFUL` reader - Prefetch buffer state
pub type PFFUL_R = crate::BitReader<PFFUL_A>;
impl PFFUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFFUL_A {
        match self.bits {
            false => PFFUL_A::_0,
            true => PFFUL_A::_1,
        }
    }
    ///The prefetch buffer has a free space.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFFUL_A::_0
    }
    ///The prefetch buffer is full.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFFUL_A::_1
    }
}
/**Prefetch function operation state

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFOFF_A {
    ///0: The prefetch function is operating.
    _0 = 0,
    ///1: The prefetch function is not enabled or is not operating.
    _1 = 1,
}
impl From<PFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: PFOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PFOFF` reader - Prefetch function operation state
pub type PFOFF_R = crate::BitReader<PFOFF_A>;
impl PFOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFOFF_A {
        match self.bits {
            false => PFOFF_A::_0,
            true => PFOFF_A::_1,
        }
    }
    ///The prefetch function is operating.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFOFF_A::_0
    }
    ///The prefetch function is not enabled or is not operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFOFF_A::_1
    }
}
impl R {
    ///Bits 0:4 - Number of bytes of prefetched dataRange: 00000 - 10010 (No combination other than the above is available.)
    #[inline(always)]
    pub fn pfcnt(&self) -> PFCNT_R {
        PFCNT_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - Prefetch buffer state
    #[inline(always)]
    pub fn pfful(&self) -> PFFUL_R {
        PFFUL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Prefetch function operation state
    #[inline(always)]
    pub fn pfoff(&self) -> PFOFF_R {
        PFOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmsst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSST_SPEC;
impl crate::RegisterSpec for SFMSST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmsst::R`](R) reader structure
impl crate::Readable for SFMSST_SPEC {}
///`reset()` method sets SFMSST to value 0x80
impl crate::Resettable for SFMSST_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
