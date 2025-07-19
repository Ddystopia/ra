///Register `MEMWAIT` reader
pub type R = crate::R<MEMWAIT_SPEC>;
///Register `MEMWAIT` writer
pub type W = crate::W<MEMWAIT_SPEC>;
/**Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\[2:0\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMWAIT_A {
    ///0: no wait
    _0 = 0,
    ///1: wait
    _1 = 1,
}
impl From<MEMWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMWAIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MEMWAIT` reader - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\[2:0\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).
pub type MEMWAIT_R = crate::BitReader<MEMWAIT_A>;
impl MEMWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MEMWAIT_A {
        match self.bits {
            false => MEMWAIT_A::_0,
            true => MEMWAIT_A::_1,
        }
    }
    ///no wait
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MEMWAIT_A::_0
    }
    ///wait
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MEMWAIT_A::_1
    }
}
///Field `MEMWAIT` writer - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\[2:0\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).
pub type MEMWAIT_W<'a, REG> = crate::BitWriter<'a, REG, MEMWAIT_A>;
impl<'a, REG> MEMWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no wait
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MEMWAIT_A::_0)
    }
    ///wait
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MEMWAIT_A::_1)
    }
}
impl R {
    ///Bit 0 - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\[2:0\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Memory Wait Cycle Select Note: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\[2:0\] bits select the system clock source that is faster than 32 MHz (ICLK > 32 MHz).
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W<'_, MEMWAIT_SPEC> {
        MEMWAIT_W::new(self, 0)
    }
}
/**Memory Wait Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`memwait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memwait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEMWAIT_SPEC;
impl crate::RegisterSpec for MEMWAIT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`memwait::R`](R) reader structure
impl crate::Readable for MEMWAIT_SPEC {}
///`write(|w| ..)` method takes [`memwait::W`](W) writer structure
impl crate::Writable for MEMWAIT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMWAIT to value 0
impl crate::Resettable for MEMWAIT_SPEC {}
