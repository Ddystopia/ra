///Register `GR%s_AB3` reader
pub type R = crate::R<GR_AB3_SPEC>;
///Register `GR%s_AB3` writer
pub type W = crate::W<GR_AB3_SPEC>;
/**Horizontal width of graphics image area.

Value on reset: 16*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GRCHW_A {
    ///0: GRCHW pixels. The valid range is 0x010 to 0x3F0.
    GRCHW = 0,
}
impl From<GRCHW_A> for u16 {
    #[inline(always)]
    fn from(variant: GRCHW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRCHW_A {
    type Ux = u16;
}
impl crate::IsEnum for GRCHW_A {}
///Field `GRCHW` reader - Horizontal width of graphics image area.
pub type GRCHW_R = crate::FieldReader<GRCHW_A>;
impl GRCHW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRCHW_A {
        match self.bits {
            _ => GRCHW_A::GRCHW,
        }
    }
    ///GRCHW pixels. The valid range is 0x010 to 0x3F0.
    #[inline(always)]
    pub fn is_grchw(&self) -> bool {
        matches!(self.variant(), GRCHW_A::GRCHW)
    }
}
///Field `GRCHW` writer - Horizontal width of graphics image area.
pub type GRCHW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GRCHW_A, crate::Safe>;
impl<'a, REG> GRCHW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GRCHW pixels. The valid range is 0x010 to 0x3F0.
    #[inline(always)]
    pub fn grchw(self) -> &'a mut crate::W<REG> {
        self.variant(GRCHW_A::GRCHW)
    }
}
/**Horizontal start position of graphics image area.

Value on reset: 5*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GRCHS_A {
    ///0: GRCHS lines. The valid range is 0x005 to 0x3ED.
    GRCHS = 0,
}
impl From<GRCHS_A> for u16 {
    #[inline(always)]
    fn from(variant: GRCHS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRCHS_A {
    type Ux = u16;
}
impl crate::IsEnum for GRCHS_A {}
///Field `GRCHS` reader - Horizontal start position of graphics image area.
pub type GRCHS_R = crate::FieldReader<GRCHS_A>;
impl GRCHS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRCHS_A {
        match self.bits {
            _ => GRCHS_A::GRCHS,
        }
    }
    ///GRCHS lines. The valid range is 0x005 to 0x3ED.
    #[inline(always)]
    pub fn is_grchs(&self) -> bool {
        matches!(self.variant(), GRCHS_A::GRCHS)
    }
}
///Field `GRCHS` writer - Horizontal start position of graphics image area.
pub type GRCHS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GRCHS_A, crate::Safe>;
impl<'a, REG> GRCHS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GRCHS lines. The valid range is 0x005 to 0x3ED.
    #[inline(always)]
    pub fn grchs(self) -> &'a mut crate::W<REG> {
        self.variant(GRCHS_A::GRCHS)
    }
}
impl R {
    ///Bits 0:10 - Horizontal width of graphics image area.
    #[inline(always)]
    pub fn grchw(&self) -> GRCHW_R {
        GRCHW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Horizontal start position of graphics image area.
    #[inline(always)]
    pub fn grchs(&self) -> GRCHS_R {
        GRCHS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Horizontal width of graphics image area.
    #[inline(always)]
    pub fn grchw(&mut self) -> GRCHW_W<GR_AB3_SPEC> {
        GRCHW_W::new(self, 0)
    }
    ///Bits 16:26 - Horizontal start position of graphics image area.
    #[inline(always)]
    pub fn grchs(&mut self) -> GRCHS_W<GR_AB3_SPEC> {
        GRCHS_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 3

You can [`read`](crate::Reg::read) this register and get [`gr_ab3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB3_SPEC;
impl crate::RegisterSpec for GR_AB3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab3::R`](R) reader structure
impl crate::Readable for GR_AB3_SPEC {}
///`write(|w| ..)` method takes [`gr_ab3::W`](W) writer structure
impl crate::Writable for GR_AB3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB3 to value 0x0005_0010
impl crate::Resettable for GR_AB3_SPEC {
    const RESET_VALUE: u32 = 0x0005_0010;
}
