///Register `OUT_CONTRAST` reader
pub type R = crate::R<OUT_CONTRAST_SPEC>;
///Register `OUT_CONTRAST` writer
pub type W = crate::W<OUT_CONTRAST_SPEC>;
/**Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONTR_A {
    ///0: CONTR/128
    CONTR = 0,
}
impl From<CONTR_A> for u8 {
    #[inline(always)]
    fn from(variant: CONTR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CONTR_A {
    type Ux = u8;
}
impl crate::IsEnum for CONTR_A {}
///Field `CONTR` reader - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point
pub type CONTR_R = crate::FieldReader<CONTR_A>;
impl CONTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONTR_A {
        match self.bits {
            _ => CONTR_A::CONTR,
        }
    }
    ///CONTR/128
    #[inline(always)]
    pub fn is_contr(&self) -> bool {
        matches!(self.variant(), CONTR_A::CONTR)
    }
}
///Field `CONTR` writer - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point
pub type CONTR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CONTR_A, crate::Safe>;
impl<'a, REG> CONTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CONTR/128
    #[inline(always)]
    pub fn contr(self) -> &'a mut crate::W<REG> {
        self.variant(CONTR_A::CONTR)
    }
}
/**Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONTB_A {
    ///0: CONTB/128
    CONTB = 0,
}
impl From<CONTB_A> for u8 {
    #[inline(always)]
    fn from(variant: CONTB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CONTB_A {
    type Ux = u8;
}
impl crate::IsEnum for CONTB_A {}
///Field `CONTB` reader - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point
pub type CONTB_R = crate::FieldReader<CONTB_A>;
impl CONTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONTB_A {
        match self.bits {
            _ => CONTB_A::CONTB,
        }
    }
    ///CONTB/128
    #[inline(always)]
    pub fn is_contb(&self) -> bool {
        matches!(self.variant(), CONTB_A::CONTB)
    }
}
///Field `CONTB` writer - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point
pub type CONTB_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CONTB_A, crate::Safe>;
impl<'a, REG> CONTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CONTB/128
    #[inline(always)]
    pub fn contb(self) -> &'a mut crate::W<REG> {
        self.variant(CONTB_A::CONTB)
    }
}
/**Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONTG_A {
    ///0: CONTG/128
    CONTG = 0,
}
impl From<CONTG_A> for u8 {
    #[inline(always)]
    fn from(variant: CONTG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CONTG_A {
    type Ux = u8;
}
impl crate::IsEnum for CONTG_A {}
///Field `CONTG` reader - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point.
pub type CONTG_R = crate::FieldReader<CONTG_A>;
impl CONTG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONTG_A {
        match self.bits {
            _ => CONTG_A::CONTG,
        }
    }
    ///CONTG/128
    #[inline(always)]
    pub fn is_contg(&self) -> bool {
        matches!(self.variant(), CONTG_A::CONTG)
    }
}
///Field `CONTG` writer - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point.
pub type CONTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CONTG_A, crate::Safe>;
impl<'a, REG> CONTG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CONTG/128
    #[inline(always)]
    pub fn contg(self) -> &'a mut crate::W<REG> {
        self.variant(CONTG_A::CONTG)
    }
}
impl R {
    ///Bits 0:7 - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point
    #[inline(always)]
    pub fn contr(&self) -> CONTR_R {
        CONTR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point
    #[inline(always)]
    pub fn contb(&self) -> CONTB_R {
        CONTB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point.
    #[inline(always)]
    pub fn contg(&self) -> CONTG_R {
        CONTG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point
    #[inline(always)]
    pub fn contr(&mut self) -> CONTR_W<OUT_CONTRAST_SPEC> {
        CONTR_W::new(self, 0)
    }
    ///Bits 8:15 - Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point
    #[inline(always)]
    pub fn contb(&mut self) -> CONTB_W<OUT_CONTRAST_SPEC> {
        CONTB_W::new(self, 8)
    }
    ///Bits 16:23 - Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point.
    #[inline(always)]
    pub fn contg(&mut self) -> CONTG_W<OUT_CONTRAST_SPEC> {
        CONTG_W::new(self, 16)
    }
}
/**Output Control Block Contrast Correction Register

You can [`read`](crate::Reg::read) this register and get [`out_contrast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_contrast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_CONTRAST_SPEC;
impl crate::RegisterSpec for OUT_CONTRAST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_contrast::R`](R) reader structure
impl crate::Readable for OUT_CONTRAST_SPEC {}
///`write(|w| ..)` method takes [`out_contrast::W`](W) writer structure
impl crate::Writable for OUT_CONTRAST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUT_CONTRAST to value 0
impl crate::Resettable for OUT_CONTRAST_SPEC {}
