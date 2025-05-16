///Register `CACR0` reader
pub type R = crate::R<CACR0_SPEC>;
///Register `CACR0` writer
pub type W = crate::W<CACR0_SPEC>;
/**Clock Frequency Measurement Enable.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFME_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<CFME_A> for bool {
    #[inline(always)]
    fn from(variant: CFME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CFME` reader - Clock Frequency Measurement Enable.
pub type CFME_R = crate::BitReader<CFME_A>;
impl CFME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CFME_A {
        match self.bits {
            false => CFME_A::_0,
            true => CFME_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFME_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFME_A::_1
    }
}
///Field `CFME` writer - Clock Frequency Measurement Enable.
pub type CFME_W<'a, REG> = crate::BitWriter<'a, REG, CFME_A>;
impl<'a, REG> CFME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CFME_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CFME_A::_1)
    }
}
impl R {
    ///Bit 0 - Clock Frequency Measurement Enable.
    #[inline(always)]
    pub fn cfme(&self) -> CFME_R {
        CFME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clock Frequency Measurement Enable.
    #[inline(always)]
    pub fn cfme(&mut self) -> CFME_W<CACR0_SPEC> {
        CFME_W::new(self, 0)
    }
}
/**CAC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cacr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACR0_SPEC;
impl crate::RegisterSpec for CACR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cacr0::R`](R) reader structure
impl crate::Readable for CACR0_SPEC {}
///`write(|w| ..)` method takes [`cacr0::W`](W) writer structure
impl crate::Writable for CACR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CACR0 to value 0
impl crate::Resettable for CACR0_SPEC {}
