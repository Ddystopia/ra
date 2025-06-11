///Register `SOPCCR` reader
pub type R = crate::R<SOPCCR_SPEC>;
///Register `SOPCCR` writer
pub type W = crate::W<SOPCCR_SPEC>;
/**Sub Operating Power Control Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOPCM_A {
    ///0: Other than Subosc-speed mode
    _0 = 0,
    ///1: Subosc-speed mode
    _1 = 1,
}
impl From<SOPCM_A> for bool {
    #[inline(always)]
    fn from(variant: SOPCM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOPCM` reader - Sub Operating Power Control Mode Select
pub type SOPCM_R = crate::BitReader<SOPCM_A>;
impl SOPCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOPCM_A {
        match self.bits {
            false => SOPCM_A::_0,
            true => SOPCM_A::_1,
        }
    }
    ///Other than Subosc-speed mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOPCM_A::_0
    }
    ///Subosc-speed mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOPCM_A::_1
    }
}
///Field `SOPCM` writer - Sub Operating Power Control Mode Select
pub type SOPCM_W<'a, REG> = crate::BitWriter<'a, REG, SOPCM_A>;
impl<'a, REG> SOPCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Other than Subosc-speed mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOPCM_A::_0)
    }
    ///Subosc-speed mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOPCM_A::_1)
    }
}
/**Sub Operating Power Control Mode Transition Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOPCMTSF_A {
    ///0: Transition completed
    _0 = 0,
    ///1: During transition
    _1 = 1,
}
impl From<SOPCMTSF_A> for bool {
    #[inline(always)]
    fn from(variant: SOPCMTSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOPCMTSF` reader - Sub Operating Power Control Mode Transition Status Flag
pub type SOPCMTSF_R = crate::BitReader<SOPCMTSF_A>;
impl SOPCMTSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOPCMTSF_A {
        match self.bits {
            false => SOPCMTSF_A::_0,
            true => SOPCMTSF_A::_1,
        }
    }
    ///Transition completed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOPCMTSF_A::_0
    }
    ///During transition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOPCMTSF_A::_1
    }
}
impl R {
    ///Bit 0 - Sub Operating Power Control Mode Select
    #[inline(always)]
    pub fn sopcm(&self) -> SOPCM_R {
        SOPCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Sub Operating Power Control Mode Transition Status Flag
    #[inline(always)]
    pub fn sopcmtsf(&self) -> SOPCMTSF_R {
        SOPCMTSF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sub Operating Power Control Mode Select
    #[inline(always)]
    pub fn sopcm(&mut self) -> SOPCM_W<SOPCCR_SPEC> {
        SOPCM_W::new(self, 0)
    }
}
/**Sub Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`sopccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SOPCCR_SPEC;
impl crate::RegisterSpec for SOPCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sopccr::R`](R) reader structure
impl crate::Readable for SOPCCR_SPEC {}
///`write(|w| ..)` method takes [`sopccr::W`](W) writer structure
impl crate::Writable for SOPCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOPCCR to value 0
impl crate::Resettable for SOPCCR_SPEC {}
