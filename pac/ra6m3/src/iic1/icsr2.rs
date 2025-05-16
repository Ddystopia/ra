///Register `ICSR2` reader
pub type R = crate::R<ICSR2_SPEC>;
///Register `ICSR2` writer
pub type W = crate::W<ICSR2_SPEC>;
/**Timeout Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOF_A {
    ///0: Timeout is not detected.
    _0 = 0,
    ///1: Timeout is detected.
    _1 = 1,
}
impl From<TMOF_A> for bool {
    #[inline(always)]
    fn from(variant: TMOF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TMOF` reader - Timeout Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TMOF_R = crate::BitReader<TMOF_A>;
impl TMOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TMOF_A {
        match self.bits {
            false => TMOF_A::_0,
            true => TMOF_A::_1,
        }
    }
    ///Timeout is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOF_A::_0
    }
    ///Timeout is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOF_A::_1
    }
}
///Field `TMOF` writer - Timeout Detection Flag
pub type TMOF_W<'a, REG> = crate::BitWriter0C<'a, REG, TMOF_A>;
impl<'a, REG> TMOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timeout is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOF_A::_0)
    }
    ///Timeout is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOF_A::_1)
    }
}
/**Arbitration-Lost Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AL_A {
    ///0: Arbitration is not lost.
    _0 = 0,
    ///1: Arbitration is lost.
    _1 = 1,
}
impl From<AL_A> for bool {
    #[inline(always)]
    fn from(variant: AL_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `AL` reader - Arbitration-Lost Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type AL_R = crate::BitReader<AL_A>;
impl AL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AL_A {
        match self.bits {
            false => AL_A::_0,
            true => AL_A::_1,
        }
    }
    ///Arbitration is not lost.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AL_A::_0
    }
    ///Arbitration is lost.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AL_A::_1
    }
}
///Field `AL` writer - Arbitration-Lost Flag
pub type AL_W<'a, REG> = crate::BitWriter0C<'a, REG, AL_A>;
impl<'a, REG> AL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Arbitration is not lost.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AL_A::_0)
    }
    ///Arbitration is lost.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AL_A::_1)
    }
}
/**Start Condition Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    ///0: Start condition is not detected.
    _0 = 0,
    ///1: Start condition is detected.
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `START` reader - Start Condition Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
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
    ///Start condition is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    ///Start condition is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
///Field `START` writer - Start Condition Detection Flag
pub type START_W<'a, REG> = crate::BitWriter0C<'a, REG, START_A>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start condition is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_0)
    }
    ///Start condition is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_1)
    }
}
/**Stop Condition Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    ///0: Stop condition is not detected.
    _0 = 0,
    ///1: Stop condition is detected.
    _1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `STOP` reader - Stop Condition Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type STOP_R = crate::BitReader<STOP_A>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_0,
            true => STOP_A::_1,
        }
    }
    ///Stop condition is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOP_A::_0
    }
    ///Stop condition is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOP_A::_1
    }
}
///Field `STOP` writer - Stop Condition Detection Flag
pub type STOP_W<'a, REG> = crate::BitWriter0C<'a, REG, STOP_A>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop condition is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_0)
    }
    ///Stop condition is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_1)
    }
}
/**NACK Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKF_A {
    ///0: NACK is not detected.
    _0 = 0,
    ///1: NACK is detected.
    _1 = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `NACKF` reader - NACK Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type NACKF_R = crate::BitReader<NACKF_A>;
impl NACKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::_0,
            true => NACKF_A::_1,
        }
    }
    ///NACK is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKF_A::_0
    }
    ///NACK is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKF_A::_1
    }
}
///Field `NACKF` writer - NACK Detection Flag
pub type NACKF_W<'a, REG> = crate::BitWriter0C<'a, REG, NACKF_A>;
impl<'a, REG> NACKF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NACK is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NACKF_A::_0)
    }
    ///NACK is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKF_A::_1)
    }
}
/**Receive Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    ///0: ICDRR contains no receive data.
    _0 = 0,
    ///1: ICDRR contains receive data.
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RDRF` reader - Receive Data Full Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RDRF_R = crate::BitReader<RDRF_A>;
impl RDRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    ///ICDRR contains no receive data.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    ///ICDRR contains receive data.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
///Field `RDRF` writer - Receive Data Full Flag
pub type RDRF_W<'a, REG> = crate::BitWriter0C<'a, REG, RDRF_A>;
impl<'a, REG> RDRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ICDRR contains no receive data.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_0)
    }
    ///ICDRR contains receive data.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_1)
    }
}
/**Transmit End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    ///0: Data is being transmitted.
    _0 = 0,
    ///1: Data has been transmitted.
    _1 = 1,
}
impl From<TEND_A> for bool {
    #[inline(always)]
    fn from(variant: TEND_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TEND` reader - Transmit End Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TEND_R = crate::BitReader<TEND_A>;
impl TEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEND_A {
        match self.bits {
            false => TEND_A::_0,
            true => TEND_A::_1,
        }
    }
    ///Data is being transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEND_A::_0
    }
    ///Data has been transmitted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEND_A::_1
    }
}
///Field `TEND` writer - Transmit End Flag
pub type TEND_W<'a, REG> = crate::BitWriter0C<'a, REG, TEND_A>;
impl<'a, REG> TEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data is being transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEND_A::_0)
    }
    ///Data has been transmitted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEND_A::_1)
    }
}
/**Transmit Data Empty Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    ///0: ICDRT contains transmit data.
    _0 = 0,
    ///1: ICDRT contains no transmit data.
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDRE` reader - Transmit Data Empty Flag
pub type TDRE_R = crate::BitReader<TDRE_A>;
impl TDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    ///ICDRT contains transmit data.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    ///ICDRT contains no transmit data.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
impl R {
    ///Bit 0 - Timeout Detection Flag
    #[inline(always)]
    pub fn tmof(&self) -> TMOF_R {
        TMOF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Arbitration-Lost Flag
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Start Condition Detection Flag
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Condition Detection Flag
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACK Detection Flag
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit End Flag
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timeout Detection Flag
    #[inline(always)]
    pub fn tmof(&mut self) -> TMOF_W<ICSR2_SPEC> {
        TMOF_W::new(self, 0)
    }
    ///Bit 1 - Arbitration-Lost Flag
    #[inline(always)]
    pub fn al(&mut self) -> AL_W<ICSR2_SPEC> {
        AL_W::new(self, 1)
    }
    ///Bit 2 - Start Condition Detection Flag
    #[inline(always)]
    pub fn start(&mut self) -> START_W<ICSR2_SPEC> {
        START_W::new(self, 2)
    }
    ///Bit 3 - Stop Condition Detection Flag
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<ICSR2_SPEC> {
        STOP_W::new(self, 3)
    }
    ///Bit 4 - NACK Detection Flag
    #[inline(always)]
    pub fn nackf(&mut self) -> NACKF_W<ICSR2_SPEC> {
        NACKF_W::new(self, 4)
    }
    ///Bit 5 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&mut self) -> RDRF_W<ICSR2_SPEC> {
        RDRF_W::new(self, 5)
    }
    ///Bit 6 - Transmit End Flag
    #[inline(always)]
    pub fn tend(&mut self) -> TEND_W<ICSR2_SPEC> {
        TEND_W::new(self, 6)
    }
}
/**I2C Bus Status Register 2

You can [`read`](crate::Reg::read) this register and get [`icsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICSR2_SPEC;
impl crate::RegisterSpec for ICSR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icsr2::R`](R) reader structure
impl crate::Readable for ICSR2_SPEC {}
///`write(|w| ..)` method takes [`icsr2::W`](W) writer structure
impl crate::Writable for ICSR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x7f;
}
///`reset()` method sets ICSR2 to value 0
impl crate::Resettable for ICSR2_SPEC {}
