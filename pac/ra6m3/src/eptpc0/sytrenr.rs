///Register `SYTRENR` reader
pub type R = crate::R<SYTRENR_SPEC>;
///Register `SYTRENR` writer
pub type W = crate::W<SYTRENR_SPEC>;
/**Announce Message Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANCE_A {
    ///0: Announce messages are not transmitted.
    _0 = 0,
    ///1: Announce messages are transmitted.
    _1 = 1,
}
impl From<ANCE_A> for bool {
    #[inline(always)]
    fn from(variant: ANCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANCE` reader - Announce Message Transmission Enable
pub type ANCE_R = crate::BitReader<ANCE_A>;
impl ANCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANCE_A {
        match self.bits {
            false => ANCE_A::_0,
            true => ANCE_A::_1,
        }
    }
    ///Announce messages are not transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANCE_A::_0
    }
    ///Announce messages are transmitted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANCE_A::_1
    }
}
///Field `ANCE` writer - Announce Message Transmission Enable
pub type ANCE_W<'a, REG> = crate::BitWriter<'a, REG, ANCE_A>;
impl<'a, REG> ANCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Announce messages are not transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANCE_A::_0)
    }
    ///Announce messages are transmitted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANCE_A::_1)
    }
}
/**Sync Message Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    ///0: Sync messages are not transmitted.
    _0 = 0,
    ///1: Sync messages are transmitted.
    _1 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC` reader - Sync Message Transmission Enable
pub type SYNC_R = crate::BitReader<SYNC_A>;
impl SYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::_0,
            true => SYNC_A::_1,
        }
    }
    ///Sync messages are not transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC_A::_0
    }
    ///Sync messages are transmitted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC_A::_1
    }
}
///Field `SYNC` writer - Sync Message Transmission Enable
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_A>;
impl<'a, REG> SYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sync messages are not transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::_0)
    }
    ///Sync messages are transmitted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::_1)
    }
}
/**Delay_Req Message Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ_A {
    ///0: Delay_Req messages are not transmitted.
    _0 = 0,
    ///1: Delay_Req messages are transmitted.
    _1 = 1,
}
impl From<DRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRQ` reader - Delay_Req Message Transmission Enable
pub type DRQ_R = crate::BitReader<DRQ_A>;
impl DRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRQ_A {
        match self.bits {
            false => DRQ_A::_0,
            true => DRQ_A::_1,
        }
    }
    ///Delay_Req messages are not transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRQ_A::_0
    }
    ///Delay_Req messages are transmitted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRQ_A::_1
    }
}
///Field `DRQ` writer - Delay_Req Message Transmission Enable
pub type DRQ_W<'a, REG> = crate::BitWriter<'a, REG, DRQ_A>;
impl<'a, REG> DRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay_Req messages are not transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ_A::_0)
    }
    ///Delay_Req messages are transmitted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ_A::_1)
    }
}
/**Pdelay_Req Message Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDRQ_A {
    ///0: Pdelay_Req messages are not transmitted.
    _0 = 0,
    ///1: Pdelay_Req messages are transmitted.
    _1 = 1,
}
impl From<PDRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PDRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDRQ` reader - Pdelay_Req Message Transmission Enable
pub type PDRQ_R = crate::BitReader<PDRQ_A>;
impl PDRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDRQ_A {
        match self.bits {
            false => PDRQ_A::_0,
            true => PDRQ_A::_1,
        }
    }
    ///Pdelay_Req messages are not transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDRQ_A::_0
    }
    ///Pdelay_Req messages are transmitted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDRQ_A::_1
    }
}
///Field `PDRQ` writer - Pdelay_Req Message Transmission Enable
pub type PDRQ_W<'a, REG> = crate::BitWriter<'a, REG, PDRQ_A>;
impl<'a, REG> PDRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pdelay_Req messages are not transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDRQ_A::_0)
    }
    ///Pdelay_Req messages are transmitted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDRQ_A::_1)
    }
}
impl R {
    ///Bit 0 - Announce Message Transmission Enable
    #[inline(always)]
    pub fn ance(&self) -> ANCE_R {
        ANCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Sync Message Transmission Enable
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Delay_Req Message Transmission Enable
    #[inline(always)]
    pub fn drq(&self) -> DRQ_R {
        DRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Pdelay_Req Message Transmission Enable
    #[inline(always)]
    pub fn pdrq(&self) -> PDRQ_R {
        PDRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Announce Message Transmission Enable
    #[inline(always)]
    pub fn ance(&mut self) -> ANCE_W<SYTRENR_SPEC> {
        ANCE_W::new(self, 0)
    }
    ///Bit 4 - Sync Message Transmission Enable
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<SYTRENR_SPEC> {
        SYNC_W::new(self, 4)
    }
    ///Bit 8 - Delay_Req Message Transmission Enable
    #[inline(always)]
    pub fn drq(&mut self) -> DRQ_W<SYTRENR_SPEC> {
        DRQ_W::new(self, 8)
    }
    ///Bit 12 - Pdelay_Req Message Transmission Enable
    #[inline(always)]
    pub fn pdrq(&mut self) -> PDRQ_W<SYTRENR_SPEC> {
        PDRQ_W::new(self, 12)
    }
}
/**SYNFP Transmission Enable Register

You can [`read`](crate::Reg::read) this register and get [`sytrenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sytrenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYTRENR_SPEC;
impl crate::RegisterSpec for SYTRENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sytrenr::R`](R) reader structure
impl crate::Readable for SYTRENR_SPEC {}
///`write(|w| ..)` method takes [`sytrenr::W`](W) writer structure
impl crate::Writable for SYTRENR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYTRENR to value 0
impl crate::Resettable for SYTRENR_SPEC {}
