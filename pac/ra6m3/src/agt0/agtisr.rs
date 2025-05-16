///Register `AGTISR` reader
pub type R = crate::R<AGTISR_SPEC>;
///Register `AGTISR` writer
pub type W = crate::W<AGTISR_SPEC>;
/**AGTEE polarty selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEPS_A {
    ///0: An event is counted during the low-level period
    _0 = 0,
    ///1: An event is counted during the high-level period
    _1 = 1,
}
impl From<EEPS_A> for bool {
    #[inline(always)]
    fn from(variant: EEPS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EEPS` reader - AGTEE polarty selection
pub type EEPS_R = crate::BitReader<EEPS_A>;
impl EEPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EEPS_A {
        match self.bits {
            false => EEPS_A::_0,
            true => EEPS_A::_1,
        }
    }
    ///An event is counted during the low-level period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEPS_A::_0
    }
    ///An event is counted during the high-level period
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEPS_A::_1
    }
}
///Field `EEPS` writer - AGTEE polarty selection
pub type EEPS_W<'a, REG> = crate::BitWriter<'a, REG, EEPS_A>;
impl<'a, REG> EEPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///An event is counted during the low-level period
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EEPS_A::_0)
    }
    ///An event is counted during the high-level period
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EEPS_A::_1)
    }
}
impl R {
    ///Bit 2 - AGTEE polarty selection
    #[inline(always)]
    pub fn eeps(&self) -> EEPS_R {
        EEPS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - AGTEE polarty selection
    #[inline(always)]
    pub fn eeps(&mut self) -> EEPS_W<AGTISR_SPEC> {
        EEPS_W::new(self, 2)
    }
}
/**AGT Event Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTISR_SPEC;
impl crate::RegisterSpec for AGTISR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtisr::R`](R) reader structure
impl crate::Readable for AGTISR_SPEC {}
///`write(|w| ..)` method takes [`agtisr::W`](W) writer structure
impl crate::Writable for AGTISR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTISR to value 0
impl crate::Resettable for AGTISR_SPEC {}
