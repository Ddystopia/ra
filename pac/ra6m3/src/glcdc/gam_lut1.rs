///Register `GAM%s_LUT1` reader
pub type R = crate::R<GAM_LUT1_SPEC>;
///Register `GAM%s_LUT1` writer
pub type W = crate::W<GAM_LUT1_SPEC>;
/**Gain value of area 1Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN01_A {
    ///0: GAIN01/1024
    GAIN01 = 0,
}
impl From<GAIN01_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN01_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN01_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN01_A {}
///Field `GAIN01` reader - Gain value of area 1Unsigned 11-bit fixed point
pub type GAIN01_R = crate::FieldReader<GAIN01_A>;
impl GAIN01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN01_A {
        match self.bits {
            _ => GAIN01_A::GAIN01,
        }
    }
    ///GAIN01/1024
    #[inline(always)]
    pub fn is_gain01(&self) -> bool {
        matches!(self.variant(), GAIN01_A::GAIN01)
    }
}
///Field `GAIN01` writer - Gain value of area 1Unsigned 11-bit fixed point
pub type GAIN01_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN01_A, crate::Safe>;
impl<'a, REG> GAIN01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN01/1024
    #[inline(always)]
    pub fn gain01(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN01_A::GAIN01)
    }
}
/**Gain value of area 0.Unsigned 11-bit fixed point.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN00_A {
    ///0: GAIN00/1024
    GAIN00 = 0,
}
impl From<GAIN00_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN00_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN00_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN00_A {}
///Field `GAIN00` reader - Gain value of area 0.Unsigned 11-bit fixed point.
pub type GAIN00_R = crate::FieldReader<GAIN00_A>;
impl GAIN00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN00_A {
        match self.bits {
            _ => GAIN00_A::GAIN00,
        }
    }
    ///GAIN00/1024
    #[inline(always)]
    pub fn is_gain00(&self) -> bool {
        matches!(self.variant(), GAIN00_A::GAIN00)
    }
}
///Field `GAIN00` writer - Gain value of area 0.Unsigned 11-bit fixed point.
pub type GAIN00_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN00_A, crate::Safe>;
impl<'a, REG> GAIN00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN00/1024
    #[inline(always)]
    pub fn gain00(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN00_A::GAIN00)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 1Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain01(&self) -> GAIN01_R {
        GAIN01_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 0.Unsigned 11-bit fixed point.
    #[inline(always)]
    pub fn gain00(&self) -> GAIN00_R {
        GAIN00_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 1Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain01(&mut self) -> GAIN01_W<GAM_LUT1_SPEC> {
        GAIN01_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 0.Unsigned 11-bit fixed point.
    #[inline(always)]
    pub fn gain00(&mut self) -> GAIN00_W<GAM_LUT1_SPEC> {
        GAIN00_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`gam_lut1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT1_SPEC;
impl crate::RegisterSpec for GAM_LUT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut1::R`](R) reader structure
impl crate::Readable for GAM_LUT1_SPEC {}
///`write(|w| ..)` method takes [`gam_lut1::W`](W) writer structure
impl crate::Writable for GAM_LUT1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT1 to value 0
impl crate::Resettable for GAM_LUT1_SPEC {}
