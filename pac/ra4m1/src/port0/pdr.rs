///Register `PDR` reader
pub type R = crate::R<PDR_SPEC>;
///Register `PDR` writer
pub type W = crate::W<PDR_SPEC>;
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PDR_A {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin).
    _1 = 1,
}
impl From<PDR_A> for u16 {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PDR_A {
    type Ux = u16;
}
impl crate::IsEnum for PDR_A {}
///Field `PDR` reader - Pmn Direction
pub type PDR_R = crate::FieldReader<PDR_A>;
impl PDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PDR_A> {
        match self.bits {
            0 => Some(PDR_A::_0),
            1 => Some(PDR_A::_1),
            _ => None,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR_A::_0
    }
    ///Output (functions as an output pin).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR_A::_1
    }
}
///Field `PDR` writer - Pmn Direction
pub type PDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PDR_A>;
impl<'a, REG> PDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDR_A::_0)
    }
    ///Output (functions as an output pin).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDR_A::_1)
    }
}
impl R {
    ///Bits 0:15 - Pmn Direction
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Pmn Direction
    #[inline(always)]
    pub fn pdr(&mut self) -> PDR_W<PDR_SPEC> {
        PDR_W::new(self, 0)
    }
}
/**Data direction register

You can [`read`](crate::Reg::read) this register and get [`pdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PDR_SPEC;
impl crate::RegisterSpec for PDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pdr::R`](R) reader structure
impl crate::Readable for PDR_SPEC {}
///`write(|w| ..)` method takes [`pdr::W`](W) writer structure
impl crate::Writable for PDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDR to value 0
impl crate::Resettable for PDR_SPEC {}
