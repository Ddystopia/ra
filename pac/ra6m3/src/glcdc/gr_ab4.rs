///Register `GR%s_AB4` reader
pub type R = crate::R<GR_AB4_SPEC>;
///Register `GR%s_AB4` writer
pub type W = crate::W<GR_AB4_SPEC>;
/**Vertical width of rectangular area alpha blending image area.

Value on reset: 16*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ARCVW_A {
    ///0: ARCVW linels. The valid range is 0x001 to 0x3F0.
    ARCVW = 0,
}
impl From<ARCVW_A> for u16 {
    #[inline(always)]
    fn from(variant: ARCVW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCVW_A {
    type Ux = u16;
}
impl crate::IsEnum for ARCVW_A {}
///Field `ARCVW` reader - Vertical width of rectangular area alpha blending image area.
pub type ARCVW_R = crate::FieldReader<ARCVW_A>;
impl ARCVW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCVW_A {
        match self.bits {
            _ => ARCVW_A::ARCVW,
        }
    }
    ///ARCVW linels. The valid range is 0x001 to 0x3F0.
    #[inline(always)]
    pub fn is_arcvw(&self) -> bool {
        matches!(self.variant(), ARCVW_A::ARCVW)
    }
}
///Field `ARCVW` writer - Vertical width of rectangular area alpha blending image area.
pub type ARCVW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, ARCVW_A, crate::Safe>;
impl<'a, REG> ARCVW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///ARCVW linels. The valid range is 0x001 to 0x3F0.
    #[inline(always)]
    pub fn arcvw(self) -> &'a mut crate::W<REG> {
        self.variant(ARCVW_A::ARCVW)
    }
}
/**Vertical start position of rectangular area alpha blending image area

Value on reset: 6*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ARCVS_A {
    ///0: ARCVS linels. The valid range is 0x002 to 0x3EE.
    ARCVS = 0,
}
impl From<ARCVS_A> for u16 {
    #[inline(always)]
    fn from(variant: ARCVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCVS_A {
    type Ux = u16;
}
impl crate::IsEnum for ARCVS_A {}
///Field `ARCVS` reader - Vertical start position of rectangular area alpha blending image area
pub type ARCVS_R = crate::FieldReader<ARCVS_A>;
impl ARCVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCVS_A {
        match self.bits {
            _ => ARCVS_A::ARCVS,
        }
    }
    ///ARCVS linels. The valid range is 0x002 to 0x3EE.
    #[inline(always)]
    pub fn is_arcvs(&self) -> bool {
        matches!(self.variant(), ARCVS_A::ARCVS)
    }
}
///Field `ARCVS` writer - Vertical start position of rectangular area alpha blending image area
pub type ARCVS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, ARCVS_A, crate::Safe>;
impl<'a, REG> ARCVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///ARCVS linels. The valid range is 0x002 to 0x3EE.
    #[inline(always)]
    pub fn arcvs(self) -> &'a mut crate::W<REG> {
        self.variant(ARCVS_A::ARCVS)
    }
}
impl R {
    ///Bits 0:10 - Vertical width of rectangular area alpha blending image area.
    #[inline(always)]
    pub fn arcvw(&self) -> ARCVW_R {
        ARCVW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Vertical start position of rectangular area alpha blending image area
    #[inline(always)]
    pub fn arcvs(&self) -> ARCVS_R {
        ARCVS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Vertical width of rectangular area alpha blending image area.
    #[inline(always)]
    pub fn arcvw(&mut self) -> ARCVW_W<GR_AB4_SPEC> {
        ARCVW_W::new(self, 0)
    }
    ///Bits 16:26 - Vertical start position of rectangular area alpha blending image area
    #[inline(always)]
    pub fn arcvs(&mut self) -> ARCVS_W<GR_AB4_SPEC> {
        ARCVS_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 4

You can [`read`](crate::Reg::read) this register and get [`gr_ab4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB4_SPEC;
impl crate::RegisterSpec for GR_AB4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab4::R`](R) reader structure
impl crate::Readable for GR_AB4_SPEC {}
///`write(|w| ..)` method takes [`gr_ab4::W`](W) writer structure
impl crate::Writable for GR_AB4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB4 to value 0x0006_0010
impl crate::Resettable for GR_AB4_SPEC {
    const RESET_VALUE: u32 = 0x0006_0010;
}
