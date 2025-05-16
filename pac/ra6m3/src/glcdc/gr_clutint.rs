///Register `GR%s_CLUTINT` reader
pub type R = crate::R<GR_CLUTINT_SPEC>;
///Register `GR%s_CLUTINT` writer
pub type W = crate::W<GR_CLUTINT_SPEC>;
/**Number of detection lines

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LINE_A {
    ///0: LINE+1 lines. The valid range is 0x000 to 0x400.
    LINE = 0,
}
impl From<LINE_A> for u16 {
    #[inline(always)]
    fn from(variant: LINE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LINE_A {
    type Ux = u16;
}
impl crate::IsEnum for LINE_A {}
///Field `LINE` reader - Number of detection lines
pub type LINE_R = crate::FieldReader<LINE_A>;
impl LINE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LINE_A {
        match self.bits {
            _ => LINE_A::LINE,
        }
    }
    ///LINE+1 lines. The valid range is 0x000 to 0x400.
    #[inline(always)]
    pub fn is_line(&self) -> bool {
        matches!(self.variant(), LINE_A::LINE)
    }
}
///Field `LINE` writer - Number of detection lines
pub type LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, LINE_A, crate::Safe>;
impl<'a, REG> LINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///LINE+1 lines. The valid range is 0x000 to 0x400.
    #[inline(always)]
    pub fn line(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_A::LINE)
    }
}
/**CLUT table control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    ///1: Uses CLUT1 plane for internal operations.
    _1 = 1,
    ///0: Uses CLUT0 plane for internal operations.
    _0 = 0,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEL` reader - CLUT table control
pub type SEL_R = crate::BitReader<SEL_A>;
impl SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            true => Some(SEL_A::_1),
            false => Some(SEL_A::_0),
            _ => None,
        }
    }
    ///Uses CLUT1 plane for internal operations.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEL_A::_1
    }
    ///Uses CLUT0 plane for internal operations.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEL_A::_0
    }
}
///Field `SEL` writer - CLUT table control
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG, SEL_A>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Uses CLUT1 plane for internal operations.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_1)
    }
    ///Uses CLUT0 plane for internal operations.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_0)
    }
}
impl R {
    ///Bits 0:10 - Number of detection lines
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 16 - CLUT table control
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:10 - Number of detection lines
    #[inline(always)]
    pub fn line(&mut self) -> LINE_W<GR_CLUTINT_SPEC> {
        LINE_W::new(self, 0)
    }
    ///Bit 16 - CLUT table control
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<GR_CLUTINT_SPEC> {
        SEL_W::new(self, 16)
    }
}
/**Graphics %s CLUT Table Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_clutint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_clutint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_CLUTINT_SPEC;
impl crate::RegisterSpec for GR_CLUTINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_clutint::R`](R) reader structure
impl crate::Readable for GR_CLUTINT_SPEC {}
///`write(|w| ..)` method takes [`gr_clutint::W`](W) writer structure
impl crate::Writable for GR_CLUTINT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_CLUTINT to value 0
impl crate::Resettable for GR_CLUTINT_SPEC {}
