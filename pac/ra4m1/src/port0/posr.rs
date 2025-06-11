///Register `POSR` writer
pub type W = crate::W<POSR_SPEC>;
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
impl W {
    ///Bits 0:15 - Pmn Output Set
    #[inline(always)]
    pub fn posr(&mut self) -> POSR_W<POSR_SPEC> {
        POSR_W::new(self, 0)
    }
}
/**Output set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POSR_SPEC;
impl crate::RegisterSpec for POSR_SPEC {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`posr::W`](W) writer structure
impl crate::Writable for POSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POSR to value 0
impl crate::Resettable for POSR_SPEC {}
