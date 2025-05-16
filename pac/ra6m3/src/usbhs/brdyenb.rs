///Register `BRDYENB` reader
pub type R = crate::R<BRDYENB_SPEC>;
///Register `BRDYENB` writer
pub type W = crate::W<BRDYENB_SPEC>;
/**BRDY Interrupt Enable for Each Pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPEBRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPEBRDYE_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPEBRDYE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPEBRDYE_A {
    type Ux = u16;
}
impl crate::IsEnum for PIPEBRDYE_A {}
///Field `PIPEBRDYE` reader - BRDY Interrupt Enable for Each Pipe
pub type PIPEBRDYE_R = crate::FieldReader<PIPEBRDYE_A>;
impl PIPEBRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPEBRDYE_A> {
        match self.bits {
            0 => Some(PIPEBRDYE_A::_0),
            1 => Some(PIPEBRDYE_A::_1),
            _ => None,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPEBRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPEBRDYE_A::_1
    }
}
///Field `PIPEBRDYE` writer - BRDY Interrupt Enable for Each Pipe
pub type PIPEBRDYE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, PIPEBRDYE_A>;
impl<'a, REG> PIPEBRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBRDYE_A::_1)
    }
}
impl R {
    ///Bits 0:9 - BRDY Interrupt Enable for Each Pipe
    #[inline(always)]
    pub fn pipebrdye(&self) -> PIPEBRDYE_R {
        PIPEBRDYE_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - BRDY Interrupt Enable for Each Pipe
    #[inline(always)]
    pub fn pipebrdye(&mut self) -> PIPEBRDYE_W<BRDYENB_SPEC> {
        PIPEBRDYE_W::new(self, 0)
    }
}
/**BRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`brdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRDYENB_SPEC;
impl crate::RegisterSpec for BRDYENB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`brdyenb::R`](R) reader structure
impl crate::Readable for BRDYENB_SPEC {}
///`write(|w| ..)` method takes [`brdyenb::W`](W) writer structure
impl crate::Writable for BRDYENB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRDYENB to value 0
impl crate::Resettable for BRDYENB_SPEC {}
