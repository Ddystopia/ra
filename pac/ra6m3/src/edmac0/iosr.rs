///Register `IOSR` reader
pub type R = crate::R<IOSR_SPEC>;
///Register `IOSR` writer
pub type W = crate::W<IOSR_SPEC>;
/**External Loopback Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELB_A {
    ///0: The ETn_EXOUT pin outputs low.
    _0 = 0,
    ///1: The ETn_EXOUT pin outputs high.
    _1 = 1,
}
impl From<ELB_A> for bool {
    #[inline(always)]
    fn from(variant: ELB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELB` reader - External Loopback Mode
pub type ELB_R = crate::BitReader<ELB_A>;
impl ELB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELB_A {
        match self.bits {
            false => ELB_A::_0,
            true => ELB_A::_1,
        }
    }
    ///The ETn_EXOUT pin outputs low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELB_A::_0
    }
    ///The ETn_EXOUT pin outputs high.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELB_A::_1
    }
}
///Field `ELB` writer - External Loopback Mode
pub type ELB_W<'a, REG> = crate::BitWriter<'a, REG, ELB_A>;
impl<'a, REG> ELB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The ETn_EXOUT pin outputs low.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELB_A::_0)
    }
    ///The ETn_EXOUT pin outputs high.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELB_A::_1)
    }
}
impl R {
    ///Bit 0 - External Loopback Mode
    #[inline(always)]
    pub fn elb(&self) -> ELB_R {
        ELB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - External Loopback Mode
    #[inline(always)]
    pub fn elb(&mut self) -> ELB_W<IOSR_SPEC> {
        ELB_W::new(self, 0)
    }
}
/**Independent Output Signal Setting Register

You can [`read`](crate::Reg::read) this register and get [`iosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IOSR_SPEC;
impl crate::RegisterSpec for IOSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`iosr::R`](R) reader structure
impl crate::Readable for IOSR_SPEC {}
///`write(|w| ..)` method takes [`iosr::W`](W) writer structure
impl crate::Writable for IOSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOSR to value 0
impl crate::Resettable for IOSR_SPEC {}
