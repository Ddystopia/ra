///Register `FLLCR1` reader
pub type R = crate::R<FLLCR1_SPEC>;
///Register `FLLCR1` writer
pub type W = crate::W<FLLCR1_SPEC>;
/**FLL Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLLEN_A {
    ///0: FLL function is disabled.
    _0 = 0,
    ///1: FLL function is enabled.
    _1 = 1,
}
impl From<FLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLLEN` reader - FLL Enable
pub type FLLEN_R = crate::BitReader<FLLEN_A>;
impl FLLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLLEN_A {
        match self.bits {
            false => FLLEN_A::_0,
            true => FLLEN_A::_1,
        }
    }
    ///FLL function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLLEN_A::_0
    }
    ///FLL function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLLEN_A::_1
    }
}
///Field `FLLEN` writer - FLL Enable
pub type FLLEN_W<'a, REG> = crate::BitWriter<'a, REG, FLLEN_A>;
impl<'a, REG> FLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLL function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLLEN_A::_0)
    }
    ///FLL function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLLEN_A::_1)
    }
}
impl R {
    ///Bit 0 - FLL Enable
    #[inline(always)]
    pub fn fllen(&self) -> FLLEN_R {
        FLLEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FLL Enable
    #[inline(always)]
    pub fn fllen(&mut self) -> FLLEN_W<FLLCR1_SPEC> {
        FLLEN_W::new(self, 0)
    }
}
/**FLL Control Register 1

You can [`read`](crate::Reg::read) this register and get [`fllcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FLLCR1_SPEC;
impl crate::RegisterSpec for FLLCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`fllcr1::R`](R) reader structure
impl crate::Readable for FLLCR1_SPEC {}
///`write(|w| ..)` method takes [`fllcr1::W`](W) writer structure
impl crate::Writable for FLLCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLLCR1 to value 0
impl crate::Resettable for FLLCR1_SPEC {}
