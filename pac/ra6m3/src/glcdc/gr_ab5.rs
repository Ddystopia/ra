///Register `GR%s_AB5` reader
pub type R = crate::R<GR_AB5_SPEC>;
///Register `GR%s_AB5` writer
pub type W = crate::W<GR_AB5_SPEC>;
/**Horizontal width of rectangular area alpha blending image area.

Value on reset: 16*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ARCHW_A {
    ///0: ARCHW pixels. The valid range is 0x001 to 0x3F0.
    ARCHW = 0,
}
impl From<ARCHW_A> for u16 {
    #[inline(always)]
    fn from(variant: ARCHW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCHW_A {
    type Ux = u16;
}
impl crate::IsEnum for ARCHW_A {}
///Field `ARCHW` reader - Horizontal width of rectangular area alpha blending image area.
pub type ARCHW_R = crate::FieldReader<ARCHW_A>;
impl ARCHW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCHW_A {
        match self.bits {
            _ => ARCHW_A::ARCHW,
        }
    }
    ///ARCHW pixels. The valid range is 0x001 to 0x3F0.
    #[inline(always)]
    pub fn is_archw(&self) -> bool {
        matches!(self.variant(), ARCHW_A::ARCHW)
    }
}
///Field `ARCHW` writer - Horizontal width of rectangular area alpha blending image area.
pub type ARCHW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, ARCHW_A, crate::Safe>;
impl<'a, REG> ARCHW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///ARCHW pixels. The valid range is 0x001 to 0x3F0.
    #[inline(always)]
    pub fn archw(self) -> &'a mut crate::W<REG> {
        self.variant(ARCHW_A::ARCHW)
    }
}
/**Horizontal start position of rectangular area alpha blending image area.

Value on reset: 5*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ARCHS_A {
    ///0: ARCHS pixel. The valid range is 0x005 to 0x3ED.
    ARCHS = 0,
}
impl From<ARCHS_A> for u16 {
    #[inline(always)]
    fn from(variant: ARCHS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCHS_A {
    type Ux = u16;
}
impl crate::IsEnum for ARCHS_A {}
///Field `ARCHS` reader - Horizontal start position of rectangular area alpha blending image area.
pub type ARCHS_R = crate::FieldReader<ARCHS_A>;
impl ARCHS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCHS_A {
        match self.bits {
            _ => ARCHS_A::ARCHS,
        }
    }
    ///ARCHS pixel. The valid range is 0x005 to 0x3ED.
    #[inline(always)]
    pub fn is_archs(&self) -> bool {
        matches!(self.variant(), ARCHS_A::ARCHS)
    }
}
///Field `ARCHS` writer - Horizontal start position of rectangular area alpha blending image area.
pub type ARCHS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, ARCHS_A, crate::Safe>;
impl<'a, REG> ARCHS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///ARCHS pixel. The valid range is 0x005 to 0x3ED.
    #[inline(always)]
    pub fn archs(self) -> &'a mut crate::W<REG> {
        self.variant(ARCHS_A::ARCHS)
    }
}
impl R {
    ///Bits 0:10 - Horizontal width of rectangular area alpha blending image area.
    #[inline(always)]
    pub fn archw(&self) -> ARCHW_R {
        ARCHW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Horizontal start position of rectangular area alpha blending image area.
    #[inline(always)]
    pub fn archs(&self) -> ARCHS_R {
        ARCHS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Horizontal width of rectangular area alpha blending image area.
    #[inline(always)]
    pub fn archw(&mut self) -> ARCHW_W<GR_AB5_SPEC> {
        ARCHW_W::new(self, 0)
    }
    ///Bits 16:26 - Horizontal start position of rectangular area alpha blending image area.
    #[inline(always)]
    pub fn archs(&mut self) -> ARCHS_W<GR_AB5_SPEC> {
        ARCHS_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 5

You can [`read`](crate::Reg::read) this register and get [`gr_ab5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB5_SPEC;
impl crate::RegisterSpec for GR_AB5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab5::R`](R) reader structure
impl crate::Readable for GR_AB5_SPEC {}
///`write(|w| ..)` method takes [`gr_ab5::W`](W) writer structure
impl crate::Writable for GR_AB5_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB5 to value 0x0005_0010
impl crate::Resettable for GR_AB5_SPEC {
    const RESET_VALUE: u32 = 0x0005_0010;
}
