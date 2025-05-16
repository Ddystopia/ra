///Register `LCIVLDR` writer
pub type W = crate::W<LCIVLDR_SPEC>;
/**Local Time Counter Initial Value Load Directive

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOAD_A {
    ///0: The initial value is not loaded into the local time counter.
    _0 = 0,
    ///1: The initial value is loaded into the local time counter.
    _1 = 1,
}
impl From<LOAD_A> for bool {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LOAD` writer - Local Time Counter Initial Value Load Directive
pub type LOAD_W<'a, REG> = crate::BitWriter<'a, REG, LOAD_A>;
impl<'a, REG> LOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The initial value is not loaded into the local time counter.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LOAD_A::_0)
    }
    ///The initial value is loaded into the local time counter.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LOAD_A::_1)
    }
}
impl W {
    ///Bit 0 - Local Time Counter Initial Value Load Directive
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<LCIVLDR_SPEC> {
        LOAD_W::new(self, 0)
    }
}
/**Local Time Counter Initial Value Load Directive Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivldr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCIVLDR_SPEC;
impl crate::RegisterSpec for LCIVLDR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lcivldr::W`](W) writer structure
impl crate::Writable for LCIVLDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCIVLDR to value 0
impl crate::Resettable for LCIVLDR_SPEC {}
