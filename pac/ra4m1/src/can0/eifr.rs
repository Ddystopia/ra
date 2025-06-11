///Register `EIFR` reader
pub type R = crate::R<EIFR_SPEC>;
///Register `EIFR` writer
pub type W = crate::W<EIFR_SPEC>;
/**Bus Error Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEIF_A {
    ///0: No bus error detected
    _0 = 0,
    ///1: Bus error detected
    _1 = 1,
}
impl From<BEIF_A> for bool {
    #[inline(always)]
    fn from(variant: BEIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BEIF` reader - Bus Error Detect Flag
pub type BEIF_R = crate::BitReader<BEIF_A>;
impl BEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BEIF_A {
        match self.bits {
            false => BEIF_A::_0,
            true => BEIF_A::_1,
        }
    }
    ///No bus error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEIF_A::_0
    }
    ///Bus error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEIF_A::_1
    }
}
///Field `BEIF` writer - Bus Error Detect Flag
pub type BEIF_W<'a, REG> = crate::BitWriter<'a, REG, BEIF_A>;
impl<'a, REG> BEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bus error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BEIF_A::_0)
    }
    ///Bus error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BEIF_A::_1)
    }
}
/**Error-Warning Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIF_A {
    ///0: No error-warning detected
    _0 = 0,
    ///1: Error-warning detected
    _1 = 1,
}
impl From<EWIF_A> for bool {
    #[inline(always)]
    fn from(variant: EWIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIF` reader - Error-Warning Detect Flag
pub type EWIF_R = crate::BitReader<EWIF_A>;
impl EWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWIF_A {
        match self.bits {
            false => EWIF_A::_0,
            true => EWIF_A::_1,
        }
    }
    ///No error-warning detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWIF_A::_0
    }
    ///Error-warning detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWIF_A::_1
    }
}
///Field `EWIF` writer - Error-Warning Detect Flag
pub type EWIF_W<'a, REG> = crate::BitWriter<'a, REG, EWIF_A>;
impl<'a, REG> EWIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error-warning detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EWIF_A::_0)
    }
    ///Error-warning detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EWIF_A::_1)
    }
}
/**Error-Passive Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIF_A {
    ///0: No error-passive detected
    _0 = 0,
    ///1: Error-passive detected
    _1 = 1,
}
impl From<EPIF_A> for bool {
    #[inline(always)]
    fn from(variant: EPIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EPIF` reader - Error-Passive Detect Flag
pub type EPIF_R = crate::BitReader<EPIF_A>;
impl EPIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EPIF_A {
        match self.bits {
            false => EPIF_A::_0,
            true => EPIF_A::_1,
        }
    }
    ///No error-passive detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIF_A::_0
    }
    ///Error-passive detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIF_A::_1
    }
}
///Field `EPIF` writer - Error-Passive Detect Flag
pub type EPIF_W<'a, REG> = crate::BitWriter<'a, REG, EPIF_A>;
impl<'a, REG> EPIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error-passive detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EPIF_A::_0)
    }
    ///Error-passive detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EPIF_A::_1)
    }
}
/**Bus-Off Entry Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOEIF_A {
    ///0: No bus-off entry detected
    _0 = 0,
    ///1: Bus-off entry detected
    _1 = 1,
}
impl From<BOEIF_A> for bool {
    #[inline(always)]
    fn from(variant: BOEIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BOEIF` reader - Bus-Off Entry Detect Flag
pub type BOEIF_R = crate::BitReader<BOEIF_A>;
impl BOEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOEIF_A {
        match self.bits {
            false => BOEIF_A::_0,
            true => BOEIF_A::_1,
        }
    }
    ///No bus-off entry detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOEIF_A::_0
    }
    ///Bus-off entry detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOEIF_A::_1
    }
}
///Field `BOEIF` writer - Bus-Off Entry Detect Flag
pub type BOEIF_W<'a, REG> = crate::BitWriter<'a, REG, BOEIF_A>;
impl<'a, REG> BOEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bus-off entry detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BOEIF_A::_0)
    }
    ///Bus-off entry detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BOEIF_A::_1)
    }
}
/**Bus-Off Recovery Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORIF_A {
    ///0: No bus-off recovery detected
    _0 = 0,
    ///1: Bus-off recovery detected
    _1 = 1,
}
impl From<BORIF_A> for bool {
    #[inline(always)]
    fn from(variant: BORIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BORIF` reader - Bus-Off Recovery Detect Flag
pub type BORIF_R = crate::BitReader<BORIF_A>;
impl BORIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BORIF_A {
        match self.bits {
            false => BORIF_A::_0,
            true => BORIF_A::_1,
        }
    }
    ///No bus-off recovery detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORIF_A::_0
    }
    ///Bus-off recovery detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORIF_A::_1
    }
}
///Field `BORIF` writer - Bus-Off Recovery Detect Flag
pub type BORIF_W<'a, REG> = crate::BitWriter<'a, REG, BORIF_A>;
impl<'a, REG> BORIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bus-off recovery detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BORIF_A::_0)
    }
    ///Bus-off recovery detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BORIF_A::_1)
    }
}
/**Receive Overrun Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORIF_A {
    ///0: No receive overrun detected
    _0 = 0,
    ///1: Receive overrun detected
    _1 = 1,
}
impl From<ORIF_A> for bool {
    #[inline(always)]
    fn from(variant: ORIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ORIF` reader - Receive Overrun Detect Flag
pub type ORIF_R = crate::BitReader<ORIF_A>;
impl ORIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORIF_A {
        match self.bits {
            false => ORIF_A::_0,
            true => ORIF_A::_1,
        }
    }
    ///No receive overrun detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORIF_A::_0
    }
    ///Receive overrun detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORIF_A::_1
    }
}
///Field `ORIF` writer - Receive Overrun Detect Flag
pub type ORIF_W<'a, REG> = crate::BitWriter<'a, REG, ORIF_A>;
impl<'a, REG> ORIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No receive overrun detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ORIF_A::_0)
    }
    ///Receive overrun detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ORIF_A::_1)
    }
}
/**Overload Frame Transmission Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OLIF_A {
    ///0: No overload frame transmission detected
    _0 = 0,
    ///1: Overload frame transmission detected
    _1 = 1,
}
impl From<OLIF_A> for bool {
    #[inline(always)]
    fn from(variant: OLIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OLIF` reader - Overload Frame Transmission Detect Flag
pub type OLIF_R = crate::BitReader<OLIF_A>;
impl OLIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OLIF_A {
        match self.bits {
            false => OLIF_A::_0,
            true => OLIF_A::_1,
        }
    }
    ///No overload frame transmission detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OLIF_A::_0
    }
    ///Overload frame transmission detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OLIF_A::_1
    }
}
///Field `OLIF` writer - Overload Frame Transmission Detect Flag
pub type OLIF_W<'a, REG> = crate::BitWriter<'a, REG, OLIF_A>;
impl<'a, REG> OLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overload frame transmission detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OLIF_A::_0)
    }
    ///Overload frame transmission detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OLIF_A::_1)
    }
}
/**Bus Lock Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLIF_A {
    ///0: No bus lock detected
    _0 = 0,
    ///1: Bus lock detected
    _1 = 1,
}
impl From<BLIF_A> for bool {
    #[inline(always)]
    fn from(variant: BLIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BLIF` reader - Bus Lock Detect Flag
pub type BLIF_R = crate::BitReader<BLIF_A>;
impl BLIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BLIF_A {
        match self.bits {
            false => BLIF_A::_0,
            true => BLIF_A::_1,
        }
    }
    ///No bus lock detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLIF_A::_0
    }
    ///Bus lock detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLIF_A::_1
    }
}
///Field `BLIF` writer - Bus Lock Detect Flag
pub type BLIF_W<'a, REG> = crate::BitWriter<'a, REG, BLIF_A>;
impl<'a, REG> BLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bus lock detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BLIF_A::_0)
    }
    ///Bus lock detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BLIF_A::_1)
    }
}
impl R {
    ///Bit 0 - Bus Error Detect Flag
    #[inline(always)]
    pub fn beif(&self) -> BEIF_R {
        BEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Error-Warning Detect Flag
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error-Passive Detect Flag
    #[inline(always)]
    pub fn epif(&self) -> EPIF_R {
        EPIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bus-Off Entry Detect Flag
    #[inline(always)]
    pub fn boeif(&self) -> BOEIF_R {
        BOEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bus-Off Recovery Detect Flag
    #[inline(always)]
    pub fn borif(&self) -> BORIF_R {
        BORIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Overrun Detect Flag
    #[inline(always)]
    pub fn orif(&self) -> ORIF_R {
        ORIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overload Frame Transmission Detect Flag
    #[inline(always)]
    pub fn olif(&self) -> OLIF_R {
        OLIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bus Lock Detect Flag
    #[inline(always)]
    pub fn blif(&self) -> BLIF_R {
        BLIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Bus Error Detect Flag
    #[inline(always)]
    pub fn beif(&mut self) -> BEIF_W<EIFR_SPEC> {
        BEIF_W::new(self, 0)
    }
    ///Bit 1 - Error-Warning Detect Flag
    #[inline(always)]
    pub fn ewif(&mut self) -> EWIF_W<EIFR_SPEC> {
        EWIF_W::new(self, 1)
    }
    ///Bit 2 - Error-Passive Detect Flag
    #[inline(always)]
    pub fn epif(&mut self) -> EPIF_W<EIFR_SPEC> {
        EPIF_W::new(self, 2)
    }
    ///Bit 3 - Bus-Off Entry Detect Flag
    #[inline(always)]
    pub fn boeif(&mut self) -> BOEIF_W<EIFR_SPEC> {
        BOEIF_W::new(self, 3)
    }
    ///Bit 4 - Bus-Off Recovery Detect Flag
    #[inline(always)]
    pub fn borif(&mut self) -> BORIF_W<EIFR_SPEC> {
        BORIF_W::new(self, 4)
    }
    ///Bit 5 - Receive Overrun Detect Flag
    #[inline(always)]
    pub fn orif(&mut self) -> ORIF_W<EIFR_SPEC> {
        ORIF_W::new(self, 5)
    }
    ///Bit 6 - Overload Frame Transmission Detect Flag
    #[inline(always)]
    pub fn olif(&mut self) -> OLIF_W<EIFR_SPEC> {
        OLIF_W::new(self, 6)
    }
    ///Bit 7 - Bus Lock Detect Flag
    #[inline(always)]
    pub fn blif(&mut self) -> BLIF_W<EIFR_SPEC> {
        BLIF_W::new(self, 7)
    }
}
/**Error Interrupt Factor Judge Register

You can [`read`](crate::Reg::read) this register and get [`eifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EIFR_SPEC;
impl crate::RegisterSpec for EIFR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eifr::R`](R) reader structure
impl crate::Readable for EIFR_SPEC {}
///`write(|w| ..)` method takes [`eifr::W`](W) writer structure
impl crate::Writable for EIFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EIFR to value 0
impl crate::Resettable for EIFR_SPEC {}
