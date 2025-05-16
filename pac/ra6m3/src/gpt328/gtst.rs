///Register `GTST` reader
pub type R = crate::R<GTST_SPEC>;
///Register `GTST` writer
pub type W = crate::W<GTST_SPEC>;
/**Input Capture/Compare Match Flag A

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFA_A {
    ///0: No input capture/compare match of GTCCRA is generated.
    _0 = 0,
    ///1: An input capture/compare match of GTCCRA is generated.
    _1 = 1,
}
impl From<TCFA_A> for bool {
    #[inline(always)]
    fn from(variant: TCFA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFA` reader - Input Capture/Compare Match Flag A
pub type TCFA_R = crate::BitReader<TCFA_A>;
impl TCFA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFA_A {
        match self.bits {
            false => TCFA_A::_0,
            true => TCFA_A::_1,
        }
    }
    ///No input capture/compare match of GTCCRA is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFA_A::_0
    }
    ///An input capture/compare match of GTCCRA is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFA_A::_1
    }
}
///Field `TCFA` writer - Input Capture/Compare Match Flag A
pub type TCFA_W<'a, REG> = crate::BitWriter<'a, REG, TCFA_A>;
impl<'a, REG> TCFA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No input capture/compare match of GTCCRA is generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFA_A::_0)
    }
    ///An input capture/compare match of GTCCRA is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFA_A::_1)
    }
}
/**Input Capture/Compare Match Flag B

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFB_A {
    ///0: No input capture/compare match of GTCCRB is generated.
    _0 = 0,
    ///1: An input capture/compare match of GTCCRB is generated.
    _1 = 1,
}
impl From<TCFB_A> for bool {
    #[inline(always)]
    fn from(variant: TCFB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFB` reader - Input Capture/Compare Match Flag B
pub type TCFB_R = crate::BitReader<TCFB_A>;
impl TCFB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFB_A {
        match self.bits {
            false => TCFB_A::_0,
            true => TCFB_A::_1,
        }
    }
    ///No input capture/compare match of GTCCRB is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFB_A::_0
    }
    ///An input capture/compare match of GTCCRB is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFB_A::_1
    }
}
///Field `TCFB` writer - Input Capture/Compare Match Flag B
pub type TCFB_W<'a, REG> = crate::BitWriter<'a, REG, TCFB_A>;
impl<'a, REG> TCFB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No input capture/compare match of GTCCRB is generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFB_A::_0)
    }
    ///An input capture/compare match of GTCCRB is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFB_A::_1)
    }
}
/**Input Compare Match Flag C

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFC_A {
    ///0: No compare match of GTCCRC is generated.
    _0 = 0,
    ///1: A compare match of GTCCRC is generated.
    _1 = 1,
}
impl From<TCFC_A> for bool {
    #[inline(always)]
    fn from(variant: TCFC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFC` reader - Input Compare Match Flag C
pub type TCFC_R = crate::BitReader<TCFC_A>;
impl TCFC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFC_A {
        match self.bits {
            false => TCFC_A::_0,
            true => TCFC_A::_1,
        }
    }
    ///No compare match of GTCCRC is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFC_A::_0
    }
    ///A compare match of GTCCRC is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFC_A::_1
    }
}
///Field `TCFC` writer - Input Compare Match Flag C
pub type TCFC_W<'a, REG> = crate::BitWriter<'a, REG, TCFC_A>;
impl<'a, REG> TCFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRC is generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFC_A::_0)
    }
    ///A compare match of GTCCRC is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFC_A::_1)
    }
}
/**Input Compare Match Flag D

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFD_A {
    ///0: No compare match of GTCCRD is generated.
    _0 = 0,
    ///1: A compare match of GTCCRD is generated.
    _1 = 1,
}
impl From<TCFD_A> for bool {
    #[inline(always)]
    fn from(variant: TCFD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFD` reader - Input Compare Match Flag D
pub type TCFD_R = crate::BitReader<TCFD_A>;
impl TCFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFD_A {
        match self.bits {
            false => TCFD_A::_0,
            true => TCFD_A::_1,
        }
    }
    ///No compare match of GTCCRD is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFD_A::_0
    }
    ///A compare match of GTCCRD is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFD_A::_1
    }
}
///Field `TCFD` writer - Input Compare Match Flag D
pub type TCFD_W<'a, REG> = crate::BitWriter<'a, REG, TCFD_A>;
impl<'a, REG> TCFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRD is generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFD_A::_0)
    }
    ///A compare match of GTCCRD is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFD_A::_1)
    }
}
/**Input Compare Match Flag E

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFE_A {
    ///0: No compare match of GTCCRE is generated.
    _0 = 0,
    ///1: A compare match of GTCCRE is generated.
    _1 = 1,
}
impl From<TCFE_A> for bool {
    #[inline(always)]
    fn from(variant: TCFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFE` reader - Input Compare Match Flag E
pub type TCFE_R = crate::BitReader<TCFE_A>;
impl TCFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFE_A {
        match self.bits {
            false => TCFE_A::_0,
            true => TCFE_A::_1,
        }
    }
    ///No compare match of GTCCRE is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFE_A::_0
    }
    ///A compare match of GTCCRE is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFE_A::_1
    }
}
///Field `TCFE` writer - Input Compare Match Flag E
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG, TCFE_A>;
impl<'a, REG> TCFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRE is generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE_A::_0)
    }
    ///A compare match of GTCCRE is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE_A::_1)
    }
}
/**Input Compare Match Flag F

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFF_A {
    ///0: No compare match of GTCCRF is generated.
    _0 = 0,
    ///1: A compare match of GTCCRF is generated.
    _1 = 1,
}
impl From<TCFF_A> for bool {
    #[inline(always)]
    fn from(variant: TCFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFF` reader - Input Compare Match Flag F
pub type TCFF_R = crate::BitReader<TCFF_A>;
impl TCFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFF_A {
        match self.bits {
            false => TCFF_A::_0,
            true => TCFF_A::_1,
        }
    }
    ///No compare match of GTCCRF is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFF_A::_0
    }
    ///A compare match of GTCCRF is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFF_A::_1
    }
}
///Field `TCFF` writer - Input Compare Match Flag F
pub type TCFF_W<'a, REG> = crate::BitWriter<'a, REG, TCFF_A>;
impl<'a, REG> TCFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match of GTCCRF is generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFF_A::_0)
    }
    ///A compare match of GTCCRF is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFF_A::_1)
    }
}
/**Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCPFO_A {
    ///0: No overflow (crest) has occurred.
    _0 = 0,
    ///1: An overflow (crest) has occurred.
    _1 = 1,
}
impl From<TCPFO_A> for bool {
    #[inline(always)]
    fn from(variant: TCPFO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCPFO` reader - Overflow Flag
pub type TCPFO_R = crate::BitReader<TCPFO_A>;
impl TCPFO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCPFO_A {
        match self.bits {
            false => TCPFO_A::_0,
            true => TCPFO_A::_1,
        }
    }
    ///No overflow (crest) has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCPFO_A::_0
    }
    ///An overflow (crest) has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCPFO_A::_1
    }
}
///Field `TCPFO` writer - Overflow Flag
pub type TCPFO_W<'a, REG> = crate::BitWriter<'a, REG, TCPFO_A>;
impl<'a, REG> TCPFO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overflow (crest) has occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCPFO_A::_0)
    }
    ///An overflow (crest) has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCPFO_A::_1)
    }
}
/**Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFPU_A {
    ///0: No underflow (trough) has occurred.
    _0 = 0,
    ///1: An underflow (trough) has occurred.
    _1 = 1,
}
impl From<TCFPU_A> for bool {
    #[inline(always)]
    fn from(variant: TCFPU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFPU` reader - Underflow Flag
pub type TCFPU_R = crate::BitReader<TCFPU_A>;
impl TCFPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFPU_A {
        match self.bits {
            false => TCFPU_A::_0,
            true => TCFPU_A::_1,
        }
    }
    ///No underflow (trough) has occurred.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFPU_A::_0
    }
    ///An underflow (trough) has occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFPU_A::_1
    }
}
///Field `TCFPU` writer - Underflow Flag
pub type TCFPU_W<'a, REG> = crate::BitWriter<'a, REG, TCFPU_A>;
impl<'a, REG> TCFPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No underflow (trough) has occurred.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFPU_A::_0)
    }
    ///An underflow (trough) has occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFPU_A::_1)
    }
}
/**Count Direction Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUCF_A {
    ///0: GTCNT counter is counting down
    _0 = 0,
    ///1: GTCNT counter is counting up.
    _1 = 1,
}
impl From<TUCF_A> for bool {
    #[inline(always)]
    fn from(variant: TUCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TUCF` reader - Count Direction Flag
pub type TUCF_R = crate::BitReader<TUCF_A>;
impl TUCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TUCF_A {
        match self.bits {
            false => TUCF_A::_0,
            true => TUCF_A::_1,
        }
    }
    ///GTCNT counter is counting down
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUCF_A::_0
    }
    ///GTCNT counter is counting up.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUCF_A::_1
    }
}
/**Output Disable Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODF_A {
    ///0: No output disable request is generated.
    _0 = 0,
    ///1: An output disable request is generated.
    _1 = 1,
}
impl From<ODF_A> for bool {
    #[inline(always)]
    fn from(variant: ODF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ODF` reader - Output Disable Flag
pub type ODF_R = crate::BitReader<ODF_A>;
impl ODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODF_A {
        match self.bits {
            false => ODF_A::_0,
            true => ODF_A::_1,
        }
    }
    ///No output disable request is generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ODF_A::_0
    }
    ///An output disable request is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ODF_A::_1
    }
}
/**Same Time Output Level High Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OABHF_A {
    ///0: GTIOCA pin and GTIOCB pin don't output 1 at the same time.
    _0 = 0,
    ///1: GTIOCA pin and GTIOCB pin output 1 at the same time.
    _1 = 1,
}
impl From<OABHF_A> for bool {
    #[inline(always)]
    fn from(variant: OABHF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OABHF` reader - Same Time Output Level High Disable Request Enable
pub type OABHF_R = crate::BitReader<OABHF_A>;
impl OABHF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OABHF_A {
        match self.bits {
            false => OABHF_A::_0,
            true => OABHF_A::_1,
        }
    }
    ///GTIOCA pin and GTIOCB pin don't output 1 at the same time.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OABHF_A::_0
    }
    ///GTIOCA pin and GTIOCB pin output 1 at the same time.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OABHF_A::_1
    }
}
/**Same Time Output Level Low Disable Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OABLF_A {
    ///0: GTIOCA pin and GTIOCB pin don't output 0 at the same time.
    _0 = 0,
    ///1: GTIOCA pin and GTIOCB pin output 0 at the same time.
    _1 = 1,
}
impl From<OABLF_A> for bool {
    #[inline(always)]
    fn from(variant: OABLF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OABLF` reader - Same Time Output Level Low Disable Request Enable
pub type OABLF_R = crate::BitReader<OABLF_A>;
impl OABLF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OABLF_A {
        match self.bits {
            false => OABLF_A::_0,
            true => OABLF_A::_1,
        }
    }
    ///GTIOCA pin and GTIOCB pin don't output 0 at the same time.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OABLF_A::_0
    }
    ///GTIOCA pin and GTIOCB pin output 0 at the same time.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OABLF_A::_1
    }
}
impl R {
    ///Bit 0 - Input Capture/Compare Match Flag A
    #[inline(always)]
    pub fn tcfa(&self) -> TCFA_R {
        TCFA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input Capture/Compare Match Flag B
    #[inline(always)]
    pub fn tcfb(&self) -> TCFB_R {
        TCFB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Input Compare Match Flag C
    #[inline(always)]
    pub fn tcfc(&self) -> TCFC_R {
        TCFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Input Compare Match Flag D
    #[inline(always)]
    pub fn tcfd(&self) -> TCFD_R {
        TCFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Input Compare Match Flag E
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Input Compare Match Flag F
    #[inline(always)]
    pub fn tcff(&self) -> TCFF_R {
        TCFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overflow Flag
    #[inline(always)]
    pub fn tcpfo(&self) -> TCPFO_R {
        TCPFO_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Underflow Flag
    #[inline(always)]
    pub fn tcfpu(&self) -> TCFPU_R {
        TCFPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - Count Direction Flag
    #[inline(always)]
    pub fn tucf(&self) -> TUCF_R {
        TUCF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - Output Disable Flag
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 29 - Same Time Output Level High Disable Request Enable
    #[inline(always)]
    pub fn oabhf(&self) -> OABHF_R {
        OABHF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Same Time Output Level Low Disable Request Enable
    #[inline(always)]
    pub fn oablf(&self) -> OABLF_R {
        OABLF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Input Capture/Compare Match Flag A
    #[inline(always)]
    pub fn tcfa(&mut self) -> TCFA_W<GTST_SPEC> {
        TCFA_W::new(self, 0)
    }
    ///Bit 1 - Input Capture/Compare Match Flag B
    #[inline(always)]
    pub fn tcfb(&mut self) -> TCFB_W<GTST_SPEC> {
        TCFB_W::new(self, 1)
    }
    ///Bit 2 - Input Compare Match Flag C
    #[inline(always)]
    pub fn tcfc(&mut self) -> TCFC_W<GTST_SPEC> {
        TCFC_W::new(self, 2)
    }
    ///Bit 3 - Input Compare Match Flag D
    #[inline(always)]
    pub fn tcfd(&mut self) -> TCFD_W<GTST_SPEC> {
        TCFD_W::new(self, 3)
    }
    ///Bit 4 - Input Compare Match Flag E
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W<GTST_SPEC> {
        TCFE_W::new(self, 4)
    }
    ///Bit 5 - Input Compare Match Flag F
    #[inline(always)]
    pub fn tcff(&mut self) -> TCFF_W<GTST_SPEC> {
        TCFF_W::new(self, 5)
    }
    ///Bit 6 - Overflow Flag
    #[inline(always)]
    pub fn tcpfo(&mut self) -> TCPFO_W<GTST_SPEC> {
        TCPFO_W::new(self, 6)
    }
    ///Bit 7 - Underflow Flag
    #[inline(always)]
    pub fn tcfpu(&mut self) -> TCFPU_W<GTST_SPEC> {
        TCFPU_W::new(self, 7)
    }
}
/**General PWM Timer Status Register

You can [`read`](crate::Reg::read) this register and get [`gtst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTST_SPEC;
impl crate::RegisterSpec for GTST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtst::R`](R) reader structure
impl crate::Readable for GTST_SPEC {}
///`write(|w| ..)` method takes [`gtst::W`](W) writer structure
impl crate::Writable for GTST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTST to value 0x8000
impl crate::Resettable for GTST_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
