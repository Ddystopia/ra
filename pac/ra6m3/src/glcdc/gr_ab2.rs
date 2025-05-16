///Register `GR%s_AB2` reader
pub type R = crate::R<GR_AB2_SPEC>;
///Register `GR%s_AB2` writer
pub type W = crate::W<GR_AB2_SPEC>;
/**Vertical width of graphics image area.

Value on reset: 16*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GRCVW_A {
    ///0: GRCVW lines. The valid range is 0x010 to 0x3F0.
    GRCVW = 0,
}
impl From<GRCVW_A> for u16 {
    #[inline(always)]
    fn from(variant: GRCVW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRCVW_A {
    type Ux = u16;
}
impl crate::IsEnum for GRCVW_A {}
///Field `GRCVW` reader - Vertical width of graphics image area.
pub type GRCVW_R = crate::FieldReader<GRCVW_A>;
impl GRCVW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRCVW_A {
        match self.bits {
            _ => GRCVW_A::GRCVW,
        }
    }
    ///GRCVW lines. The valid range is 0x010 to 0x3F0.
    #[inline(always)]
    pub fn is_grcvw(&self) -> bool {
        matches!(self.variant(), GRCVW_A::GRCVW)
    }
}
///Field `GRCVW` writer - Vertical width of graphics image area.
pub type GRCVW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GRCVW_A, crate::Safe>;
impl<'a, REG> GRCVW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GRCVW lines. The valid range is 0x010 to 0x3F0.
    #[inline(always)]
    pub fn grcvw(self) -> &'a mut crate::W<REG> {
        self.variant(GRCVW_A::GRCVW)
    }
}
/**Vertical start position of graphics image area.

Value on reset: 6*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GRCVS_A {
    ///0: GRCVS lines. The valid range is 0x002 to 0x3EE.
    GRCVS = 0,
}
impl From<GRCVS_A> for u16 {
    #[inline(always)]
    fn from(variant: GRCVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRCVS_A {
    type Ux = u16;
}
impl crate::IsEnum for GRCVS_A {}
///Field `GRCVS` reader - Vertical start position of graphics image area.
pub type GRCVS_R = crate::FieldReader<GRCVS_A>;
impl GRCVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRCVS_A {
        match self.bits {
            _ => GRCVS_A::GRCVS,
        }
    }
    ///GRCVS lines. The valid range is 0x002 to 0x3EE.
    #[inline(always)]
    pub fn is_grcvs(&self) -> bool {
        matches!(self.variant(), GRCVS_A::GRCVS)
    }
}
///Field `GRCVS` writer - Vertical start position of graphics image area.
pub type GRCVS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GRCVS_A, crate::Safe>;
impl<'a, REG> GRCVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GRCVS lines. The valid range is 0x002 to 0x3EE.
    #[inline(always)]
    pub fn grcvs(self) -> &'a mut crate::W<REG> {
        self.variant(GRCVS_A::GRCVS)
    }
}
impl R {
    ///Bits 0:10 - Vertical width of graphics image area.
    #[inline(always)]
    pub fn grcvw(&self) -> GRCVW_R {
        GRCVW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Vertical start position of graphics image area.
    #[inline(always)]
    pub fn grcvs(&self) -> GRCVS_R {
        GRCVS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Vertical width of graphics image area.
    #[inline(always)]
    pub fn grcvw(&mut self) -> GRCVW_W<GR_AB2_SPEC> {
        GRCVW_W::new(self, 0)
    }
    ///Bits 16:26 - Vertical start position of graphics image area.
    #[inline(always)]
    pub fn grcvs(&mut self) -> GRCVS_W<GR_AB2_SPEC> {
        GRCVS_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 2

You can [`read`](crate::Reg::read) this register and get [`gr_ab2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB2_SPEC;
impl crate::RegisterSpec for GR_AB2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab2::R`](R) reader structure
impl crate::Readable for GR_AB2_SPEC {}
///`write(|w| ..)` method takes [`gr_ab2::W`](W) writer structure
impl crate::Writable for GR_AB2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB2 to value 0x0006_0010
impl crate::Resettable for GR_AB2_SPEC {
    const RESET_VALUE: u32 = 0x0006_0010;
}
