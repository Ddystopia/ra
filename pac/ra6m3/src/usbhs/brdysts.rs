///Register `BRDYSTS` reader
pub type R = crate::R<BRDYSTS_SPEC>;
///Register `BRDYSTS` writer
pub type W = crate::W<BRDYSTS_SPEC>;
/**BRDY Interrupt Status for Each Pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPEBRDY_A {
    ///0: Interrupts are not generated
    _0 = 0,
    ///1: Interrupts are generated
    _1 = 1,
}
impl From<PIPEBRDY_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPEBRDY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPEBRDY_A {
    type Ux = u16;
}
impl crate::IsEnum for PIPEBRDY_A {}
///Field `PIPEBRDY` reader - BRDY Interrupt Status for Each Pipe
pub type PIPEBRDY_R = crate::FieldReader<PIPEBRDY_A>;
impl PIPEBRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPEBRDY_A> {
        match self.bits {
            0 => Some(PIPEBRDY_A::_0),
            1 => Some(PIPEBRDY_A::_1),
            _ => None,
        }
    }
    ///Interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPEBRDY_A::_0
    }
    ///Interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPEBRDY_A::_1
    }
}
///Field `PIPEBRDY` writer - BRDY Interrupt Status for Each Pipe
pub type PIPEBRDY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, PIPEBRDY_A>;
impl<'a, REG> PIPEBRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBRDY_A::_0)
    }
    ///Interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBRDY_A::_1)
    }
}
impl R {
    ///Bits 0:9 - BRDY Interrupt Status for Each Pipe
    #[inline(always)]
    pub fn pipebrdy(&self) -> PIPEBRDY_R {
        PIPEBRDY_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - BRDY Interrupt Status for Each Pipe
    #[inline(always)]
    pub fn pipebrdy(&mut self) -> PIPEBRDY_W<BRDYSTS_SPEC> {
        PIPEBRDY_W::new(self, 0)
    }
}
/**BRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`brdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRDYSTS_SPEC;
impl crate::RegisterSpec for BRDYSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`brdysts::R`](R) reader structure
impl crate::Readable for BRDYSTS_SPEC {}
///`write(|w| ..)` method takes [`brdysts::W`](W) writer structure
impl crate::Writable for BRDYSTS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRDYSTS to value 0
impl crate::Resettable for BRDYSTS_SPEC {}
