///Register `PORR` writer
pub type W = crate::W<PORR_SPEC>;
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
    ///Bits 0:15 - Pmn Output Reset
    #[inline(always)]
    pub fn porr(&mut self) -> PORR_W<PORR_SPEC> {
        PORR_W::new(self, 0)
    }
}
/**Output set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PORR_SPEC;
impl crate::RegisterSpec for PORR_SPEC {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`porr::W`](W) writer structure
impl crate::Writable for PORR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PORR to value 0
impl crate::Resettable for PORR_SPEC {}
