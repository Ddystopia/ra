///Register `GAM%s_LUT2` reader
pub type R = crate::R<GAM_LUT2_SPEC>;
///Register `GAM%s_LUT2` writer
pub type W = crate::W<GAM_LUT2_SPEC>;
/**Gain value of area 3Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN03_A {
    ///0: GAIN03/1024
    GAIN03 = 0,
}
impl From<GAIN03_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN03_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN03_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN03_A {}
///Field `GAIN03` reader - Gain value of area 3Unsigned 11-bit fixed point
pub type GAIN03_R = crate::FieldReader<GAIN03_A>;
impl GAIN03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN03_A {
        match self.bits {
            _ => GAIN03_A::GAIN03,
        }
    }
    ///GAIN03/1024
    #[inline(always)]
    pub fn is_gain03(&self) -> bool {
        matches!(self.variant(), GAIN03_A::GAIN03)
    }
}
///Field `GAIN03` writer - Gain value of area 3Unsigned 11-bit fixed point
pub type GAIN03_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN03_A, crate::Safe>;
impl<'a, REG> GAIN03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN03/1024
    #[inline(always)]
    pub fn gain03(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN03_A::GAIN03)
    }
}
/**Gain value of area 2Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN02_A {
    ///0: GAIN02/1024
    GAIN02 = 0,
}
impl From<GAIN02_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN02_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN02_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN02_A {}
///Field `GAIN02` reader - Gain value of area 2Unsigned 11-bit fixed point
pub type GAIN02_R = crate::FieldReader<GAIN02_A>;
impl GAIN02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN02_A {
        match self.bits {
            _ => GAIN02_A::GAIN02,
        }
    }
    ///GAIN02/1024
    #[inline(always)]
    pub fn is_gain02(&self) -> bool {
        matches!(self.variant(), GAIN02_A::GAIN02)
    }
}
///Field `GAIN02` writer - Gain value of area 2Unsigned 11-bit fixed point
pub type GAIN02_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN02_A, crate::Safe>;
impl<'a, REG> GAIN02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN02/1024
    #[inline(always)]
    pub fn gain02(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN02_A::GAIN02)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 3Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain03(&self) -> GAIN03_R {
        GAIN03_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 2Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain02(&self) -> GAIN02_R {
        GAIN02_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 3Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain03(&mut self) -> GAIN03_W<GAM_LUT2_SPEC> {
        GAIN03_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 2Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain02(&mut self) -> GAIN02_W<GAM_LUT2_SPEC> {
        GAIN02_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 2

You can [`read`](crate::Reg::read) this register and get [`gam_lut2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT2_SPEC;
impl crate::RegisterSpec for GAM_LUT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut2::R`](R) reader structure
impl crate::Readable for GAM_LUT2_SPEC {}
///`write(|w| ..)` method takes [`gam_lut2::W`](W) writer structure
impl crate::Writable for GAM_LUT2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT2 to value 0
impl crate::Resettable for GAM_LUT2_SPEC {}
