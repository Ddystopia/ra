///Register `HOCOCR` reader
pub type R = crate::R<HOCOCR_SPEC>;
///Register `HOCOCR` writer
pub type W = crate::W<HOCOCR_SPEC>;
/**HOCO Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCSTP_A {
    ///0: Operate the HOCO clock
    _0 = 0,
    ///1: Stop the HOCO clock
    _1 = 1,
}
impl From<HCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: HCSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HCSTP` reader - HOCO Stop
pub type HCSTP_R = crate::BitReader<HCSTP_A>;
impl HCSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HCSTP_A {
        match self.bits {
            false => HCSTP_A::_0,
            true => HCSTP_A::_1,
        }
    }
    ///Operate the HOCO clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCSTP_A::_0
    }
    ///Stop the HOCO clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCSTP_A::_1
    }
}
///Field `HCSTP` writer - HOCO Stop
pub type HCSTP_W<'a, REG> = crate::BitWriter<'a, REG, HCSTP_A>;
impl<'a, REG> HCSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate the HOCO clock
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HCSTP_A::_0)
    }
    ///Stop the HOCO clock
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HCSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - HOCO Stop
    #[inline(always)]
    pub fn hcstp(&self) -> HCSTP_R {
        HCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - HOCO Stop
    #[inline(always)]
    pub fn hcstp(&mut self) -> HCSTP_W<HOCOCR_SPEC> {
        HCSTP_W::new(self, 0)
    }
}
/**High-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`hococr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOCOCR_SPEC;
impl crate::RegisterSpec for HOCOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`hococr::R`](R) reader structure
impl crate::Readable for HOCOCR_SPEC {}
///`write(|w| ..)` method takes [`hococr::W`](W) writer structure
impl crate::Writable for HOCOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HOCOCR to value 0
impl crate::Resettable for HOCOCR_SPEC {}
