///Register `OUT_PDTHA` reader
pub type R = crate::R<OUT_PDTHA_SPEC>;
///Register `OUT_PDTHA` writer
pub type W = crate::W<OUT_PDTHA_SPEC>;
///Field `PD` reader - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PD_R = crate::FieldReader;
///Field `PD` writer - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PC` reader - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PC_R = crate::FieldReader;
///Field `PC` writer - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PB` reader - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PB_R = crate::FieldReader;
///Field `PB` writer - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PA` reader - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PA_R = crate::FieldReader;
///Field `PA` writer - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Output format select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORM_A {
    ///3: Setting prohibited
    _11 = 3,
    ///2: RGB565; select RGB565 as output interface format.
    _10 = 2,
    ///1: RGB666; select RGB666 as output interface format.
    _01 = 1,
    ///0: RGB888; select RGB888 or serial RGB as output interface format.
    _00 = 0,
}
impl From<FORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORM_A {
    type Ux = u8;
}
impl crate::IsEnum for FORM_A {}
///Field `FORM` reader - Output format select
pub type FORM_R = crate::FieldReader<FORM_A>;
impl FORM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORM_A {
        match self.bits {
            3 => FORM_A::_11,
            2 => FORM_A::_10,
            1 => FORM_A::_01,
            0 => FORM_A::_00,
            _ => unreachable!(),
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FORM_A::_11
    }
    ///RGB565; select RGB565 as output interface format.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FORM_A::_10
    }
    ///RGB666; select RGB666 as output interface format.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FORM_A::_01
    }
    ///RGB888; select RGB888 or serial RGB as output interface format.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FORM_A::_00
    }
}
///Field `FORM` writer - Output format select
pub type FORM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FORM_A, crate::Safe>;
impl<'a, REG> FORM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(FORM_A::_11)
    }
    ///RGB565; select RGB565 as output interface format.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(FORM_A::_10)
    }
    ///RGB666; select RGB666 as output interface format.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(FORM_A::_01)
    }
    ///RGB888; select RGB888 or serial RGB as output interface format.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(FORM_A::_00)
    }
}
/**Operation mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    ///3: Setting prohibited
    _11 = 3,
    ///2: 2x2 pattern dither
    _10 = 2,
    ///1: Round-off
    _01 = 1,
    ///0: Truncate
    _00 = 0,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SEL_A {}
///Field `SEL` reader - Operation mode
pub type SEL_R = crate::FieldReader<SEL_A>;
impl SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEL_A {
        match self.bits {
            3 => SEL_A::_11,
            2 => SEL_A::_10,
            1 => SEL_A::_01,
            0 => SEL_A::_00,
            _ => unreachable!(),
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL_A::_11
    }
    ///2x2 pattern dither
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL_A::_10
    }
    ///Round-off
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEL_A::_01
    }
    ///Truncate
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEL_A::_00
    }
}
///Field `SEL` writer - Operation mode
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SEL_A, crate::Safe>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_11)
    }
    ///2x2 pattern dither
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_10)
    }
    ///Round-off
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_01)
    }
    ///Truncate
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_00)
    }
}
impl R {
    ///Bits 0:1 - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - Output format select
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - Operation mode
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<OUT_PDTHA_SPEC> {
        PD_W::new(self, 0)
    }
    ///Bits 4:5 - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<OUT_PDTHA_SPEC> {
        PC_W::new(self, 4)
    }
    ///Bits 8:9 - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W<OUT_PDTHA_SPEC> {
        PB_W::new(self, 8)
    }
    ///Bits 12:13 - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<OUT_PDTHA_SPEC> {
        PA_W::new(self, 12)
    }
    ///Bits 16:17 - Output format select
    #[inline(always)]
    pub fn form(&mut self) -> FORM_W<OUT_PDTHA_SPEC> {
        FORM_W::new(self, 16)
    }
    ///Bits 20:21 - Operation mode
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<OUT_PDTHA_SPEC> {
        SEL_W::new(self, 20)
    }
}
/**Output Control Block Panel Dither Correction Register

You can [`read`](crate::Reg::read) this register and get [`out_pdtha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_pdtha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_PDTHA_SPEC;
impl crate::RegisterSpec for OUT_PDTHA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_pdtha::R`](R) reader structure
impl crate::Readable for OUT_PDTHA_SPEC {}
///`write(|w| ..)` method takes [`out_pdtha::W`](W) writer structure
impl crate::Writable for OUT_PDTHA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUT_PDTHA to value 0
impl crate::Resettable for OUT_PDTHA_SPEC {}
