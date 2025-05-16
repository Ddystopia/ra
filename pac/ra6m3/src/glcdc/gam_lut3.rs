///Register `GAM%s_LUT3` reader
pub type R = crate::R<GAM_LUT3_SPEC>;
///Register `GAM%s_LUT3` writer
pub type W = crate::W<GAM_LUT3_SPEC>;
/**Gain value of area 5Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN05_A {
    ///0: GAIN05/1024
    GAIN05 = 0,
}
impl From<GAIN05_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN05_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN05_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN05_A {}
///Field `GAIN05` reader - Gain value of area 5Unsigned 11-bit fixed point
pub type GAIN05_R = crate::FieldReader<GAIN05_A>;
impl GAIN05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN05_A {
        match self.bits {
            _ => GAIN05_A::GAIN05,
        }
    }
    ///GAIN05/1024
    #[inline(always)]
    pub fn is_gain05(&self) -> bool {
        matches!(self.variant(), GAIN05_A::GAIN05)
    }
}
///Field `GAIN05` writer - Gain value of area 5Unsigned 11-bit fixed point
pub type GAIN05_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN05_A, crate::Safe>;
impl<'a, REG> GAIN05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN05/1024
    #[inline(always)]
    pub fn gain05(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN05_A::GAIN05)
    }
}
/**Gain value of area 4Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN04_A {
    ///0: GAIN04/1024
    GAIN04 = 0,
}
impl From<GAIN04_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN04_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN04_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN04_A {}
///Field `GAIN04` reader - Gain value of area 4Unsigned 11-bit fixed point
pub type GAIN04_R = crate::FieldReader<GAIN04_A>;
impl GAIN04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN04_A {
        match self.bits {
            _ => GAIN04_A::GAIN04,
        }
    }
    ///GAIN04/1024
    #[inline(always)]
    pub fn is_gain04(&self) -> bool {
        matches!(self.variant(), GAIN04_A::GAIN04)
    }
}
///Field `GAIN04` writer - Gain value of area 4Unsigned 11-bit fixed point
pub type GAIN04_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN04_A, crate::Safe>;
impl<'a, REG> GAIN04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN04/1024
    #[inline(always)]
    pub fn gain04(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN04_A::GAIN04)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 5Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain05(&self) -> GAIN05_R {
        GAIN05_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 4Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain04(&self) -> GAIN04_R {
        GAIN04_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 5Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain05(&mut self) -> GAIN05_W<GAM_LUT3_SPEC> {
        GAIN05_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 4Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain04(&mut self) -> GAIN04_W<GAM_LUT3_SPEC> {
        GAIN04_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 3

You can [`read`](crate::Reg::read) this register and get [`gam_lut3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT3_SPEC;
impl crate::RegisterSpec for GAM_LUT3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut3::R`](R) reader structure
impl crate::Readable for GAM_LUT3_SPEC {}
///`write(|w| ..)` method takes [`gam_lut3::W`](W) writer structure
impl crate::Writable for GAM_LUT3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT3 to value 0
impl crate::Resettable for GAM_LUT3_SPEC {}
