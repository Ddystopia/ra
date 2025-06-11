///Register `RCR2` reader
pub type R = crate::R<RCR2_SPEC>;
///Register `RCR2` writer
pub type W = crate::W<RCR2_SPEC>;
/**Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    ///0: Prescaler and time counter are stopped.
    _0 = 0,
    ///1: Prescaler and time counter operate normally.
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<START_A>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    ///Prescaler and time counter are stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    ///Prescaler and time counter operate normally.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
///Field `START` writer - Start
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START_A>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prescaler and time counter are stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_0)
    }
    ///Prescaler and time counter operate normally.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_1)
    }
}
/**RTC Software Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    ///0: Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)
    _0 = 0,
    ///1: The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)
    _1 = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` reader - RTC Software Reset
pub type RESET_R = crate::BitReader<RESET_A>;
impl RESET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::_0,
            true => RESET_A::_1,
        }
    }
    ///Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESET_A::_0
    }
    ///The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESET_A::_1
    }
}
///Field `RESET` writer - RTC Software Reset
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::_0)
    }
    ///The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::_1)
    }
}
/**30-Second Adjustment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADJ30_A {
    ///0: Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)
    _0 = 0,
    ///1: 30-second adjustment is executed.(write) / During 30-second adjustment.(read)
    _1 = 1,
}
impl From<ADJ30_A> for bool {
    #[inline(always)]
    fn from(variant: ADJ30_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADJ30` reader - 30-Second Adjustment
pub type ADJ30_R = crate::BitReader<ADJ30_A>;
impl ADJ30_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADJ30_A {
        match self.bits {
            false => ADJ30_A::_0,
            true => ADJ30_A::_1,
        }
    }
    ///Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADJ30_A::_0
    }
    ///30-second adjustment is executed.(write) / During 30-second adjustment.(read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADJ30_A::_1
    }
}
///Field `ADJ30` writer - 30-Second Adjustment
pub type ADJ30_W<'a, REG> = crate::BitWriter<'a, REG, ADJ30_A>;
impl<'a, REG> ADJ30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADJ30_A::_0)
    }
    ///30-second adjustment is executed.(write) / During 30-second adjustment.(read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADJ30_A::_1)
    }
}
/**RTCOUT Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOE_A {
    ///0: RTCOUT output disabled.
    _0 = 0,
    ///1: RTCOUT output enabled.
    _1 = 1,
}
impl From<RTCOE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCOE` reader - RTCOUT Output Enable
pub type RTCOE_R = crate::BitReader<RTCOE_A>;
impl RTCOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCOE_A {
        match self.bits {
            false => RTCOE_A::_0,
            true => RTCOE_A::_1,
        }
    }
    ///RTCOUT output disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCOE_A::_0
    }
    ///RTCOUT output enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCOE_A::_1
    }
}
///Field `RTCOE` writer - RTCOUT Output Enable
pub type RTCOE_W<'a, REG> = crate::BitWriter<'a, REG, RTCOE_A>;
impl<'a, REG> RTCOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTCOUT output disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOE_A::_0)
    }
    ///RTCOUT output enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOE_A::_1)
    }
}
/**Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADJE_A {
    ///0: Automatic adjustment is disabled.
    _0 = 0,
    ///1: Automatic adjustment is enabled.
    _1 = 1,
}
impl From<AADJE_A> for bool {
    #[inline(always)]
    fn from(variant: AADJE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AADJE` reader - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)
pub type AADJE_R = crate::BitReader<AADJE_A>;
impl AADJE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AADJE_A {
        match self.bits {
            false => AADJE_A::_0,
            true => AADJE_A::_1,
        }
    }
    ///Automatic adjustment is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AADJE_A::_0
    }
    ///Automatic adjustment is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AADJE_A::_1
    }
}
///Field `AADJE` writer - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)
pub type AADJE_W<'a, REG> = crate::BitWriter<'a, REG, AADJE_A>;
impl<'a, REG> AADJE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic adjustment is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AADJE_A::_0)
    }
    ///Automatic adjustment is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AADJE_A::_1)
    }
}
/**Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADJP_A {
    ///0: The RADJ.ADJ\[5:0\] setting value is adjusted from the count value of the prescaler every minute.
    _0 = 0,
    ///1: The RADJ.ADJ\[5:0\] setting value is adjusted from the count value of the prescaler every 10 seconds.
    _1 = 1,
}
impl From<AADJP_A> for bool {
    #[inline(always)]
    fn from(variant: AADJP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AADJP` reader - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)
pub type AADJP_R = crate::BitReader<AADJP_A>;
impl AADJP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AADJP_A {
        match self.bits {
            false => AADJP_A::_0,
            true => AADJP_A::_1,
        }
    }
    ///The RADJ.ADJ\[5:0\] setting value is adjusted from the count value of the prescaler every minute.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AADJP_A::_0
    }
    ///The RADJ.ADJ\[5:0\] setting value is adjusted from the count value of the prescaler every 10 seconds.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AADJP_A::_1
    }
}
///Field `AADJP` writer - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)
pub type AADJP_W<'a, REG> = crate::BitWriter<'a, REG, AADJP_A>;
impl<'a, REG> AADJP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RADJ.ADJ\[5:0\] setting value is adjusted from the count value of the prescaler every minute.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AADJP_A::_0)
    }
    ///The RADJ.ADJ\[5:0\] setting value is adjusted from the count value of the prescaler every 10 seconds.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AADJP_A::_1)
    }
}
/**Hours Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HR24_A {
    ///0: The RTC operates in 12-hour mode.
    _0 = 0,
    ///1: The RTC operates in 24-hour mode.
    _1 = 1,
}
impl From<HR24_A> for bool {
    #[inline(always)]
    fn from(variant: HR24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HR24` reader - Hours Mode
pub type HR24_R = crate::BitReader<HR24_A>;
impl HR24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HR24_A {
        match self.bits {
            false => HR24_A::_0,
            true => HR24_A::_1,
        }
    }
    ///The RTC operates in 12-hour mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HR24_A::_0
    }
    ///The RTC operates in 24-hour mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HR24_A::_1
    }
}
///Field `HR24` writer - Hours Mode
pub type HR24_W<'a, REG> = crate::BitWriter<'a, REG, HR24_A>;
impl<'a, REG> HR24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RTC operates in 12-hour mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HR24_A::_0)
    }
    ///The RTC operates in 24-hour mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HR24_A::_1)
    }
}
/**Count Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMD_A {
    ///0: The calendar count mode.
    _0 = 0,
    ///1: The binary count mode.
    _1 = 1,
}
impl From<CNTMD_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CNTMD` reader - Count Mode Select
pub type CNTMD_R = crate::BitReader<CNTMD_A>;
impl CNTMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNTMD_A {
        match self.bits {
            false => CNTMD_A::_0,
            true => CNTMD_A::_1,
        }
    }
    ///The calendar count mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMD_A::_0
    }
    ///The binary count mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMD_A::_1
    }
}
///Field `CNTMD` writer - Count Mode Select
pub type CNTMD_W<'a, REG> = crate::BitWriter<'a, REG, CNTMD_A>;
impl<'a, REG> CNTMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The calendar count mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CNTMD_A::_0)
    }
    ///The binary count mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTMD_A::_1)
    }
}
impl R {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC Software Reset
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 30-Second Adjustment
    #[inline(always)]
    pub fn adj30(&self) -> ADJ30_R {
        ADJ30_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTCOUT Output Enable
    #[inline(always)]
    pub fn rtcoe(&self) -> RTCOE_R {
        RTCOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)
    #[inline(always)]
    pub fn aadje(&self) -> AADJE_R {
        AADJE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)
    #[inline(always)]
    pub fn aadjp(&self) -> AADJP_R {
        AADJP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hours Mode
    #[inline(always)]
    pub fn hr24(&self) -> HR24_R {
        HR24_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Count Mode Select
    #[inline(always)]
    pub fn cntmd(&self) -> CNTMD_R {
        CNTMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W<RCR2_SPEC> {
        START_W::new(self, 0)
    }
    ///Bit 1 - RTC Software Reset
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<RCR2_SPEC> {
        RESET_W::new(self, 1)
    }
    ///Bit 2 - 30-Second Adjustment
    #[inline(always)]
    pub fn adj30(&mut self) -> ADJ30_W<RCR2_SPEC> {
        ADJ30_W::new(self, 2)
    }
    ///Bit 3 - RTCOUT Output Enable
    #[inline(always)]
    pub fn rtcoe(&mut self) -> RTCOE_W<RCR2_SPEC> {
        RTCOE_W::new(self, 3)
    }
    ///Bit 4 - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)
    #[inline(always)]
    pub fn aadje(&mut self) -> AADJE_W<RCR2_SPEC> {
        AADJE_W::new(self, 4)
    }
    ///Bit 5 - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)
    #[inline(always)]
    pub fn aadjp(&mut self) -> AADJP_W<RCR2_SPEC> {
        AADJP_W::new(self, 5)
    }
    ///Bit 6 - Hours Mode
    #[inline(always)]
    pub fn hr24(&mut self) -> HR24_W<RCR2_SPEC> {
        HR24_W::new(self, 6)
    }
    ///Bit 7 - Count Mode Select
    #[inline(always)]
    pub fn cntmd(&mut self) -> CNTMD_W<RCR2_SPEC> {
        CNTMD_W::new(self, 7)
    }
}
/**RTC Control Register 2

You can [`read`](crate::Reg::read) this register and get [`rcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RCR2_SPEC;
impl crate::RegisterSpec for RCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rcr2::R`](R) reader structure
impl crate::Readable for RCR2_SPEC {}
///`write(|w| ..)` method takes [`rcr2::W`](W) writer structure
impl crate::Writable for RCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR2 to value 0
impl crate::Resettable for RCR2_SPEC {}
