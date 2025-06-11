///Register `DCPCFG` reader
pub type R = crate::R<DCPCFG_SPEC>;
///Register `DCPCFG` writer
pub type W = crate::W<DCPCFG_SPEC>;
/**Transfer Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    ///0: Data receiving direction
    _0 = 0,
    ///1: Data transmitting direction
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Transfer Direction
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    ///Data receiving direction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIR_A::_0
    }
    ///Data transmitting direction
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIR_A::_1
    }
}
///Field `DIR` writer - Transfer Direction
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR_A>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data receiving direction
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::_0)
    }
    ///Data transmitting direction
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::_1)
    }
}
/**Pipe Disabled at End of Transfer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTNAK_A {
    ///0: Pipe continued at the end of transfer
    _0 = 0,
    ///1: Pipe disabled at the end of transfer
    _1 = 1,
}
impl From<SHTNAK_A> for bool {
    #[inline(always)]
    fn from(variant: SHTNAK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHTNAK` reader - Pipe Disabled at End of Transfer
pub type SHTNAK_R = crate::BitReader<SHTNAK_A>;
impl SHTNAK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHTNAK_A {
        match self.bits {
            false => SHTNAK_A::_0,
            true => SHTNAK_A::_1,
        }
    }
    ///Pipe continued at the end of transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTNAK_A::_0
    }
    ///Pipe disabled at the end of transfer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTNAK_A::_1
    }
}
///Field `SHTNAK` writer - Pipe Disabled at End of Transfer
pub type SHTNAK_W<'a, REG> = crate::BitWriter<'a, REG, SHTNAK_A>;
impl<'a, REG> SHTNAK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pipe continued at the end of transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHTNAK_A::_0)
    }
    ///Pipe disabled at the end of transfer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHTNAK_A::_1)
    }
}
impl R {
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&self) -> SHTNAK_R {
        SHTNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<DCPCFG_SPEC> {
        DIR_W::new(self, 4)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&mut self) -> SHTNAK_W<DCPCFG_SPEC> {
        SHTNAK_W::new(self, 7)
    }
}
/**DCP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dcpcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCPCFG_SPEC;
impl crate::RegisterSpec for DCPCFG_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dcpcfg::R`](R) reader structure
impl crate::Readable for DCPCFG_SPEC {}
///`write(|w| ..)` method takes [`dcpcfg::W`](W) writer structure
impl crate::Writable for DCPCFG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCPCFG to value 0
impl crate::Resettable for DCPCFG_SPEC {}
