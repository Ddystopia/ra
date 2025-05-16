///Register `STIPR` reader
pub type R = crate::R<STIPR_SPEC>;
///Register `STIPR` writer
pub type W = crate::W<STIPR_SPEC>;
/**SYNC Status Notification Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    ///0: Disable notification of the STSR.SYNC state
    _0 = 0,
    ///1: Enable notification of the STSR.SYNC state
    _1 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC` reader - SYNC Status Notification Enable
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
    ///Disable notification of the STSR.SYNC state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC_A::_0
    }
    ///Enable notification of the STSR.SYNC state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC_A::_1
    }
}
///Field `SYNC` writer - SYNC Status Notification Enable
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_A>;
impl<'a, REG> SYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable notification of the STSR.SYNC state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::_0)
    }
    ///Enable notification of the STSR.SYNC state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::_1)
    }
}
/**SYNCOUT Status Notification Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOUT_A {
    ///0: Disable notification of the STSR.SYNCOUT state
    _0 = 0,
    ///1: Enable notification of the STSR.SYNCOUT state
    _1 = 1,
}
impl From<SYNCOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCOUT` reader - SYNCOUT Status Notification Enable
pub type SYNCOUT_R = crate::BitReader<SYNCOUT_A>;
impl SYNCOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOUT_A {
        match self.bits {
            false => SYNCOUT_A::_0,
            true => SYNCOUT_A::_1,
        }
    }
    ///Disable notification of the STSR.SYNCOUT state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCOUT_A::_0
    }
    ///Enable notification of the STSR.SYNCOUT state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCOUT_A::_1
    }
}
///Field `SYNCOUT` writer - SYNCOUT Status Notification Enable
pub type SYNCOUT_W<'a, REG> = crate::BitWriter<'a, REG, SYNCOUT_A>;
impl<'a, REG> SYNCOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable notification of the STSR.SYNCOUT state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT_A::_0)
    }
    ///Enable notification of the STSR.SYNCOUT state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT_A::_1)
    }
}
/**SYNTOUT Status Notification Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNTOUT_A {
    ///0: Disable notification of the STSR.SYNTOUT state
    _0 = 0,
    ///1: Enable notification of the STSR.SYNTOUT state
    _1 = 1,
}
impl From<SYNTOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNTOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNTOUT` reader - SYNTOUT Status Notification Enable
pub type SYNTOUT_R = crate::BitReader<SYNTOUT_A>;
impl SYNTOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNTOUT_A {
        match self.bits {
            false => SYNTOUT_A::_0,
            true => SYNTOUT_A::_1,
        }
    }
    ///Disable notification of the STSR.SYNTOUT state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNTOUT_A::_0
    }
    ///Enable notification of the STSR.SYNTOUT state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNTOUT_A::_1
    }
}
///Field `SYNTOUT` writer - SYNTOUT Status Notification Enable
pub type SYNTOUT_W<'a, REG> = crate::BitWriter<'a, REG, SYNTOUT_A>;
impl<'a, REG> SYNTOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable notification of the STSR.SYNTOUT state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNTOUT_A::_0)
    }
    ///Enable notification of the STSR.SYNTOUT state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNTOUT_A::_1)
    }
}
/**W10D Status Notification Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W10D_A {
    ///0: Disable notification of the STSR.W10D state
    _0 = 0,
    ///1: Enable notification of the STSR.W10D state
    _1 = 1,
}
impl From<W10D_A> for bool {
    #[inline(always)]
    fn from(variant: W10D_A) -> Self {
        variant as u8 != 0
    }
}
///Field `W10D` reader - W10D Status Notification Enable
pub type W10D_R = crate::BitReader<W10D_A>;
impl W10D_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> W10D_A {
        match self.bits {
            false => W10D_A::_0,
            true => W10D_A::_1,
        }
    }
    ///Disable notification of the STSR.W10D state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == W10D_A::_0
    }
    ///Enable notification of the STSR.W10D state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == W10D_A::_1
    }
}
///Field `W10D` writer - W10D Status Notification Enable
pub type W10D_W<'a, REG> = crate::BitWriter<'a, REG, W10D_A>;
impl<'a, REG> W10D_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable notification of the STSR.W10D state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(W10D_A::_0)
    }
    ///Enable notification of the STSR.W10D state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(W10D_A::_1)
    }
}
impl R {
    ///Bit 0 - SYNC Status Notification Enable
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNCOUT Status Notification Enable
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - SYNTOUT Status Notification Enable
    #[inline(always)]
    pub fn syntout(&self) -> SYNTOUT_R {
        SYNTOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - W10D Status Notification Enable
    #[inline(always)]
    pub fn w10d(&self) -> W10D_R {
        W10D_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYNC Status Notification Enable
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<STIPR_SPEC> {
        SYNC_W::new(self, 0)
    }
    ///Bit 1 - SYNCOUT Status Notification Enable
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W<STIPR_SPEC> {
        SYNCOUT_W::new(self, 1)
    }
    ///Bit 3 - SYNTOUT Status Notification Enable
    #[inline(always)]
    pub fn syntout(&mut self) -> SYNTOUT_W<STIPR_SPEC> {
        SYNTOUT_W::new(self, 3)
    }
    ///Bit 4 - W10D Status Notification Enable
    #[inline(always)]
    pub fn w10d(&mut self) -> W10D_W<STIPR_SPEC> {
        W10D_W::new(self, 4)
    }
}
/**STCA Status Notification Permission Register

You can [`read`](crate::Reg::read) this register and get [`stipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STIPR_SPEC;
impl crate::RegisterSpec for STIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`stipr::R`](R) reader structure
impl crate::Readable for STIPR_SPEC {}
///`write(|w| ..)` method takes [`stipr::W`](W) writer structure
impl crate::Writable for STIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STIPR to value 0
impl crate::Resettable for STIPR_SPEC {}
