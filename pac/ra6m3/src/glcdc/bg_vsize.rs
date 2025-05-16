///Register `BG_VSIZE` reader
pub type R = crate::R<BG_VSIZE_SPEC>;
///Register `BG_VSIZE` writer
pub type W = crate::W<BG_VSIZE_SPEC>;
/**Background plane vertical valid pixel width on the basis of line

Value on reset: 16*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VW_A {
    ///0: VW lines. The valid range is 0x010 to 0x3F0.
    VW = 0,
}
impl From<VW_A> for u16 {
    #[inline(always)]
    fn from(variant: VW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VW_A {
    type Ux = u16;
}
impl crate::IsEnum for VW_A {}
///Field `VW` reader - Background plane vertical valid pixel width on the basis of line
pub type VW_R = crate::FieldReader<VW_A>;
impl VW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VW_A {
        match self.bits {
            _ => VW_A::VW,
        }
    }
    ///VW lines. The valid range is 0x010 to 0x3F0.
    #[inline(always)]
    pub fn is_vw(&self) -> bool {
        matches!(self.variant(), VW_A::VW)
    }
}
///Field `VW` writer - Background plane vertical valid pixel width on the basis of line
pub type VW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, VW_A, crate::Safe>;
impl<'a, REG> VW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///VW lines. The valid range is 0x010 to 0x3F0.
    #[inline(always)]
    pub fn vw(self) -> &'a mut crate::W<REG> {
        self.variant(VW_A::VW)
    }
}
/**Background plane vertical valid pixel start position on the basis of line

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VP_A {
    ///0: VP lines. The valid range is 0x003 to 0x3EF.
    VP = 0,
}
impl From<VP_A> for u16 {
    #[inline(always)]
    fn from(variant: VP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VP_A {
    type Ux = u16;
}
impl crate::IsEnum for VP_A {}
///Field `VP` reader - Background plane vertical valid pixel start position on the basis of line
pub type VP_R = crate::FieldReader<VP_A>;
impl VP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VP_A {
        match self.bits {
            _ => VP_A::VP,
        }
    }
    ///VP lines. The valid range is 0x003 to 0x3EF.
    #[inline(always)]
    pub fn is_vp(&self) -> bool {
        matches!(self.variant(), VP_A::VP)
    }
}
///Field `VP` writer - Background plane vertical valid pixel start position on the basis of line
pub type VP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, VP_A, crate::Safe>;
impl<'a, REG> VP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///VP lines. The valid range is 0x003 to 0x3EF.
    #[inline(always)]
    pub fn vp(self) -> &'a mut crate::W<REG> {
        self.variant(VP_A::VP)
    }
}
impl R {
    ///Bits 0:10 - Background plane vertical valid pixel width on the basis of line
    #[inline(always)]
    pub fn vw(&self) -> VW_R {
        VW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Background plane vertical valid pixel start position on the basis of line
    #[inline(always)]
    pub fn vp(&self) -> VP_R {
        VP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Background plane vertical valid pixel width on the basis of line
    #[inline(always)]
    pub fn vw(&mut self) -> VW_W<BG_VSIZE_SPEC> {
        VW_W::new(self, 0)
    }
    ///Bits 16:26 - Background plane vertical valid pixel start position on the basis of line
    #[inline(always)]
    pub fn vp(&mut self) -> VP_W<BG_VSIZE_SPEC> {
        VP_W::new(self, 16)
    }
}
/**Background Plane Setting Full Image Vertical Size Register

You can [`read`](crate::Reg::read) this register and get [`bg_vsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_vsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_VSIZE_SPEC;
impl crate::RegisterSpec for BG_VSIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_vsize::R`](R) reader structure
impl crate::Readable for BG_VSIZE_SPEC {}
///`write(|w| ..)` method takes [`bg_vsize::W`](W) writer structure
impl crate::Writable for BG_VSIZE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BG_VSIZE to value 0x0007_0010
impl crate::Resettable for BG_VSIZE_SPEC {
    const RESET_VALUE: u32 = 0x0007_0010;
}
