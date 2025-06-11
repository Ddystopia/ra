///Register `AMPTRM` reader
pub type R = crate::R<AMPTRM_SPEC>;
///Register `AMPTRM` writer
pub type W = crate::W<AMPTRM_SPEC>;
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM00_A {
    ///0: Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1).
    _0 = 0,
    ///1: An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1).
    _1 = 1,
}
impl From<AMPTRM00_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM00` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM00_R = crate::BitReader<AMPTRM00_A>;
impl AMPTRM00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM00_A {
        match self.bits {
            false => AMPTRM00_A::_0,
            true => AMPTRM00_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM00_A::_0
    }
    ///An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM00_A::_1
    }
}
///Field `AMPTRM00` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM00_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM00_A>;
impl<'a, REG> AMPTRM00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM00_A::_0)
    }
    ///An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM00_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM01_A {
    ///0: Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1).
    _0 = 0,
    ///1: Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1).
    _1 = 1,
}
impl From<AMPTRM01_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM01` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM01_R = crate::BitReader<AMPTRM01_A>;
impl AMPTRM01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM01_A {
        match self.bits {
            false => AMPTRM01_A::_0,
            true => AMPTRM01_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM01_A::_0
    }
    ///Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM01_A::_1
    }
}
///Field `AMPTRM01` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM01_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM01_A>;
impl<'a, REG> AMPTRM01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM01_A::_0)
    }
    ///Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM01_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM10_A {
    ///0: Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1).
    _0 = 0,
    ///1: An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1).
    _1 = 1,
}
impl From<AMPTRM10_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM10` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM10_R = crate::BitReader<AMPTRM10_A>;
impl AMPTRM10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM10_A {
        match self.bits {
            false => AMPTRM10_A::_0,
            true => AMPTRM10_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM10_A::_0
    }
    ///An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM10_A::_1
    }
}
///Field `AMPTRM10` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM10_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM10_A>;
impl<'a, REG> AMPTRM10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM10_A::_0)
    }
    ///An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM10_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM11_A {
    ///0: Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1).
    _0 = 0,
    ///1: Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1).
    _1 = 1,
}
impl From<AMPTRM11_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM11` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM11_R = crate::BitReader<AMPTRM11_A>;
impl AMPTRM11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM11_A {
        match self.bits {
            false => AMPTRM11_A::_0,
            true => AMPTRM11_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM11_A::_0
    }
    ///Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM11_A::_1
    }
}
///Field `AMPTRM11` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM11_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM11_A>;
impl<'a, REG> AMPTRM11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM11_A::_0)
    }
    ///Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM11_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM20_A {
    ///0: Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1).
    _0 = 0,
    ///1: An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1).
    _1 = 1,
}
impl From<AMPTRM20_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM20` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM20_R = crate::BitReader<AMPTRM20_A>;
impl AMPTRM20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM20_A {
        match self.bits {
            false => AMPTRM20_A::_0,
            true => AMPTRM20_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM20_A::_0
    }
    ///An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM20_A::_1
    }
}
///Field `AMPTRM20` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM20_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM20_A>;
impl<'a, REG> AMPTRM20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM20_A::_0)
    }
    ///An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM20_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM21_A {
    ///0: Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1).
    _0 = 0,
    ///1: Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1).
    _1 = 1,
}
impl From<AMPTRM21_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM21_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM21` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM21_R = crate::BitReader<AMPTRM21_A>;
impl AMPTRM21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM21_A {
        match self.bits {
            false => AMPTRM21_A::_0,
            true => AMPTRM21_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM21_A::_0
    }
    ///Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM21_A::_1
    }
}
///Field `AMPTRM21` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM21_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM21_A>;
impl<'a, REG> AMPTRM21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM21_A::_0)
    }
    ///Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM21_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM30_A {
    ///0: Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1).
    _0 = 0,
    ///1: An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1).
    _1 = 1,
}
impl From<AMPTRM30_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM30_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM30` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM30_R = crate::BitReader<AMPTRM30_A>;
impl AMPTRM30_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM30_A {
        match self.bits {
            false => AMPTRM30_A::_0,
            true => AMPTRM30_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM30_A::_0
    }
    ///An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM30_A::_1
    }
}
///Field `AMPTRM30` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM30_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM30_A>;
impl<'a, REG> AMPTRM30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM30_A::_0)
    }
    ///An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM30_A::_1)
    }
}
/**Operational amplifier function activation/stop trigger control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM31_A {
    ///0: Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1).
    _0 = 0,
    ///1: Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1).
    _1 = 1,
}
impl From<AMPTRM31_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AMPTRM31` reader - Operational amplifier function activation/stop trigger control
pub type AMPTRM31_R = crate::BitReader<AMPTRM31_A>;
impl AMPTRM31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRM31_A {
        match self.bits {
            false => AMPTRM31_A::_0,
            true => AMPTRM31_A::_1,
        }
    }
    ///Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM31_A::_0
    }
    ///Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM31_A::_1
    }
}
///Field `AMPTRM31` writer - Operational amplifier function activation/stop trigger control
pub type AMPTRM31_W<'a, REG> = crate::BitWriter<'a, REG, AMPTRM31_A>;
impl<'a, REG> AMPTRM31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM31_A::_0)
    }
    ///Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRM31_A::_1)
    }
}
impl R {
    ///Bit 0 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm00(&self) -> AMPTRM00_R {
        AMPTRM00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm01(&self) -> AMPTRM01_R {
        AMPTRM01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm10(&self) -> AMPTRM10_R {
        AMPTRM10_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm11(&self) -> AMPTRM11_R {
        AMPTRM11_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm20(&self) -> AMPTRM20_R {
        AMPTRM20_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm21(&self) -> AMPTRM21_R {
        AMPTRM21_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm30(&self) -> AMPTRM30_R {
        AMPTRM30_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm31(&self) -> AMPTRM31_R {
        AMPTRM31_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm00(&mut self) -> AMPTRM00_W<AMPTRM_SPEC> {
        AMPTRM00_W::new(self, 0)
    }
    ///Bit 1 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm01(&mut self) -> AMPTRM01_W<AMPTRM_SPEC> {
        AMPTRM01_W::new(self, 1)
    }
    ///Bit 2 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm10(&mut self) -> AMPTRM10_W<AMPTRM_SPEC> {
        AMPTRM10_W::new(self, 2)
    }
    ///Bit 3 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm11(&mut self) -> AMPTRM11_W<AMPTRM_SPEC> {
        AMPTRM11_W::new(self, 3)
    }
    ///Bit 4 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm20(&mut self) -> AMPTRM20_W<AMPTRM_SPEC> {
        AMPTRM20_W::new(self, 4)
    }
    ///Bit 5 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm21(&mut self) -> AMPTRM21_W<AMPTRM_SPEC> {
        AMPTRM21_W::new(self, 5)
    }
    ///Bit 6 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm30(&mut self) -> AMPTRM30_W<AMPTRM_SPEC> {
        AMPTRM30_W::new(self, 6)
    }
    ///Bit 7 - Operational amplifier function activation/stop trigger control
    #[inline(always)]
    pub fn amptrm31(&mut self) -> AMPTRM31_W<AMPTRM_SPEC> {
        AMPTRM31_W::new(self, 7)
    }
}
/**Operational amplifier trigger mode control register

You can [`read`](crate::Reg::read) this register and get [`amptrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AMPTRM_SPEC;
impl crate::RegisterSpec for AMPTRM_SPEC {
    type Ux = u8;
}
///`read()` method returns [`amptrm::R`](R) reader structure
impl crate::Readable for AMPTRM_SPEC {}
///`write(|w| ..)` method takes [`amptrm::W`](W) writer structure
impl crate::Writable for AMPTRM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AMPTRM to value 0
impl crate::Resettable for AMPTRM_SPEC {}
