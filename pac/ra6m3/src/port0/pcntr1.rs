///Register `PCNTR1` reader
pub type R = crate::R<PCNTR1_SPEC>;
///Register `PCNTR1` writer
pub type W = crate::W<PCNTR1_SPEC>;
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
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PODR_A {
    ///0: Low output
    _0 = 0,
    ///1: High output.
    _1 = 1,
}
impl From<PODR_A> for u16 {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PODR_A {
    type Ux = u16;
}
impl crate::IsEnum for PODR_A {}
///Field `PODR` reader - Pmn Output Data
pub type PODR_R = crate::FieldReader<PODR_A>;
impl PODR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PODR_A> {
        match self.bits {
            0 => Some(PODR_A::_0),
            1 => Some(PODR_A::_1),
            _ => None,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    ///High output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
///Field `PODR` writer - Pmn Output Data
pub type PODR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PODR_A>;
impl<'a, REG> PODR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_0)
    }
    ///High output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_1)
    }
}
impl R {
    ///Bits 0:15 - Pmn Direction
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Pmn Output Data
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Pmn Direction
    #[inline(always)]
    pub fn pdr(&mut self) -> PDR_W<PCNTR1_SPEC> {
        PDR_W::new(self, 0)
    }
    ///Bits 16:31 - Pmn Output Data
    #[inline(always)]
    pub fn podr(&mut self) -> PODR_W<PCNTR1_SPEC> {
        PODR_W::new(self, 16)
    }
}
/**Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pcntr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCNTR1_SPEC;
impl crate::RegisterSpec for PCNTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcntr1::R`](R) reader structure
impl crate::Readable for PCNTR1_SPEC {}
///`write(|w| ..)` method takes [`pcntr1::W`](W) writer structure
impl crate::Writable for PCNTR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCNTR1 to value 0
impl crate::Resettable for PCNTR1_SPEC {}
