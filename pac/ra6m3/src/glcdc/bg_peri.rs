///Register `BG_PERI` reader
pub type R = crate::R<BG_PERI_SPEC>;
///Register `BG_PERI` writer
pub type W = crate::W<BG_PERI_SPEC>;
/**Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK).

Value on reset: 23*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FH_A {
    ///0: FH lines. The valid range is 0x017 to 0x3FF.
    FH = 0,
}
impl From<FH_A> for u16 {
    #[inline(always)]
    fn from(variant: FH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FH_A {
    type Ux = u16;
}
impl crate::IsEnum for FH_A {}
///Field `FH` reader - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK).
pub type FH_R = crate::FieldReader<FH_A>;
impl FH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FH_A {
        match self.bits {
            _ => FH_A::FH,
        }
    }
    ///FH lines. The valid range is 0x017 to 0x3FF.
    #[inline(always)]
    pub fn is_fh(&self) -> bool {
        matches!(self.variant(), FH_A::FH)
    }
}
///Field `FH` writer - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK).
pub type FH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, FH_A, crate::Safe>;
impl<'a, REG> FH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///FH lines. The valid range is 0x017 to 0x3FF.
    #[inline(always)]
    pub fn fh(self) -> &'a mut crate::W<REG> {
        self.variant(FH_A::FH)
    }
}
/**Background plane vertical synchronization signal period on the basis of line.

Value on reset: 23*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FV_A {
    ///0: FV lines.The valid range is 0x013 to 0x3FF.
    FV = 0,
}
impl From<FV_A> for u16 {
    #[inline(always)]
    fn from(variant: FV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FV_A {
    type Ux = u16;
}
impl crate::IsEnum for FV_A {}
///Field `FV` reader - Background plane vertical synchronization signal period on the basis of line.
pub type FV_R = crate::FieldReader<FV_A>;
impl FV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FV_A {
        match self.bits {
            _ => FV_A::FV,
        }
    }
    ///FV lines.The valid range is 0x013 to 0x3FF.
    #[inline(always)]
    pub fn is_fv(&self) -> bool {
        matches!(self.variant(), FV_A::FV)
    }
}
///Field `FV` writer - Background plane vertical synchronization signal period on the basis of line.
pub type FV_W<'a, REG> = crate::FieldWriter<'a, REG, 11, FV_A, crate::Safe>;
impl<'a, REG> FV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///FV lines.The valid range is 0x013 to 0x3FF.
    #[inline(always)]
    pub fn fv(self) -> &'a mut crate::W<REG> {
        self.variant(FV_A::FV)
    }
}
impl R {
    ///Bits 0:10 - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK).
    #[inline(always)]
    pub fn fh(&self) -> FH_R {
        FH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Background plane vertical synchronization signal period on the basis of line.
    #[inline(always)]
    pub fn fv(&self) -> FV_R {
        FV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK).
    #[inline(always)]
    pub fn fh(&mut self) -> FH_W<BG_PERI_SPEC> {
        FH_W::new(self, 0)
    }
    ///Bits 16:26 - Background plane vertical synchronization signal period on the basis of line.
    #[inline(always)]
    pub fn fv(&mut self) -> FV_W<BG_PERI_SPEC> {
        FV_W::new(self, 16)
    }
}
/**Background Plane Setting Free-Running Period Register

You can [`read`](crate::Reg::read) this register and get [`bg_peri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_peri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_PERI_SPEC;
impl crate::RegisterSpec for BG_PERI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_peri::R`](R) reader structure
impl crate::Readable for BG_PERI_SPEC {}
///`write(|w| ..)` method takes [`bg_peri::W`](W) writer structure
impl crate::Writable for BG_PERI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BG_PERI to value 0x0017_0017
impl crate::Resettable for BG_PERI_SPEC {
    const RESET_VALUE: u32 = 0x0017_0017;
}
