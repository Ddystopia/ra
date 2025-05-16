///Register `DPUSR0R` reader
pub type R = crate::R<DPUSR0R_SPEC>;
///Register `DPUSR0R` writer
pub type W = crate::W<DPUSR0R_SPEC>;
/**USB Single End Receiver Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRPC0_A {
    ///0: Input through the DP and DM inputs is disabled.
    _0 = 0,
    ///1: Input through the DP and DM inputs is enabled.
    _1 = 1,
}
impl From<SRPC0_A> for bool {
    #[inline(always)]
    fn from(variant: SRPC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRPC0` reader - USB Single End Receiver Control
pub type SRPC0_R = crate::BitReader<SRPC0_A>;
impl SRPC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRPC0_A {
        match self.bits {
            false => SRPC0_A::_0,
            true => SRPC0_A::_1,
        }
    }
    ///Input through the DP and DM inputs is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRPC0_A::_0
    }
    ///Input through the DP and DM inputs is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRPC0_A::_1
    }
}
///Field `SRPC0` writer - USB Single End Receiver Control
pub type SRPC0_W<'a, REG> = crate::BitWriter<'a, REG, SRPC0_A>;
impl<'a, REG> SRPC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input through the DP and DM inputs is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SRPC0_A::_0)
    }
    ///Input through the DP and DM inputs is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SRPC0_A::_1)
    }
}
/**DP Pull-Up Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPUE0_A {
    ///0: Disables DP pull-up resistor.
    _0 = 0,
    ///1: Enables DP pull-up resistor.
    _1 = 1,
}
impl From<RPUE0_A> for bool {
    #[inline(always)]
    fn from(variant: RPUE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPUE0` reader - DP Pull-Up Resistor Control
pub type RPUE0_R = crate::BitReader<RPUE0_A>;
impl RPUE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPUE0_A {
        match self.bits {
            false => RPUE0_A::_0,
            true => RPUE0_A::_1,
        }
    }
    ///Disables DP pull-up resistor.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPUE0_A::_0
    }
    ///Enables DP pull-up resistor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPUE0_A::_1
    }
}
///Field `RPUE0` writer - DP Pull-Up Resistor Control
pub type RPUE0_W<'a, REG> = crate::BitWriter<'a, REG, RPUE0_A>;
impl<'a, REG> RPUE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables DP pull-up resistor.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPUE0_A::_0)
    }
    ///Enables DP pull-up resistor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPUE0_A::_1)
    }
}
/**D+/D- Pull-Down Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPD0_A {
    ///0: Disables DP/DM pull-down resistor.
    _0 = 0,
    ///1: Enables DP/DM pull-down resistor.
    _1 = 1,
}
impl From<DRPD0_A> for bool {
    #[inline(always)]
    fn from(variant: DRPD0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRPD0` reader - D+/D- Pull-Down Resistor Control
pub type DRPD0_R = crate::BitReader<DRPD0_A>;
impl DRPD0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRPD0_A {
        match self.bits {
            false => DRPD0_A::_0,
            true => DRPD0_A::_1,
        }
    }
    ///Disables DP/DM pull-down resistor.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPD0_A::_0
    }
    ///Enables DP/DM pull-down resistor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPD0_A::_1
    }
}
///Field `DRPD0` writer - D+/D- Pull-Down Resistor Control
pub type DRPD0_W<'a, REG> = crate::BitWriter<'a, REG, DRPD0_A>;
impl<'a, REG> DRPD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables DP/DM pull-down resistor.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRPD0_A::_0)
    }
    ///Enables DP/DM pull-down resistor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRPD0_A::_1)
    }
}
/**USB Transceiver Output Fix

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXPHY0_A {
    ///0: The outputs are fixed in normal mode and on return from deep software standby mode.
    _0 = 0,
    ///1: The outputs are fixed on transitions to deep software standby mode.
    _1 = 1,
}
impl From<FIXPHY0_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPHY0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXPHY0` reader - USB Transceiver Output Fix
pub type FIXPHY0_R = crate::BitReader<FIXPHY0_A>;
impl FIXPHY0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FIXPHY0_A {
        match self.bits {
            false => FIXPHY0_A::_0,
            true => FIXPHY0_A::_1,
        }
    }
    ///The outputs are fixed in normal mode and on return from deep software standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXPHY0_A::_0
    }
    ///The outputs are fixed on transitions to deep software standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXPHY0_A::_1
    }
}
///Field `FIXPHY0` writer - USB Transceiver Output Fix
pub type FIXPHY0_W<'a, REG> = crate::BitWriter<'a, REG, FIXPHY0_A>;
impl<'a, REG> FIXPHY0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The outputs are fixed in normal mode and on return from deep software standby mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FIXPHY0_A::_0)
    }
    ///The outputs are fixed on transitions to deep software standby mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FIXPHY0_A::_1)
    }
}
///Field `DP0` reader - USB0 D+ InputIndicates the D+ input signal of the USB.
pub type DP0_R = crate::BitReader;
///Field `DM0` reader - USB D-InputIndicates the D- input signal of the USB.
pub type DM0_R = crate::BitReader;
///Field `DOVCA0` reader - USB OVRCURA InputIndicates the OVRCURA input signal of the USB.
pub type DOVCA0_R = crate::BitReader;
///Field `DOVCB0` reader - USB OVRCURB InputIndicates the OVRCURB input signal of the USB.
pub type DOVCB0_R = crate::BitReader;
///Field `DVBSTS0` reader - USB VBUS InputIndicates the VBUS input signal of the USB.
pub type DVBSTS0_R = crate::BitReader;
impl R {
    ///Bit 0 - USB Single End Receiver Control
    #[inline(always)]
    pub fn srpc0(&self) -> SRPC0_R {
        SRPC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DP Pull-Up Resistor Control
    #[inline(always)]
    pub fn rpue0(&self) -> RPUE0_R {
        RPUE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - D+/D- Pull-Down Resistor Control
    #[inline(always)]
    pub fn drpd0(&self) -> DRPD0_R {
        DRPD0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USB Transceiver Output Fix
    #[inline(always)]
    pub fn fixphy0(&self) -> FIXPHY0_R {
        FIXPHY0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - USB0 D+ InputIndicates the D+ input signal of the USB.
    #[inline(always)]
    pub fn dp0(&self) -> DP0_R {
        DP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USB D-InputIndicates the D- input signal of the USB.
    #[inline(always)]
    pub fn dm0(&self) -> DM0_R {
        DM0_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - USB OVRCURA InputIndicates the OVRCURA input signal of the USB.
    #[inline(always)]
    pub fn dovca0(&self) -> DOVCA0_R {
        DOVCA0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - USB OVRCURB InputIndicates the OVRCURB input signal of the USB.
    #[inline(always)]
    pub fn dovcb0(&self) -> DOVCB0_R {
        DOVCB0_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - USB VBUS InputIndicates the VBUS input signal of the USB.
    #[inline(always)]
    pub fn dvbsts0(&self) -> DVBSTS0_R {
        DVBSTS0_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USB Single End Receiver Control
    #[inline(always)]
    pub fn srpc0(&mut self) -> SRPC0_W<DPUSR0R_SPEC> {
        SRPC0_W::new(self, 0)
    }
    ///Bit 1 - DP Pull-Up Resistor Control
    #[inline(always)]
    pub fn rpue0(&mut self) -> RPUE0_W<DPUSR0R_SPEC> {
        RPUE0_W::new(self, 1)
    }
    ///Bit 3 - D+/D- Pull-Down Resistor Control
    #[inline(always)]
    pub fn drpd0(&mut self) -> DRPD0_W<DPUSR0R_SPEC> {
        DRPD0_W::new(self, 3)
    }
    ///Bit 4 - USB Transceiver Output Fix
    #[inline(always)]
    pub fn fixphy0(&mut self) -> FIXPHY0_W<DPUSR0R_SPEC> {
        FIXPHY0_W::new(self, 4)
    }
}
/**Deep Software Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPUSR0R_SPEC;
impl crate::RegisterSpec for DPUSR0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpusr0r::R`](R) reader structure
impl crate::Readable for DPUSR0R_SPEC {}
///`write(|w| ..)` method takes [`dpusr0r::W`](W) writer structure
impl crate::Writable for DPUSR0R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR0R to value 0
impl crate::Resettable for DPUSR0R_SPEC {}
