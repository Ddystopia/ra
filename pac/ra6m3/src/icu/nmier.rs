///Register `NMIER` reader
pub type R = crate::R<NMIER_SPEC>;
///Register `NMIER` writer
pub type W = crate::W<NMIER_SPEC>;
/**IWDT Underflow/Refresh Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTEN_A {
    ///0: IWDT underflow/refresh error interrupt is disabled.
    _0 = 0,
    ///1: IWDT underflow/refresh error interrupt is enabled.
    _1 = 1,
}
impl From<IWDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTEN` reader - IWDT Underflow/Refresh Error Interrupt Enable
pub type IWDTEN_R = crate::BitReader<IWDTEN_A>;
impl IWDTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDTEN_A {
        match self.bits {
            false => IWDTEN_A::_0,
            true => IWDTEN_A::_1,
        }
    }
    ///IWDT underflow/refresh error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTEN_A::_0
    }
    ///IWDT underflow/refresh error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTEN_A::_1
    }
}
///Field `IWDTEN` writer - IWDT Underflow/Refresh Error Interrupt Enable
pub type IWDTEN_W<'a, REG> = crate::BitWriter<'a, REG, IWDTEN_A>;
impl<'a, REG> IWDTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IWDT underflow/refresh error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTEN_A::_0)
    }
    ///IWDT underflow/refresh error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTEN_A::_1)
    }
}
/**WDT Underflow/Refresh Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTEN_A {
    ///0: WDT underflow/refresh error interrupt is disabled.
    _0 = 0,
    ///1: WDT underflow/refresh error interrupt is enabled.
    _1 = 1,
}
impl From<WDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTEN` reader - WDT Underflow/Refresh Error Interrupt Enable
pub type WDTEN_R = crate::BitReader<WDTEN_A>;
impl WDTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDTEN_A {
        match self.bits {
            false => WDTEN_A::_0,
            true => WDTEN_A::_1,
        }
    }
    ///WDT underflow/refresh error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTEN_A::_0
    }
    ///WDT underflow/refresh error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTEN_A::_1
    }
}
///Field `WDTEN` writer - WDT Underflow/Refresh Error Interrupt Enable
pub type WDTEN_W<'a, REG> = crate::BitWriter<'a, REG, WDTEN_A>;
impl<'a, REG> WDTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///WDT underflow/refresh error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTEN_A::_0)
    }
    ///WDT underflow/refresh error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTEN_A::_1)
    }
}
/**Voltage-Monitoring 1 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1EN_A {
    ///0: Voltage-monitoring 1 interrupt is disabled.
    _0 = 0,
    ///1: Voltage-monitoring 1 interrupt is enabled.
    _1 = 1,
}
impl From<LVD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1EN` reader - Voltage-Monitoring 1 Interrupt Enable
pub type LVD1EN_R = crate::BitReader<LVD1EN_A>;
impl LVD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1EN_A {
        match self.bits {
            false => LVD1EN_A::_0,
            true => LVD1EN_A::_1,
        }
    }
    ///Voltage-monitoring 1 interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1EN_A::_0
    }
    ///Voltage-monitoring 1 interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1EN_A::_1
    }
}
///Field `LVD1EN` writer - Voltage-Monitoring 1 Interrupt Enable
pub type LVD1EN_W<'a, REG> = crate::BitWriter<'a, REG, LVD1EN_A>;
impl<'a, REG> LVD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage-monitoring 1 interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1EN_A::_0)
    }
    ///Voltage-monitoring 1 interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1EN_A::_1)
    }
}
/**Voltage-Monitoring 2 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2EN_A {
    ///0: Voltage-monitoring 2 interrupt is disabled.
    _0 = 0,
    ///1: Voltage-monitoring 2 interrupt is enabled.
    _1 = 1,
}
impl From<LVD2EN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2EN` reader - Voltage-Monitoring 2 Interrupt Enable
pub type LVD2EN_R = crate::BitReader<LVD2EN_A>;
impl LVD2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2EN_A {
        match self.bits {
            false => LVD2EN_A::_0,
            true => LVD2EN_A::_1,
        }
    }
    ///Voltage-monitoring 2 interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2EN_A::_0
    }
    ///Voltage-monitoring 2 interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2EN_A::_1
    }
}
///Field `LVD2EN` writer - Voltage-Monitoring 2 Interrupt Enable
pub type LVD2EN_W<'a, REG> = crate::BitWriter<'a, REG, LVD2EN_A>;
impl<'a, REG> LVD2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage-monitoring 2 interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2EN_A::_0)
    }
    ///Voltage-monitoring 2 interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2EN_A::_1)
    }
}
/**Oscillation Stop Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTEN_A {
    ///0: Oscillation stop detection interrupt is disabled.
    _0 = 0,
    ///1: Oscillation stop detection interrupt is enabled.
    _1 = 1,
}
impl From<OSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTEN` reader - Oscillation Stop Detection Interrupt Enable
pub type OSTEN_R = crate::BitReader<OSTEN_A>;
impl OSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTEN_A {
        match self.bits {
            false => OSTEN_A::_0,
            true => OSTEN_A::_1,
        }
    }
    ///Oscillation stop detection interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTEN_A::_0
    }
    ///Oscillation stop detection interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTEN_A::_1
    }
}
///Field `OSTEN` writer - Oscillation Stop Detection Interrupt Enable
pub type OSTEN_W<'a, REG> = crate::BitWriter<'a, REG, OSTEN_A>;
impl<'a, REG> OSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oscillation stop detection interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTEN_A::_0)
    }
    ///Oscillation stop detection interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTEN_A::_1)
    }
}
/**NMI Pin Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIEN_A {
    ///0: NMI pin interrupt is disabled.
    _0 = 0,
    ///1: NMI pin interrupt is enabled.
    _1 = 1,
}
impl From<NMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NMIEN` reader - NMI Pin Interrupt Enable
pub type NMIEN_R = crate::BitReader<NMIEN_A>;
impl NMIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NMIEN_A {
        match self.bits {
            false => NMIEN_A::_0,
            true => NMIEN_A::_1,
        }
    }
    ///NMI pin interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIEN_A::_0
    }
    ///NMI pin interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIEN_A::_1
    }
}
///Field `NMIEN` writer - NMI Pin Interrupt Enable
pub type NMIEN_W<'a, REG> = crate::BitWriter<'a, REG, NMIEN_A>;
impl<'a, REG> NMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NMI pin interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NMIEN_A::_0)
    }
    ///NMI pin interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NMIEN_A::_1)
    }
}
/**RAM Parity Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPEEN_A {
    ///0: RAM Parity Error interrupt is disabled.
    _0 = 0,
    ///1: RAM Parity Error interrupt is enabled.
    _1 = 1,
}
impl From<RPEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RPEEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPEEN` reader - RAM Parity Error Interrupt Enable
pub type RPEEN_R = crate::BitReader<RPEEN_A>;
impl RPEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPEEN_A {
        match self.bits {
            false => RPEEN_A::_0,
            true => RPEEN_A::_1,
        }
    }
    ///RAM Parity Error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPEEN_A::_0
    }
    ///RAM Parity Error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPEEN_A::_1
    }
}
///Field `RPEEN` writer - RAM Parity Error Interrupt Enable
pub type RPEEN_W<'a, REG> = crate::BitWriter<'a, REG, RPEEN_A>;
impl<'a, REG> RPEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RAM Parity Error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPEEN_A::_0)
    }
    ///RAM Parity Error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPEEN_A::_1)
    }
}
/**RAM ECC Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECCEN_A {
    ///0: RAM ECC Error interrupt is disabled.
    _0 = 0,
    ///1: RAM ECC Error interrupt is enabled.
    _1 = 1,
}
impl From<RECCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RECCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RECCEN` reader - RAM ECC Error Interrupt Enable
pub type RECCEN_R = crate::BitReader<RECCEN_A>;
impl RECCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RECCEN_A {
        match self.bits {
            false => RECCEN_A::_0,
            true => RECCEN_A::_1,
        }
    }
    ///RAM ECC Error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECCEN_A::_0
    }
    ///RAM ECC Error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECCEN_A::_1
    }
}
///Field `RECCEN` writer - RAM ECC Error Interrupt Enable
pub type RECCEN_W<'a, REG> = crate::BitWriter<'a, REG, RECCEN_A>;
impl<'a, REG> RECCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RAM ECC Error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RECCEN_A::_0)
    }
    ///RAM ECC Error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RECCEN_A::_1)
    }
}
/**MPU Bus Slave Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSEN_A {
    ///0: MPU Bus Slave Error interrupt is disabled.
    _0 = 0,
    ///1: MPU Bus Slave Error interrupt is enabled.
    _1 = 1,
}
impl From<BUSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSSEN` reader - MPU Bus Slave Error Interrupt Enable
pub type BUSSEN_R = crate::BitReader<BUSSEN_A>;
impl BUSSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSSEN_A {
        match self.bits {
            false => BUSSEN_A::_0,
            true => BUSSEN_A::_1,
        }
    }
    ///MPU Bus Slave Error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSEN_A::_0
    }
    ///MPU Bus Slave Error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSEN_A::_1
    }
}
///Field `BUSSEN` writer - MPU Bus Slave Error Interrupt Enable
pub type BUSSEN_W<'a, REG> = crate::BitWriter<'a, REG, BUSSEN_A>;
impl<'a, REG> BUSSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MPU Bus Slave Error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSEN_A::_0)
    }
    ///MPU Bus Slave Error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSEN_A::_1)
    }
}
/**MPU Bus Master Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMEN_A {
    ///0: MPU Bus Master Error interrupt is disabled.
    _0 = 0,
    ///1: MPU Bus Master Error interrupt is enabled.
    _1 = 1,
}
impl From<BUSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMEN` reader - MPU Bus Master Error Interrupt Enable
pub type BUSMEN_R = crate::BitReader<BUSMEN_A>;
impl BUSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSMEN_A {
        match self.bits {
            false => BUSMEN_A::_0,
            true => BUSMEN_A::_1,
        }
    }
    ///MPU Bus Master Error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMEN_A::_0
    }
    ///MPU Bus Master Error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMEN_A::_1
    }
}
///Field `BUSMEN` writer - MPU Bus Master Error Interrupt Enable
pub type BUSMEN_W<'a, REG> = crate::BitWriter<'a, REG, BUSMEN_A>;
impl<'a, REG> BUSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MPU Bus Master Error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMEN_A::_0)
    }
    ///MPU Bus Master Error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMEN_A::_1)
    }
}
/**MPU Stack Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEEN_A {
    ///0: MPU Stack Error interrupt is disabled.
    _0 = 0,
    ///1: MPU Stack Error interrupt is enabled.
    _1 = 1,
}
impl From<SPEEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPEEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPEEN` reader - MPU Stack Error Interrupt Enable
pub type SPEEN_R = crate::BitReader<SPEEN_A>;
impl SPEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPEEN_A {
        match self.bits {
            false => SPEEN_A::_0,
            true => SPEEN_A::_1,
        }
    }
    ///MPU Stack Error interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEEN_A::_0
    }
    ///MPU Stack Error interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEEN_A::_1
    }
}
///Field `SPEEN` writer - MPU Stack Error Interrupt Enable
pub type SPEEN_W<'a, REG> = crate::BitWriter<'a, REG, SPEEN_A>;
impl<'a, REG> SPEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MPU Stack Error interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEN_A::_0)
    }
    ///MPU Stack Error interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEN_A::_1)
    }
}
impl R {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn iwdten(&self) -> IWDTEN_R {
        IWDTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage-Monitoring 1 Interrupt Enable
    #[inline(always)]
    pub fn lvd1en(&self) -> LVD1EN_R {
        LVD1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage-Monitoring 2 Interrupt Enable
    #[inline(always)]
    pub fn lvd2en(&self) -> LVD2EN_R {
        LVD2EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Oscillation Stop Detection Interrupt Enable
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NMI Pin Interrupt Enable
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RAM Parity Error Interrupt Enable
    #[inline(always)]
    pub fn rpeen(&self) -> RPEEN_R {
        RPEEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RAM ECC Error Interrupt Enable
    #[inline(always)]
    pub fn reccen(&self) -> RECCEN_R {
        RECCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MPU Bus Slave Error Interrupt Enable
    #[inline(always)]
    pub fn bussen(&self) -> BUSSEN_R {
        BUSSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MPU Bus Master Error Interrupt Enable
    #[inline(always)]
    pub fn busmen(&self) -> BUSMEN_R {
        BUSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - MPU Stack Error Interrupt Enable
    #[inline(always)]
    pub fn speen(&self) -> SPEEN_R {
        SPEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn iwdten(&mut self) -> IWDTEN_W<NMIER_SPEC> {
        IWDTEN_W::new(self, 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W<NMIER_SPEC> {
        WDTEN_W::new(self, 1)
    }
    ///Bit 2 - Voltage-Monitoring 1 Interrupt Enable
    #[inline(always)]
    pub fn lvd1en(&mut self) -> LVD1EN_W<NMIER_SPEC> {
        LVD1EN_W::new(self, 2)
    }
    ///Bit 3 - Voltage-Monitoring 2 Interrupt Enable
    #[inline(always)]
    pub fn lvd2en(&mut self) -> LVD2EN_W<NMIER_SPEC> {
        LVD2EN_W::new(self, 3)
    }
    ///Bit 6 - Oscillation Stop Detection Interrupt Enable
    #[inline(always)]
    pub fn osten(&mut self) -> OSTEN_W<NMIER_SPEC> {
        OSTEN_W::new(self, 6)
    }
    ///Bit 7 - NMI Pin Interrupt Enable
    #[inline(always)]
    pub fn nmien(&mut self) -> NMIEN_W<NMIER_SPEC> {
        NMIEN_W::new(self, 7)
    }
    ///Bit 8 - RAM Parity Error Interrupt Enable
    #[inline(always)]
    pub fn rpeen(&mut self) -> RPEEN_W<NMIER_SPEC> {
        RPEEN_W::new(self, 8)
    }
    ///Bit 9 - RAM ECC Error Interrupt Enable
    #[inline(always)]
    pub fn reccen(&mut self) -> RECCEN_W<NMIER_SPEC> {
        RECCEN_W::new(self, 9)
    }
    ///Bit 10 - MPU Bus Slave Error Interrupt Enable
    #[inline(always)]
    pub fn bussen(&mut self) -> BUSSEN_W<NMIER_SPEC> {
        BUSSEN_W::new(self, 10)
    }
    ///Bit 11 - MPU Bus Master Error Interrupt Enable
    #[inline(always)]
    pub fn busmen(&mut self) -> BUSMEN_W<NMIER_SPEC> {
        BUSMEN_W::new(self, 11)
    }
    ///Bit 12 - MPU Stack Error Interrupt Enable
    #[inline(always)]
    pub fn speen(&mut self) -> SPEEN_W<NMIER_SPEC> {
        SPEEN_W::new(self, 12)
    }
}
/**Non-Maskable Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nmier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NMIER_SPEC;
impl crate::RegisterSpec for NMIER_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nmier::R`](R) reader structure
impl crate::Readable for NMIER_SPEC {}
///`write(|w| ..)` method takes [`nmier::W`](W) writer structure
impl crate::Writable for NMIER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NMIER to value 0
impl crate::Resettable for NMIER_SPEC {}
