///Register `LOCOCR` reader
pub type R = crate::R<LOCOCR_SPEC>;
///Register `LOCOCR` writer
pub type W = crate::W<LOCOCR_SPEC>;
/**LOCO Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCSTP_A {
    ///0: Operate the LOCO clock
    _0 = 0,
    ///1: Stop the LOCO clock
    _1 = 1,
}
impl From<LCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: LCSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCSTP` reader - LOCO Stop
pub type LCSTP_R = crate::BitReader<LCSTP_A>;
impl LCSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCSTP_A {
        match self.bits {
            false => LCSTP_A::_0,
            true => LCSTP_A::_1,
        }
    }
    ///Operate the LOCO clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCSTP_A::_0
    }
    ///Stop the LOCO clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCSTP_A::_1
    }
}
///Field `LCSTP` writer - LOCO Stop
pub type LCSTP_W<'a, REG> = crate::BitWriter<'a, REG, LCSTP_A>;
impl<'a, REG> LCSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate the LOCO clock
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCSTP_A::_0)
    }
    ///Stop the LOCO clock
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - LOCO Stop
    #[inline(always)]
    pub fn lcstp(&self) -> LCSTP_R {
        LCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LOCO Stop
    #[inline(always)]
    pub fn lcstp(&mut self) -> LCSTP_W<LOCOCR_SPEC> {
        LCSTP_W::new(self, 0)
    }
}
/**Low-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`lococr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lococr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOCOCR_SPEC;
impl crate::RegisterSpec for LOCOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lococr::R`](R) reader structure
impl crate::Readable for LOCOCR_SPEC {}
///`write(|w| ..)` method takes [`lococr::W`](W) writer structure
impl crate::Writable for LOCOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCOCR to value 0
impl crate::Resettable for LOCOCR_SPEC {}
