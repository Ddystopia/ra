///Register `NRDYENB` reader
pub type R = crate::R<NRDYENB_SPEC>;
///Register `NRDYENB` writer
pub type W = crate::W<NRDYENB_SPEC>;
/**NRDY Interrupt Enable for Each Pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPENRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPENRDYE_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPENRDYE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPENRDYE_A {
    type Ux = u16;
}
impl crate::IsEnum for PIPENRDYE_A {}
///Field `PIPENRDYE` reader - NRDY Interrupt Enable for Each Pipe
pub type PIPENRDYE_R = crate::FieldReader<PIPENRDYE_A>;
impl PIPENRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPENRDYE_A> {
        match self.bits {
            0 => Some(PIPENRDYE_A::_0),
            1 => Some(PIPENRDYE_A::_1),
            _ => None,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPENRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPENRDYE_A::_1
    }
}
///Field `PIPENRDYE` writer - NRDY Interrupt Enable for Each Pipe
pub type PIPENRDYE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, PIPENRDYE_A>;
impl<'a, REG> PIPENRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPENRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPENRDYE_A::_1)
    }
}
impl R {
    ///Bits 0:9 - NRDY Interrupt Enable for Each Pipe
    #[inline(always)]
    pub fn pipenrdye(&self) -> PIPENRDYE_R {
        PIPENRDYE_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - NRDY Interrupt Enable for Each Pipe
    #[inline(always)]
    pub fn pipenrdye(&mut self) -> PIPENRDYE_W<NRDYENB_SPEC> {
        PIPENRDYE_W::new(self, 0)
    }
}
/**NRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NRDYENB_SPEC;
impl crate::RegisterSpec for NRDYENB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nrdyenb::R`](R) reader structure
impl crate::Readable for NRDYENB_SPEC {}
///`write(|w| ..)` method takes [`nrdyenb::W`](W) writer structure
impl crate::Writable for NRDYENB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NRDYENB to value 0
impl crate::Resettable for NRDYENB_SPEC {}
