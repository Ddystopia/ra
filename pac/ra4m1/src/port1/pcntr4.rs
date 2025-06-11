///Register `PCNTR4` reader
pub type R = crate::R<PCNTR4_SPEC>;
///Register `PCNTR4` writer
pub type W = crate::W<PCNTR4_SPEC>;
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EOSR_A {
    ///0: No affect to output
    _0 = 0,
    ///1: High output.
    _1 = 1,
}
impl From<EOSR_A> for u16 {
    #[inline(always)]
    fn from(variant: EOSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EOSR_A {
    type Ux = u16;
}
impl crate::IsEnum for EOSR_A {}
///Field `EOSR` reader - Pmn Event Output Set
pub type EOSR_R = crate::FieldReader<EOSR_A>;
impl EOSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EOSR_A> {
        match self.bits {
            0 => Some(EOSR_A::_0),
            1 => Some(EOSR_A::_1),
            _ => None,
        }
    }
    ///No affect to output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOSR_A::_0
    }
    ///High output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOSR_A::_1
    }
}
///Field `EOSR` writer - Pmn Event Output Set
pub type EOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, EOSR_A>;
impl<'a, REG> EOSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No affect to output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSR_A::_0)
    }
    ///High output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSR_A::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EORR_A {
    ///0: No affect to output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<EORR_A> for u16 {
    #[inline(always)]
    fn from(variant: EORR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EORR_A {
    type Ux = u16;
}
impl crate::IsEnum for EORR_A {}
///Field `EORR` reader - Pmn Event Output Reset
pub type EORR_R = crate::FieldReader<EORR_A>;
impl EORR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EORR_A> {
        match self.bits {
            0 => Some(EORR_A::_0),
            1 => Some(EORR_A::_1),
            _ => None,
        }
    }
    ///No affect to output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EORR_A::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EORR_A::_1
    }
}
///Field `EORR` writer - Pmn Event Output Reset
pub type EORR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, EORR_A>;
impl<'a, REG> EORR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No affect to output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EORR_A::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EORR_A::_1)
    }
}
impl R {
    ///Bits 0:15 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr(&self) -> EOSR_R {
        EOSR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr(&self) -> EORR_R {
        EORR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr(&mut self) -> EOSR_W<PCNTR4_SPEC> {
        EOSR_W::new(self, 0)
    }
    ///Bits 16:31 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr(&mut self) -> EORR_W<PCNTR4_SPEC> {
        EORR_W::new(self, 16)
    }
}
/**Port Control Register 4

You can [`read`](crate::Reg::read) this register and get [`pcntr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCNTR4_SPEC;
impl crate::RegisterSpec for PCNTR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcntr4::R`](R) reader structure
impl crate::Readable for PCNTR4_SPEC {}
///`write(|w| ..)` method takes [`pcntr4::W`](W) writer structure
impl crate::Writable for PCNTR4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCNTR4 to value 0
impl crate::Resettable for PCNTR4_SPEC {}
