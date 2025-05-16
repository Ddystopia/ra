///Register `GAM%s_LUT5` reader
pub type R = crate::R<GAM_LUT5_SPEC>;
///Register `GAM%s_LUT5` writer
pub type W = crate::W<GAM_LUT5_SPEC>;
/**Gain value of area 9Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN09_A {
    ///0: GAIN09/1024
    GAIN09 = 0,
}
impl From<GAIN09_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN09_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN09_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN09_A {}
///Field `GAIN09` reader - Gain value of area 9Unsigned 11-bit fixed point
pub type GAIN09_R = crate::FieldReader<GAIN09_A>;
impl GAIN09_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN09_A {
        match self.bits {
            _ => GAIN09_A::GAIN09,
        }
    }
    ///GAIN09/1024
    #[inline(always)]
    pub fn is_gain09(&self) -> bool {
        matches!(self.variant(), GAIN09_A::GAIN09)
    }
}
///Field `GAIN09` writer - Gain value of area 9Unsigned 11-bit fixed point
pub type GAIN09_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN09_A, crate::Safe>;
impl<'a, REG> GAIN09_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN09/1024
    #[inline(always)]
    pub fn gain09(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN09_A::GAIN09)
    }
}
/**Gain value of area 8Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN08_A {
    ///0: GAIN08/1024
    GAIN08 = 0,
}
impl From<GAIN08_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN08_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN08_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN08_A {}
///Field `GAIN08` reader - Gain value of area 8Unsigned 11-bit fixed point
pub type GAIN08_R = crate::FieldReader<GAIN08_A>;
impl GAIN08_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN08_A {
        match self.bits {
            _ => GAIN08_A::GAIN08,
        }
    }
    ///GAIN08/1024
    #[inline(always)]
    pub fn is_gain08(&self) -> bool {
        matches!(self.variant(), GAIN08_A::GAIN08)
    }
}
///Field `GAIN08` writer - Gain value of area 8Unsigned 11-bit fixed point
pub type GAIN08_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN08_A, crate::Safe>;
impl<'a, REG> GAIN08_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN08/1024
    #[inline(always)]
    pub fn gain08(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN08_A::GAIN08)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 9Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain09(&self) -> GAIN09_R {
        GAIN09_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 8Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain08(&self) -> GAIN08_R {
        GAIN08_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 9Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain09(&mut self) -> GAIN09_W<GAM_LUT5_SPEC> {
        GAIN09_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 8Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain08(&mut self) -> GAIN08_W<GAM_LUT5_SPEC> {
        GAIN08_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 5

You can [`read`](crate::Reg::read) this register and get [`gam_lut5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT5_SPEC;
impl crate::RegisterSpec for GAM_LUT5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut5::R`](R) reader structure
impl crate::Readable for GAM_LUT5_SPEC {}
///`write(|w| ..)` method takes [`gam_lut5::W`](W) writer structure
impl crate::Writable for GAM_LUT5_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT5 to value 0
impl crate::Resettable for GAM_LUT5_SPEC {}
