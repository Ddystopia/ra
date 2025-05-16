///Register `GAM%s_LUT7` reader
pub type R = crate::R<GAM_LUT7_SPEC>;
///Register `GAM%s_LUT7` writer
pub type W = crate::W<GAM_LUT7_SPEC>;
/**Gain value of area 13Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN13_A {
    ///0: GAIN13/1024
    GAIN13 = 0,
}
impl From<GAIN13_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN13_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN13_A {}
///Field `GAIN13` reader - Gain value of area 13Unsigned 11-bit fixed point
pub type GAIN13_R = crate::FieldReader<GAIN13_A>;
impl GAIN13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN13_A {
        match self.bits {
            _ => GAIN13_A::GAIN13,
        }
    }
    ///GAIN13/1024
    #[inline(always)]
    pub fn is_gain13(&self) -> bool {
        matches!(self.variant(), GAIN13_A::GAIN13)
    }
}
///Field `GAIN13` writer - Gain value of area 13Unsigned 11-bit fixed point
pub type GAIN13_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN13_A, crate::Safe>;
impl<'a, REG> GAIN13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN13/1024
    #[inline(always)]
    pub fn gain13(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN13_A::GAIN13)
    }
}
/**Gain value of area 12Unsigned 11-bit fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GAIN12_A {
    ///0: GAIN12/1024
    GAIN12 = 0,
}
impl From<GAIN12_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GAIN12_A {
    type Ux = u16;
}
impl crate::IsEnum for GAIN12_A {}
///Field `GAIN12` reader - Gain value of area 12Unsigned 11-bit fixed point
pub type GAIN12_R = crate::FieldReader<GAIN12_A>;
impl GAIN12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAIN12_A {
        match self.bits {
            _ => GAIN12_A::GAIN12,
        }
    }
    ///GAIN12/1024
    #[inline(always)]
    pub fn is_gain12(&self) -> bool {
        matches!(self.variant(), GAIN12_A::GAIN12)
    }
}
///Field `GAIN12` writer - Gain value of area 12Unsigned 11-bit fixed point
pub type GAIN12_W<'a, REG> = crate::FieldWriter<'a, REG, 11, GAIN12_A, crate::Safe>;
impl<'a, REG> GAIN12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///GAIN12/1024
    #[inline(always)]
    pub fn gain12(self) -> &'a mut crate::W<REG> {
        self.variant(GAIN12_A::GAIN12)
    }
}
impl R {
    ///Bits 0:10 - Gain value of area 13Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain13(&self) -> GAIN13_R {
        GAIN13_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Gain value of area 12Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain12(&self) -> GAIN12_R {
        GAIN12_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Gain value of area 13Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain13(&mut self) -> GAIN13_W<GAM_LUT7_SPEC> {
        GAIN13_W::new(self, 0)
    }
    ///Bits 16:26 - Gain value of area 12Unsigned 11-bit fixed point
    #[inline(always)]
    pub fn gain12(&mut self) -> GAIN12_W<GAM_LUT7_SPEC> {
        GAIN12_W::new(self, 16)
    }
}
/**Gamma %s Correction Block Table Setting Register 7

You can [`read`](crate::Reg::read) this register and get [`gam_lut7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LUT7_SPEC;
impl crate::RegisterSpec for GAM_LUT7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_lut7::R`](R) reader structure
impl crate::Readable for GAM_LUT7_SPEC {}
///`write(|w| ..)` method takes [`gam_lut7::W`](W) writer structure
impl crate::Writable for GAM_LUT7_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM%s_LUT7 to value 0
impl crate::Resettable for GAM_LUT7_SPEC {}
