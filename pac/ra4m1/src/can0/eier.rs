///Register `EIER` reader
pub type R = crate::R<EIER_SPEC>;
///Register `EIER` writer
pub type W = crate::W<EIER_SPEC>;
/**Bus Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEIE_A {
    ///0: Bus error interrupt disabled
    _0 = 0,
    ///1: Bus error interrupt enabled
    _1 = 1,
}
impl From<BEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BEIE` reader - Bus Error Interrupt Enable
pub type BEIE_R = crate::BitReader<BEIE_A>;
impl BEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BEIE_A {
        match self.bits {
            false => BEIE_A::_0,
            true => BEIE_A::_1,
        }
    }
    ///Bus error interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEIE_A::_0
    }
    ///Bus error interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEIE_A::_1
    }
}
///Field `BEIE` writer - Bus Error Interrupt Enable
pub type BEIE_W<'a, REG> = crate::BitWriter<'a, REG, BEIE_A>;
impl<'a, REG> BEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus error interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BEIE_A::_0)
    }
    ///Bus error interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BEIE_A::_1)
    }
}
/**Error-Warning Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIE_A {
    ///0: Error-warning interrupt disabled
    _0 = 0,
    ///1: Error-warning interrupt enabled
    _1 = 1,
}
impl From<EWIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIE` reader - Error-Warning Interrupt Enable
pub type EWIE_R = crate::BitReader<EWIE_A>;
impl EWIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWIE_A {
        match self.bits {
            false => EWIE_A::_0,
            true => EWIE_A::_1,
        }
    }
    ///Error-warning interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWIE_A::_0
    }
    ///Error-warning interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWIE_A::_1
    }
}
///Field `EWIE` writer - Error-Warning Interrupt Enable
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG, EWIE_A>;
impl<'a, REG> EWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error-warning interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EWIE_A::_0)
    }
    ///Error-warning interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EWIE_A::_1)
    }
}
/**Error-Passive Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIE_A {
    ///0: Error-passive interrupt disabled
    _0 = 0,
    ///1: Error-passive interrupt enabled
    _1 = 1,
}
impl From<EPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EPIE` reader - Error-Passive Interrupt Enable
pub type EPIE_R = crate::BitReader<EPIE_A>;
impl EPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EPIE_A {
        match self.bits {
            false => EPIE_A::_0,
            true => EPIE_A::_1,
        }
    }
    ///Error-passive interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIE_A::_0
    }
    ///Error-passive interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIE_A::_1
    }
}
///Field `EPIE` writer - Error-Passive Interrupt Enable
pub type EPIE_W<'a, REG> = crate::BitWriter<'a, REG, EPIE_A>;
impl<'a, REG> EPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error-passive interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EPIE_A::_0)
    }
    ///Error-passive interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EPIE_A::_1)
    }
}
/**Bus-Off Entry Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOEIE_A {
    ///0: Bus-off entry interrupt disabled
    _0 = 0,
    ///1: Bus-off entry interrupt enabled
    _1 = 1,
}
impl From<BOEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BOEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BOEIE` reader - Bus-Off Entry Interrupt Enable
pub type BOEIE_R = crate::BitReader<BOEIE_A>;
impl BOEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOEIE_A {
        match self.bits {
            false => BOEIE_A::_0,
            true => BOEIE_A::_1,
        }
    }
    ///Bus-off entry interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOEIE_A::_0
    }
    ///Bus-off entry interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOEIE_A::_1
    }
}
///Field `BOEIE` writer - Bus-Off Entry Interrupt Enable
pub type BOEIE_W<'a, REG> = crate::BitWriter<'a, REG, BOEIE_A>;
impl<'a, REG> BOEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus-off entry interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BOEIE_A::_0)
    }
    ///Bus-off entry interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BOEIE_A::_1)
    }
}
/**Bus-Off Recovery Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORIE_A {
    ///0: Bus-off recovery interrupt disabled
    _0 = 0,
    ///1: Bus-off recovery interrupt enabled
    _1 = 1,
}
impl From<BORIE_A> for bool {
    #[inline(always)]
    fn from(variant: BORIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BORIE` reader - Bus-Off Recovery Interrupt Enable
pub type BORIE_R = crate::BitReader<BORIE_A>;
impl BORIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORIE_A {
        match self.bits {
            false => BORIE_A::_0,
            true => BORIE_A::_1,
        }
    }
    ///Bus-off recovery interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORIE_A::_0
    }
    ///Bus-off recovery interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORIE_A::_1
    }
}
///Field `BORIE` writer - Bus-Off Recovery Interrupt Enable
pub type BORIE_W<'a, REG> = crate::BitWriter<'a, REG, BORIE_A>;
impl<'a, REG> BORIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus-off recovery interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BORIE_A::_0)
    }
    ///Bus-off recovery interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BORIE_A::_1)
    }
}
/**Overrun Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORIE_A {
    ///0: Receive overrun interrupt disabled
    _0 = 0,
    ///1: Receive overrun interrupt enabled
    _1 = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ORIE` reader - Overrun Interrupt Enable
pub type ORIE_R = crate::BitReader<ORIE_A>;
impl ORIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::_0,
            true => ORIE_A::_1,
        }
    }
    ///Receive overrun interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORIE_A::_0
    }
    ///Receive overrun interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORIE_A::_1
    }
}
///Field `ORIE` writer - Overrun Interrupt Enable
pub type ORIE_W<'a, REG> = crate::BitWriter<'a, REG, ORIE_A>;
impl<'a, REG> ORIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive overrun interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ORIE_A::_0)
    }
    ///Receive overrun interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ORIE_A::_1)
    }
}
/**Overload Frame Transmit Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OLIE_A {
    ///0: Overload frame transmit interrupt disabled
    _0 = 0,
    ///1: Overload frame transmit interrupt enabled
    _1 = 1,
}
impl From<OLIE_A> for bool {
    #[inline(always)]
    fn from(variant: OLIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OLIE` reader - Overload Frame Transmit Interrupt Enable
pub type OLIE_R = crate::BitReader<OLIE_A>;
impl OLIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OLIE_A {
        match self.bits {
            false => OLIE_A::_0,
            true => OLIE_A::_1,
        }
    }
    ///Overload frame transmit interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OLIE_A::_0
    }
    ///Overload frame transmit interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OLIE_A::_1
    }
}
///Field `OLIE` writer - Overload Frame Transmit Interrupt Enable
pub type OLIE_W<'a, REG> = crate::BitWriter<'a, REG, OLIE_A>;
impl<'a, REG> OLIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overload frame transmit interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OLIE_A::_0)
    }
    ///Overload frame transmit interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OLIE_A::_1)
    }
}
/**Bus Lock Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLIE_A {
    ///0: Bus lock interrupt disabled
    _0 = 0,
    ///1: Bus lock interrupt enabled
    _1 = 1,
}
impl From<BLIE_A> for bool {
    #[inline(always)]
    fn from(variant: BLIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BLIE` reader - Bus Lock Interrupt Enable
pub type BLIE_R = crate::BitReader<BLIE_A>;
impl BLIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BLIE_A {
        match self.bits {
            false => BLIE_A::_0,
            true => BLIE_A::_1,
        }
    }
    ///Bus lock interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLIE_A::_0
    }
    ///Bus lock interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLIE_A::_1
    }
}
///Field `BLIE` writer - Bus Lock Interrupt Enable
pub type BLIE_W<'a, REG> = crate::BitWriter<'a, REG, BLIE_A>;
impl<'a, REG> BLIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus lock interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BLIE_A::_0)
    }
    ///Bus lock interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BLIE_A::_1)
    }
}
impl R {
    ///Bit 0 - Bus Error Interrupt Enable
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Error-Warning Interrupt Enable
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error-Passive Interrupt Enable
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bus-Off Entry Interrupt Enable
    #[inline(always)]
    pub fn boeie(&self) -> BOEIE_R {
        BOEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bus-Off Recovery Interrupt Enable
    #[inline(always)]
    pub fn borie(&self) -> BORIE_R {
        BORIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overload Frame Transmit Interrupt Enable
    #[inline(always)]
    pub fn olie(&self) -> OLIE_R {
        OLIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bus Lock Interrupt Enable
    #[inline(always)]
    pub fn blie(&self) -> BLIE_R {
        BLIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Bus Error Interrupt Enable
    #[inline(always)]
    pub fn beie(&mut self) -> BEIE_W<EIER_SPEC> {
        BEIE_W::new(self, 0)
    }
    ///Bit 1 - Error-Warning Interrupt Enable
    #[inline(always)]
    pub fn ewie(&mut self) -> EWIE_W<EIER_SPEC> {
        EWIE_W::new(self, 1)
    }
    ///Bit 2 - Error-Passive Interrupt Enable
    #[inline(always)]
    pub fn epie(&mut self) -> EPIE_W<EIER_SPEC> {
        EPIE_W::new(self, 2)
    }
    ///Bit 3 - Bus-Off Entry Interrupt Enable
    #[inline(always)]
    pub fn boeie(&mut self) -> BOEIE_W<EIER_SPEC> {
        BOEIE_W::new(self, 3)
    }
    ///Bit 4 - Bus-Off Recovery Interrupt Enable
    #[inline(always)]
    pub fn borie(&mut self) -> BORIE_W<EIER_SPEC> {
        BORIE_W::new(self, 4)
    }
    ///Bit 5 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn orie(&mut self) -> ORIE_W<EIER_SPEC> {
        ORIE_W::new(self, 5)
    }
    ///Bit 6 - Overload Frame Transmit Interrupt Enable
    #[inline(always)]
    pub fn olie(&mut self) -> OLIE_W<EIER_SPEC> {
        OLIE_W::new(self, 6)
    }
    ///Bit 7 - Bus Lock Interrupt Enable
    #[inline(always)]
    pub fn blie(&mut self) -> BLIE_W<EIER_SPEC> {
        BLIE_W::new(self, 7)
    }
}
/**Error Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`eier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EIER_SPEC;
impl crate::RegisterSpec for EIER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eier::R`](R) reader structure
impl crate::Readable for EIER_SPEC {}
///`write(|w| ..)` method takes [`eier::W`](W) writer structure
impl crate::Writable for EIER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EIER to value 0
impl crate::Resettable for EIER_SPEC {}
