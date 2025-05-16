///Register `SPSR` reader
pub type R = crate::R<SPSR_SPEC>;
///Register `SPSR` writer
pub type W = crate::W<SPSR_SPEC>;
/**Overrun Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRF_A {
    ///0: No overrun error occurs
    _0 = 0,
    ///1: An overrun error occurs
    _1 = 1,
}
impl From<OVRF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OVRF` reader - Overrun Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OVRF_R = crate::BitReader<OVRF_A>;
impl OVRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRF_A {
        match self.bits {
            false => OVRF_A::_0,
            true => OVRF_A::_1,
        }
    }
    ///No overrun error occurs
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRF_A::_0
    }
    ///An overrun error occurs
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRF_A::_1
    }
}
///Field `OVRF` writer - Overrun Error Flag
pub type OVRF_W<'a, REG> = crate::BitWriter0C<'a, REG, OVRF_A>;
impl<'a, REG> OVRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overrun error occurs
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRF_A::_0)
    }
    ///An overrun error occurs
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRF_A::_1)
    }
}
/**SPI Idle Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLNF_A {
    ///0: SPI is in the idle state
    _0 = 0,
    ///1: SPI is in the transfer state
    _1 = 1,
}
impl From<IDLNF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLNF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLNF` reader - SPI Idle Flag
pub type IDLNF_R = crate::BitReader<IDLNF_A>;
impl IDLNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLNF_A {
        match self.bits {
            false => IDLNF_A::_0,
            true => IDLNF_A::_1,
        }
    }
    ///SPI is in the idle state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDLNF_A::_0
    }
    ///SPI is in the transfer state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDLNF_A::_1
    }
}
/**Mode Fault Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF_A {
    ///0: Neither mode fault error nor underrun error occurs
    _0 = 0,
    ///1: A mode fault error or an underrun error occurs.
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `MODF` reader - Mode Fault Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type MODF_R = crate::BitReader<MODF_A>;
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    ///Neither mode fault error nor underrun error occurs
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODF_A::_0
    }
    ///A mode fault error or an underrun error occurs.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODF_A::_1
    }
}
///Field `MODF` writer - Mode Fault Error Flag
pub type MODF_W<'a, REG> = crate::BitWriter0C<'a, REG, MODF_A>;
impl<'a, REG> MODF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Neither mode fault error nor underrun error occurs
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MODF_A::_0)
    }
    ///A mode fault error or an underrun error occurs.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MODF_A::_1)
    }
}
/**Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERF_A {
    ///0: No parity error occurs
    _0 = 0,
    ///1: A parity error occurs
    _1 = 1,
}
impl From<PERF_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PERF` reader - Parity Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PERF_R = crate::BitReader<PERF_A>;
impl PERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PERF_A {
        match self.bits {
            false => PERF_A::_0,
            true => PERF_A::_1,
        }
    }
    ///No parity error occurs
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PERF_A::_0
    }
    ///A parity error occurs
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PERF_A::_1
    }
}
///Field `PERF` writer - Parity Error Flag
pub type PERF_W<'a, REG> = crate::BitWriter0C<'a, REG, PERF_A>;
impl<'a, REG> PERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No parity error occurs
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PERF_A::_0)
    }
    ///A parity error occurs
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PERF_A::_1)
    }
}
/**Underrun Error Flag(When MODF is 0, This bit is invalid.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRF_A {
    ///0: A mode fault error occurs (MODF=1)
    _0 = 0,
    ///1: An underrun error occurs (MODF=1)
    _1 = 1,
}
impl From<UDRF_A> for bool {
    #[inline(always)]
    fn from(variant: UDRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `UDRF` reader - Underrun Error Flag(When MODF is 0, This bit is invalid.)

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type UDRF_R = crate::BitReader<UDRF_A>;
impl UDRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDRF_A {
        match self.bits {
            false => UDRF_A::_0,
            true => UDRF_A::_1,
        }
    }
    ///A mode fault error occurs (MODF=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRF_A::_0
    }
    ///An underrun error occurs (MODF=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRF_A::_1
    }
}
///Field `UDRF` writer - Underrun Error Flag(When MODF is 0, This bit is invalid.)
pub type UDRF_W<'a, REG> = crate::BitWriter0C<'a, REG, UDRF_A>;
impl<'a, REG> UDRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A mode fault error occurs (MODF=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UDRF_A::_0)
    }
    ///An underrun error occurs (MODF=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UDRF_A::_1)
    }
}
/**SPI Transmit Buffer Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPTEF_A {
    ///0: Data found in the transmit buffer
    _0 = 0,
    ///1: No data in the transmit buffer
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SPTEF` reader - SPI Transmit Buffer Empty Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SPTEF_R = crate::BitReader<SPTEF_A>;
impl SPTEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    ///Data found in the transmit buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTEF_A::_0
    }
    ///No data in the transmit buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTEF_A::_1
    }
}
///Field `SPTEF` writer - SPI Transmit Buffer Empty Flag
pub type SPTEF_W<'a, REG> = crate::BitWriter0C<'a, REG, SPTEF_A>;
impl<'a, REG> SPTEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data found in the transmit buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPTEF_A::_0)
    }
    ///No data in the transmit buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPTEF_A::_1)
    }
}
/**SPI Receive Buffer Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRF_A {
    ///0: No valid data in SPDR
    _0 = 0,
    ///1: Valid data found in SPDR
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SPRF` reader - SPI Receive Buffer Full Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SPRF_R = crate::BitReader<SPRF_A>;
impl SPRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    ///No valid data in SPDR
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRF_A::_0
    }
    ///Valid data found in SPDR
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRF_A::_1
    }
}
///Field `SPRF` writer - SPI Receive Buffer Full Flag
pub type SPRF_W<'a, REG> = crate::BitWriter0C<'a, REG, SPRF_A>;
impl<'a, REG> SPRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No valid data in SPDR
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPRF_A::_0)
    }
    ///Valid data found in SPDR
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPRF_A::_1)
    }
}
impl R {
    ///Bit 0 - Overrun Error Flag
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPI Idle Flag
    #[inline(always)]
    pub fn idlnf(&self) -> IDLNF_R {
        IDLNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mode Fault Error Flag
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Underrun Error Flag(When MODF is 0, This bit is invalid.)
    #[inline(always)]
    pub fn udrf(&self) -> UDRF_R {
        UDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPI Transmit Buffer Empty Flag
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - SPI Receive Buffer Full Flag
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Overrun Error Flag
    #[inline(always)]
    pub fn ovrf(&mut self) -> OVRF_W<SPSR_SPEC> {
        OVRF_W::new(self, 0)
    }
    ///Bit 2 - Mode Fault Error Flag
    #[inline(always)]
    pub fn modf(&mut self) -> MODF_W<SPSR_SPEC> {
        MODF_W::new(self, 2)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn perf(&mut self) -> PERF_W<SPSR_SPEC> {
        PERF_W::new(self, 3)
    }
    ///Bit 4 - Underrun Error Flag(When MODF is 0, This bit is invalid.)
    #[inline(always)]
    pub fn udrf(&mut self) -> UDRF_W<SPSR_SPEC> {
        UDRF_W::new(self, 4)
    }
    ///Bit 5 - SPI Transmit Buffer Empty Flag
    #[inline(always)]
    pub fn sptef(&mut self) -> SPTEF_W<SPSR_SPEC> {
        SPTEF_W::new(self, 5)
    }
    ///Bit 7 - SPI Receive Buffer Full Flag
    #[inline(always)]
    pub fn sprf(&mut self) -> SPRF_W<SPSR_SPEC> {
        SPRF_W::new(self, 7)
    }
}
/**SPI Status Register

You can [`read`](crate::Reg::read) this register and get [`spsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPSR_SPEC;
impl crate::RegisterSpec for SPSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spsr::R`](R) reader structure
impl crate::Readable for SPSR_SPEC {}
///`write(|w| ..)` method takes [`spsr::W`](W) writer structure
impl crate::Writable for SPSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xbd;
}
///`reset()` method sets SPSR to value 0x20
impl crate::Resettable for SPSR_SPEC {
    const RESET_VALUE: u8 = 0x20;
}
