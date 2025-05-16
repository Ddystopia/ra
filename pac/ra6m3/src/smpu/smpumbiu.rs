///Register `SMPUMBIU` reader
pub type R = crate::R<SMPUMBIU_SPEC>;
///Register `SMPUMBIU` writer
pub type W = crate::W<SMPUMBIU_SPEC>;
/**Master Group A Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPA_A {
    ///0: Memory protection for master group A reads disabled
    _0 = 0,
    ///1: Memory protection for master group A reads enabled.
    _1 = 1,
}
impl From<RPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPGRPA` reader - Master Group A Read protection
pub type RPGRPA_R = crate::BitReader<RPGRPA_A>;
impl RPGRPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPGRPA_A {
        match self.bits {
            false => RPGRPA_A::_0,
            true => RPGRPA_A::_1,
        }
    }
    ///Memory protection for master group A reads disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPA_A::_0
    }
    ///Memory protection for master group A reads enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPA_A::_1
    }
}
///Field `RPGRPA` writer - Master Group A Read protection
pub type RPGRPA_W<'a, REG> = crate::BitWriter<'a, REG, RPGRPA_A>;
impl<'a, REG> RPGRPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for master group A reads disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPA_A::_0)
    }
    ///Memory protection for master group A reads enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPA_A::_1)
    }
}
/**Master Group A Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPA_A {
    ///0: Memory protection for master group A writes disabled
    _0 = 0,
    ///1: Memory protection for master group A writes enabled.
    _1 = 1,
}
impl From<WPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPGRPA` reader - Master Group A Write protection
pub type WPGRPA_R = crate::BitReader<WPGRPA_A>;
impl WPGRPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPGRPA_A {
        match self.bits {
            false => WPGRPA_A::_0,
            true => WPGRPA_A::_1,
        }
    }
    ///Memory protection for master group A writes disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPA_A::_0
    }
    ///Memory protection for master group A writes enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPA_A::_1
    }
}
///Field `WPGRPA` writer - Master Group A Write protection
pub type WPGRPA_W<'a, REG> = crate::BitWriter<'a, REG, WPGRPA_A>;
impl<'a, REG> WPGRPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for master group A writes disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPA_A::_0)
    }
    ///Memory protection for master group A writes enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPA_A::_1)
    }
}
/**Master Group B Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPB_A {
    ///0: Memory protection for master group B reads disabled
    _0 = 0,
    ///1: Memory protection for master group B reads enabled.
    _1 = 1,
}
impl From<RPGRPB_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPGRPB` reader - Master Group B Read protection
pub type RPGRPB_R = crate::BitReader<RPGRPB_A>;
impl RPGRPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPGRPB_A {
        match self.bits {
            false => RPGRPB_A::_0,
            true => RPGRPB_A::_1,
        }
    }
    ///Memory protection for master group B reads disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPB_A::_0
    }
    ///Memory protection for master group B reads enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPB_A::_1
    }
}
///Field `RPGRPB` writer - Master Group B Read protection
pub type RPGRPB_W<'a, REG> = crate::BitWriter<'a, REG, RPGRPB_A>;
impl<'a, REG> RPGRPB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for master group B reads disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPB_A::_0)
    }
    ///Memory protection for master group B reads enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPB_A::_1)
    }
}
/**Master Group B Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPB_A {
    ///0: Memory protection for master group B writes disabled
    _0 = 0,
    ///1: Memory protection for master group B writes enabled.
    _1 = 1,
}
impl From<WPGRPB_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPGRPB` reader - Master Group B Write protection
pub type WPGRPB_R = crate::BitReader<WPGRPB_A>;
impl WPGRPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPGRPB_A {
        match self.bits {
            false => WPGRPB_A::_0,
            true => WPGRPB_A::_1,
        }
    }
    ///Memory protection for master group B writes disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPB_A::_0
    }
    ///Memory protection for master group B writes enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPB_A::_1
    }
}
///Field `WPGRPB` writer - Master Group B Write protection
pub type WPGRPB_W<'a, REG> = crate::BitWriter<'a, REG, WPGRPB_A>;
impl<'a, REG> WPGRPB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for master group B writes disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPB_A::_0)
    }
    ///Memory protection for master group B writes enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPB_A::_1)
    }
}
/**Master Group C Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPC_A {
    ///0: Memory protection for master group C reads disabled
    _0 = 0,
    ///1: Memory protection for master group C reads enabled.
    _1 = 1,
}
impl From<RPGRPC_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPGRPC` reader - Master Group C Read protection
pub type RPGRPC_R = crate::BitReader<RPGRPC_A>;
impl RPGRPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPGRPC_A {
        match self.bits {
            false => RPGRPC_A::_0,
            true => RPGRPC_A::_1,
        }
    }
    ///Memory protection for master group C reads disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPC_A::_0
    }
    ///Memory protection for master group C reads enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPC_A::_1
    }
}
///Field `RPGRPC` writer - Master Group C Read protection
pub type RPGRPC_W<'a, REG> = crate::BitWriter<'a, REG, RPGRPC_A>;
impl<'a, REG> RPGRPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for master group C reads disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPC_A::_0)
    }
    ///Memory protection for master group C reads enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPC_A::_1)
    }
}
/**Master Group C Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPC_A {
    ///0: Memory protection for master group C writes disabled
    _0 = 0,
    ///1: Memory protection for master group C writes enabled.
    _1 = 1,
}
impl From<WPGRPC_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPGRPC` reader - Master Group C Write protection
pub type WPGRPC_R = crate::BitReader<WPGRPC_A>;
impl WPGRPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPGRPC_A {
        match self.bits {
            false => WPGRPC_A::_0,
            true => WPGRPC_A::_1,
        }
    }
    ///Memory protection for master group C writes disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPC_A::_0
    }
    ///Memory protection for master group C writes enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPC_A::_1
    }
}
///Field `WPGRPC` writer - Master Group C Write protection
pub type WPGRPC_W<'a, REG> = crate::BitWriter<'a, REG, WPGRPC_A>;
impl<'a, REG> WPGRPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for master group C writes disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPC_A::_0)
    }
    ///Memory protection for master group C writes enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPC_A::_1)
    }
}
/**Code Flash Memory Read Protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPFLI_A {
    ///0: Memory protection for code flash memory reads from master group A, B, and C disabled
    _0 = 0,
    ///1: Memory protection for code flash memory reads from master group A, B, and C enabled.
    _1 = 1,
}
impl From<RPFLI_A> for bool {
    #[inline(always)]
    fn from(variant: RPFLI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPFLI` reader - Code Flash Memory Read Protection
pub type RPFLI_R = crate::BitReader<RPFLI_A>;
impl RPFLI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPFLI_A {
        match self.bits {
            false => RPFLI_A::_0,
            true => RPFLI_A::_1,
        }
    }
    ///Memory protection for code flash memory reads from master group A, B, and C disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPFLI_A::_0
    }
    ///Memory protection for code flash memory reads from master group A, B, and C enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPFLI_A::_1
    }
}
///Field `RPFLI` writer - Code Flash Memory Read Protection
pub type RPFLI_W<'a, REG> = crate::BitWriter<'a, REG, RPFLI_A>;
impl<'a, REG> RPFLI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for code flash memory reads from master group A, B, and C disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPFLI_A::_0)
    }
    ///Memory protection for code flash memory reads from master group A, B, and C enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPFLI_A::_1)
    }
}
/**Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPFLI_A {
    ///0: Setting prohibited
    _0 = 0,
    ///1: Memory protection for code flash memory writes from master group A, B, and C enabled.
    _1 = 1,
}
impl From<WPFLI_A> for bool {
    #[inline(always)]
    fn from(variant: WPFLI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPFLI` reader - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)
pub type WPFLI_R = crate::BitReader<WPFLI_A>;
impl WPFLI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPFLI_A {
        match self.bits {
            false => WPFLI_A::_0,
            true => WPFLI_A::_1,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPFLI_A::_0
    }
    ///Memory protection for code flash memory writes from master group A, B, and C enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPFLI_A::_1
    }
}
///Field `WPFLI` writer - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)
pub type WPFLI_W<'a, REG> = crate::BitWriter<'a, REG, WPFLI_A>;
impl<'a, REG> WPFLI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPFLI_A::_0)
    }
    ///Memory protection for code flash memory writes from master group A, B, and C enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPFLI_A::_1)
    }
}
/**SRAMHS Read Protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPSRAMHS_A {
    ///0: Memory protection for SRAMHS reads from master group A, B, and C disabled
    _0 = 0,
    ///1: Memory protection for SRAMHS reads from master group A, B, and C enabled.
    _1 = 1,
}
impl From<RPSRAMHS_A> for bool {
    #[inline(always)]
    fn from(variant: RPSRAMHS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPSRAMHS` reader - SRAMHS Read Protection
pub type RPSRAMHS_R = crate::BitReader<RPSRAMHS_A>;
impl RPSRAMHS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPSRAMHS_A {
        match self.bits {
            false => RPSRAMHS_A::_0,
            true => RPSRAMHS_A::_1,
        }
    }
    ///Memory protection for SRAMHS reads from master group A, B, and C disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPSRAMHS_A::_0
    }
    ///Memory protection for SRAMHS reads from master group A, B, and C enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPSRAMHS_A::_1
    }
}
///Field `RPSRAMHS` writer - SRAMHS Read Protection
pub type RPSRAMHS_W<'a, REG> = crate::BitWriter<'a, REG, RPSRAMHS_A>;
impl<'a, REG> RPSRAMHS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for SRAMHS reads from master group A, B, and C disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPSRAMHS_A::_0)
    }
    ///Memory protection for SRAMHS reads from master group A, B, and C enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPSRAMHS_A::_1)
    }
}
/**SRAMHS Write Protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSRAMHS_A {
    ///0: Memory protection for SRAMHS writes from master group A, B, and C disabled
    _0 = 0,
    ///1: Memory protection for SRAMHS writes from master group A, B, and C enabled.
    _1 = 1,
}
impl From<WPSRAMHS_A> for bool {
    #[inline(always)]
    fn from(variant: WPSRAMHS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPSRAMHS` reader - SRAMHS Write Protection
pub type WPSRAMHS_R = crate::BitReader<WPSRAMHS_A>;
impl WPSRAMHS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPSRAMHS_A {
        match self.bits {
            false => WPSRAMHS_A::_0,
            true => WPSRAMHS_A::_1,
        }
    }
    ///Memory protection for SRAMHS writes from master group A, B, and C disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPSRAMHS_A::_0
    }
    ///Memory protection for SRAMHS writes from master group A, B, and C enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPSRAMHS_A::_1
    }
}
///Field `WPSRAMHS` writer - SRAMHS Write Protection
pub type WPSRAMHS_W<'a, REG> = crate::BitWriter<'a, REG, WPSRAMHS_A>;
impl<'a, REG> WPSRAMHS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory protection for SRAMHS writes from master group A, B, and C disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPSRAMHS_A::_0)
    }
    ///Memory protection for SRAMHS writes from master group A, B, and C enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPSRAMHS_A::_1)
    }
}
impl R {
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rpgrpa(&self) -> RPGRPA_R {
        RPGRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wpgrpa(&self) -> WPGRPA_R {
        WPGRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Group B Read protection
    #[inline(always)]
    pub fn rpgrpb(&self) -> RPGRPB_R {
        RPGRPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master Group B Write protection
    #[inline(always)]
    pub fn wpgrpb(&self) -> WPGRPB_R {
        WPGRPB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master Group C Read protection
    #[inline(always)]
    pub fn rpgrpc(&self) -> RPGRPC_R {
        RPGRPC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master Group C Write protection
    #[inline(always)]
    pub fn wpgrpc(&self) -> WPGRPC_R {
        WPGRPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - Code Flash Memory Read Protection
    #[inline(always)]
    pub fn rpfli(&self) -> RPFLI_R {
        RPFLI_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)
    #[inline(always)]
    pub fn wpfli(&self) -> WPFLI_R {
        WPFLI_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAMHS Read Protection
    #[inline(always)]
    pub fn rpsramhs(&self) -> RPSRAMHS_R {
        RPSRAMHS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAMHS Write Protection
    #[inline(always)]
    pub fn wpsramhs(&self) -> WPSRAMHS_R {
        WPSRAMHS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<SMPUMBIU_SPEC> {
        RPGRPA_W::new(self, 2)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<SMPUMBIU_SPEC> {
        WPGRPA_W::new(self, 3)
    }
    ///Bit 4 - Master Group B Read protection
    #[inline(always)]
    pub fn rpgrpb(&mut self) -> RPGRPB_W<SMPUMBIU_SPEC> {
        RPGRPB_W::new(self, 4)
    }
    ///Bit 5 - Master Group B Write protection
    #[inline(always)]
    pub fn wpgrpb(&mut self) -> WPGRPB_W<SMPUMBIU_SPEC> {
        WPGRPB_W::new(self, 5)
    }
    ///Bit 6 - Master Group C Read protection
    #[inline(always)]
    pub fn rpgrpc(&mut self) -> RPGRPC_W<SMPUMBIU_SPEC> {
        RPGRPC_W::new(self, 6)
    }
    ///Bit 7 - Master Group C Write protection
    #[inline(always)]
    pub fn wpgrpc(&mut self) -> WPGRPC_W<SMPUMBIU_SPEC> {
        WPGRPC_W::new(self, 7)
    }
    ///Bit 12 - Code Flash Memory Read Protection
    #[inline(always)]
    pub fn rpfli(&mut self) -> RPFLI_W<SMPUMBIU_SPEC> {
        RPFLI_W::new(self, 12)
    }
    ///Bit 13 - Code Flash Memory Write Protection (Note: This bit is read as 1. The write value should be 1.)
    #[inline(always)]
    pub fn wpfli(&mut self) -> WPFLI_W<SMPUMBIU_SPEC> {
        WPFLI_W::new(self, 13)
    }
    ///Bit 14 - SRAMHS Read Protection
    #[inline(always)]
    pub fn rpsramhs(&mut self) -> RPSRAMHS_W<SMPUMBIU_SPEC> {
        RPSRAMHS_W::new(self, 14)
    }
    ///Bit 15 - SRAMHS Write Protection
    #[inline(always)]
    pub fn wpsramhs(&mut self) -> WPSRAMHS_W<SMPUMBIU_SPEC> {
        WPSRAMHS_W::new(self, 15)
    }
}
/**Access Control Register for MBIU

You can [`read`](crate::Reg::read) this register and get [`smpumbiu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpumbiu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMPUMBIU_SPEC;
impl crate::RegisterSpec for SMPUMBIU_SPEC {
    type Ux = u16;
}
///`read()` method returns [`smpumbiu::R`](R) reader structure
impl crate::Readable for SMPUMBIU_SPEC {}
///`write(|w| ..)` method takes [`smpumbiu::W`](W) writer structure
impl crate::Writable for SMPUMBIU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPUMBIU to value 0x2000
impl crate::Resettable for SMPUMBIU_SPEC {
    const RESET_VALUE: u16 = 0x2000;
}
