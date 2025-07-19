///Register `SMPUMBIU` reader
pub type R = crate::R<SMPUMBIU_SPEC>;
///Register `SMPUMBIU` writer
pub type W = crate::W<SMPUMBIU_SPEC>;
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
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rpgrpa(&mut self) -> RPGRPA_W<'_, SMPUMBIU_SPEC> {
        RPGRPA_W::new(self, 2)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wpgrpa(&mut self) -> WPGRPA_W<'_, SMPUMBIU_SPEC> {
        WPGRPA_W::new(self, 3)
    }
}
/**Access Control Register for MBIU

You can [`read`](crate::Reg::read) this register and get [`smpumbiu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpumbiu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMPUMBIU_SPEC;
impl crate::RegisterSpec for SMPUMBIU_SPEC {
    type Ux = u16;
}
///`read()` method returns [`smpumbiu::R`](R) reader structure
impl crate::Readable for SMPUMBIU_SPEC {}
///`write(|w| ..)` method takes [`smpumbiu::W`](W) writer structure
impl crate::Writable for SMPUMBIU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPUMBIU to value 0
impl crate::Resettable for SMPUMBIU_SPEC {}
