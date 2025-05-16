///Register `SOFCFG` reader
pub type R = crate::R<SOFCFG_SPEC>;
///Register `SOFCFG` writer
pub type W = crate::W<SOFCFG_SPEC>;
/**Edge Interrupt Output Status Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGESTS_A {
    ///0: before stopping the clock supply to the USB module
    _0 = 0,
    ///1: the edge interrupt output signal is in the middle of the edge processing
    _1 = 1,
}
impl From<EDGESTS_A> for bool {
    #[inline(always)]
    fn from(variant: EDGESTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EDGESTS` reader - Edge Interrupt Output Status Monitor
pub type EDGESTS_R = crate::BitReader<EDGESTS_A>;
impl EDGESTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDGESTS_A {
        match self.bits {
            false => EDGESTS_A::_0,
            true => EDGESTS_A::_1,
        }
    }
    ///before stopping the clock supply to the USB module
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDGESTS_A::_0
    }
    ///the edge interrupt output signal is in the middle of the edge processing
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDGESTS_A::_1
    }
}
/**BRDY Interrupt Status Clear Timing

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDYM_A {
    ///0: Software clears the status.
    _0 = 0,
    ///1: The USB clears the status when data has been read from the FIFO buffer or data has been written to the FIFO buffer.
    _1 = 1,
}
impl From<BRDYM_A> for bool {
    #[inline(always)]
    fn from(variant: BRDYM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDYM` reader - BRDY Interrupt Status Clear Timing
pub type BRDYM_R = crate::BitReader<BRDYM_A>;
impl BRDYM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRDYM_A {
        match self.bits {
            false => BRDYM_A::_0,
            true => BRDYM_A::_1,
        }
    }
    ///Software clears the status.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRDYM_A::_0
    }
    ///The USB clears the status when data has been read from the FIFO buffer or data has been written to the FIFO buffer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRDYM_A::_1
    }
}
///Field `BRDYM` writer - BRDY Interrupt Status Clear Timing
pub type BRDYM_W<'a, REG> = crate::BitWriter<'a, REG, BRDYM_A>;
impl<'a, REG> BRDYM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software clears the status.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRDYM_A::_0)
    }
    ///The USB clears the status when data has been read from the FIFO buffer or data has been written to the FIFO buffer.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRDYM_A::_1)
    }
}
/**Transaction-Enabled Time Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNENSEL_A {
    ///0: For non-low-speed communication
    _0 = 0,
    ///1: For low-speed communication
    _1 = 1,
}
impl From<TRNENSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRNENSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRNENSEL` reader - Transaction-Enabled Time Select
pub type TRNENSEL_R = crate::BitReader<TRNENSEL_A>;
impl TRNENSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRNENSEL_A {
        match self.bits {
            false => TRNENSEL_A::_0,
            true => TRNENSEL_A::_1,
        }
    }
    ///For non-low-speed communication
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRNENSEL_A::_0
    }
    ///For low-speed communication
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRNENSEL_A::_1
    }
}
///Field `TRNENSEL` writer - Transaction-Enabled Time Select
pub type TRNENSEL_W<'a, REG> = crate::BitWriter<'a, REG, TRNENSEL_A>;
impl<'a, REG> TRNENSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///For non-low-speed communication
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRNENSEL_A::_0)
    }
    ///For low-speed communication
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRNENSEL_A::_1)
    }
}
impl R {
    ///Bit 4 - Edge Interrupt Output Status Monitor
    #[inline(always)]
    pub fn edgests(&self) -> EDGESTS_R {
        EDGESTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - BRDY Interrupt Status Clear Timing
    #[inline(always)]
    pub fn brdym(&self) -> BRDYM_R {
        BRDYM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Transaction-Enabled Time Select
    #[inline(always)]
    pub fn trnensel(&self) -> TRNENSEL_R {
        TRNENSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - BRDY Interrupt Status Clear Timing
    #[inline(always)]
    pub fn brdym(&mut self) -> BRDYM_W<SOFCFG_SPEC> {
        BRDYM_W::new(self, 6)
    }
    ///Bit 8 - Transaction-Enabled Time Select
    #[inline(always)]
    pub fn trnensel(&mut self) -> TRNENSEL_W<SOFCFG_SPEC> {
        TRNENSEL_W::new(self, 8)
    }
}
/**SOF Output Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sofcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SOFCFG_SPEC;
impl crate::RegisterSpec for SOFCFG_SPEC {
    type Ux = u16;
}
///`read()` method returns [`sofcfg::R`](R) reader structure
impl crate::Readable for SOFCFG_SPEC {}
///`write(|w| ..)` method takes [`sofcfg::W`](W) writer structure
impl crate::Writable for SOFCFG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFCFG to value 0
impl crate::Resettable for SOFCFG_SPEC {}
