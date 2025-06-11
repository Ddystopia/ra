///Register `MSTPCRB` reader
pub type R = crate::R<MSTPCRB_SPEC>;
///Register `MSTPCRB` writer
pub type W = crate::W<MSTPCRB_SPEC>;
/**Controller Area Network Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB2_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB2` reader - Controller Area Network Module Stop
pub type MSTPB2_R = crate::BitReader<MSTPB2_A>;
impl MSTPB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB2_A {
        match self.bits {
            false => MSTPB2_A::_0,
            true => MSTPB2_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB2_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB2_A::_1
    }
}
///Field `MSTPB2` writer - Controller Area Network Module Stop
pub type MSTPB2_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB2_A>;
impl<'a, REG> MSTPB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB2_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB2_A::_1)
    }
}
/**I2C Bus Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB8_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB8_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB8` reader - I2C Bus Interface 1 Module Stop
pub type MSTPB8_R = crate::BitReader<MSTPB8_A>;
impl MSTPB8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB8_A {
        match self.bits {
            false => MSTPB8_A::_0,
            true => MSTPB8_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB8_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB8_A::_1
    }
}
///Field `MSTPB8` writer - I2C Bus Interface 1 Module Stop
pub type MSTPB8_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB8_A>;
impl<'a, REG> MSTPB8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB8_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB8_A::_1)
    }
}
/**I2C Bus Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB9_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB9_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB9` reader - I2C Bus Interface 0 Module Stop
pub type MSTPB9_R = crate::BitReader<MSTPB9_A>;
impl MSTPB9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB9_A {
        match self.bits {
            false => MSTPB9_A::_0,
            true => MSTPB9_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB9_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB9_A::_1
    }
}
///Field `MSTPB9` writer - I2C Bus Interface 0 Module Stop
pub type MSTPB9_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB9_A>;
impl<'a, REG> MSTPB9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB9_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB9_A::_1)
    }
}
/**Universal Serial Bus 2.0 FS Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB11_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB11_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB11` reader - Universal Serial Bus 2.0 FS Interface Module Stop
pub type MSTPB11_R = crate::BitReader<MSTPB11_A>;
impl MSTPB11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB11_A {
        match self.bits {
            false => MSTPB11_A::_0,
            true => MSTPB11_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB11_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB11_A::_1
    }
}
///Field `MSTPB11` writer - Universal Serial Bus 2.0 FS Interface Module Stop
pub type MSTPB11_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB11_A>;
impl<'a, REG> MSTPB11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB11_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB11_A::_1)
    }
}
/**Serial Peripheral Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB18_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB18_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB18` reader - Serial Peripheral Interface 1 Module Stop
pub type MSTPB18_R = crate::BitReader<MSTPB18_A>;
impl MSTPB18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB18_A {
        match self.bits {
            false => MSTPB18_A::_0,
            true => MSTPB18_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB18_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB18_A::_1
    }
}
///Field `MSTPB18` writer - Serial Peripheral Interface 1 Module Stop
pub type MSTPB18_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB18_A>;
impl<'a, REG> MSTPB18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB18_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB18_A::_1)
    }
}
/**Serial Peripheral Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB19_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB19_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB19` reader - Serial Peripheral Interface 0 Module Stop
pub type MSTPB19_R = crate::BitReader<MSTPB19_A>;
impl MSTPB19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB19_A {
        match self.bits {
            false => MSTPB19_A::_0,
            true => MSTPB19_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB19_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB19_A::_1
    }
}
///Field `MSTPB19` writer - Serial Peripheral Interface 0 Module Stop
pub type MSTPB19_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB19_A>;
impl<'a, REG> MSTPB19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB19_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB19_A::_1)
    }
}
/**Serial Communication Interface 9 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB22_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB22` reader - Serial Communication Interface 9 Module Stop
pub type MSTPB22_R = crate::BitReader<MSTPB22_A>;
impl MSTPB22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB22_A {
        match self.bits {
            false => MSTPB22_A::_0,
            true => MSTPB22_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB22_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB22_A::_1
    }
}
///Field `MSTPB22` writer - Serial Communication Interface 9 Module Stop
pub type MSTPB22_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB22_A>;
impl<'a, REG> MSTPB22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB22_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB22_A::_1)
    }
}
/**Serial Communication Interface 2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB29_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB29_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB29_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB29` reader - Serial Communication Interface 2 Module Stop
pub type MSTPB29_R = crate::BitReader<MSTPB29_A>;
impl MSTPB29_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB29_A {
        match self.bits {
            false => MSTPB29_A::_0,
            true => MSTPB29_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB29_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB29_A::_1
    }
}
///Field `MSTPB29` writer - Serial Communication Interface 2 Module Stop
pub type MSTPB29_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB29_A>;
impl<'a, REG> MSTPB29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB29_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB29_A::_1)
    }
}
/**Serial Communication Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB30_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB30_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB30_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB30` reader - Serial Communication Interface 1 Module Stop
pub type MSTPB30_R = crate::BitReader<MSTPB30_A>;
impl MSTPB30_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB30_A {
        match self.bits {
            false => MSTPB30_A::_0,
            true => MSTPB30_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB30_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB30_A::_1
    }
}
///Field `MSTPB30` writer - Serial Communication Interface 1 Module Stop
pub type MSTPB30_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB30_A>;
impl<'a, REG> MSTPB30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB30_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB30_A::_1)
    }
}
/**Serial Communication Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB31_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB31` reader - Serial Communication Interface 0 Module Stop
pub type MSTPB31_R = crate::BitReader<MSTPB31_A>;
impl MSTPB31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB31_A {
        match self.bits {
            false => MSTPB31_A::_0,
            true => MSTPB31_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB31_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB31_A::_1
    }
}
///Field `MSTPB31` writer - Serial Communication Interface 0 Module Stop
pub type MSTPB31_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB31_A>;
impl<'a, REG> MSTPB31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB31_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB31_A::_1)
    }
}
impl R {
    ///Bit 2 - Controller Area Network Module Stop
    #[inline(always)]
    pub fn mstpb2(&self) -> MSTPB2_R {
        MSTPB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - I2C Bus Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb8(&self) -> MSTPB8_R {
        MSTPB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I2C Bus Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb9(&self) -> MSTPB9_R {
        MSTPB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop
    #[inline(always)]
    pub fn mstpb11(&self) -> MSTPB11_R {
        MSTPB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 18 - Serial Peripheral Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb18(&self) -> MSTPB18_R {
        MSTPB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Serial Peripheral Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb19(&self) -> MSTPB19_R {
        MSTPB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Serial Communication Interface 9 Module Stop
    #[inline(always)]
    pub fn mstpb22(&self) -> MSTPB22_R {
        MSTPB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 29 - Serial Communication Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb29(&self) -> MSTPB29_R {
        MSTPB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Serial Communication Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb30(&self) -> MSTPB30_R {
        MSTPB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Serial Communication Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb31(&self) -> MSTPB31_R {
        MSTPB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Controller Area Network Module Stop
    #[inline(always)]
    pub fn mstpb2(&mut self) -> MSTPB2_W<MSTPCRB_SPEC> {
        MSTPB2_W::new(self, 2)
    }
    ///Bit 8 - I2C Bus Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb8(&mut self) -> MSTPB8_W<MSTPCRB_SPEC> {
        MSTPB8_W::new(self, 8)
    }
    ///Bit 9 - I2C Bus Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb9(&mut self) -> MSTPB9_W<MSTPCRB_SPEC> {
        MSTPB9_W::new(self, 9)
    }
    ///Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop
    #[inline(always)]
    pub fn mstpb11(&mut self) -> MSTPB11_W<MSTPCRB_SPEC> {
        MSTPB11_W::new(self, 11)
    }
    ///Bit 18 - Serial Peripheral Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb18(&mut self) -> MSTPB18_W<MSTPCRB_SPEC> {
        MSTPB18_W::new(self, 18)
    }
    ///Bit 19 - Serial Peripheral Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb19(&mut self) -> MSTPB19_W<MSTPCRB_SPEC> {
        MSTPB19_W::new(self, 19)
    }
    ///Bit 22 - Serial Communication Interface 9 Module Stop
    #[inline(always)]
    pub fn mstpb22(&mut self) -> MSTPB22_W<MSTPCRB_SPEC> {
        MSTPB22_W::new(self, 22)
    }
    ///Bit 29 - Serial Communication Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb29(&mut self) -> MSTPB29_W<MSTPCRB_SPEC> {
        MSTPB29_W::new(self, 29)
    }
    ///Bit 30 - Serial Communication Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb30(&mut self) -> MSTPB30_W<MSTPCRB_SPEC> {
        MSTPB30_W::new(self, 30)
    }
    ///Bit 31 - Serial Communication Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb31(&mut self) -> MSTPB31_W<MSTPCRB_SPEC> {
        MSTPB31_W::new(self, 31)
    }
}
/**Module Stop Control Register B

You can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSTPCRB_SPEC;
impl crate::RegisterSpec for MSTPCRB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mstpcrb::R`](R) reader structure
impl crate::Readable for MSTPCRB_SPEC {}
///`write(|w| ..)` method takes [`mstpcrb::W`](W) writer structure
impl crate::Writable for MSTPCRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRB to value 0xffff_ffff
impl crate::Resettable for MSTPCRB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
