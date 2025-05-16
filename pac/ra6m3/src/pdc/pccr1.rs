///Register `PCCR1` reader
pub type R = crate::R<PCCR1_SPEC>;
///Register `PCCR1` writer
pub type W = crate::W<PCCR1_SPEC>;
/**PDC Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE_A {
    ///0: Operations for reception are disabled.
    _0 = 0,
    ///1: Operations for reception are enabled.
    _1 = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCE` reader - PDC Operation Enable
pub type PCE_R = crate::BitReader<PCE_A>;
impl PCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::_0,
            true => PCE_A::_1,
        }
    }
    ///Operations for reception are disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCE_A::_0
    }
    ///Operations for reception are enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCE_A::_1
    }
}
///Field `PCE` writer - PDC Operation Enable
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG, PCE_A>;
impl<'a, REG> PCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operations for reception are disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PCE_A::_0)
    }
    ///Operations for reception are enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PCE_A::_1)
    }
}
impl R {
    ///Bit 0 - PDC Operation Enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PDC Operation Enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<PCCR1_SPEC> {
        PCE_W::new(self, 0)
    }
}
/**PDC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCCR1_SPEC;
impl crate::RegisterSpec for PCCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pccr1::R`](R) reader structure
impl crate::Readable for PCCR1_SPEC {}
///`write(|w| ..)` method takes [`pccr1::W`](W) writer structure
impl crate::Writable for PCCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCCR1 to value 0
impl crate::Resettable for PCCR1_SPEC {}
