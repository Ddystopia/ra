///Register `GR%s_AB1` reader
pub type R = crate::R<GR_AB1_SPEC>;
///Register `GR%s_AB1` writer
pub type W = crate::W<GR_AB1_SPEC>;
/**Graphics display plane control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISPSEL_A {
    ///3: Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)
    _11 = 3,
    ///2: Current graphics display
    _10 = 2,
    ///1: Lower-layer graphics display
    _01 = 1,
    ///0: Background color display (value set by the GRn_BASE register).
    _00 = 0,
}
impl From<DISPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DISPSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DISPSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for DISPSEL_A {}
///Field `DISPSEL` reader - Graphics display plane control.
pub type DISPSEL_R = crate::FieldReader<DISPSEL_A>;
impl DISPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISPSEL_A {
        match self.bits {
            3 => DISPSEL_A::_11,
            2 => DISPSEL_A::_10,
            1 => DISPSEL_A::_01,
            0 => DISPSEL_A::_00,
            _ => unreachable!(),
        }
    }
    ///Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DISPSEL_A::_11
    }
    ///Current graphics display
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DISPSEL_A::_10
    }
    ///Lower-layer graphics display
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DISPSEL_A::_01
    }
    ///Background color display (value set by the GRn_BASE register).
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DISPSEL_A::_00
    }
}
///Field `DISPSEL` writer - Graphics display plane control.
pub type DISPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DISPSEL_A, crate::Safe>;
impl<'a, REG> DISPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DISPSEL_A::_11)
    }
    ///Current graphics display
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DISPSEL_A::_10)
    }
    ///Lower-layer graphics display
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DISPSEL_A::_01)
    }
    ///Background color display (value set by the GRn_BASE register).
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DISPSEL_A::_00)
    }
}
/**Graphics image area border display control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRCDISPON_A {
    ///1: Display on
    _1 = 1,
    ///0: Display off
    _0 = 0,
}
impl From<GRCDISPON_A> for bool {
    #[inline(always)]
    fn from(variant: GRCDISPON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GRCDISPON` reader - Graphics image area border display control.
pub type GRCDISPON_R = crate::BitReader<GRCDISPON_A>;
impl GRCDISPON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRCDISPON_A {
        match self.bits {
            true => GRCDISPON_A::_1,
            false => GRCDISPON_A::_0,
        }
    }
    ///Display on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRCDISPON_A::_1
    }
    ///Display off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRCDISPON_A::_0
    }
}
///Field `GRCDISPON` writer - Graphics image area border display control.
pub type GRCDISPON_W<'a, REG> = crate::BitWriter<'a, REG, GRCDISPON_A>;
impl<'a, REG> GRCDISPON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Display on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GRCDISPON_A::_1)
    }
    ///Display off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GRCDISPON_A::_0)
    }
}
/**Image area border display control for rectangular area alpha blending.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCDISPON_A {
    ///1: Display on
    _1 = 1,
    ///0: Display off
    _0 = 0,
}
impl From<ARCDISPON_A> for bool {
    #[inline(always)]
    fn from(variant: ARCDISPON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARCDISPON` reader - Image area border display control for rectangular area alpha blending.
pub type ARCDISPON_R = crate::BitReader<ARCDISPON_A>;
impl ARCDISPON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCDISPON_A {
        match self.bits {
            true => ARCDISPON_A::_1,
            false => ARCDISPON_A::_0,
        }
    }
    ///Display on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARCDISPON_A::_1
    }
    ///Display off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARCDISPON_A::_0
    }
}
///Field `ARCDISPON` writer - Image area border display control for rectangular area alpha blending.
pub type ARCDISPON_W<'a, REG> = crate::BitWriter<'a, REG, ARCDISPON_A>;
impl<'a, REG> ARCDISPON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Display on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ARCDISPON_A::_1)
    }
    ///Display off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ARCDISPON_A::_0)
    }
}
/**Rectangular area alpha blending control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCON_A {
    ///1: On
    _1 = 1,
    ///0: Off
    _0 = 0,
}
impl From<ARCON_A> for bool {
    #[inline(always)]
    fn from(variant: ARCON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARCON` reader - Rectangular area alpha blending control.
pub type ARCON_R = crate::BitReader<ARCON_A>;
impl ARCON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCON_A {
        match self.bits {
            true => ARCON_A::_1,
            false => ARCON_A::_0,
        }
    }
    ///On
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARCON_A::_1
    }
    ///Off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARCON_A::_0
    }
}
///Field `ARCON` writer - Rectangular area alpha blending control.
pub type ARCON_W<'a, REG> = crate::BitWriter<'a, REG, ARCON_A>;
impl<'a, REG> ARCON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///On
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ARCON_A::_1)
    }
    ///Off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ARCON_A::_0)
    }
}
impl R {
    ///Bits 0:1 - Graphics display plane control.
    #[inline(always)]
    pub fn dispsel(&self) -> DISPSEL_R {
        DISPSEL_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Graphics image area border display control.
    #[inline(always)]
    pub fn grcdispon(&self) -> GRCDISPON_R {
        GRCDISPON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Image area border display control for rectangular area alpha blending.
    #[inline(always)]
    pub fn arcdispon(&self) -> ARCDISPON_R {
        ARCDISPON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Rectangular area alpha blending control.
    #[inline(always)]
    pub fn arcon(&self) -> ARCON_R {
        ARCON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Graphics display plane control.
    #[inline(always)]
    pub fn dispsel(&mut self) -> DISPSEL_W<GR_AB1_SPEC> {
        DISPSEL_W::new(self, 0)
    }
    ///Bit 4 - Graphics image area border display control.
    #[inline(always)]
    pub fn grcdispon(&mut self) -> GRCDISPON_W<GR_AB1_SPEC> {
        GRCDISPON_W::new(self, 4)
    }
    ///Bit 8 - Image area border display control for rectangular area alpha blending.
    #[inline(always)]
    pub fn arcdispon(&mut self) -> ARCDISPON_W<GR_AB1_SPEC> {
        ARCDISPON_W::new(self, 8)
    }
    ///Bit 12 - Rectangular area alpha blending control.
    #[inline(always)]
    pub fn arcon(&mut self) -> ARCON_W<GR_AB1_SPEC> {
        ARCON_W::new(self, 12)
    }
}
/**Graphics %s Alpha Blending Control Register 1

You can [`read`](crate::Reg::read) this register and get [`gr_ab1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB1_SPEC;
impl crate::RegisterSpec for GR_AB1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab1::R`](R) reader structure
impl crate::Readable for GR_AB1_SPEC {}
///`write(|w| ..)` method takes [`gr_ab1::W`](W) writer structure
impl crate::Writable for GR_AB1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB1 to value 0
impl crate::Resettable for GR_AB1_SPEC {}
