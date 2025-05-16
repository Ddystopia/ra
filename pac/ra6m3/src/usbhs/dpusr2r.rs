///Register `DPUSR2R` reader
pub type R = crate::R<DPUSR2R_SPEC>;
///Register `DPUSR2R` writer
pub type W = crate::W<DPUSR2R_SPEC>;
/**Indication of Return from DP Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINT_A {
    ///0: Indicates deep software standby mode
    _0 = 0,
    ///1: Indicates return from deep software standby mode
    _1 = 1,
}
impl From<DPINT_A> for bool {
    #[inline(always)]
    fn from(variant: DPINT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINT` reader - Indication of Return from DP Interrupt Source
pub type DPINT_R = crate::BitReader<DPINT_A>;
impl DPINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPINT_A {
        match self.bits {
            false => DPINT_A::_0,
            true => DPINT_A::_1,
        }
    }
    ///Indicates deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINT_A::_0
    }
    ///Indicates return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINT_A::_1
    }
}
/**Indication of Return from DM Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINT_A {
    ///0: Indicates deep software standby mode
    _0 = 0,
    ///1: Indicates return from deep software standby mode
    _1 = 1,
}
impl From<DMINT_A> for bool {
    #[inline(always)]
    fn from(variant: DMINT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINT` reader - Indication of Return from DM Interrupt Source
pub type DMINT_R = crate::BitReader<DMINT_A>;
impl DMINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMINT_A {
        match self.bits {
            false => DMINT_A::_0,
            true => DMINT_A::_1,
        }
    }
    ///Indicates deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINT_A::_0
    }
    ///Indicates return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINT_A::_1
    }
}
///Field `DPVAL` reader - DP InputIndicates DP input signal on the HS side of USB port.
pub type DPVAL_R = crate::BitReader;
///Field `DMVAL` reader - DM InputIndicates DM input signal on the HS side of USB port.
pub type DMVAL_R = crate::BitReader;
/**DP Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINTE_A {
    ///0: Disables return from deep software standby mode
    _0 = 0,
    ///1: Enables return from deep software standby mode
    _1 = 1,
}
impl From<DPINTE_A> for bool {
    #[inline(always)]
    fn from(variant: DPINTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINTE` reader - DP Interrupt Enable Clear
pub type DPINTE_R = crate::BitReader<DPINTE_A>;
impl DPINTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPINTE_A {
        match self.bits {
            false => DPINTE_A::_0,
            true => DPINTE_A::_1,
        }
    }
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINTE_A::_0
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINTE_A::_1
    }
}
///Field `DPINTE` writer - DP Interrupt Enable Clear
pub type DPINTE_W<'a, REG> = crate::BitWriter<'a, REG, DPINTE_A>;
impl<'a, REG> DPINTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPINTE_A::_0)
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPINTE_A::_1)
    }
}
/**DM Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINTE_A {
    ///0: Disables return from deep software standby mode
    _0 = 0,
    ///1: Enables return from deep software standby mode
    _1 = 1,
}
impl From<DMINTE_A> for bool {
    #[inline(always)]
    fn from(variant: DMINTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINTE` reader - DM Interrupt Enable Clear
pub type DMINTE_R = crate::BitReader<DMINTE_A>;
impl DMINTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMINTE_A {
        match self.bits {
            false => DMINTE_A::_0,
            true => DMINTE_A::_1,
        }
    }
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINTE_A::_0
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINTE_A::_1
    }
}
///Field `DMINTE` writer - DM Interrupt Enable Clear
pub type DMINTE_W<'a, REG> = crate::BitWriter<'a, REG, DMINTE_A>;
impl<'a, REG> DMINTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DMINTE_A::_0)
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DMINTE_A::_1)
    }
}
impl R {
    ///Bit 0 - Indication of Return from DP Interrupt Source
    #[inline(always)]
    pub fn dpint(&self) -> DPINT_R {
        DPINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indication of Return from DM Interrupt Source
    #[inline(always)]
    pub fn dmint(&self) -> DMINT_R {
        DMINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - DP InputIndicates DP input signal on the HS side of USB port.
    #[inline(always)]
    pub fn dpval(&self) -> DPVAL_R {
        DPVAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DM InputIndicates DM input signal on the HS side of USB port.
    #[inline(always)]
    pub fn dmval(&self) -> DMVAL_R {
        DMVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - DP Interrupt Enable Clear
    #[inline(always)]
    pub fn dpinte(&self) -> DPINTE_R {
        DPINTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DM Interrupt Enable Clear
    #[inline(always)]
    pub fn dminte(&self) -> DMINTE_R {
        DMINTE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - DP Interrupt Enable Clear
    #[inline(always)]
    pub fn dpinte(&mut self) -> DPINTE_W<DPUSR2R_SPEC> {
        DPINTE_W::new(self, 8)
    }
    ///Bit 9 - DM Interrupt Enable Clear
    #[inline(always)]
    pub fn dminte(&mut self) -> DMINTE_W<DPUSR2R_SPEC> {
        DMINTE_W::new(self, 9)
    }
}
/**Deep Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPUSR2R_SPEC;
impl crate::RegisterSpec for DPUSR2R_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dpusr2r::R`](R) reader structure
impl crate::Readable for DPUSR2R_SPEC {}
///`write(|w| ..)` method takes [`dpusr2r::W`](W) writer structure
impl crate::Writable for DPUSR2R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR2R to value 0
impl crate::Resettable for DPUSR2R_SPEC {}
