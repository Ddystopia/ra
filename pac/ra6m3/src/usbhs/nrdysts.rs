///Register `NRDYSTS` reader
pub type R = crate::R<NRDYSTS_SPEC>;
///Register `NRDYSTS` writer
pub type W = crate::W<NRDYSTS_SPEC>;
/**NRDY Interrupt Status for Each Pipe

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIPENRDY_A {
    ///0: Interrupts are not generated
    _0 = 0,
    ///1: Interrupts are generated
    _1 = 1,
}
impl From<PIPENRDY_A> for u16 {
    #[inline(always)]
    fn from(variant: PIPENRDY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPENRDY_A {
    type Ux = u16;
}
impl crate::IsEnum for PIPENRDY_A {}
///Field `PIPENRDY` reader - NRDY Interrupt Status for Each Pipe
pub type PIPENRDY_R = crate::FieldReader<PIPENRDY_A>;
impl PIPENRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPENRDY_A> {
        match self.bits {
            0 => Some(PIPENRDY_A::_0),
            1 => Some(PIPENRDY_A::_1),
            _ => None,
        }
    }
    ///Interrupts are not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPENRDY_A::_0
    }
    ///Interrupts are generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPENRDY_A::_1
    }
}
///Field `PIPENRDY` writer - NRDY Interrupt Status for Each Pipe
pub type PIPENRDY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, PIPENRDY_A>;
impl<'a, REG> PIPENRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Interrupts are not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPENRDY_A::_0)
    }
    ///Interrupts are generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPENRDY_A::_1)
    }
}
impl R {
    ///Bits 0:9 - NRDY Interrupt Status for Each Pipe
    #[inline(always)]
    pub fn pipenrdy(&self) -> PIPENRDY_R {
        PIPENRDY_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - NRDY Interrupt Status for Each Pipe
    #[inline(always)]
    pub fn pipenrdy(&mut self) -> PIPENRDY_W<NRDYSTS_SPEC> {
        PIPENRDY_W::new(self, 0)
    }
}
/**NRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nrdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NRDYSTS_SPEC;
impl crate::RegisterSpec for NRDYSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nrdysts::R`](R) reader structure
impl crate::Readable for NRDYSTS_SPEC {}
///`write(|w| ..)` method takes [`nrdysts::W`](W) writer structure
impl crate::Writable for NRDYSTS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NRDYSTS to value 0
impl crate::Resettable for NRDYSTS_SPEC {}
