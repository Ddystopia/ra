///Register `RSTSR1` reader
pub type R = crate::R<RSTSR1_SPEC>;
///Register `RSTSR1` writer
pub type W = crate::W<RSTSR1_SPEC>;
/**Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTRF_A {
    ///0: Independent watchdog timer reset not detected.
    _0 = 0,
    ///1: Independent watchdog timer reset detected.
    _1 = 1,
}
impl From<IWDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `IWDTRF` reader - Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type IWDTRF_R = crate::BitReader<IWDTRF_A>;
impl IWDTRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDTRF_A {
        match self.bits {
            false => IWDTRF_A::_0,
            true => IWDTRF_A::_1,
        }
    }
    ///Independent watchdog timer reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTRF_A::_0
    }
    ///Independent watchdog timer reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTRF_A::_1
    }
}
///Field `IWDTRF` writer - Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type IWDTRF_W<'a, REG> = crate::BitWriter0C<'a, REG, IWDTRF_A>;
impl<'a, REG> IWDTRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Independent watchdog timer reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTRF_A::_0)
    }
    ///Independent watchdog timer reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTRF_A::_1)
    }
}
/**Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRF_A {
    ///0: Watchdog timer reset not detected.
    _0 = 0,
    ///1: Watchdog timer reset detected.
    _1 = 1,
}
impl From<WDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `WDTRF` reader - Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type WDTRF_R = crate::BitReader<WDTRF_A>;
impl WDTRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDTRF_A {
        match self.bits {
            false => WDTRF_A::_0,
            true => WDTRF_A::_1,
        }
    }
    ///Watchdog timer reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTRF_A::_0
    }
    ///Watchdog timer reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTRF_A::_1
    }
}
///Field `WDTRF` writer - Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type WDTRF_W<'a, REG> = crate::BitWriter0C<'a, REG, WDTRF_A>;
impl<'a, REG> WDTRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Watchdog timer reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRF_A::_0)
    }
    ///Watchdog timer reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRF_A::_1)
    }
}
/**Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRF_A {
    ///0: Software reset not detected.
    _0 = 0,
    ///1: Software reset detected.
    _1 = 1,
}
impl From<SWRF_A> for bool {
    #[inline(always)]
    fn from(variant: SWRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SWRF` reader - Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SWRF_R = crate::BitReader<SWRF_A>;
impl SWRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWRF_A {
        match self.bits {
            false => SWRF_A::_0,
            true => SWRF_A::_1,
        }
    }
    ///Software reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRF_A::_0
    }
    ///Software reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRF_A::_1
    }
}
///Field `SWRF` writer - Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type SWRF_W<'a, REG> = crate::BitWriter0C<'a, REG, SWRF_A>;
impl<'a, REG> SWRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWRF_A::_0)
    }
    ///Software reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWRF_A::_1)
    }
}
/**RAM Parity Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPERF_A {
    ///0: RAM parity error reset not detected.
    _0 = 0,
    ///1: RAM parity error reset detected.
    _1 = 1,
}
impl From<RPERF_A> for bool {
    #[inline(always)]
    fn from(variant: RPERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RPERF` reader - RAM Parity Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RPERF_R = crate::BitReader<RPERF_A>;
impl RPERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPERF_A {
        match self.bits {
            false => RPERF_A::_0,
            true => RPERF_A::_1,
        }
    }
    ///RAM parity error reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPERF_A::_0
    }
    ///RAM parity error reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPERF_A::_1
    }
}
///Field `RPERF` writer - RAM Parity Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type RPERF_W<'a, REG> = crate::BitWriter0C<'a, REG, RPERF_A>;
impl<'a, REG> RPERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RAM parity error reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPERF_A::_0)
    }
    ///RAM parity error reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPERF_A::_1)
    }
}
/**RAM ECC Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REERF_A {
    ///0: RAM ECC error reset not detected.
    _0 = 0,
    ///1: RAM ECC error reset detected.
    _1 = 1,
}
impl From<REERF_A> for bool {
    #[inline(always)]
    fn from(variant: REERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `REERF` reader - RAM ECC Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type REERF_R = crate::BitReader<REERF_A>;
impl REERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REERF_A {
        match self.bits {
            false => REERF_A::_0,
            true => REERF_A::_1,
        }
    }
    ///RAM ECC error reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REERF_A::_0
    }
    ///RAM ECC error reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REERF_A::_1
    }
}
///Field `REERF` writer - RAM ECC Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type REERF_W<'a, REG> = crate::BitWriter0C<'a, REG, REERF_A>;
impl<'a, REG> REERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RAM ECC error reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REERF_A::_0)
    }
    ///RAM ECC error reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REERF_A::_1)
    }
}
/**Bus Slave MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSRF_A {
    ///0: Bus Slave MPU reset not detected.
    _0 = 0,
    ///1: Bus Slave MPU reset detected.
    _1 = 1,
}
impl From<BUSSRF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `BUSSRF` reader - Bus Slave MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type BUSSRF_R = crate::BitReader<BUSSRF_A>;
impl BUSSRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSSRF_A {
        match self.bits {
            false => BUSSRF_A::_0,
            true => BUSSRF_A::_1,
        }
    }
    ///Bus Slave MPU reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSRF_A::_0
    }
    ///Bus Slave MPU reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSRF_A::_1
    }
}
///Field `BUSSRF` writer - Bus Slave MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type BUSSRF_W<'a, REG> = crate::BitWriter0C<'a, REG, BUSSRF_A>;
impl<'a, REG> BUSSRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus Slave MPU reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSRF_A::_0)
    }
    ///Bus Slave MPU reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSRF_A::_1)
    }
}
/**Bus Master MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMRF_A {
    ///0: Bus Master MPU reset not detected.
    _0 = 0,
    ///1: Bus Master MPU reset detected.
    _1 = 1,
}
impl From<BUSMRF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `BUSMRF` reader - Bus Master MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type BUSMRF_R = crate::BitReader<BUSMRF_A>;
impl BUSMRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSMRF_A {
        match self.bits {
            false => BUSMRF_A::_0,
            true => BUSMRF_A::_1,
        }
    }
    ///Bus Master MPU reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMRF_A::_0
    }
    ///Bus Master MPU reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMRF_A::_1
    }
}
///Field `BUSMRF` writer - Bus Master MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type BUSMRF_W<'a, REG> = crate::BitWriter0C<'a, REG, BUSMRF_A>;
impl<'a, REG> BUSMRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus Master MPU reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMRF_A::_0)
    }
    ///Bus Master MPU reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMRF_A::_1)
    }
}
/**SP Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPERF_A {
    ///0: SP error reset not detected.
    _0 = 0,
    ///1: SP error reset detected.
    _1 = 1,
}
impl From<SPERF_A> for bool {
    #[inline(always)]
    fn from(variant: SPERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SPERF` reader - SP Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SPERF_R = crate::BitReader<SPERF_A>;
impl SPERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPERF_A {
        match self.bits {
            false => SPERF_A::_0,
            true => SPERF_A::_1,
        }
    }
    ///SP error reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPERF_A::_0
    }
    ///SP error reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPERF_A::_1
    }
}
///Field `SPERF` writer - SP Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
pub type SPERF_W<'a, REG> = crate::BitWriter0C<'a, REG, SPERF_A>;
impl<'a, REG> SPERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SP error reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPERF_A::_0)
    }
    ///SP error reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPERF_A::_1)
    }
}
impl R {
    ///Bit 0 - Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn iwdtrf(&self) -> IWDTRF_R {
        IWDTRF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn wdtrf(&self) -> WDTRF_R {
        WDTRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn swrf(&self) -> SWRF_R {
        SWRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - RAM Parity Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn rperf(&self) -> RPERF_R {
        RPERF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RAM ECC Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn reerf(&self) -> REERF_R {
        REERF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Bus Slave MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn bussrf(&self) -> BUSSRF_R {
        BUSSRF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Bus Master MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn busmrf(&self) -> BUSMRF_R {
        BUSMRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SP Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn sperf(&self) -> SPERF_R {
        SPERF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn iwdtrf(&mut self) -> IWDTRF_W<RSTSR1_SPEC> {
        IWDTRF_W::new(self, 0)
    }
    ///Bit 1 - Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn wdtrf(&mut self) -> WDTRF_W<RSTSR1_SPEC> {
        WDTRF_W::new(self, 1)
    }
    ///Bit 2 - Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn swrf(&mut self) -> SWRF_W<RSTSR1_SPEC> {
        SWRF_W::new(self, 2)
    }
    ///Bit 8 - RAM Parity Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn rperf(&mut self) -> RPERF_W<RSTSR1_SPEC> {
        RPERF_W::new(self, 8)
    }
    ///Bit 9 - RAM ECC Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn reerf(&mut self) -> REERF_W<RSTSR1_SPEC> {
        REERF_W::new(self, 9)
    }
    ///Bit 10 - Bus Slave MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn bussrf(&mut self) -> BUSSRF_W<RSTSR1_SPEC> {
        BUSSRF_W::new(self, 10)
    }
    ///Bit 11 - Bus Master MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn busmrf(&mut self) -> BUSMRF_W<RSTSR1_SPEC> {
        BUSMRF_W::new(self, 11)
    }
    ///Bit 12 - SP Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0.
    #[inline(always)]
    pub fn sperf(&mut self) -> SPERF_W<RSTSR1_SPEC> {
        SPERF_W::new(self, 12)
    }
}
/**Reset Status Register 1

You can [`read`](crate::Reg::read) this register and get [`rstsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSTSR1_SPEC;
impl crate::RegisterSpec for RSTSR1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`rstsr1::R`](R) reader structure
impl crate::Readable for RSTSR1_SPEC {}
///`write(|w| ..)` method takes [`rstsr1::W`](W) writer structure
impl crate::Writable for RSTSR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x1f07;
}
///`reset()` method sets RSTSR1 to value 0
impl crate::Resettable for RSTSR1_SPEC {}
