///Register `GAM%s_LUT4` reader
pub type R = crate::R<GAM_LUT4_SPEC>;
///Register `GAM%s_LUT4` writer
pub type W = crate::W<GAM_LUT4_SPEC>;
/**Gain value of area 7Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN07_A {
    ///0: GAIN07/1024
    GAIN07 = 0,
}
impl From<GAIN07_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN07_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN07_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN07_A {}
///Field `GAIN07` reader - Gain value of area 7Unsigned 11-bit fixed point
pub type GAIN07_R = crate::FieldReader<GAIN07_A>;
impl GAIN07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN07_A {
        match self.bits {
            _ => GAIN07_A::GAIN07,
        }
    }
    ///GAIN07/1024
    #[inline(always)]
    pub fn is_gain07(&self) -> bool {
        matches!(self.variant(), GAIN07_A::GAIN07)
    }
}
///Field `GAIN07` writer - Gain value of area 7Unsigned 11-bit fixed point
pub type GAIN07_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN07_A, crate::Safe>;
impl<'a, REG> GAIN07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN07/1024
    #[inline(always)]
    pub fn gain07(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN07_A::GAIN07)
    }
}
/**Gain value of area 6Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN06_A {
    ///0: GAIN06/1024
    GAIN06 = 0,
}
impl From<GAIN06_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN06_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN06_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN06_A {}
///Field `GAIN06` reader - Gain value of area 6Unsigned 11-bit fixed point
pub type GAIN06_R = crate::FieldReader<GAIN06_A>;
impl GAIN06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN06_A {
        match self.bits {
            _ => GAIN06_A::GAIN06,
        }
    }
    ///GAIN06/1024
    #[inline(always)]
    pub fn is_gain06(&self) -> bool {
        matches!(self.variant(), GAIN06_A::GAIN06)
    }
}
///Field `GAIN06` writer - Gain value of area 6Unsigned 11-bit fixed point
pub type GAIN06_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN06_A, crate::Safe>;
impl<'a, REG> GAIN06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN06/1024
    #[inline(always)]
    pub fn gain06(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN06_A::GAIN06)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 7Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain07(&self) -> GAIN07_R {
        GAIN07_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 6Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain06(&self) -> GAIN06_R {
        GAIN06_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 7Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain07(&mut self) -> GAIN07_W<GAM_LUT4_SPEC> {
        GAIN07_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 6Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain06(&mut self) -> GAIN06_W<GAM_LUT4_SPEC> {
        GAIN06_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 4

You can [`read`](crate::Reg::read) this register and get [`gam_lut4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT4_SPEC;
impl crate::RegisterSpec for GAM_LUT4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut4::R`](R) reader structure
impl crate::Readable for GAM_LUT4_SPEC {}
///`write(|w| ..)` method takes [`gam_lut4::W`](W) writer structure
impl crate::Writable for GAM_LUT4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT4 to value 0
impl crate::Resettable for GAM_LUT4_SPEC {}
