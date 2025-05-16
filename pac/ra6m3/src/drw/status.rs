///Register `STATUS` reader
pub type R = crate::R<STATUS_SPEC>;
/**Enumeration unit status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYENUM_A {
    ///0: enumeration unit idle
    _0 = 0,
    ///1: enumeration unit busy, new primitive can not be started
    _1 = 1,
}
impl From<BUSYENUM_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYENUM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSYENUM` reader - Enumeration unit status
pub type BUSYENUM_R = crate::BitReader<BUSYENUM_A>;
impl BUSYENUM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSYENUM_A {
        match self.bits {
            false => BUSYENUM_A::_0,
            true => BUSYENUM_A::_1,
        }
    }
    ///enumeration unit idle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSYENUM_A::_0
    }
    ///enumeration unit busy, new primitive can not be started
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSYENUM_A::_1
    }
}
/**Framebuffer writeback status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYWRITE_A {
    ///0: framebuffer writeback finished
    _0 = 0,
    ///1: framebuffer writeback busy, framebuffer type can not be changed
    _1 = 1,
}
impl From<BUSYWRITE_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYWRITE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSYWRITE` reader - Framebuffer writeback status
pub type BUSYWRITE_R = crate::BitReader<BUSYWRITE_A>;
impl BUSYWRITE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSYWRITE_A {
        match self.bits {
            false => BUSYWRITE_A::_0,
            true => BUSYWRITE_A::_1,
        }
    }
    ///framebuffer writeback finished
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSYWRITE_A::_0
    }
    ///framebuffer writeback busy, framebuffer type can not be changed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSYWRITE_A::_1
    }
}
/**Framebuffer cache status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHEDIRTY_A {
    ///0: framebuffer cache is not dirty
    _0 = 0,
    ///1: framebuffer cache is dirty, frame should not be flipped
    _1 = 1,
}
impl From<CACHEDIRTY_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEDIRTY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CACHEDIRTY` reader - Framebuffer cache status
pub type CACHEDIRTY_R = crate::BitReader<CACHEDIRTY_A>;
impl CACHEDIRTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CACHEDIRTY_A {
        match self.bits {
            false => CACHEDIRTY_A::_0,
            true => CACHEDIRTY_A::_1,
        }
    }
    ///framebuffer cache is not dirty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CACHEDIRTY_A::_0
    }
    ///framebuffer cache is dirty, frame should not be flipped
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CACHEDIRTY_A::_1
    }
}
/**Display list reader status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLISTACTIVE_A {
    ///0: display list reader is idle
    _0 = 0,
    ///1: display list reader busy, no direct write access to registers allowed
    _1 = 1,
}
impl From<DLISTACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: DLISTACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLISTACTIVE` reader - Display list reader status
pub type DLISTACTIVE_R = crate::BitReader<DLISTACTIVE_A>;
impl DLISTACTIVE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLISTACTIVE_A {
        match self.bits {
            false => DLISTACTIVE_A::_0,
            true => DLISTACTIVE_A::_1,
        }
    }
    ///display list reader is idle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLISTACTIVE_A::_0
    }
    ///display list reader busy, no direct write access to registers allowed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLISTACTIVE_A::_1
    }
}
/**enumeration finished interrupt triggered

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUMIRQ_A {
    ///0: enumeration not finished or interrupt disabled
    _0 = 0,
    ///1: enumeration finished interrupt triggered
    _1 = 1,
}
impl From<ENUMIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ENUMIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENUMIRQ` reader - enumeration finished interrupt triggered
pub type ENUMIRQ_R = crate::BitReader<ENUMIRQ_A>;
impl ENUMIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENUMIRQ_A {
        match self.bits {
            false => ENUMIRQ_A::_0,
            true => ENUMIRQ_A::_1,
        }
    }
    ///enumeration not finished or interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENUMIRQ_A::_0
    }
    ///enumeration finished interrupt triggered
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENUMIRQ_A::_1
    }
}
/**display list finished interrupt triggered

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLISTIRQ_A {
    ///0: display list not finished or interrupt disabled
    _0 = 0,
    ///1: display list finished interrupt triggered
    _1 = 1,
}
impl From<DLISTIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DLISTIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLISTIRQ` reader - display list finished interrupt triggered
pub type DLISTIRQ_R = crate::BitReader<DLISTIRQ_A>;
impl DLISTIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLISTIRQ_A {
        match self.bits {
            false => DLISTIRQ_A::_0,
            true => DLISTIRQ_A::_1,
        }
    }
    ///display list not finished or interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLISTIRQ_A::_0
    }
    ///display list finished interrupt triggered
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLISTIRQ_A::_1
    }
}
/**bus error interrupt triggered

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSIRQ_A {
    ///0: no bus error occurred or interrupt disabled
    _0 = 0,
    ///1: bus error interrupt triggered
    _1 = 1,
}
impl From<BUSIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: BUSIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSIRQ` reader - bus error interrupt triggered
pub type BUSIRQ_R = crate::BitReader<BUSIRQ_A>;
impl BUSIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSIRQ_A {
        match self.bits {
            false => BUSIRQ_A::_0,
            true => BUSIRQ_A::_1,
        }
    }
    ///no bus error occurred or interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSIRQ_A::_0
    }
    ///bus error interrupt triggered
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSIRQ_A::_1
    }
}
/**framebuffer bus error interrupt triggered

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSERRMFB_A {
    ///0: no framebuffer bus error occured or interrupt disabled
    _0 = 0,
    ///1: framebuffer bus error interrupt triggered
    _1 = 1,
}
impl From<BUSERRMFB_A> for bool {
    #[inline(always)]
    fn from(variant: BUSERRMFB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSERRMFB` reader - framebuffer bus error interrupt triggered
pub type BUSERRMFB_R = crate::BitReader<BUSERRMFB_A>;
impl BUSERRMFB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSERRMFB_A {
        match self.bits {
            false => BUSERRMFB_A::_0,
            true => BUSERRMFB_A::_1,
        }
    }
    ///no framebuffer bus error occured or interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSERRMFB_A::_0
    }
    ///framebuffer bus error interrupt triggered
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSERRMFB_A::_1
    }
}
/**texture bus error interrupt triggered

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSERRMTXMRL_A {
    ///0: no texture bus error occurred or interrupt disabled
    _0 = 0,
    ///1: texture bus error interrupt triggered
    _1 = 1,
}
impl From<BUSERRMTXMRL_A> for bool {
    #[inline(always)]
    fn from(variant: BUSERRMTXMRL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSERRMTXMRL` reader - texture bus error interrupt triggered
pub type BUSERRMTXMRL_R = crate::BitReader<BUSERRMTXMRL_A>;
impl BUSERRMTXMRL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSERRMTXMRL_A {
        match self.bits {
            false => BUSERRMTXMRL_A::_0,
            true => BUSERRMTXMRL_A::_1,
        }
    }
    ///no texture bus error occurred or interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSERRMTXMRL_A::_0
    }
    ///texture bus error interrupt triggered
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSERRMTXMRL_A::_1
    }
}
/**display list bus error interrupt triggered

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSERRMDL_A {
    ///0: no display list bus error occurred or interrupt disabled
    _0 = 0,
    ///1: display list bus error interrupt triggered
    _1 = 1,
}
impl From<BUSERRMDL_A> for bool {
    #[inline(always)]
    fn from(variant: BUSERRMDL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSERRMDL` reader - display list bus error interrupt triggered
pub type BUSERRMDL_R = crate::BitReader<BUSERRMDL_A>;
impl BUSERRMDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSERRMDL_A {
        match self.bits {
            false => BUSERRMDL_A::_0,
            true => BUSERRMDL_A::_1,
        }
    }
    ///no display list bus error occurred or interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSERRMDL_A::_0
    }
    ///display list bus error interrupt triggered
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSERRMDL_A::_1
    }
}
impl R {
    ///Bit 0 - Enumeration unit status
    #[inline(always)]
    pub fn busyenum(&self) -> BUSYENUM_R {
        BUSYENUM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framebuffer writeback status
    #[inline(always)]
    pub fn busywrite(&self) -> BUSYWRITE_R {
        BUSYWRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Framebuffer cache status
    #[inline(always)]
    pub fn cachedirty(&self) -> CACHEDIRTY_R {
        CACHEDIRTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Display list reader status
    #[inline(always)]
    pub fn dlistactive(&self) -> DLISTACTIVE_R {
        DLISTACTIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enumeration finished interrupt triggered
    #[inline(always)]
    pub fn enumirq(&self) -> ENUMIRQ_R {
        ENUMIRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - display list finished interrupt triggered
    #[inline(always)]
    pub fn dlistirq(&self) -> DLISTIRQ_R {
        DLISTIRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - bus error interrupt triggered
    #[inline(always)]
    pub fn busirq(&self) -> BUSIRQ_R {
        BUSIRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - framebuffer bus error interrupt triggered
    #[inline(always)]
    pub fn buserrmfb(&self) -> BUSERRMFB_R {
        BUSERRMFB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - texture bus error interrupt triggered
    #[inline(always)]
    pub fn buserrmtxmrl(&self) -> BUSERRMTXMRL_R {
        BUSERRMTXMRL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - display list bus error interrupt triggered
    #[inline(always)]
    pub fn buserrmdl(&self) -> BUSERRMDL_R {
        BUSERRMDL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
/**Status Control Register

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUS_SPEC {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUS_SPEC {}
