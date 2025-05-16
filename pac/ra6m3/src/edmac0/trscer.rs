///Register `TRSCER` reader
pub type R = crate::R<TRSCER_SPEC>;
///Register `TRSCER` writer
pub type W = crate::W<TRSCER_SPEC>;
/**RRF Flag Copy Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFCE_A {
    ///0: The EDMACn.EESR.RRF flag status is reflected in the RDn.RFE bit of the receive descriptor.
    _0 = 0,
    ///1: The EDMACn.EESR.RRF flag status is not reflected in the RDn.RFE bit of the receive descriptor.
    _1 = 1,
}
impl From<RRFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFCE` reader - RRF Flag Copy Enable
pub type RRFCE_R = crate::BitReader<RRFCE_A>;
impl RRFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRFCE_A {
        match self.bits {
            false => RRFCE_A::_0,
            true => RRFCE_A::_1,
        }
    }
    ///The EDMACn.EESR.RRF flag status is reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRFCE_A::_0
    }
    ///The EDMACn.EESR.RRF flag status is not reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRFCE_A::_1
    }
}
///Field `RRFCE` writer - RRF Flag Copy Enable
pub type RRFCE_W<'a, REG> = crate::BitWriter<'a, REG, RRFCE_A>;
impl<'a, REG> RRFCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The EDMACn.EESR.RRF flag status is reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RRFCE_A::_0)
    }
    ///The EDMACn.EESR.RRF flag status is not reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RRFCE_A::_1)
    }
}
/**RMAF Flag Copy Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMAFCE_A {
    ///0: The EDMACn.EESR.RMAF flag status is reflected in the RDn.RFE bit of the receive descriptor.
    _0 = 0,
    ///1: The EDMACn.EESR.RMAF flag status is not reflected in the RDn.RFE bit of the receive descriptor.
    _1 = 1,
}
impl From<RMAFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RMAFCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RMAFCE` reader - RMAF Flag Copy Enable
pub type RMAFCE_R = crate::BitReader<RMAFCE_A>;
impl RMAFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMAFCE_A {
        match self.bits {
            false => RMAFCE_A::_0,
            true => RMAFCE_A::_1,
        }
    }
    ///The EDMACn.EESR.RMAF flag status is reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMAFCE_A::_0
    }
    ///The EDMACn.EESR.RMAF flag status is not reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMAFCE_A::_1
    }
}
///Field `RMAFCE` writer - RMAF Flag Copy Enable
pub type RMAFCE_W<'a, REG> = crate::BitWriter<'a, REG, RMAFCE_A>;
impl<'a, REG> RMAFCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The EDMACn.EESR.RMAF flag status is reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RMAFCE_A::_0)
    }
    ///The EDMACn.EESR.RMAF flag status is not reflected in the RDn.RFE bit of the receive descriptor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RMAFCE_A::_1)
    }
}
impl R {
    ///Bit 4 - RRF Flag Copy Enable
    #[inline(always)]
    pub fn rrfce(&self) -> RRFCE_R {
        RRFCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - RMAF Flag Copy Enable
    #[inline(always)]
    pub fn rmafce(&self) -> RMAFCE_R {
        RMAFCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - RRF Flag Copy Enable
    #[inline(always)]
    pub fn rrfce(&mut self) -> RRFCE_W<TRSCER_SPEC> {
        RRFCE_W::new(self, 4)
    }
    ///Bit 7 - RMAF Flag Copy Enable
    #[inline(always)]
    pub fn rmafce(&mut self) -> RMAFCE_W<TRSCER_SPEC> {
        RMAFCE_W::new(self, 7)
    }
}
/**ETHERC/EDMAC Transmit/Receive Status Copy Enable Register

You can [`read`](crate::Reg::read) this register and get [`trscer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trscer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TRSCER_SPEC;
impl crate::RegisterSpec for TRSCER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`trscer::R`](R) reader structure
impl crate::Readable for TRSCER_SPEC {}
///`write(|w| ..)` method takes [`trscer::W`](W) writer structure
impl crate::Writable for TRSCER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRSCER to value 0
impl crate::Resettable for TRSCER_SPEC {}
