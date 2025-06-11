///Register `SSISR` reader
pub type R = crate::R<SSISR_SPEC>;
///Register `SSISR` writer
pub type W = crate::W<SSISR_SPEC>;
/**Idle Mode Status Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIRQ_A {
    ///0: In the communication state
    _0 = 0,
    ///1: In the idle state
    _1 = 1,
}
impl From<IIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IIRQ` reader - Idle Mode Status Flag
pub type IIRQ_R = crate::BitReader<IIRQ_A>;
impl IIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IIRQ_A {
        match self.bits {
            false => IIRQ_A::_0,
            true => IIRQ_A::_1,
        }
    }
    ///In the communication state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIRQ_A::_0
    }
    ///In the idle state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIRQ_A::_1
    }
}
/**Receive Overflow Error Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIRQ_A {
    ///0: No receive overflow error is generated
    _0 = 0,
    ///1: A receive overflow error is generated.
    _1 = 1,
}
impl From<ROIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ROIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ROIRQ` reader - Receive Overflow Error Status Flag
pub type ROIRQ_R = crate::BitReader<ROIRQ_A>;
impl ROIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROIRQ_A {
        match self.bits {
            false => ROIRQ_A::_0,
            true => ROIRQ_A::_1,
        }
    }
    ///No receive overflow error is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROIRQ_A::_0
    }
    ///A receive overflow error is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROIRQ_A::_1
    }
}
///Field `ROIRQ` writer - Receive Overflow Error Status Flag
pub type ROIRQ_W<'a, REG> = crate::BitWriter<'a, REG, ROIRQ_A>;
impl<'a, REG> ROIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No receive overflow error is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ROIRQ_A::_0)
    }
    ///A receive overflow error is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ROIRQ_A::_1)
    }
}
/**Receive Underflow Error Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIRQ_A {
    ///0: No receive underflow error is generated
    _0 = 0,
    ///1: A receive underflow error is generated.
    _1 = 1,
}
impl From<RUIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RUIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RUIRQ` reader - Receive Underflow Error Status Flag
pub type RUIRQ_R = crate::BitReader<RUIRQ_A>;
impl RUIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RUIRQ_A {
        match self.bits {
            false => RUIRQ_A::_0,
            true => RUIRQ_A::_1,
        }
    }
    ///No receive underflow error is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RUIRQ_A::_0
    }
    ///A receive underflow error is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RUIRQ_A::_1
    }
}
///Field `RUIRQ` writer - Receive Underflow Error Status Flag
pub type RUIRQ_W<'a, REG> = crate::BitWriter<'a, REG, RUIRQ_A>;
impl<'a, REG> RUIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No receive underflow error is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RUIRQ_A::_0)
    }
    ///A receive underflow error is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RUIRQ_A::_1)
    }
}
/**Transmit Overflow Error Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIRQ_A {
    ///0: No transmit overflow error is generated
    _0 = 0,
    ///1: A transmit overflow error is generated.
    _1 = 1,
}
impl From<TOIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TOIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOIRQ` reader - Transmit Overflow Error Status Flag
pub type TOIRQ_R = crate::BitReader<TOIRQ_A>;
impl TOIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOIRQ_A {
        match self.bits {
            false => TOIRQ_A::_0,
            true => TOIRQ_A::_1,
        }
    }
    ///No transmit overflow error is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIRQ_A::_0
    }
    ///A transmit overflow error is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIRQ_A::_1
    }
}
///Field `TOIRQ` writer - Transmit Overflow Error Status Flag
pub type TOIRQ_W<'a, REG> = crate::BitWriter<'a, REG, TOIRQ_A>;
impl<'a, REG> TOIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No transmit overflow error is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOIRQ_A::_0)
    }
    ///A transmit overflow error is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOIRQ_A::_1)
    }
}
/**Transmit Underflow Error Status flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUIRQ_A {
    ///0: No transmit underflow error is generated
    _0 = 0,
    ///1: A transmit underflow error is generated.
    _1 = 1,
}
impl From<TUIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TUIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TUIRQ` reader - Transmit Underflow Error Status flag
pub type TUIRQ_R = crate::BitReader<TUIRQ_A>;
impl TUIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TUIRQ_A {
        match self.bits {
            false => TUIRQ_A::_0,
            true => TUIRQ_A::_1,
        }
    }
    ///No transmit underflow error is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUIRQ_A::_0
    }
    ///A transmit underflow error is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUIRQ_A::_1
    }
}
///Field `TUIRQ` writer - Transmit Underflow Error Status flag
pub type TUIRQ_W<'a, REG> = crate::BitWriter<'a, REG, TUIRQ_A>;
impl<'a, REG> TUIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No transmit underflow error is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TUIRQ_A::_0)
    }
    ///A transmit underflow error is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TUIRQ_A::_1)
    }
}
impl R {
    ///Bit 25 - Idle Mode Status Flag
    #[inline(always)]
    pub fn iirq(&self) -> IIRQ_R {
        IIRQ_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive Overflow Error Status Flag
    #[inline(always)]
    pub fn roirq(&self) -> ROIRQ_R {
        ROIRQ_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Receive Underflow Error Status Flag
    #[inline(always)]
    pub fn ruirq(&self) -> RUIRQ_R {
        RUIRQ_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Transmit Overflow Error Status Flag
    #[inline(always)]
    pub fn toirq(&self) -> TOIRQ_R {
        TOIRQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transmit Underflow Error Status flag
    #[inline(always)]
    pub fn tuirq(&self) -> TUIRQ_R {
        TUIRQ_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 26 - Receive Overflow Error Status Flag
    #[inline(always)]
    pub fn roirq(&mut self) -> ROIRQ_W<SSISR_SPEC> {
        ROIRQ_W::new(self, 26)
    }
    ///Bit 27 - Receive Underflow Error Status Flag
    #[inline(always)]
    pub fn ruirq(&mut self) -> RUIRQ_W<SSISR_SPEC> {
        RUIRQ_W::new(self, 27)
    }
    ///Bit 28 - Transmit Overflow Error Status Flag
    #[inline(always)]
    pub fn toirq(&mut self) -> TOIRQ_W<SSISR_SPEC> {
        TOIRQ_W::new(self, 28)
    }
    ///Bit 29 - Transmit Underflow Error Status flag
    #[inline(always)]
    pub fn tuirq(&mut self) -> TUIRQ_W<SSISR_SPEC> {
        TUIRQ_W::new(self, 29)
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`ssisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSISR_SPEC;
impl crate::RegisterSpec for SSISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssisr::R`](R) reader structure
impl crate::Readable for SSISR_SPEC {}
///`write(|w| ..)` method takes [`ssisr::W`](W) writer structure
impl crate::Writable for SSISR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSISR to value 0x0200_0000
impl crate::Resettable for SSISR_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
