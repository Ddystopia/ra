///Register `ICWUR` reader
pub type R = crate::R<ICWUR_SPEC>;
///Register `ICWUR` writer
pub type W = crate::W<ICWUR_SPEC>;
/**Wake-Up Analog Filter Additional Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUAFA_A {
    ///0: Do not add the Wake Up analog filter.
    _0 = 0,
    ///1: Add the Wake Up analog filter.
    _1 = 1,
}
impl From<WUAFA_A> for bool {
    #[inline(always)]
    fn from(variant: WUAFA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUAFA` reader - Wake-Up Analog Filter Additional Selection
pub type WUAFA_R = crate::BitReader<WUAFA_A>;
impl WUAFA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUAFA_A {
        match self.bits {
            false => WUAFA_A::_0,
            true => WUAFA_A::_1,
        }
    }
    ///Do not add the Wake Up analog filter.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUAFA_A::_0
    }
    ///Add the Wake Up analog filter.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUAFA_A::_1
    }
}
///Field `WUAFA` writer - Wake-Up Analog Filter Additional Selection
pub type WUAFA_W<'a, REG> = crate::BitWriter<'a, REG, WUAFA_A>;
impl<'a, REG> WUAFA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not add the Wake Up analog filter.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUAFA_A::_0)
    }
    ///Add the Wake Up analog filter.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUAFA_A::_1)
    }
}
/**Asynchronous/Synchronous Operation State Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUACK_A {
    ///0: State of synchronous operation
    _0 = 0,
    ///1: State of asynchronous operation
    _1 = 1,
}
impl From<WUACK_A> for bool {
    #[inline(always)]
    fn from(variant: WUACK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUACK` reader - Asynchronous/Synchronous Operation State Flag
pub type WUACK_R = crate::BitReader<WUACK_A>;
impl WUACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUACK_A {
        match self.bits {
            false => WUACK_A::_0,
            true => WUACK_A::_1,
        }
    }
    ///State of synchronous operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUACK_A::_0
    }
    ///State of asynchronous operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUACK_A::_1
    }
}
///Field `WUACK` writer - Asynchronous/Synchronous Operation State Flag
pub type WUACK_W<'a, REG> = crate::BitWriter<'a, REG, WUACK_A>;
impl<'a, REG> WUACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///State of synchronous operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUACK_A::_0)
    }
    ///State of asynchronous operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUACK_A::_1)
    }
}
/**Wake-Up Event Occurrence Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF_A {
    ///0: Slave address match during Wake-Up function.
    _0 = 0,
    ///1: Slave address not match during Wake-Up function.
    _1 = 1,
}
impl From<WUF_A> for bool {
    #[inline(always)]
    fn from(variant: WUF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF` reader - Wake-Up Event Occurrence Flag
pub type WUF_R = crate::BitReader<WUF_A>;
impl WUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF_A {
        match self.bits {
            false => WUF_A::_0,
            true => WUF_A::_1,
        }
    }
    ///Slave address match during Wake-Up function.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF_A::_0
    }
    ///Slave address not match during Wake-Up function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF_A::_1
    }
}
///Field `WUF` writer - Wake-Up Event Occurrence Flag
pub type WUF_W<'a, REG> = crate::BitWriter<'a, REG, WUF_A>;
impl<'a, REG> WUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address match during Wake-Up function.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUF_A::_0)
    }
    ///Slave address not match during Wake-Up function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUF_A::_1)
    }
}
/**Wake Up Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUIE_A {
    ///0: Wake Up Interrupt Request (WUI) is disabled.
    _0 = 0,
    ///1: Wake Up Interrupt Request (WUI) is enabled.
    _1 = 1,
}
impl From<WUIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUIE` reader - Wake Up Interrupt Request Enable
pub type WUIE_R = crate::BitReader<WUIE_A>;
impl WUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUIE_A {
        match self.bits {
            false => WUIE_A::_0,
            true => WUIE_A::_1,
        }
    }
    ///Wake Up Interrupt Request (WUI) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUIE_A::_0
    }
    ///Wake Up Interrupt Request (WUI) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUIE_A::_1
    }
}
///Field `WUIE` writer - Wake Up Interrupt Request Enable
pub type WUIE_W<'a, REG> = crate::BitWriter<'a, REG, WUIE_A>;
impl<'a, REG> WUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wake Up Interrupt Request (WUI) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUIE_A::_0)
    }
    ///Wake Up Interrupt Request (WUI) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUIE_A::_1)
    }
}
/**Wake Up function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUE_A {
    ///0: Wake-up function is disabled
    _0 = 0,
    ///1: Wake-up function is enabled.
    _1 = 1,
}
impl From<WUE_A> for bool {
    #[inline(always)]
    fn from(variant: WUE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUE` reader - Wake Up function Enable
pub type WUE_R = crate::BitReader<WUE_A>;
impl WUE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUE_A {
        match self.bits {
            false => WUE_A::_0,
            true => WUE_A::_1,
        }
    }
    ///Wake-up function is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUE_A::_0
    }
    ///Wake-up function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUE_A::_1
    }
}
///Field `WUE` writer - Wake Up function Enable
pub type WUE_W<'a, REG> = crate::BitWriter<'a, REG, WUE_A>;
impl<'a, REG> WUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wake-up function is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUE_A::_0)
    }
    ///Wake-up function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUE_A::_1)
    }
}
impl R {
    ///Bit 0 - Wake-Up Analog Filter Additional Selection
    #[inline(always)]
    pub fn wuafa(&self) -> WUAFA_R {
        WUAFA_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Asynchronous/Synchronous Operation State Flag
    #[inline(always)]
    pub fn wuack(&self) -> WUACK_R {
        WUACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wake-Up Event Occurrence Flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wake Up Interrupt Request Enable
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wake Up function Enable
    #[inline(always)]
    pub fn wue(&self) -> WUE_R {
        WUE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Wake-Up Analog Filter Additional Selection
    #[inline(always)]
    pub fn wuafa(&mut self) -> WUAFA_W<ICWUR_SPEC> {
        WUAFA_W::new(self, 0)
    }
    ///Bit 4 - Asynchronous/Synchronous Operation State Flag
    #[inline(always)]
    pub fn wuack(&mut self) -> WUACK_W<ICWUR_SPEC> {
        WUACK_W::new(self, 4)
    }
    ///Bit 5 - Wake-Up Event Occurrence Flag
    #[inline(always)]
    pub fn wuf(&mut self) -> WUF_W<ICWUR_SPEC> {
        WUF_W::new(self, 5)
    }
    ///Bit 6 - Wake Up Interrupt Request Enable
    #[inline(always)]
    pub fn wuie(&mut self) -> WUIE_W<ICWUR_SPEC> {
        WUIE_W::new(self, 6)
    }
    ///Bit 7 - Wake Up function Enable
    #[inline(always)]
    pub fn wue(&mut self) -> WUE_W<ICWUR_SPEC> {
        WUE_W::new(self, 7)
    }
}
/**I2C Bus Wake Up Unit Register

You can [`read`](crate::Reg::read) this register and get [`icwur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICWUR_SPEC;
impl crate::RegisterSpec for ICWUR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icwur::R`](R) reader structure
impl crate::Readable for ICWUR_SPEC {}
///`write(|w| ..)` method takes [`icwur::W`](W) writer structure
impl crate::Writable for ICWUR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICWUR to value 0
impl crate::Resettable for ICWUR_SPEC {}
