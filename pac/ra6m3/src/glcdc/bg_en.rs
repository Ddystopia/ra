///Register `BG_EN` reader
pub type R = crate::R<BG_EN_SPEC>;
///Register `BG_EN` writer
pub type W = crate::W<BG_EN_SPEC>;
/**Background plane generation module operation enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///1: Enables operation.
    _1 = 1,
    ///0: Disables operation.
    _0 = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Background plane generation module operation enable
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::_1,
            false => EN_A::_0,
        }
    }
    ///Enables operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
    ///Disables operation.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
}
///Field `EN` writer - Background plane generation module operation enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::_1)
    }
    ///Disables operation.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::_0)
    }
}
/**Control of LCDC internal register value reflection to internal operations

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEN_A {
    ///1: Enables
    _1 = 1,
    ///0: Disables(Cleared to 0 by an internal source)
    _0 = 0,
}
impl From<VEN_A> for bool {
    #[inline(always)]
    fn from(variant: VEN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VEN` reader - Control of LCDC internal register value reflection to internal operations

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VEN_R = crate::BitReader<VEN_A>;
impl VEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VEN_A {
        match self.bits {
            true => VEN_A::_1,
            false => VEN_A::_0,
        }
    }
    ///Enables
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VEN_A::_1
    }
    ///Disables(Cleared to 0 by an internal source)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VEN_A::_0
    }
}
///Field `VEN` writer - Control of LCDC internal register value reflection to internal operations
pub type VEN_W<'a, REG> = crate::BitWriter1S<'a, REG, VEN_A>;
impl<'a, REG> VEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VEN_A::_1)
    }
    ///Disables(Cleared to 0 by an internal source)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VEN_A::_0)
    }
}
/**Entire module SW reset control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    ///1: Releases the entire module from the SW reset state.
    _1 = 1,
    ///0: Places the entire module in the SW reset state.
    _0 = 0,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWRST` reader - Entire module SW reset control
pub type SWRST_R = crate::BitReader<SWRST_A>;
impl SWRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWRST_A {
        match self.bits {
            true => SWRST_A::_1,
            false => SWRST_A::_0,
        }
    }
    ///Releases the entire module from the SW reset state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRST_A::_1
    }
    ///Places the entire module in the SW reset state.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRST_A::_0
    }
}
///Field `SWRST` writer - Entire module SW reset control
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG, SWRST_A>;
impl<'a, REG> SWRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Releases the entire module from the SW reset state.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWRST_A::_1)
    }
    ///Places the entire module in the SW reset state.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWRST_A::_0)
    }
}
impl R {
    ///Bit 0 - Background plane generation module operation enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Control of LCDC internal register value reflection to internal operations
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Entire module SW reset control
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Background plane generation module operation enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<BG_EN_SPEC> {
        EN_W::new(self, 0)
    }
    ///Bit 8 - Control of LCDC internal register value reflection to internal operations
    #[inline(always)]
    pub fn ven(&mut self) -> VEN_W<BG_EN_SPEC> {
        VEN_W::new(self, 8)
    }
    ///Bit 16 - Entire module SW reset control
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<BG_EN_SPEC> {
        SWRST_W::new(self, 16)
    }
}
/**Background Plane Setting Operation Control Register

You can [`read`](crate::Reg::read) this register and get [`bg_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_EN_SPEC;
impl crate::RegisterSpec for BG_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_en::R`](R) reader structure
impl crate::Readable for BG_EN_SPEC {}
///`write(|w| ..)` method takes [`bg_en::W`](W) writer structure
impl crate::Writable for BG_EN_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100;
}
///`reset()` method sets BG_EN to value 0
impl crate::Resettable for BG_EN_SPEC {}
