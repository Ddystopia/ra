///Register `EDRRR` reader
pub type R = crate::R<EDRRR_SPEC>;
///Register `EDRRR` writer
pub type W = crate::W<EDRRR_SPEC>;
/**Receive Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR_A {
    ///0: Receive function is disabled.
    _0 = 0,
    ///1: Receive descriptor is read, and the receive function is enabled.
    _1 = 1,
}
impl From<RR_A> for bool {
    #[inline(always)]
    fn from(variant: RR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RR` reader - Receive Request
pub type RR_R = crate::BitReader<RR_A>;
impl RR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RR_A {
        match self.bits {
            false => RR_A::_0,
            true => RR_A::_1,
        }
    }
    ///Receive function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RR_A::_0
    }
    ///Receive descriptor is read, and the receive function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RR_A::_1
    }
}
///Field `RR` writer - Receive Request
pub type RR_W<'a, REG> = crate::BitWriter<'a, REG, RR_A>;
impl<'a, REG> RR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RR_A::_0)
    }
    ///Receive descriptor is read, and the receive function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RR_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive Request
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive Request
    #[inline(always)]
    pub fn rr(&mut self) -> RR_W<EDRRR_SPEC> {
        RR_W::new(self, 0)
    }
}
/**EDMAC Receive Request Register

You can [`read`](crate::Reg::read) this register and get [`edrrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edrrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EDRRR_SPEC;
impl crate::RegisterSpec for EDRRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`edrrr::R`](R) reader structure
impl crate::Readable for EDRRR_SPEC {}
///`write(|w| ..)` method takes [`edrrr::W`](W) writer structure
impl crate::Writable for EDRRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDRRR to value 0
impl crate::Resettable for EDRRR_SPEC {}
