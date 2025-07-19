///Register `GTCLR` writer
pub type W = crate::W<GTCLR_SPEC>;
/**Channel 0 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR0_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT320.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR0_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR0` writer - Channel 0 GTCNT Count Clear
pub type CCLR0_W<'a, REG> = crate::BitWriter<'a, REG, CCLR0_A>;
impl<'a, REG> CCLR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR0_A::_0)
    }
    ///GPT320.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR0_A::_1)
    }
}
/**Channel 1 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR1_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT321.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR1_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR1` writer - Channel 1 GTCNT Count Clear
pub type CCLR1_W<'a, REG> = crate::BitWriter<'a, REG, CCLR1_A>;
impl<'a, REG> CCLR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR1_A::_0)
    }
    ///GPT321.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR1_A::_1)
    }
}
/**Channel 2 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR2_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT322.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR2_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR2` writer - Channel 2 GTCNT Count Clear
pub type CCLR2_W<'a, REG> = crate::BitWriter<'a, REG, CCLR2_A>;
impl<'a, REG> CCLR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR2_A::_0)
    }
    ///GPT322.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR2_A::_1)
    }
}
/**Channel 3 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR3_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT323.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR3_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR3` writer - Channel 3 GTCNT Count Clear
pub type CCLR3_W<'a, REG> = crate::BitWriter<'a, REG, CCLR3_A>;
impl<'a, REG> CCLR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR3_A::_0)
    }
    ///GPT323.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR3_A::_1)
    }
}
/**Channel 4 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR4_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT164.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR4_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR4` writer - Channel 4 GTCNT Count Clear
pub type CCLR4_W<'a, REG> = crate::BitWriter<'a, REG, CCLR4_A>;
impl<'a, REG> CCLR4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR4_A::_0)
    }
    ///GPT164.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR4_A::_1)
    }
}
/**Channel 5 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR5_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT165.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR5_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR5` writer - Channel 5 GTCNT Count Clear
pub type CCLR5_W<'a, REG> = crate::BitWriter<'a, REG, CCLR5_A>;
impl<'a, REG> CCLR5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR5_A::_0)
    }
    ///GPT165.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR5_A::_1)
    }
}
/**Channel 6 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR6_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT166.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR6_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR6` writer - Channel 6 GTCNT Count Clear
pub type CCLR6_W<'a, REG> = crate::BitWriter<'a, REG, CCLR6_A>;
impl<'a, REG> CCLR6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR6_A::_0)
    }
    ///GPT166.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR6_A::_1)
    }
}
/**Channel 7 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR7_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT167.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR7_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR7` writer - Channel 7 GTCNT Count Clear
pub type CCLR7_W<'a, REG> = crate::BitWriter<'a, REG, CCLR7_A>;
impl<'a, REG> CCLR7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR7_A::_0)
    }
    ///GPT167.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR7_A::_1)
    }
}
impl W {
    ///Bit 0 - Channel 0 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr0(&mut self) -> CCLR0_W<'_, GTCLR_SPEC> {
        CCLR0_W::new(self, 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr1(&mut self) -> CCLR1_W<'_, GTCLR_SPEC> {
        CCLR1_W::new(self, 1)
    }
    ///Bit 2 - Channel 2 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr2(&mut self) -> CCLR2_W<'_, GTCLR_SPEC> {
        CCLR2_W::new(self, 2)
    }
    ///Bit 3 - Channel 3 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr3(&mut self) -> CCLR3_W<'_, GTCLR_SPEC> {
        CCLR3_W::new(self, 3)
    }
    ///Bit 4 - Channel 4 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr4(&mut self) -> CCLR4_W<'_, GTCLR_SPEC> {
        CCLR4_W::new(self, 4)
    }
    ///Bit 5 - Channel 5 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr5(&mut self) -> CCLR5_W<'_, GTCLR_SPEC> {
        CCLR5_W::new(self, 5)
    }
    ///Bit 6 - Channel 6 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr6(&mut self) -> CCLR6_W<'_, GTCLR_SPEC> {
        CCLR6_W::new(self, 6)
    }
    ///Bit 7 - Channel 7 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr7(&mut self) -> CCLR7_W<'_, GTCLR_SPEC> {
        CCLR7_W::new(self, 7)
    }
}
/**General PWM Timer Software Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCLR_SPEC;
impl crate::RegisterSpec for GTCLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gtclr::W`](W) writer structure
impl crate::Writable for GTCLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCLR to value 0
impl crate::Resettable for GTCLR_SPEC {}
