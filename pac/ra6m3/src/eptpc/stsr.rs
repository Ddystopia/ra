///Register `STSR` reader
pub type R = crate::R<STSR_SPEC>;
///Register `STSR` writer
pub type W = crate::W<STSR_SPEC>;
/**Synchronized State Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    ///0: Synchronization not detected
    _0 = 0,
    ///1: Synchronization detected
    _1 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC` reader - Synchronized State Detection Flag
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
    ///Synchronization not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNC_A::_0
    }
    ///Synchronization detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNC_A::_1
    }
}
///Field `SYNC` writer - Synchronized State Detection Flag
pub type SYNC_W<'a, REG> = crate::BitWriter1C<'a, REG, SYNC_A>;
impl<'a, REG> SYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::_0)
    }
    ///Synchronization detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::_1)
    }
}
/**Synchronization Loss Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOUT_A {
    ///0: Loss of synchronization not detected
    _0 = 0,
    ///1: Loss of synchronization detected
    _1 = 1,
}
impl From<SYNCOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCOUT` reader - Synchronization Loss Detection Flag
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
    ///Loss of synchronization not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCOUT_A::_0
    }
    ///Loss of synchronization detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCOUT_A::_1
    }
}
///Field `SYNCOUT` writer - Synchronization Loss Detection Flag
pub type SYNCOUT_W<'a, REG> = crate::BitWriter1C<'a, REG, SYNCOUT_A>;
impl<'a, REG> SYNCOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Loss of synchronization not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT_A::_0)
    }
    ///Loss of synchronization detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOUT_A::_1)
    }
}
/**Sync Message Reception Timeout Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNTOUT_A {
    ///0: Sync message reception timeout not detected
    _0 = 0,
    ///1: Sync message reception timeout detected
    _1 = 1,
}
impl From<SYNTOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SYNTOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNTOUT` reader - Sync Message Reception Timeout Detection Flag
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
    ///Sync message reception timeout not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNTOUT_A::_0
    }
    ///Sync message reception timeout detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNTOUT_A::_1
    }
}
///Field `SYNTOUT` writer - Sync Message Reception Timeout Detection Flag
pub type SYNTOUT_W<'a, REG> = crate::BitWriter1C<'a, REG, SYNTOUT_A>;
impl<'a, REG> SYNTOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sync message reception timeout not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNTOUT_A::_0)
    }
    ///Sync message reception timeout detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNTOUT_A::_1)
    }
}
/**Worst 10 Acquisition Completion Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W10D_A {
    ///0: Ten worst values not acquired yet
    _0 = 0,
    ///1: Ten worst values acquired
    _1 = 1,
}
impl From<W10D_A> for bool {
    #[inline(always)]
    fn from(variant: W10D_A) -> Self {
        variant as u8 != 0
    }
}
///Field `W10D` reader - Worst 10 Acquisition Completion Flag
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
    ///Ten worst values not acquired yet
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == W10D_A::_0
    }
    ///Ten worst values acquired
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == W10D_A::_1
    }
}
///Field `W10D` writer - Worst 10 Acquisition Completion Flag
pub type W10D_W<'a, REG> = crate::BitWriter1C<'a, REG, W10D_A>;
impl<'a, REG> W10D_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Ten worst values not acquired yet
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(W10D_A::_0)
    }
    ///Ten worst values acquired
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(W10D_A::_1)
    }
}
impl R {
    ///Bit 0 - Synchronized State Detection Flag
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Synchronization Loss Detection Flag
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Sync Message Reception Timeout Detection Flag
    #[inline(always)]
    pub fn syntout(&self) -> SYNTOUT_R {
        SYNTOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Worst 10 Acquisition Completion Flag
    #[inline(always)]
    pub fn w10d(&self) -> W10D_R {
        W10D_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Synchronized State Detection Flag
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<STSR_SPEC> {
        SYNC_W::new(self, 0)
    }
    ///Bit 1 - Synchronization Loss Detection Flag
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W<STSR_SPEC> {
        SYNCOUT_W::new(self, 1)
    }
    ///Bit 3 - Sync Message Reception Timeout Detection Flag
    #[inline(always)]
    pub fn syntout(&mut self) -> SYNTOUT_W<STSR_SPEC> {
        SYNTOUT_W::new(self, 3)
    }
    ///Bit 4 - Worst 10 Acquisition Completion Flag
    #[inline(always)]
    pub fn w10d(&mut self) -> W10D_W<STSR_SPEC> {
        W10D_W::new(self, 4)
    }
}
/**STCA Status Register

You can [`read`](crate::Reg::read) this register and get [`stsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STSR_SPEC;
impl crate::RegisterSpec for STSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`stsr::R`](R) reader structure
impl crate::Readable for STSR_SPEC {}
///`write(|w| ..)` method takes [`stsr::W`](W) writer structure
impl crate::Writable for STSR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1b;
}
///`reset()` method sets STSR to value 0
impl crate::Resettable for STSR_SPEC {}
