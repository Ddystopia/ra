///Register `GR%s_FLM6` reader
pub type R = crate::R<GR_FLM6_SPEC>;
///Register `GR%s_FLM6` writer
pub type W = crate::W<GR_FLM6_SPEC>;
/**Data format for accessing graphics data (frame buffer data).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    ///7: CLUT11bit/pix)
    _111 = 7,
    ///6: CLUT4 (4 bits/pix)
    _110 = 6,
    ///5: CLUT8 (8 bits/pix)
    _101 = 5,
    ///4: ARGB8888 (32 bits/pix)
    _100 = 4,
    ///3: ARGB4444 (16 bits/pix)
    _011 = 3,
    ///2: ARGB1555 (16 bits/pix, 1 bit of A is LUT data)
    _010 = 2,
    ///1: RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)
    _001 = 1,
    ///0: RGB565 (16 bits/pix)
    _000 = 0,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORMAT_A {
    type Ux = u8;
}
impl crate::IsEnum for FORMAT_A {}
///Field `FORMAT` reader - Data format for accessing graphics data (frame buffer data).
pub type FORMAT_R = crate::FieldReader<FORMAT_A>;
impl FORMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            7 => Some(FORMAT_A::_111),
            6 => Some(FORMAT_A::_110),
            5 => Some(FORMAT_A::_101),
            4 => Some(FORMAT_A::_100),
            3 => Some(FORMAT_A::_011),
            2 => Some(FORMAT_A::_010),
            1 => Some(FORMAT_A::_001),
            0 => Some(FORMAT_A::_000),
            _ => None,
        }
    }
    ///CLUT11bit/pix)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FORMAT_A::_111
    }
    ///CLUT4 (4 bits/pix)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FORMAT_A::_110
    }
    ///CLUT8 (8 bits/pix)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FORMAT_A::_101
    }
    ///ARGB8888 (32 bits/pix)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FORMAT_A::_100
    }
    ///ARGB4444 (16 bits/pix)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FORMAT_A::_011
    }
    ///ARGB1555 (16 bits/pix, 1 bit of A is LUT data)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FORMAT_A::_010
    }
    ///RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FORMAT_A::_001
    }
    ///RGB565 (16 bits/pix)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FORMAT_A::_000
    }
}
///Field `FORMAT` writer - Data format for accessing graphics data (frame buffer data).
pub type FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FORMAT_A, crate::Safe>;
impl<'a, REG> FORMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CLUT11bit/pix)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_111)
    }
    ///CLUT4 (4 bits/pix)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_110)
    }
    ///CLUT8 (8 bits/pix)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_101)
    }
    ///ARGB8888 (32 bits/pix)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_100)
    }
    ///ARGB4444 (16 bits/pix)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_011)
    }
    ///ARGB1555 (16 bits/pix, 1 bit of A is LUT data)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_010)
    }
    ///RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_001)
    }
    ///RGB565 (16 bits/pix)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_000)
    }
}
impl R {
    ///Bits 28:30 - Data format for accessing graphics data (frame buffer data).
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 28:30 - Data format for accessing graphics data (frame buffer data).
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W<GR_FLM6_SPEC> {
        FORMAT_W::new(self, 28)
    }
}
/**Graphics %s Frame Buffer Control Register 6

You can [`read`](crate::Reg::read) this register and get [`gr_flm6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_FLM6_SPEC;
impl crate::RegisterSpec for GR_FLM6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_flm6::R`](R) reader structure
impl crate::Readable for GR_FLM6_SPEC {}
///`write(|w| ..)` method takes [`gr_flm6::W`](W) writer structure
impl crate::Writable for GR_FLM6_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_FLM6 to value 0
impl crate::Resettable for GR_FLM6_SPEC {}
