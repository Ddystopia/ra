///Register `RMCR` reader
pub type R = crate::R<RMCR_SPEC>;
///Register `RMCR` writer
pub type W = crate::W<RMCR_SPEC>;
/**Receive Request Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNR_A {
    ///0: EDRRR.RR bit (receive request bit) is set to 0 when one frame has been received.
    _0 = 0,
    ///1: EDRRR.RR bit (receive request bit) is not set to 0 when one frame has been received.
    _1 = 1,
}
impl From<RNR_A> for bool {
    #[inline(always)]
    fn from(variant: RNR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RNR` reader - Receive Request Reset
pub type RNR_R = crate::BitReader<RNR_A>;
impl RNR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNR_A {
        match self.bits {
            false => RNR_A::_0,
            true => RNR_A::_1,
        }
    }
    ///EDRRR.RR bit (receive request bit) is set to 0 when one frame has been received.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNR_A::_0
    }
    ///EDRRR.RR bit (receive request bit) is not set to 0 when one frame has been received.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNR_A::_1
    }
}
///Field `RNR` writer - Receive Request Reset
pub type RNR_W<'a, REG> = crate::BitWriter<'a, REG, RNR_A>;
impl<'a, REG> RNR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EDRRR.RR bit (receive request bit) is set to 0 when one frame has been received.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RNR_A::_0)
    }
    ///EDRRR.RR bit (receive request bit) is not set to 0 when one frame has been received.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RNR_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive Request Reset
    #[inline(always)]
    pub fn rnr(&self) -> RNR_R {
        RNR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive Request Reset
    #[inline(always)]
    pub fn rnr(&mut self) -> RNR_W<RMCR_SPEC> {
        RNR_W::new(self, 0)
    }
}
/**Receive Method Control Register

You can [`read`](crate::Reg::read) this register and get [`rmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMCR_SPEC;
impl crate::RegisterSpec for RMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rmcr::R`](R) reader structure
impl crate::Readable for RMCR_SPEC {}
///`write(|w| ..)` method takes [`rmcr::W`](W) writer structure
impl crate::Writable for RMCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMCR to value 0
impl crate::Resettable for RMCR_SPEC {}
