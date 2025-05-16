///Register `DAADUSR` reader
pub type R = crate::R<DAADUSR_SPEC>;
///Register `DAADUSR` writer
pub type W = crate::W<DAADUSR_SPEC>;
/**A/D Unit 1 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMADSEL1_A {
    ///0: Unit 1 is not selected.
    _0 = 0,
    ///1: Unit 1 is selected.
    _1 = 1,
}
impl From<AMADSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: AMADSEL1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMADSEL1` reader - A/D Unit 1 Select
pub type AMADSEL1_R = crate::BitReader<AMADSEL1_A>;
impl AMADSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMADSEL1_A {
        match self.bits {
            false => AMADSEL1_A::_0,
            true => AMADSEL1_A::_1,
        }
    }
    ///Unit 1 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMADSEL1_A::_0
    }
    ///Unit 1 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMADSEL1_A::_1
    }
}
///Field `AMADSEL1` writer - A/D Unit 1 Select
pub type AMADSEL1_W<'a, REG> = crate::BitWriter<'a, REG, AMADSEL1_A>;
impl<'a, REG> AMADSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Unit 1 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMADSEL1_A::_0)
    }
    ///Unit 1 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMADSEL1_A::_1)
    }
}
impl R {
    ///Bit 1 - A/D Unit 1 Select
    #[inline(always)]
    pub fn amadsel1(&self) -> AMADSEL1_R {
        AMADSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - A/D Unit 1 Select
    #[inline(always)]
    pub fn amadsel1(&mut self) -> AMADSEL1_W<DAADUSR_SPEC> {
        AMADSEL1_W::new(self, 1)
    }
}
/**D/A A/D Synchronous Unit Select Register

You can [`read`](crate::Reg::read) this register and get [`daadusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAADUSR_SPEC;
impl crate::RegisterSpec for DAADUSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`daadusr::R`](R) reader structure
impl crate::Readable for DAADUSR_SPEC {}
///`write(|w| ..)` method takes [`daadusr::W`](W) writer structure
impl crate::Writable for DAADUSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAADUSR to value 0
impl crate::Resettable for DAADUSR_SPEC {}
