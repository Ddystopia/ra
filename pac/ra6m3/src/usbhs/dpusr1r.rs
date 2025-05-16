///Register `DPUSR1R` reader
pub type R = crate::R<DPUSR1R_SPEC>;
///Register `DPUSR1R` writer
pub type W = crate::W<DPUSR1R_SPEC>;
/**OVRCURA Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCAHE_A {
    ///0: Disables return from deep software standby mode
    _0 = 0,
    ///1: Enables return from deep software standby mode
    _1 = 1,
}
impl From<DOVCAHE_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCAHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCAHE` reader - OVRCURA Interrupt Enable Clear
pub type DOVCAHE_R = crate::BitReader<DOVCAHE_A>;
impl DOVCAHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVCAHE_A {
        match self.bits {
            false => DOVCAHE_A::_0,
            true => DOVCAHE_A::_1,
        }
    }
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCAHE_A::_0
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCAHE_A::_1
    }
}
///Field `DOVCAHE` writer - OVRCURA Interrupt Enable Clear
pub type DOVCAHE_W<'a, REG> = crate::BitWriter<'a, REG, DOVCAHE_A>;
impl<'a, REG> DOVCAHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOVCAHE_A::_0)
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOVCAHE_A::_1)
    }
}
/**OVRCURB Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCBHE_A {
    ///0: Disables return from deep software standby mode
    _0 = 0,
    ///1: Enables return from deep software standby mode
    _1 = 1,
}
impl From<DOVCBHE_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCBHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCBHE` reader - OVRCURB Interrupt Enable Clear
pub type DOVCBHE_R = crate::BitReader<DOVCBHE_A>;
impl DOVCBHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVCBHE_A {
        match self.bits {
            false => DOVCBHE_A::_0,
            true => DOVCBHE_A::_1,
        }
    }
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCBHE_A::_0
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCBHE_A::_1
    }
}
///Field `DOVCBHE` writer - OVRCURB Interrupt Enable Clear
pub type DOVCBHE_W<'a, REG> = crate::BitWriter<'a, REG, DOVCBHE_A>;
impl<'a, REG> DOVCBHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOVCBHE_A::_0)
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOVCBHE_A::_1)
    }
}
/**VBUS Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBSTSHE_A {
    ///0: Disables return from deep software standby mode
    _0 = 0,
    ///1: Enables return from deep software standby mode
    _1 = 1,
}
impl From<DVBSTSHE_A> for bool {
    #[inline(always)]
    fn from(variant: DVBSTSHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBSTSHE` reader - VBUS Interrupt Enable/Clear
pub type DVBSTSHE_R = crate::BitReader<DVBSTSHE_A>;
impl DVBSTSHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVBSTSHE_A {
        match self.bits {
            false => DVBSTSHE_A::_0,
            true => DVBSTSHE_A::_1,
        }
    }
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBSTSHE_A::_0
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBSTSHE_A::_1
    }
}
///Field `DVBSTSHE` writer - VBUS Interrupt Enable/Clear
pub type DVBSTSHE_W<'a, REG> = crate::BitWriter<'a, REG, DVBSTSHE_A>;
impl<'a, REG> DVBSTSHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables return from deep software standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVBSTSHE_A::_0)
    }
    ///Enables return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVBSTSHE_A::_1)
    }
}
/**Indication of Return from OVRCURA Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCAH_A {
    ///0: Indicates deep software standby mode
    _0 = 0,
    ///1: Indicates return from deep software standby mode
    _1 = 1,
}
impl From<DOVCAH_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCAH` reader - Indication of Return from OVRCURA Interrupt Source
pub type DOVCAH_R = crate::BitReader<DOVCAH_A>;
impl DOVCAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVCAH_A {
        match self.bits {
            false => DOVCAH_A::_0,
            true => DOVCAH_A::_1,
        }
    }
    ///Indicates deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCAH_A::_0
    }
    ///Indicates return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCAH_A::_1
    }
}
/**Indication of Return from OVRCURB Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVCBH_A {
    ///0: Indicates deep software standby mode
    _0 = 0,
    ///1: Indicates return from deep software standby mode
    _1 = 1,
}
impl From<DOVCBH_A> for bool {
    #[inline(always)]
    fn from(variant: DOVCBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCBH` reader - Indication of Return from OVRCURB Interrupt Source
pub type DOVCBH_R = crate::BitReader<DOVCBH_A>;
impl DOVCBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVCBH_A {
        match self.bits {
            false => DOVCBH_A::_0,
            true => DOVCBH_A::_1,
        }
    }
    ///Indicates deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVCBH_A::_0
    }
    ///Indicates return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVCBH_A::_1
    }
}
/**Indication of Return from VBUS Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBSTSH_A {
    ///0: Indicates deep software standby mode
    _0 = 0,
    ///1: Indicates return from deep software standby mode
    _1 = 1,
}
impl From<DVBSTSH_A> for bool {
    #[inline(always)]
    fn from(variant: DVBSTSH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBSTSH` reader - Indication of Return from VBUS Interrupt Source
pub type DVBSTSH_R = crate::BitReader<DVBSTSH_A>;
impl DVBSTSH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVBSTSH_A {
        match self.bits {
            false => DVBSTSH_A::_0,
            true => DVBSTSH_A::_1,
        }
    }
    ///Indicates deep software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBSTSH_A::_0
    }
    ///Indicates return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBSTSH_A::_1
    }
}
impl R {
    ///Bit 4 - OVRCURA Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcahe(&self) -> DOVCAHE_R {
        DOVCAHE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OVRCURB Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcbhe(&self) -> DOVCBHE_R {
        DOVCBHE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbstshe(&self) -> DVBSTSHE_R {
        DVBSTSHE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 20 - Indication of Return from OVRCURA Interrupt Source
    #[inline(always)]
    pub fn dovcah(&self) -> DOVCAH_R {
        DOVCAH_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Indication of Return from OVRCURB Interrupt Source
    #[inline(always)]
    pub fn dovcbh(&self) -> DOVCBH_R {
        DOVCBH_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - Indication of Return from VBUS Interrupt Source
    #[inline(always)]
    pub fn dvbstsh(&self) -> DVBSTSH_R {
        DVBSTSH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - OVRCURA Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcahe(&mut self) -> DOVCAHE_W<DPUSR1R_SPEC> {
        DOVCAHE_W::new(self, 4)
    }
    ///Bit 5 - OVRCURB Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcbhe(&mut self) -> DOVCBHE_W<DPUSR1R_SPEC> {
        DOVCBHE_W::new(self, 5)
    }
    ///Bit 7 - VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbstshe(&mut self) -> DVBSTSHE_W<DPUSR1R_SPEC> {
        DVBSTSHE_W::new(self, 7)
    }
}
/**Deep Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPUSR1R_SPEC;
impl crate::RegisterSpec for DPUSR1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpusr1r::R`](R) reader structure
impl crate::Readable for DPUSR1R_SPEC {}
///`write(|w| ..)` method takes [`dpusr1r::W`](W) writer structure
impl crate::Writable for DPUSR1R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR1R to value 0
impl crate::Resettable for DPUSR1R_SPEC {}
