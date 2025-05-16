///Register `GTCLR` writer
pub type W = crate::W<GTCLR_SPEC>;
/**Channel 0 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR0_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT32EH0.GTCNT counter clears
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
    ///GPT32EH0.GTCNT counter clears
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
    ///1: GPT32EH1.GTCNT counter clears
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
    ///GPT32EH1.GTCNT counter clears
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
    ///1: GPT32EH2.GTCNT counter clears
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
    ///GPT32EH2.GTCNT counter clears
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
    ///1: GPT32EH3.GTCNT counter clears
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
    ///GPT32EH3.GTCNT counter clears
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
    ///1: GPT32E4.GTCNT counter clears
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
    ///GPT32E4.GTCNT counter clears
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
    ///1: GPT32E5.GTCNT counter clears
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
    ///GPT32E5.GTCNT counter clears
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
    ///1: GPT32E6.GTCNT counter clears
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
    ///GPT32E6.GTCNT counter clears
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
    ///1: GPT32E7.GTCNT counter clears
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
    ///GPT32E7.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR7_A::_1)
    }
}
/**Channel 8 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR8_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT328.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR8_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR8` writer - Channel 8 GTCNT Count Clear
pub type CCLR8_W<'a, REG> = crate::BitWriter<'a, REG, CCLR8_A>;
impl<'a, REG> CCLR8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR8_A::_0)
    }
    ///GPT328.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR8_A::_1)
    }
}
/**Channel 9 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR9_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT329.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR9_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR9` writer - Channel 9 GTCNT Count Clear
pub type CCLR9_W<'a, REG> = crate::BitWriter<'a, REG, CCLR9_A>;
impl<'a, REG> CCLR9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR9_A::_0)
    }
    ///GPT329.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR9_A::_1)
    }
}
/**Channel 10 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR10_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT3210.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR10_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR10` writer - Channel 10 GTCNT Count Clear
pub type CCLR10_W<'a, REG> = crate::BitWriter<'a, REG, CCLR10_A>;
impl<'a, REG> CCLR10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR10_A::_0)
    }
    ///GPT3210.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR10_A::_1)
    }
}
/**Channel 11 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR11_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT3211.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR11_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR11` writer - Channel 11 GTCNT Count Clear
pub type CCLR11_W<'a, REG> = crate::BitWriter<'a, REG, CCLR11_A>;
impl<'a, REG> CCLR11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR11_A::_0)
    }
    ///GPT3211.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR11_A::_1)
    }
}
/**Channel 12 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR12_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT3212.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR12_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR12` writer - Channel 12 GTCNT Count Clear
pub type CCLR12_W<'a, REG> = crate::BitWriter<'a, REG, CCLR12_A>;
impl<'a, REG> CCLR12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR12_A::_0)
    }
    ///GPT3212.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR12_A::_1)
    }
}
/**Channel 13 GTCNT Count Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR13_A {
    ///0: No effect
    _0 = 0,
    ///1: GPT3213.GTCNT counter clears
    _1 = 1,
}
impl From<CCLR13_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR13` writer - Channel 13 GTCNT Count Clear
pub type CCLR13_W<'a, REG> = crate::BitWriter<'a, REG, CCLR13_A>;
impl<'a, REG> CCLR13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR13_A::_0)
    }
    ///GPT3213.GTCNT counter clears
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR13_A::_1)
    }
}
impl W {
    ///Bit 0 - Channel 0 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr0(&mut self) -> CCLR0_W<GTCLR_SPEC> {
        CCLR0_W::new(self, 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr1(&mut self) -> CCLR1_W<GTCLR_SPEC> {
        CCLR1_W::new(self, 1)
    }
    ///Bit 2 - Channel 2 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr2(&mut self) -> CCLR2_W<GTCLR_SPEC> {
        CCLR2_W::new(self, 2)
    }
    ///Bit 3 - Channel 3 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr3(&mut self) -> CCLR3_W<GTCLR_SPEC> {
        CCLR3_W::new(self, 3)
    }
    ///Bit 4 - Channel 4 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr4(&mut self) -> CCLR4_W<GTCLR_SPEC> {
        CCLR4_W::new(self, 4)
    }
    ///Bit 5 - Channel 5 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr5(&mut self) -> CCLR5_W<GTCLR_SPEC> {
        CCLR5_W::new(self, 5)
    }
    ///Bit 6 - Channel 6 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr6(&mut self) -> CCLR6_W<GTCLR_SPEC> {
        CCLR6_W::new(self, 6)
    }
    ///Bit 7 - Channel 7 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr7(&mut self) -> CCLR7_W<GTCLR_SPEC> {
        CCLR7_W::new(self, 7)
    }
    ///Bit 8 - Channel 8 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr8(&mut self) -> CCLR8_W<GTCLR_SPEC> {
        CCLR8_W::new(self, 8)
    }
    ///Bit 9 - Channel 9 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr9(&mut self) -> CCLR9_W<GTCLR_SPEC> {
        CCLR9_W::new(self, 9)
    }
    ///Bit 10 - Channel 10 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr10(&mut self) -> CCLR10_W<GTCLR_SPEC> {
        CCLR10_W::new(self, 10)
    }
    ///Bit 11 - Channel 11 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr11(&mut self) -> CCLR11_W<GTCLR_SPEC> {
        CCLR11_W::new(self, 11)
    }
    ///Bit 12 - Channel 12 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr12(&mut self) -> CCLR12_W<GTCLR_SPEC> {
        CCLR12_W::new(self, 12)
    }
    ///Bit 13 - Channel 13 GTCNT Count Clear
    #[inline(always)]
    pub fn cclr13(&mut self) -> CCLR13_W<GTCLR_SPEC> {
        CCLR13_W::new(self, 13)
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
