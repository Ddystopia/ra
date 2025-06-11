///Register `MSTPCRA` reader
pub type R = crate::R<MSTPCRA_SPEC>;
///Register `MSTPCRA` writer
pub type W = crate::W<MSTPCRA_SPEC>;
/**RAM0 Module Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA0_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPA0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPA0` reader - RAM0 Module Stop
pub type MSTPA0_R = crate::BitReader<MSTPA0_A>;
impl MSTPA0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPA0_A {
        match self.bits {
            false => MSTPA0_A::_0,
            true => MSTPA0_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA0_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA0_A::_1
    }
}
///Field `MSTPA0` writer - RAM0 Module Stop
pub type MSTPA0_W<'a, REG> = crate::BitWriter<'a, REG, MSTPA0_A>;
impl<'a, REG> MSTPA0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA0_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA0_A::_1)
    }
}
/**ECCRAM Module Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA6_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPA6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPA6` reader - ECCRAM Module Stop
pub type MSTPA6_R = crate::BitReader<MSTPA6_A>;
impl MSTPA6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPA6_A {
        match self.bits {
            false => MSTPA6_A::_0,
            true => MSTPA6_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA6_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA6_A::_1
    }
}
///Field `MSTPA6` writer - ECCRAM Module Stop
pub type MSTPA6_W<'a, REG> = crate::BitWriter<'a, REG, MSTPA6_A>;
impl<'a, REG> MSTPA6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA6_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA6_A::_1)
    }
}
/**DMA Controller/Data Transfer Controller Module Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPA22_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPA22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPA22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPA22` reader - DMA Controller/Data Transfer Controller Module Stop
pub type MSTPA22_R = crate::BitReader<MSTPA22_A>;
impl MSTPA22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPA22_A {
        match self.bits {
            false => MSTPA22_A::_0,
            true => MSTPA22_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPA22_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPA22_A::_1
    }
}
///Field `MSTPA22` writer - DMA Controller/Data Transfer Controller Module Stop
pub type MSTPA22_W<'a, REG> = crate::BitWriter<'a, REG, MSTPA22_A>;
impl<'a, REG> MSTPA22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA22_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPA22_A::_1)
    }
}
impl R {
    ///Bit 0 - RAM0 Module Stop
    #[inline(always)]
    pub fn mstpa0(&self) -> MSTPA0_R {
        MSTPA0_R::new((self.bits & 1) != 0)
    }
    ///Bit 6 - ECCRAM Module Stop
    #[inline(always)]
    pub fn mstpa6(&self) -> MSTPA6_R {
        MSTPA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 22 - DMA Controller/Data Transfer Controller Module Stop
    #[inline(always)]
    pub fn mstpa22(&self) -> MSTPA22_R {
        MSTPA22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RAM0 Module Stop
    #[inline(always)]
    pub fn mstpa0(&mut self) -> MSTPA0_W<MSTPCRA_SPEC> {
        MSTPA0_W::new(self, 0)
    }
    ///Bit 6 - ECCRAM Module Stop
    #[inline(always)]
    pub fn mstpa6(&mut self) -> MSTPA6_W<MSTPCRA_SPEC> {
        MSTPA6_W::new(self, 6)
    }
    ///Bit 22 - DMA Controller/Data Transfer Controller Module Stop
    #[inline(always)]
    pub fn mstpa22(&mut self) -> MSTPA22_W<MSTPCRA_SPEC> {
        MSTPA22_W::new(self, 22)
    }
}
/**Module Stop Control Register A

You can [`read`](crate::Reg::read) this register and get [`mstpcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSTPCRA_SPEC;
impl crate::RegisterSpec for MSTPCRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mstpcra::R`](R) reader structure
impl crate::Readable for MSTPCRA_SPEC {}
///`write(|w| ..)` method takes [`mstpcra::W`](W) writer structure
impl crate::Writable for MSTPCRA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRA to value 0xffbf_ffbe
impl crate::Resettable for MSTPCRA_SPEC {
    const RESET_VALUE: u32 = 0xffbf_ffbe;
}
