///Register `STCHSELR` reader
pub type R = crate::R<STCHSELR_SPEC>;
///Register `STCHSELR` writer
pub type W = crate::W<STCHSELR_SPEC>;
/**Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSEL_A {
    ///0: Time information from synchronization from the SYNFP0 module is used.
    _0 = 0,
    ///1: Time information from synchronization from the SYNFP1 module is used.
    _1 = 1,
}
impl From<SYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSEL` reader - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1.
pub type SYSEL_R = crate::BitReader<SYSEL_A>;
impl SYSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSEL_A {
        match self.bits {
            false => SYSEL_A::_0,
            true => SYSEL_A::_1,
        }
    }
    ///Time information from synchronization from the SYNFP0 module is used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYSEL_A::_0
    }
    ///Time information from synchronization from the SYNFP1 module is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYSEL_A::_1
    }
}
///Field `SYSEL` writer - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1.
pub type SYSEL_W<'a, REG> = crate::BitWriter<'a, REG, SYSEL_A>;
impl<'a, REG> SYSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Time information from synchronization from the SYNFP0 module is used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSEL_A::_0)
    }
    ///Time information from synchronization from the SYNFP1 module is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSEL_A::_1)
    }
}
impl R {
    ///Bit 0 - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1.
    #[inline(always)]
    pub fn sysel(&self) -> SYSEL_R {
        SYSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1.
    #[inline(always)]
    pub fn sysel(&mut self) -> SYSEL_W<STCHSELR_SPEC> {
        SYSEL_W::new(self, 0)
    }
}
/**Time Synchronization Channel Select Register

You can [`read`](crate::Reg::read) this register and get [`stchselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stchselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STCHSELR_SPEC;
impl crate::RegisterSpec for STCHSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`stchselr::R`](R) reader structure
impl crate::Readable for STCHSELR_SPEC {}
///`write(|w| ..)` method takes [`stchselr::W`](W) writer structure
impl crate::Writable for STCHSELR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STCHSELR to value 0
impl crate::Resettable for STCHSELR_SPEC {}
