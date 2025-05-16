///Register `MOCOCR` reader
pub type R = crate::R<MOCOCR_SPEC>;
///Register `MOCOCR` writer
pub type W = crate::W<MOCOCR_SPEC>;
/**MOCO Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCSTP_A {
    ///0: Operate the MOCO clock
    _0 = 0,
    ///1: Stop the MOCO clock
    _1 = 1,
}
impl From<MCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: MCSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MCSTP` reader - MOCO Stop
pub type MCSTP_R = crate::BitReader<MCSTP_A>;
impl MCSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCSTP_A {
        match self.bits {
            false => MCSTP_A::_0,
            true => MCSTP_A::_1,
        }
    }
    ///Operate the MOCO clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCSTP_A::_0
    }
    ///Stop the MOCO clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCSTP_A::_1
    }
}
///Field `MCSTP` writer - MOCO Stop
pub type MCSTP_W<'a, REG> = crate::BitWriter<'a, REG, MCSTP_A>;
impl<'a, REG> MCSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate the MOCO clock
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MCSTP_A::_0)
    }
    ///Stop the MOCO clock
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MCSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - MOCO Stop
    #[inline(always)]
    pub fn mcstp(&self) -> MCSTP_R {
        MCSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MOCO Stop
    #[inline(always)]
    pub fn mcstp(&mut self) -> MCSTP_W<MOCOCR_SPEC> {
        MCSTP_W::new(self, 0)
    }
}
/**Middle-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mococr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mococr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MOCOCR_SPEC;
impl crate::RegisterSpec for MOCOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mococr::R`](R) reader structure
impl crate::Readable for MOCOCR_SPEC {}
///`write(|w| ..)` method takes [`mococr::W`](W) writer structure
impl crate::Writable for MOCOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOCOCR to value 0
impl crate::Resettable for MOCOCR_SPEC {}
