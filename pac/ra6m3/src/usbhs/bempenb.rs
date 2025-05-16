///Register `BEMPENB` reader
pub type R = crate::R<BEMPENB_SPEC>;
///Register `BEMPENB` writer
pub type W = crate::W<BEMPENB_SPEC>;
/**BEMP Interrupt Enable for Each Pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPEBEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPEBEMPE_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPEBEMPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPEBEMPE_A {
    type Ux = u16;
}
impl crate::IsEnum for PIPEBEMPE_A {}
///Field `PIPEBEMPE` reader - BEMP Interrupt Enable for Each Pipe
pub type PIPEBEMPE_R = crate::FieldReader<PIPEBEMPE_A>;
impl PIPEBEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPEBEMPE_A> {
        match self.bits {
            0 => Some(PIPEBEMPE_A::_0),
            1 => Some(PIPEBEMPE_A::_1),
            _ => None,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPEBEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPEBEMPE_A::_1
    }
}
///Field `PIPEBEMPE` writer - BEMP Interrupt Enable for Each Pipe
pub type PIPEBEMPE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, PIPEBEMPE_A>;
impl<'a, REG> PIPEBEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBEMPE_A::_1)
    }
}
impl R {
    ///Bits 0:9 - BEMP Interrupt Enable for Each Pipe
    #[inline(always)]
    pub fn pipebempe(&self) -> PIPEBEMPE_R {
        PIPEBEMPE_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - BEMP Interrupt Enable for Each Pipe
    #[inline(always)]
    pub fn pipebempe(&mut self) -> PIPEBEMPE_W<BEMPENB_SPEC> {
        PIPEBEMPE_W::new(self, 0)
    }
}
/**BEMP Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`bempenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BEMPENB_SPEC;
impl crate::RegisterSpec for BEMPENB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`bempenb::R`](R) reader structure
impl crate::Readable for BEMPENB_SPEC {}
///`write(|w| ..)` method takes [`bempenb::W`](W) writer structure
impl crate::Writable for BEMPENB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BEMPENB to value 0
impl crate::Resettable for BEMPENB_SPEC {}
