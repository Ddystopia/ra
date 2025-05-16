///Register `BEMPSTS` reader
pub type R = crate::R<BEMPSTS_SPEC>;
///Register `BEMPSTS` writer
pub type W = crate::W<BEMPSTS_SPEC>;
/**BEMP Interrupt Status for Each Pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPEBEMP_A {
    ///0: Interrupts are not generated
    _0 = 0,
    ///1: Interrupts are generated
    _1 = 1,
}
impl From<PIPEBEMP_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPEBEMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPEBEMP_A {
    type Ux = u16;
}
impl crate::IsEnum for PIPEBEMP_A {}
///Field `PIPEBEMP` reader - BEMP Interrupt Status for Each Pipe
pub type PIPEBEMP_R = crate::FieldReader<PIPEBEMP_A>;
impl PIPEBEMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPEBEMP_A> {
        match self.bits {
            0 => Some(PIPEBEMP_A::_0),
            1 => Some(PIPEBEMP_A::_1),
            _ => None,
        }
    }
    ///Interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPEBEMP_A::_0
    }
    ///Interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPEBEMP_A::_1
    }
}
///Field `PIPEBEMP` writer - BEMP Interrupt Status for Each Pipe
pub type PIPEBEMP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, PIPEBEMP_A>;
impl<'a, REG> PIPEBEMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBEMP_A::_0)
    }
    ///Interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPEBEMP_A::_1)
    }
}
impl R {
    ///Bits 0:9 - BEMP Interrupt Status for Each Pipe
    #[inline(always)]
    pub fn pipebemp(&self) -> PIPEBEMP_R {
        PIPEBEMP_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - BEMP Interrupt Status for Each Pipe
    #[inline(always)]
    pub fn pipebemp(&mut self) -> PIPEBEMP_W<BEMPSTS_SPEC> {
        PIPEBEMP_W::new(self, 0)
    }
}
/**BEMP Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`bempsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BEMPSTS_SPEC;
impl crate::RegisterSpec for BEMPSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`bempsts::R`](R) reader structure
impl crate::Readable for BEMPSTS_SPEC {}
///`write(|w| ..)` method takes [`bempsts::W`](W) writer structure
impl crate::Writable for BEMPSTS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BEMPSTS to value 0
impl crate::Resettable for BEMPSTS_SPEC {}
