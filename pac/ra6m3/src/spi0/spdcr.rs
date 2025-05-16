///Register `SPDCR` reader
pub type R = crate::R<SPDCR_SPEC>;
///Register `SPDCR` writer
pub type W = crate::W<SPDCR_SPEC>;
/**Number of Frames Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPFC_A {
    ///0: 1 frame
    _00 = 0,
    ///1: 2 frames
    _01 = 1,
    ///2: 3 frames
    _10 = 2,
    ///3: 4 frames.
    _11 = 3,
}
impl From<SPFC_A> for u8 {
    #[inline(always)]
    fn from(variant: SPFC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPFC_A {
    type Ux = u8;
}
impl crate::IsEnum for SPFC_A {}
///Field `SPFC` reader - Number of Frames Specification
pub type SPFC_R = crate::FieldReader<SPFC_A>;
impl SPFC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPFC_A {
        match self.bits {
            0 => SPFC_A::_00,
            1 => SPFC_A::_01,
            2 => SPFC_A::_10,
            3 => SPFC_A::_11,
            _ => unreachable!(),
        }
    }
    ///1 frame
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SPFC_A::_00
    }
    ///2 frames
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SPFC_A::_01
    }
    ///3 frames
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SPFC_A::_10
    }
    ///4 frames.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SPFC_A::_11
    }
}
///Field `SPFC` writer - Number of Frames Specification
pub type SPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPFC_A, crate::Safe>;
impl<'a, REG> SPFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 frame
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SPFC_A::_00)
    }
    ///2 frames
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SPFC_A::_01)
    }
    ///3 frames
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SPFC_A::_10)
    }
    ///4 frames.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SPFC_A::_11)
    }
}
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
/**SPI Byte Access Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPBYT_A {
    ///0: SPDR is accessed in word or longword (SPLW is valid)
    _0 = 0,
    ///1: SPDR is accessed in byte (SPLW is invalid)
    _1 = 1,
}
impl From<SPBYT_A> for bool {
    #[inline(always)]
    fn from(variant: SPBYT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPBYT` reader - SPI Byte Access Specification
pub type SPBYT_R = crate::BitReader<SPBYT_A>;
impl SPBYT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPBYT_A {
        match self.bits {
            false => SPBYT_A::_0,
            true => SPBYT_A::_1,
        }
    }
    ///SPDR is accessed in word or longword (SPLW is valid)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPBYT_A::_0
    }
    ///SPDR is accessed in byte (SPLW is invalid)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPBYT_A::_1
    }
}
///Field `SPBYT` writer - SPI Byte Access Specification
pub type SPBYT_W<'a, REG> = crate::BitWriter<'a, REG, SPBYT_A>;
impl<'a, REG> SPBYT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPDR is accessed in word or longword (SPLW is valid)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPBYT_A::_0)
    }
    ///SPDR is accessed in byte (SPLW is invalid)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPBYT_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Number of Frames Specification
    #[inline(always)]
    pub fn spfc(&self) -> SPFC_R {
        SPFC_R::new(self.bits & 3)
    }
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
    ///Bit 6 - SPI Byte Access Specification
    #[inline(always)]
    pub fn spbyt(&self) -> SPBYT_R {
        SPBYT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Number of Frames Specification
    #[inline(always)]
    pub fn spfc(&mut self) -> SPFC_W<SPDCR_SPEC> {
        SPFC_W::new(self, 0)
    }
    ///Bit 4 - RSPI Receive/Transmit Data Selection
    #[inline(always)]
    pub fn sprdtd(&mut self) -> SPRDTD_W<SPDCR_SPEC> {
        SPRDTD_W::new(self, 4)
    }
    ///Bit 5 - SPI Word Access/Halfword Access Specification
    #[inline(always)]
    pub fn splw(&mut self) -> SPLW_W<SPDCR_SPEC> {
        SPLW_W::new(self, 5)
    }
    ///Bit 6 - SPI Byte Access Specification
    #[inline(always)]
    pub fn spbyt(&mut self) -> SPBYT_W<SPDCR_SPEC> {
        SPBYT_W::new(self, 6)
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
