///Register `GAM%s_LUT6` reader
pub type R = crate::R<GAM_LUT6_SPEC>;
///Register `GAM%s_LUT6` writer
pub type W = crate::W<GAM_LUT6_SPEC>;
/**Gain value of area 11Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN11_A {
    ///0: GAIN11/1024
    GAIN11 = 0,
}
impl From<GAIN11_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN11_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN11_A {}
///Field `GAIN11` reader - Gain value of area 11Unsigned 11-bit fixed point
pub type GAIN11_R = crate::FieldReader<GAIN11_A>;
impl GAIN11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN11_A {
        match self.bits {
            _ => GAIN11_A::GAIN11,
        }
    }
    ///GAIN11/1024
    #[inline(always)]
    pub fn is_gain11(&self) -> bool {
        matches!(self.variant(), GAIN11_A::GAIN11)
    }
}
///Field `GAIN11` writer - Gain value of area 11Unsigned 11-bit fixed point
pub type GAIN11_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN11_A, crate::Safe>;
impl<'a, REG> GAIN11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN11/1024
    #[inline(always)]
    pub fn gain11(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN11_A::GAIN11)
    }
}
/**Gain value of area 10Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN10_A {
    ///0: GAIN10/1024
    GAIN10 = 0,
}
impl From<GAIN10_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN10_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN10_A {}
///Field `GAIN10` reader - Gain value of area 10Unsigned 11-bit fixed point
pub type GAIN10_R = crate::FieldReader<GAIN10_A>;
impl GAIN10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN10_A {
        match self.bits {
            _ => GAIN10_A::GAIN10,
        }
    }
    ///GAIN10/1024
    #[inline(always)]
    pub fn is_gain10(&self) -> bool {
        matches!(self.variant(), GAIN10_A::GAIN10)
    }
}
///Field `GAIN10` writer - Gain value of area 10Unsigned 11-bit fixed point
pub type GAIN10_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN10_A, crate::Safe>;
impl<'a, REG> GAIN10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN10/1024
    #[inline(always)]
    pub fn gain10(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN10_A::GAIN10)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 11Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain11(&self) -> GAIN11_R {
        GAIN11_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 10Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain10(&self) -> GAIN10_R {
        GAIN10_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 11Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain11(&mut self) -> GAIN11_W<GAM_LUT6_SPEC> {
        GAIN11_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 10Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain10(&mut self) -> GAIN10_W<GAM_LUT6_SPEC> {
        GAIN10_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 6

You can [`read`](crate::Reg::read) this register and get [`gam_lut6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT6_SPEC;
impl crate::RegisterSpec for GAM_LUT6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut6::R`](R) reader structure
impl crate::Readable for GAM_LUT6_SPEC {}
///`write(|w| ..)` method takes [`gam_lut6::W`](W) writer structure
impl crate::Writable for GAM_LUT6_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT6 to value 0
impl crate::Resettable for GAM_LUT6_SPEC {}
