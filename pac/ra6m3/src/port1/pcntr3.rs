///Register `PCNTR3` writer
pub type W = crate::W<PCNTR3_SPEC>;
/**Pmn Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum POSR_A {
    ///0: No affect to output
    _0 = 0,
    ///1: High output.
    _1 = 1,
}
impl From<POSR_A> for u16 {
    #[inline(always)]
    fn from(variant: POSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POSR_A {
    type Ux = u16;
}
impl crate::IsEnum for POSR_A {}
///Field `POSR` writer - Pmn Output Set
pub type POSR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, POSR_A>;
impl<'a, REG> POSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No affect to output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(POSR_A::_0)
    }
    ///High output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(POSR_A::_1)
    }
}
/**Pmn Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PORR_A {
    ///0: No affect to output
    _0 = 0,
    ///1: Low output.
    _1 = 1,
}
impl From<PORR_A> for u16 {
    #[inline(always)]
    fn from(variant: PORR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORR_A {
    type Ux = u16;
}
impl crate::IsEnum for PORR_A {}
///Field `PORR` writer - Pmn Output Reset
pub type PORR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PORR_A>;
impl<'a, REG> PORR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///No affect to output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PORR_A::_0)
    }
    ///Low output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PORR_A::_1)
    }
}
impl W {
    ///Bits 0:15 - Pmn Output Set
    #[inline(always)]
    pub fn posr(&mut self) -> POSR_W<PCNTR3_SPEC> {
        POSR_W::new(self, 0)
    }
    ///Bits 16:31 - Pmn Output Reset
    #[inline(always)]
    pub fn porr(&mut self) -> PORR_W<PCNTR3_SPEC> {
        PORR_W::new(self, 16)
    }
}
/**Port Control Register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCNTR3_SPEC;
impl crate::RegisterSpec for PCNTR3_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pcntr3::W`](W) writer structure
impl crate::Writable for PCNTR3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCNTR3 to value 0
impl crate::Resettable for PCNTR3_SPEC {}
