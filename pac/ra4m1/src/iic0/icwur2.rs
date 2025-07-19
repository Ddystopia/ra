///Register `ICWUR2` reader
pub type R = crate::R<ICWUR2_SPEC>;
///Register `ICWUR2` writer
pub type W = crate::W<ICWUR2_SPEC>;
/**Wake-up Function Synchronous Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUSEN_A {
    ///0: IIC asynchronous circuit enable
    _0 = 0,
    ///1: IIC synchronous circuit enable
    _1 = 1,
}
impl From<WUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUSEN` reader - Wake-up Function Synchronous Enable
pub type WUSEN_R = crate::BitReader<WUSEN_A>;
impl WUSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUSEN_A {
        match self.bits {
            false => WUSEN_A::_0,
            true => WUSEN_A::_1,
        }
    }
    ///IIC asynchronous circuit enable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUSEN_A::_0
    }
    ///IIC synchronous circuit enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUSEN_A::_1
    }
}
///Field `WUSEN` writer - Wake-up Function Synchronous Enable
pub type WUSEN_W<'a, REG> = crate::BitWriter<'a, REG, WUSEN_A>;
impl<'a, REG> WUSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IIC asynchronous circuit enable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WUSEN_A::_0)
    }
    ///IIC synchronous circuit enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WUSEN_A::_1)
    }
}
/**Wake-up Function Asynchronous Operation Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUASYF_A {
    ///0: IIC synchronous circuit enable condition
    _0 = 0,
    ///1: IIC asynchronous circuit enable condition.
    _1 = 1,
}
impl From<WUASYF_A> for bool {
    #[inline(always)]
    fn from(variant: WUASYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUASYF` reader - Wake-up Function Asynchronous Operation Status Flag
pub type WUASYF_R = crate::BitReader<WUASYF_A>;
impl WUASYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUASYF_A {
        match self.bits {
            false => WUASYF_A::_0,
            true => WUASYF_A::_1,
        }
    }
    ///IIC synchronous circuit enable condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUASYF_A::_0
    }
    ///IIC asynchronous circuit enable condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUASYF_A::_1
    }
}
/**Wake-up Function Synchronous Operation Status Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUSYF_A {
    ///0: IIC asynchronous circuit enable condition
    _0 = 0,
    ///1: IIC synchronous circuit enable condition.
    _1 = 1,
}
impl From<WUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: WUSYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUSYF` reader - Wake-up Function Synchronous Operation Status Flag
pub type WUSYF_R = crate::BitReader<WUSYF_A>;
impl WUSYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUSYF_A {
        match self.bits {
            false => WUSYF_A::_0,
            true => WUSYF_A::_1,
        }
    }
    ///IIC asynchronous circuit enable condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUSYF_A::_0
    }
    ///IIC synchronous circuit enable condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUSYF_A::_1
    }
}
impl R {
    ///Bit 0 - Wake-up Function Synchronous Enable
    #[inline(always)]
    pub fn wusen(&self) -> WUSEN_R {
        WUSEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wake-up Function Asynchronous Operation Status Flag
    #[inline(always)]
    pub fn wuasyf(&self) -> WUASYF_R {
        WUASYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wake-up Function Synchronous Operation Status Flag
    #[inline(always)]
    pub fn wusyf(&self) -> WUSYF_R {
        WUSYF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Wake-up Function Synchronous Enable
    #[inline(always)]
    pub fn wusen(&mut self) -> WUSEN_W<'_, ICWUR2_SPEC> {
        WUSEN_W::new(self, 0)
    }
}
/**I2C Bus Wake up Unit Register 2

You can [`read`](crate::Reg::read) this register and get [`icwur2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICWUR2_SPEC;
impl crate::RegisterSpec for ICWUR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icwur2::R`](R) reader structure
impl crate::Readable for ICWUR2_SPEC {}
///`write(|w| ..)` method takes [`icwur2::W`](W) writer structure
impl crate::Writable for ICWUR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICWUR2 to value 0xfd
impl crate::Resettable for ICWUR2_SPEC {
    const RESET_VALUE: u8 = 0xfd;
}
