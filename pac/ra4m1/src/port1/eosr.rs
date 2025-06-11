///Register `EOSR` reader
pub type R = crate::R<EOSR_SPEC>;
///Register `EOSR` writer
pub type W = crate::W<EOSR_SPEC>;
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
impl R {
    ///Bits 0:15 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr(&self) -> EOSR_R {
        EOSR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr(&mut self) -> EOSR_W<EOSR_SPEC> {
        EOSR_W::new(self, 0)
    }
}
/**Event output reset register

You can [`read`](crate::Reg::read) this register and get [`eosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EOSR_SPEC;
impl crate::RegisterSpec for EOSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`eosr::R`](R) reader structure
impl crate::Readable for EOSR_SPEC {}
///`write(|w| ..)` method takes [`eosr::W`](W) writer structure
impl crate::Writable for EOSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EOSR to value 0
impl crate::Resettable for EOSR_SPEC {}
