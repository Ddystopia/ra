///Register `BG_HSIZE` reader
pub type R = crate::R<BG_HSIZE_SPEC>;
///Register `BG_HSIZE` writer
pub type W = crate::W<BG_HSIZE_SPEC>;
/**Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field.

Value on reset: 16*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HW_A {
    ///0: HW cycles. The valid range is 0x010 to 0x3F8.
    HW = 0,
}
impl From<HW_A> for u16 {
    #[inline(always)]
    fn from(variant: HW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HW_A {
    type Ux = u16;
}
impl crate::IsEnum for HW_A {}
///Field `HW` reader - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field.
pub type HW_R = crate::FieldReader<HW_A>;
impl HW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HW_A {
        match self.bits {
            _ => HW_A::HW,
        }
    }
    ///HW cycles. The valid range is 0x010 to 0x3F8.
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        matches!(self.variant(), HW_A::HW)
    }
}
///Field `HW` writer - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field.
pub type HW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, HW_A, crate::Safe>;
impl<'a, REG> HW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///HW cycles. The valid range is 0x010 to 0x3F8.
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(HW_A::HW)
    }
}
/**Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK).

Value on reset: 6*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HP_A {
    ///0: HP cycle(pixel). The valid range is 0x006 to 0x3EE.
    HP = 0,
}
impl From<HP_A> for u16 {
    #[inline(always)]
    fn from(variant: HP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HP_A {
    type Ux = u16;
}
impl crate::IsEnum for HP_A {}
///Field `HP` reader - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK).
pub type HP_R = crate::FieldReader<HP_A>;
impl HP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HP_A {
        match self.bits {
            _ => HP_A::HP,
        }
    }
    ///HP cycle(pixel). The valid range is 0x006 to 0x3EE.
    #[inline(always)]
    pub fn is_hp(&self) -> bool {
        matches!(self.variant(), HP_A::HP)
    }
}
///Field `HP` writer - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK).
pub type HP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, HP_A, crate::Safe>;
impl<'a, REG> HP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///HP cycle(pixel). The valid range is 0x006 to 0x3EE.
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(HP_A::HP)
    }
}
impl R {
    ///Bits 0:10 - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field.
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK).
    #[inline(always)]
    pub fn hp(&self) -> HP_R {
        HP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field.
    #[inline(always)]
    pub fn hw(&mut self) -> HW_W<BG_HSIZE_SPEC> {
        HW_W::new(self, 0)
    }
    ///Bits 16:26 - Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK).
    #[inline(always)]
    pub fn hp(&mut self) -> HP_W<BG_HSIZE_SPEC> {
        HP_W::new(self, 16)
    }
}
/**Background Plane Setting Full Image Horizontal Size Register

You can [`read`](crate::Reg::read) this register and get [`bg_hsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_hsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_HSIZE_SPEC;
impl crate::RegisterSpec for BG_HSIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_hsize::R`](R) reader structure
impl crate::Readable for BG_HSIZE_SPEC {}
///`write(|w| ..)` method takes [`bg_hsize::W`](W) writer structure
impl crate::Writable for BG_HSIZE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BG_HSIZE to value 0x0006_0010
impl crate::Resettable for BG_HSIZE_SPEC {
    const RESET_VALUE: u32 = 0x0006_0010;
}
