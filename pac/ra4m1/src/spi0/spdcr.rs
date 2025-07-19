///Register `SPDCR` reader
pub type R = crate::R<SPDCR_SPEC>;
///Register `SPDCR` writer
pub type W = crate::W<SPDCR_SPEC>;
/**RSPI Receive/Transmit Data Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRDTD_A {
    ///0: SPDR values are read from the receive buffer
    _0 = 0,
    ///1: SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)
    _1 = 1,
}
impl From<SPRDTD_A> for bool {
    #[inline(always)]
    fn from(variant: SPRDTD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPRDTD` reader - RSPI Receive/Transmit Data Selection
pub type SPRDTD_R = crate::BitReader<SPRDTD_A>;
impl SPRDTD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPRDTD_A {
        match self.bits {
            false => SPRDTD_A::_0,
            true => SPRDTD_A::_1,
        }
    }
    ///SPDR values are read from the receive buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRDTD_A::_0
    }
    ///SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRDTD_A::_1
    }
}
///Field `SPRDTD` writer - RSPI Receive/Transmit Data Selection
pub type SPRDTD_W<'a, REG> = crate::BitWriter<'a, REG, SPRDTD_A>;
impl<'a, REG> SPRDTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPDR values are read from the receive buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPRDTD_A::_0)
    }
    ///SPDR values are read from the transmit buffer (but only if the transmit buffer is empty)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPRDTD_A::_1)
    }
}
/**SPI Word Access/Halfword Access Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLW_A {
    ///0: SPDR_HA is valid to access in halfwords
    _0 = 0,
    ///1: SPDR is valid (to access in words).
    _1 = 1,
}
impl From<SPLW_A> for bool {
    #[inline(always)]
    fn from(variant: SPLW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPLW` reader - SPI Word Access/Halfword Access Specification
pub type SPLW_R = crate::BitReader<SPLW_A>;
impl SPLW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPLW_A {
        match self.bits {
            false => SPLW_A::_0,
            true => SPLW_A::_1,
        }
    }
    ///SPDR_HA is valid to access in halfwords
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLW_A::_0
    }
    ///SPDR is valid (to access in words).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLW_A::_1
    }
}
///Field `SPLW` writer - SPI Word Access/Halfword Access Specification
pub type SPLW_W<'a, REG> = crate::BitWriter<'a, REG, SPLW_A>;
impl<'a, REG> SPLW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPDR_HA is valid to access in halfwords
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPLW_A::_0)
    }
    ///SPDR is valid (to access in words).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPLW_A::_1)
    }
}
impl R {
    ///Bit 4 - RSPI Receive/Transmit Data Selection
    #[inline(always)]
    pub fn sprdtd(&self) -> SPRDTD_R {
        SPRDTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPI Word Access/Halfword Access Specification
    #[inline(always)]
    pub fn splw(&self) -> SPLW_R {
        SPLW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - RSPI Receive/Transmit Data Selection
    #[inline(always)]
    pub fn sprdtd(&mut self) -> SPRDTD_W<'_, SPDCR_SPEC> {
        SPRDTD_W::new(self, 4)
    }
    ///Bit 5 - SPI Word Access/Halfword Access Specification
    #[inline(always)]
    pub fn splw(&mut self) -> SPLW_W<'_, SPDCR_SPEC> {
        SPLW_W::new(self, 5)
    }
}
/**SPI Data Control Register

You can [`read`](crate::Reg::read) this register and get [`spdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPDCR_SPEC;
impl crate::RegisterSpec for SPDCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spdcr::R`](R) reader structure
impl crate::Readable for SPDCR_SPEC {}
///`write(|w| ..)` method takes [`spdcr::W`](W) writer structure
impl crate::Writable for SPDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDCR to value 0
impl crate::Resettable for SPDCR_SPEC {}
