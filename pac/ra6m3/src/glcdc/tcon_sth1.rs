///Register `TCON_STH%s1` reader
pub type R = crate::R<TCON_STH1_SPEC>;
///Register `TCON_STH%s1` writer
pub type W = crate::W<TCON_STH1_SPEC>;
/**STHx1 second change timing.Sets the signal assertion width.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HW_A {
    ///0: HW pixels. The valid range is 0x000 to 0x3FF.
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
///Field `HW` reader - STHx1 second change timing.Sets the signal assertion width.
pub type HW_R = crate::FieldReader<HW_A>;
impl HW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HW_A {
        match self.bits {
            _ => HW_A::HW,
        }
    }
    ///HW pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        matches!(self.variant(), HW_A::HW)
    }
}
///Field `HW` writer - STHx1 second change timing.Sets the signal assertion width.
pub type HW_W<'a, REG> = crate::FieldWriter<'a, REG, 11, HW_A, crate::Safe>;
impl<'a, REG> HW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///HW pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(HW_A::HW)
    }
}
/**STHx1 first change timing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HS_A {
    ///0: HS lines. The valid range is 0x000 to 0x3FF.
    HS = 0,
}
impl From<HS_A> for u16 {
    #[inline(always)]
    fn from(variant: HS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HS_A {
    type Ux = u16;
}
impl crate::IsEnum for HS_A {}
///Field `HS` reader - STHx1 first change timing
pub type HS_R = crate::FieldReader<HS_A>;
impl HS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HS_A {
        match self.bits {
            _ => HS_A::HS,
        }
    }
    ///HS lines. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        matches!(self.variant(), HS_A::HS)
    }
}
///Field `HS` writer - STHx1 first change timing
pub type HS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, HS_A, crate::Safe>;
impl<'a, REG> HS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///HS lines. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(HS_A::HS)
    }
}
impl R {
    ///Bits 0:10 - STHx1 second change timing.Sets the signal assertion width.
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - STHx1 first change timing
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - STHx1 second change timing.Sets the signal assertion width.
    #[inline(always)]
    pub fn hw(&mut self) -> HW_W<TCON_STH1_SPEC> {
        HW_W::new(self, 0)
    }
    ///Bits 16:26 - STHx1 first change timing
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W<TCON_STH1_SPEC> {
        HS_W::new(self, 16)
    }
}
/**TCON Horizontal Timing Setting Register STH%s1

You can [`read`](crate::Reg::read) this register and get [`tcon_sth1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_sth1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCON_STH1_SPEC;
impl crate::RegisterSpec for TCON_STH1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tcon_sth1::R`](R) reader structure
impl crate::Readable for TCON_STH1_SPEC {}
///`write(|w| ..)` method takes [`tcon_sth1::W`](W) writer structure
impl crate::Writable for TCON_STH1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCON_STH%s1 to value 0
impl crate::Resettable for TCON_STH1_SPEC {}
