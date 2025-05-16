///Register `SYSCFG` reader
pub type R = crate::R<SYSCFG_SPEC>;
///Register `SYSCFG` writer
pub type W = crate::W<SYSCFG_SPEC>;
/**USB Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBE_A {
    ///0: USB operation is disabled.
    _0 = 0,
    ///1: USB operation is enabled.
    _1 = 1,
}
impl From<USBE_A> for bool {
    #[inline(always)]
    fn from(variant: USBE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USBE` reader - USB Operation Enable
pub type USBE_R = crate::BitReader<USBE_A>;
impl USBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBE_A {
        match self.bits {
            false => USBE_A::_0,
            true => USBE_A::_1,
        }
    }
    ///USB operation is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBE_A::_0
    }
    ///USB operation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBE_A::_1
    }
}
///Field `USBE` writer - USB Operation Enable
pub type USBE_W<'a, REG> = crate::BitWriter<'a, REG, USBE_A>;
impl<'a, REG> USBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB operation is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBE_A::_0)
    }
    ///USB operation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBE_A::_1)
    }
}
/**D+ Line Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPRPU_A {
    ///0: Pulling up the line is disabled.
    _0 = 0,
    ///1: Pulling up the line is enabled.
    _1 = 1,
}
impl From<DPRPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPRPU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPRPU` reader - D+ Line Resistor Control
pub type DPRPU_R = crate::BitReader<DPRPU_A>;
impl DPRPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPRPU_A {
        match self.bits {
            false => DPRPU_A::_0,
            true => DPRPU_A::_1,
        }
    }
    ///Pulling up the line is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPRPU_A::_0
    }
    ///Pulling up the line is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPRPU_A::_1
    }
}
///Field `DPRPU` writer - D+ Line Resistor Control
pub type DPRPU_W<'a, REG> = crate::BitWriter<'a, REG, DPRPU_A>;
impl<'a, REG> DPRPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulling up the line is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPRPU_A::_0)
    }
    ///Pulling up the line is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPRPU_A::_1)
    }
}
/**D+/D- Line Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRPD_A {
    ///0: Pulling down the lines is disabled.
    _0 = 0,
    ///1: Pulling down the lines is enabled.
    _1 = 1,
}
impl From<DRPD_A> for bool {
    #[inline(always)]
    fn from(variant: DRPD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRPD` reader - D+/D- Line Resistor Control
pub type DRPD_R = crate::BitReader<DRPD_A>;
impl DRPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRPD_A {
        match self.bits {
            false => DRPD_A::_0,
            true => DRPD_A::_1,
        }
    }
    ///Pulling down the lines is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRPD_A::_0
    }
    ///Pulling down the lines is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRPD_A::_1
    }
}
///Field `DRPD` writer - D+/D- Line Resistor Control
pub type DRPD_W<'a, REG> = crate::BitWriter<'a, REG, DRPD_A>;
impl<'a, REG> DRPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulling down the lines is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRPD_A::_0)
    }
    ///Pulling down the lines is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRPD_A::_1)
    }
}
/**Controller Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCFM_A {
    ///0: Function controller is selected.
    _0 = 0,
    ///1: Host controller is selected.
    _1 = 1,
}
impl From<DCFM_A> for bool {
    #[inline(always)]
    fn from(variant: DCFM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DCFM` reader - Controller Function Select
pub type DCFM_R = crate::BitReader<DCFM_A>;
impl DCFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCFM_A {
        match self.bits {
            false => DCFM_A::_0,
            true => DCFM_A::_1,
        }
    }
    ///Function controller is selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCFM_A::_0
    }
    ///Host controller is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCFM_A::_1
    }
}
///Field `DCFM` writer - Controller Function Select
pub type DCFM_W<'a, REG> = crate::BitWriter<'a, REG, DCFM_A>;
impl<'a, REG> DCFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Function controller is selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCFM_A::_0)
    }
    ///Host controller is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCFM_A::_1)
    }
}
/**USB Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKE_A {
    ///0: Stops supplying the clock signal to the USB.
    _0 = 0,
    ///1: Enables supplying the clock signal to the USB.
    _1 = 1,
}
impl From<SCKE_A> for bool {
    #[inline(always)]
    fn from(variant: SCKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCKE` reader - USB Clock Enable
pub type SCKE_R = crate::BitReader<SCKE_A>;
impl SCKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCKE_A {
        match self.bits {
            false => SCKE_A::_0,
            true => SCKE_A::_1,
        }
    }
    ///Stops supplying the clock signal to the USB.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKE_A::_0
    }
    ///Enables supplying the clock signal to the USB.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKE_A::_1
    }
}
///Field `SCKE` writer - USB Clock Enable
pub type SCKE_W<'a, REG> = crate::BitWriter<'a, REG, SCKE_A>;
impl<'a, REG> SCKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops supplying the clock signal to the USB.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCKE_A::_0)
    }
    ///Enables supplying the clock signal to the USB.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCKE_A::_1)
    }
}
impl R {
    ///Bit 0 - USB Operation Enable
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - D+ Line Resistor Control
    #[inline(always)]
    pub fn dprpu(&self) -> DPRPU_R {
        DPRPU_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - D+/D- Line Resistor Control
    #[inline(always)]
    pub fn drpd(&self) -> DRPD_R {
        DRPD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Controller Function Select
    #[inline(always)]
    pub fn dcfm(&self) -> DCFM_R {
        DCFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - USB Clock Enable
    #[inline(always)]
    pub fn scke(&self) -> SCKE_R {
        SCKE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USB Operation Enable
    #[inline(always)]
    pub fn usbe(&mut self) -> USBE_W<SYSCFG_SPEC> {
        USBE_W::new(self, 0)
    }
    ///Bit 4 - D+ Line Resistor Control
    #[inline(always)]
    pub fn dprpu(&mut self) -> DPRPU_W<SYSCFG_SPEC> {
        DPRPU_W::new(self, 4)
    }
    ///Bit 5 - D+/D- Line Resistor Control
    #[inline(always)]
    pub fn drpd(&mut self) -> DRPD_W<SYSCFG_SPEC> {
        DRPD_W::new(self, 5)
    }
    ///Bit 6 - Controller Function Select
    #[inline(always)]
    pub fn dcfm(&mut self) -> DCFM_W<SYSCFG_SPEC> {
        DCFM_W::new(self, 6)
    }
    ///Bit 10 - USB Clock Enable
    #[inline(always)]
    pub fn scke(&mut self) -> SCKE_W<SYSCFG_SPEC> {
        SCKE_W::new(self, 10)
    }
}
/**System Configuration Control Register

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCFG_SPEC;
impl crate::RegisterSpec for SYSCFG_SPEC {
    type Ux = u16;
}
///`read()` method returns [`syscfg::R`](R) reader structure
impl crate::Readable for SYSCFG_SPEC {}
///`write(|w| ..)` method takes [`syscfg::W`](W) writer structure
impl crate::Writable for SYSCFG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCFG to value 0
impl crate::Resettable for SYSCFG_SPEC {}
