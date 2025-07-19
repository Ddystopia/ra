///Register `GTSTR` reader
pub type R = crate::R<GTSTR_SPEC>;
///Register `GTSTR` writer
pub type W = crate::W<GTSTR_SPEC>;
/**Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT0_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT320.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT0` reader - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT0_R = crate::BitReader<CSTRT0_A>;
impl CSTRT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT0_A {
        match self.bits {
            false => CSTRT0_A::_0,
            true => CSTRT0_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT0_A::_0
    }
    ///GPT320.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT0_A::_1
    }
}
///Field `CSTRT0` writer - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT0_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT0_A>;
impl<'a, REG> CSTRT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT0_A::_0)
    }
    ///GPT320.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT0_A::_1)
    }
}
/**Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT1_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT321.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT1` reader - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT1_R = crate::BitReader<CSTRT1_A>;
impl CSTRT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT1_A {
        match self.bits {
            false => CSTRT1_A::_0,
            true => CSTRT1_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT1_A::_0
    }
    ///GPT321.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT1_A::_1
    }
}
///Field `CSTRT1` writer - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT1_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT1_A>;
impl<'a, REG> CSTRT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT1_A::_0)
    }
    ///GPT321.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT1_A::_1)
    }
}
/**Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT2_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT322.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT2` reader - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT2_R = crate::BitReader<CSTRT2_A>;
impl CSTRT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT2_A {
        match self.bits {
            false => CSTRT2_A::_0,
            true => CSTRT2_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT2_A::_0
    }
    ///GPT322.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT2_A::_1
    }
}
///Field `CSTRT2` writer - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT2_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT2_A>;
impl<'a, REG> CSTRT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT2_A::_0)
    }
    ///GPT322.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT2_A::_1)
    }
}
/**Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT3_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT323.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT3` reader - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT3_R = crate::BitReader<CSTRT3_A>;
impl CSTRT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT3_A {
        match self.bits {
            false => CSTRT3_A::_0,
            true => CSTRT3_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT3_A::_0
    }
    ///GPT323.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT3_A::_1
    }
}
///Field `CSTRT3` writer - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT3_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT3_A>;
impl<'a, REG> CSTRT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT3_A::_0)
    }
    ///GPT323.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT3_A::_1)
    }
}
/**Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT4_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT164.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT4` reader - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT4_R = crate::BitReader<CSTRT4_A>;
impl CSTRT4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT4_A {
        match self.bits {
            false => CSTRT4_A::_0,
            true => CSTRT4_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT4_A::_0
    }
    ///GPT164.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT4_A::_1
    }
}
///Field `CSTRT4` writer - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT4_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT4_A>;
impl<'a, REG> CSTRT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT4_A::_0)
    }
    ///GPT164.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT4_A::_1)
    }
}
/**Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT5_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT165.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT5` reader - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT5_R = crate::BitReader<CSTRT5_A>;
impl CSTRT5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT5_A {
        match self.bits {
            false => CSTRT5_A::_0,
            true => CSTRT5_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT5_A::_0
    }
    ///GPT165.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT5_A::_1
    }
}
///Field `CSTRT5` writer - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT5_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT5_A>;
impl<'a, REG> CSTRT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT5_A::_0)
    }
    ///GPT165.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT5_A::_1)
    }
}
/**Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT6_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT166.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT6` reader - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT6_R = crate::BitReader<CSTRT6_A>;
impl CSTRT6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT6_A {
        match self.bits {
            false => CSTRT6_A::_0,
            true => CSTRT6_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT6_A::_0
    }
    ///GPT166.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT6_A::_1
    }
}
///Field `CSTRT6` writer - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT6_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT6_A>;
impl<'a, REG> CSTRT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT6_A::_0)
    }
    ///GPT166.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT6_A::_1)
    }
}
/**Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT7_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT167.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT7` reader - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT7_R = crate::BitReader<CSTRT7_A>;
impl CSTRT7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT7_A {
        match self.bits {
            false => CSTRT7_A::_0,
            true => CSTRT7_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT7_A::_0
    }
    ///GPT167.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT7_A::_1
    }
}
///Field `CSTRT7` writer - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT7_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT7_A>;
impl<'a, REG> CSTRT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT7_A::_0)
    }
    ///GPT167.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT7_A::_1)
    }
}
impl R {
    ///Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt0(&self) -> CSTRT0_R {
        CSTRT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt1(&self) -> CSTRT1_R {
        CSTRT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt2(&self) -> CSTRT2_R {
        CSTRT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt3(&self) -> CSTRT3_R {
        CSTRT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt4(&self) -> CSTRT4_R {
        CSTRT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt5(&self) -> CSTRT5_R {
        CSTRT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt6(&self) -> CSTRT6_R {
        CSTRT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt7(&self) -> CSTRT7_R {
        CSTRT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel 0 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt0(&mut self) -> CSTRT0_W<'_, GTSTR_SPEC> {
        CSTRT0_W::new(self, 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt1(&mut self) -> CSTRT1_W<'_, GTSTR_SPEC> {
        CSTRT1_W::new(self, 1)
    }
    ///Bit 2 - Channel 2 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt2(&mut self) -> CSTRT2_W<'_, GTSTR_SPEC> {
        CSTRT2_W::new(self, 2)
    }
    ///Bit 3 - Channel 3 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt3(&mut self) -> CSTRT3_W<'_, GTSTR_SPEC> {
        CSTRT3_W::new(self, 3)
    }
    ///Bit 4 - Channel 4 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt4(&mut self) -> CSTRT4_W<'_, GTSTR_SPEC> {
        CSTRT4_W::new(self, 4)
    }
    ///Bit 5 - Channel 5 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt5(&mut self) -> CSTRT5_W<'_, GTSTR_SPEC> {
        CSTRT5_W::new(self, 5)
    }
    ///Bit 6 - Channel 6 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt6(&mut self) -> CSTRT6_W<'_, GTSTR_SPEC> {
        CSTRT6_W::new(self, 6)
    }
    ///Bit 7 - Channel 7 GTCNT Count Start Read data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt7(&mut self) -> CSTRT7_W<'_, GTSTR_SPEC> {
        CSTRT7_W::new(self, 7)
    }
}
/**General PWM Timer Software Start Register

You can [`read`](crate::Reg::read) this register and get [`gtstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTSTR_SPEC;
impl crate::RegisterSpec for GTSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtstr::R`](R) reader structure
impl crate::Readable for GTSTR_SPEC {}
///`write(|w| ..)` method takes [`gtstr::W`](W) writer structure
impl crate::Writable for GTSTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSTR to value 0
impl crate::Resettable for GTSTR_SPEC {}
