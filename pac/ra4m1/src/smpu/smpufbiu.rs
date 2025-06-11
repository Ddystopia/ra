///Register `SMPUFBIU` reader
pub type R = crate::R<SMPUFBIU_SPEC>;
///Register `SMPUFBIU` writer
pub type W = crate::W<SMPUFBIU_SPEC>;
/**CPU Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPCPU_A {
    ///0: CPU read of memory protection disabled.
    _0 = 0,
    ///1: CPU read of memory protection enabled.
    _1 = 1,
}
impl From<RPCPU_A> for bool {
    #[inline(always)]
    fn from(variant: RPCPU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPCPU` reader - CPU Read protection
pub type RPCPU_R = crate::BitReader<RPCPU_A>;
impl RPCPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPCPU_A {
        match self.bits {
            false => RPCPU_A::_0,
            true => RPCPU_A::_1,
        }
    }
    ///CPU read of memory protection disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPCPU_A::_0
    }
    ///CPU read of memory protection enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPCPU_A::_1
    }
}
///Field `RPCPU` writer - CPU Read protection
pub type RPCPU_W<'a, REG> = crate::BitWriter<'a, REG, RPCPU_A>;
impl<'a, REG> RPCPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU read of memory protection disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPCPU_A::_0)
    }
    ///CPU read of memory protection enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPCPU_A::_1)
    }
}
/**CPU Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPCPU_A {
    ///0: CPU write of memory protection disabled.
    _0 = 0,
    ///1: CPU write of memory protection enabled.
    _1 = 1,
}
impl From<WPCPU_A> for bool {
    #[inline(always)]
    fn from(variant: WPCPU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPCPU` reader - CPU Write protection
pub type WPCPU_R = crate::BitReader<WPCPU_A>;
impl WPCPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPCPU_A {
        match self.bits {
            false => WPCPU_A::_0,
            true => WPCPU_A::_1,
        }
    }
    ///CPU write of memory protection disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPCPU_A::_0
    }
    ///CPU write of memory protection enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPCPU_A::_1
    }
}
///Field `WPCPU` writer - CPU Write protection
pub type WPCPU_W<'a, REG> = crate::BitWriter<'a, REG, WPCPU_A>;
impl<'a, REG> WPCPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU write of memory protection disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPCPU_A::_0)
    }
    ///CPU write of memory protection enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPCPU_A::_1)
    }
}
/**Master Group A Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPGRPA_A {
    ///0: Master group A read of memory protection disabled.
    _0 = 0,
    ///1: Master group A read of memory protection enabled.
    _1 = 1,
}
impl From<RPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPGRPA` reader - Master Group A Read protection
pub type RPGRPA_R = crate::BitReader<RPGRPA_A>;
impl RPGRPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPGRPA_A {
        match self.bits {
            false => RPGRPA_A::_0,
            true => RPGRPA_A::_1,
        }
    }
    ///Master group A read of memory protection disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPGRPA_A::_0
    }
    ///Master group A read of memory protection enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPGRPA_A::_1
    }
}
///Field `RPGRPA` writer - Master Group A Read protection
pub type RPGRPA_W<'a, REG> = crate::BitWriter<'a, REG, RPGRPA_A>;
impl<'a, REG> RPGRPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master group A read of memory protection disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPA_A::_0)
    }
    ///Master group A read of memory protection enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPGRPA_A::_1)
    }
}
/**Master Group A Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPGRPA_A {
    ///0: Master group A write of memory protection disabled.
    _0 = 0,
    ///1: Master group A write of memory protection enabled.
    _1 = 1,
}
impl From<WPGRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WPGRPA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPGRPA` reader - Master Group A Write protection
pub type WPGRPA_R = crate::BitReader<WPGRPA_A>;
impl WPGRPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPGRPA_A {
        match self.bits {
            false => WPGRPA_A::_0,
            true => WPGRPA_A::_1,
        }
    }
    ///Master group A write of memory protection disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPGRPA_A::_0
    }
    ///Master group A write of memory protection enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPGRPA_A::_1
    }
}
///Field `WPGRPA` writer - Master Group A Write protection
pub type WPGRPA_W<'a, REG> = crate::BitWriter<'a, REG, WPGRPA_A>;
impl<'a, REG> WPGRPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master group A write of memory protection disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPA_A::_0)
    }
    ///Master group A write of memory protection enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WPGRPA_A::_1)
    }
}
impl R {
    ///Bit 0 - CPU Read protection
    #[inline(always)]
    pub fn rpcpu(&self) -> RPCPU_R {
        RPCPU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU Write protection
    #[inline(always)]
    pub fn wpcpu(&self) -> WPCPU_R {
        WPCPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rpgrpa(&self) -> RPGRPA_R {
        RPGRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wpgrpa(&self) -> WPGRPA_R {
        WPGRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU Read protection
    #[inline(always)]
    pub fn rpcpu(&mut self) -> RPCPU_W<SMPUFBIU_SPEC> {
        RPCPU_W::new(self, 0)
    }
    ///Bit 1 - CPU Write protection
    #[inline(always)]
    pub fn wpcpu(&mut self) -> WPCPU_W<SMPUFBIU_SPEC> {
        WPCPU_W::new(self, 1)
    }
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<SMPUFBIU_SPEC> {
        RPGRPA_W::new(self, 2)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<SMPUFBIU_SPEC> {
        WPGRPA_W::new(self, 3)
    }
}
/**Access Control Register for FBIU

You can [`read`](crate::Reg::read) this register and get [`smpufbiu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpufbiu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMPUFBIU_SPEC;
impl crate::RegisterSpec for SMPUFBIU_SPEC {
    type Ux = u16;
}
///`read()` method returns [`smpufbiu::R`](R) reader structure
impl crate::Readable for SMPUFBIU_SPEC {}
///`write(|w| ..)` method takes [`smpufbiu::W`](W) writer structure
impl crate::Writable for SMPUFBIU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPUFBIU to value 0
impl crate::Resettable for SMPUFBIU_SPEC {}
