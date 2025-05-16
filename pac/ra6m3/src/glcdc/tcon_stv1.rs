///Register `TCON_STV%s1` reader
pub type R = crate::R<TCON_STV1_SPEC>;
///Register `TCON_STV%s1` writer
pub type W = crate::W<TCON_STV1_SPEC>;
/**STVx1 second change timingSets the signal assertion width.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VW_A {
    ///0: VW pixels. The valid range is 0x000 to 0x3FF.
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
///Field `VW` reader - STVx1 second change timingSets the signal assertion width.
pub type VW_R = crate::FieldReader<VW_A>;
impl VW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VW_A {
        match self.bits {
            _ => VW_A::VW,
        }
    }
    ///VW pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn is_vw(&self) -> bool {
        matches!(self.variant(), VW_A::VW)
    }
}
///Field `VW` writer - STVx1 second change timingSets the signal assertion width.
pub type VW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, VW_A, crate::Safe>;
impl<'a, REG> VW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///VW pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn vw(self) -> &'a mut crate::W<REG> {
        self.variant(VW_A::VW)
    }
}
/**STVx1 first change timing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VS_A {
    ///0: VS pixels. The valid range is 0x000 to 0x3FF.
    VS = 0,
}
impl From<VS_A> for u16 {
    #[inline(always)]
    fn from(variant: VS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VS_A {
    type Ux = u16;
}
impl crate::IsEnum for VS_A {}
///Field `VS` reader - STVx1 first change timing
pub type VS_R = crate::FieldReader<VS_A>;
impl VS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VS_A {
        match self.bits {
            _ => VS_A::VS,
        }
    }
    ///VS pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn is_vs(&self) -> bool {
        matches!(self.variant(), VS_A::VS)
    }
}
///Field `VS` writer - STVx1 first change timing
pub type VS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, VS_A, crate::Safe>;
impl<'a, REG> VS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///VS pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn vs(self) -> &'a mut crate::W<REG> {
        self.variant(VS_A::VS)
    }
}
impl R {
    ///Bits 0:10 - STVx1 second change timingSets the signal assertion width.
    #[inline(always)]
    pub fn vw(&self) -> VW_R {
        VW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - STVx1 first change timing
    #[inline(always)]
    pub fn vs(&self) -> VS_R {
        VS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - STVx1 second change timingSets the signal assertion width.
    #[inline(always)]
    pub fn vw(&mut self) -> VW_W<TCON_STV1_SPEC> {
        VW_W::new(self, 0)
    }
    ///Bits 16:26 - STVx1 first change timing
    #[inline(always)]
    pub fn vs(&mut self) -> VS_W<TCON_STV1_SPEC> {
        VS_W::new(self, 16)
    }
}
/**TCON Vertical Timing Setting Register %s1

You can [`read`](crate::Reg::read) this register and get [`tcon_stv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_stv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCON_STV1_SPEC;
impl crate::RegisterSpec for TCON_STV1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tcon_stv1::R`](R) reader structure
impl crate::Readable for TCON_STV1_SPEC {}
///`write(|w| ..)` method takes [`tcon_stv1::W`](W) writer structure
impl crate::Writable for TCON_STV1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCON_STV%s1 to value 0
impl crate::Resettable for TCON_STV1_SPEC {}
