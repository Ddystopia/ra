///Register `PHYSET` reader
pub type R = crate::R<PHYSET_SPEC>;
///Register `PHYSET` writer
pub type W = crate::W<PHYSET_SPEC>;
/**Power-Down Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRPD_A {
    ///0: Does not enter low-power consumption mode
    _0 = 0,
    ///1: Enter low-power consumption mode
    _1 = 1,
}
impl From<DIRPD_A> for bool {
    #[inline(always)]
    fn from(variant: DIRPD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRPD` reader - Power-Down Control
pub type DIRPD_R = crate::BitReader<DIRPD_A>;
impl DIRPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRPD_A {
        match self.bits {
            false => DIRPD_A::_0,
            true => DIRPD_A::_1,
        }
    }
    ///Does not enter low-power consumption mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRPD_A::_0
    }
    ///Enter low-power consumption mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRPD_A::_1
    }
}
///Field `DIRPD` writer - Power-Down Control
pub type DIRPD_W<'a, REG> = crate::BitWriter<'a, REG, DIRPD_A>;
impl<'a, REG> DIRPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not enter low-power consumption mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRPD_A::_0)
    }
    ///Enter low-power consumption mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRPD_A::_1)
    }
}
/**PLL Reset Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRESET_A {
    ///0: Disable PLL reset control for UTMI_PHY
    _0 = 0,
    ///1: Enable PLL reset control for UTMI_PHY
    _1 = 1,
}
impl From<PLLRESET_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRESET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRESET` reader - PLL Reset Control
pub type PLLRESET_R = crate::BitReader<PLLRESET_A>;
impl PLLRESET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLRESET_A {
        match self.bits {
            false => PLLRESET_A::_0,
            true => PLLRESET_A::_1,
        }
    }
    ///Disable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLRESET_A::_0
    }
    ///Enable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLRESET_A::_1
    }
}
///Field `PLLRESET` writer - PLL Reset Control
pub type PLLRESET_W<'a, REG> = crate::BitWriter<'a, REG, PLLRESET_A>;
impl<'a, REG> PLLRESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRESET_A::_0)
    }
    ///Enable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLRESET_A::_1)
    }
}
/**Charging Downstream Port Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDPEN_A {
    ///0: Disable charging downstream port
    _0 = 0,
    ///1: Enable charging downstream port
    _1 = 1,
}
impl From<CDPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDPEN` reader - Charging Downstream Port Enable
pub type CDPEN_R = crate::BitReader<CDPEN_A>;
impl CDPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDPEN_A {
        match self.bits {
            false => CDPEN_A::_0,
            true => CDPEN_A::_1,
        }
    }
    ///Disable charging downstream port
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDPEN_A::_0
    }
    ///Enable charging downstream port
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDPEN_A::_1
    }
}
///Field `CDPEN` writer - Charging Downstream Port Enable
pub type CDPEN_W<'a, REG> = crate::BitWriter<'a, REG, CDPEN_A>;
impl<'a, REG> CDPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable charging downstream port
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDPEN_A::_0)
    }
    ///Enable charging downstream port
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDPEN_A::_1)
    }
}
/**Input System Clock Frequency

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    ///0: Setting Prohibited
    _00 = 0,
    ///1: 12 MHz
    _01 = 1,
    ///2: 20 MHz
    _10 = 2,
    ///3: 24 MHz
    _11 = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKSEL_A {}
///Field `CLKSEL` reader - Input System Clock Frequency
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
impl CLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::_00,
            1 => CLKSEL_A::_01,
            2 => CLKSEL_A::_10,
            3 => CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///Setting Prohibited
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKSEL_A::_00
    }
    ///12 MHz
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKSEL_A::_01
    }
    ///20 MHz
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKSEL_A::_10
    }
    ///24 MHz
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKSEL_A::_11
    }
}
///Field `CLKSEL` writer - Input System Clock Frequency
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL_A, crate::Safe>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting Prohibited
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_00)
    }
    ///12 MHz
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_01)
    }
    ///20 MHz
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_10)
    }
    ///24 MHz
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_11)
    }
}
/**Terminating Resistance Adjustment Cycle

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPSEL_A {
    ///0: No cycle is set.
    _00 = 0,
    ///1: Adjust terminating resistance at 16-second intervals.
    _01 = 1,
    ///2: Adjust terminating resistance at 64-second intervals.
    _10 = 2,
    ///3: Adjust terminating resistance at 128-second intervals.
    _11 = 3,
}
impl From<REPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REPSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REPSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for REPSEL_A {}
///Field `REPSEL` reader - Terminating Resistance Adjustment Cycle
pub type REPSEL_R = crate::FieldReader<REPSEL_A>;
impl REPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REPSEL_A {
        match self.bits {
            0 => REPSEL_A::_00,
            1 => REPSEL_A::_01,
            2 => REPSEL_A::_10,
            3 => REPSEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///No cycle is set.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == REPSEL_A::_00
    }
    ///Adjust terminating resistance at 16-second intervals.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == REPSEL_A::_01
    }
    ///Adjust terminating resistance at 64-second intervals.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == REPSEL_A::_10
    }
    ///Adjust terminating resistance at 128-second intervals.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == REPSEL_A::_11
    }
}
///Field `REPSEL` writer - Terminating Resistance Adjustment Cycle
pub type REPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REPSEL_A, crate::Safe>;
impl<'a, REG> REPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No cycle is set.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(REPSEL_A::_00)
    }
    ///Adjust terminating resistance at 16-second intervals.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(REPSEL_A::_01)
    }
    ///Adjust terminating resistance at 64-second intervals.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(REPSEL_A::_10)
    }
    ///Adjust terminating resistance at 128-second intervals.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(REPSEL_A::_11)
    }
}
/**Forcibly Start Terminating Resistance Adjustment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPSTART_A {
    ///0: Terminating resistance adjustment is forcibly started
    _0 = 0,
    ///1: Terminating resistance adjustment is not forcibly started
    _1 = 1,
}
impl From<REPSTART_A> for bool {
    #[inline(always)]
    fn from(variant: REPSTART_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REPSTART` reader - Forcibly Start Terminating Resistance Adjustment
pub type REPSTART_R = crate::BitReader<REPSTART_A>;
impl REPSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REPSTART_A {
        match self.bits {
            false => REPSTART_A::_0,
            true => REPSTART_A::_1,
        }
    }
    ///Terminating resistance adjustment is forcibly started
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REPSTART_A::_0
    }
    ///Terminating resistance adjustment is not forcibly started
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REPSTART_A::_1
    }
}
///Field `REPSTART` writer - Forcibly Start Terminating Resistance Adjustment
pub type REPSTART_W<'a, REG> = crate::BitWriter<'a, REG, REPSTART_A>;
impl<'a, REG> REPSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Terminating resistance adjustment is forcibly started
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REPSTART_A::_0)
    }
    ///Terminating resistance adjustment is not forcibly started
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REPSTART_A::_1)
    }
}
/**CL-Only Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEB_A {
    ///0: CL-only mode is not activated.
    _0 = 0,
    ///1: CL-only mode is activated.
    _1 = 1,
}
impl From<HSEB_A> for bool {
    #[inline(always)]
    fn from(variant: HSEB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEB` reader - CL-Only Mode
pub type HSEB_R = crate::BitReader<HSEB_A>;
impl HSEB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEB_A {
        match self.bits {
            false => HSEB_A::_0,
            true => HSEB_A::_1,
        }
    }
    ///CL-only mode is not activated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSEB_A::_0
    }
    ///CL-only mode is activated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSEB_A::_1
    }
}
///Field `HSEB` writer - CL-Only Mode
pub type HSEB_W<'a, REG> = crate::BitWriter<'a, REG, HSEB_A>;
impl<'a, REG> HSEB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CL-only mode is not activated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEB_A::_0)
    }
    ///CL-only mode is activated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEB_A::_1)
    }
}
impl R {
    ///Bit 0 - Power-Down Control
    #[inline(always)]
    pub fn dirpd(&self) -> DIRPD_R {
        DIRPD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL Reset Control
    #[inline(always)]
    pub fn pllreset(&self) -> PLLRESET_R {
        PLLRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Charging Downstream Port Enable
    #[inline(always)]
    pub fn cdpen(&self) -> CDPEN_R {
        CDPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Input System Clock Frequency
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Terminating Resistance Adjustment Cycle
    #[inline(always)]
    pub fn repsel(&self) -> REPSEL_R {
        REPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - Forcibly Start Terminating Resistance Adjustment
    #[inline(always)]
    pub fn repstart(&self) -> REPSTART_R {
        REPSTART_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - CL-Only Mode
    #[inline(always)]
    pub fn hseb(&self) -> HSEB_R {
        HSEB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Power-Down Control
    #[inline(always)]
    pub fn dirpd(&mut self) -> DIRPD_W<PHYSET_SPEC> {
        DIRPD_W::new(self, 0)
    }
    ///Bit 1 - PLL Reset Control
    #[inline(always)]
    pub fn pllreset(&mut self) -> PLLRESET_W<PHYSET_SPEC> {
        PLLRESET_W::new(self, 1)
    }
    ///Bit 3 - Charging Downstream Port Enable
    #[inline(always)]
    pub fn cdpen(&mut self) -> CDPEN_W<PHYSET_SPEC> {
        CDPEN_W::new(self, 3)
    }
    ///Bits 4:5 - Input System Clock Frequency
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W<PHYSET_SPEC> {
        CLKSEL_W::new(self, 4)
    }
    ///Bits 8:9 - Terminating Resistance Adjustment Cycle
    #[inline(always)]
    pub fn repsel(&mut self) -> REPSEL_W<PHYSET_SPEC> {
        REPSEL_W::new(self, 8)
    }
    ///Bit 11 - Forcibly Start Terminating Resistance Adjustment
    #[inline(always)]
    pub fn repstart(&mut self) -> REPSTART_W<PHYSET_SPEC> {
        REPSTART_W::new(self, 11)
    }
    ///Bit 15 - CL-Only Mode
    #[inline(always)]
    pub fn hseb(&mut self) -> HSEB_W<PHYSET_SPEC> {
        HSEB_W::new(self, 15)
    }
}
/**PHY Setting Register

You can [`read`](crate::Reg::read) this register and get [`physet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHYSET_SPEC;
impl crate::RegisterSpec for PHYSET_SPEC {
    type Ux = u16;
}
///`read()` method returns [`physet::R`](R) reader structure
impl crate::Readable for PHYSET_SPEC {}
///`write(|w| ..)` method takes [`physet::W`](W) writer structure
impl crate::Writable for PHYSET_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHYSET to value 0x33
impl crate::Resettable for PHYSET_SPEC {
    const RESET_VALUE: u16 = 0x33;
}
