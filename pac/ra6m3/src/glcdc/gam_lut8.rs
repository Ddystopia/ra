///Register `GAM%s_LUT8` reader
pub type R = crate::R<GAM_LUT8_SPEC>;
///Register `GAM%s_LUT8` writer
pub type W = crate::W<GAM_LUT8_SPEC>;
/**Gain value of area 15Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN15_A {
    ///0: GAIN15/1024
    GAIN15 = 0,
}
impl From<GAIN15_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN15_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN15_A {}
///Field `GAIN15` reader - Gain value of area 15Unsigned 11-bit fixed point
pub type GAIN15_R = crate::FieldReader<GAIN15_A>;
impl GAIN15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN15_A {
        match self.bits {
            _ => GAIN15_A::GAIN15,
        }
    }
    ///GAIN15/1024
    #[inline(always)]
    pub fn is_gain15(&self) -> bool {
        matches!(self.variant(), GAIN15_A::GAIN15)
    }
}
///Field `GAIN15` writer - Gain value of area 15Unsigned 11-bit fixed point
pub type GAIN15_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN15_A, crate::Safe>;
impl<'a, REG> GAIN15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN15/1024
    #[inline(always)]
    pub fn gain15(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN15_A::GAIN15)
    }
}
/**Gain value of area 14Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN14_A {
    ///0: GAIN14/1024
    GAIN14 = 0,
}
impl From<GAIN14_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN14_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN14_A {}
///Field `GAIN14` reader - Gain value of area 14Unsigned 11-bit fixed point
pub type GAIN14_R = crate::FieldReader<GAIN14_A>;
impl GAIN14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN14_A {
        match self.bits {
            _ => GAIN14_A::GAIN14,
        }
    }
    ///GAIN14/1024
    #[inline(always)]
    pub fn is_gain14(&self) -> bool {
        matches!(self.variant(), GAIN14_A::GAIN14)
    }
}
///Field `GAIN14` writer - Gain value of area 14Unsigned 11-bit fixed point
pub type GAIN14_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN14_A, crate::Safe>;
impl<'a, REG> GAIN14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN14/1024
    #[inline(always)]
    pub fn gain14(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN14_A::GAIN14)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 15Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain15(&self) -> GAIN15_R {
        GAIN15_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 14Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain14(&self) -> GAIN14_R {
        GAIN14_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 15Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain15(&mut self) -> GAIN15_W<GAM_LUT8_SPEC> {
        GAIN15_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 14Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain14(&mut self) -> GAIN14_W<GAM_LUT8_SPEC> {
        GAIN14_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 8

You can [`read`](crate::Reg::read) this register and get [`gam_lut8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT8_SPEC;
impl crate::RegisterSpec for GAM_LUT8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut8::R`](R) reader structure
impl crate::Readable for GAM_LUT8_SPEC {}
///`write(|w| ..)` method takes [`gam_lut8::W`](W) writer structure
impl crate::Writable for GAM_LUT8_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT8 to value 0
impl crate::Resettable for GAM_LUT8_SPEC {}
