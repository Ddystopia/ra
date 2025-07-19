///Register `SBYCR` reader
pub type R = crate::R<SBYCR_SPEC>;
///Register `SBYCR` writer
pub type W = crate::W<SBYCR_SPEC>;
/**Software Standby

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSBY_A {
    ///0: Sleep mode
    _0 = 0,
    ///1: Software Standby mode
    _1 = 1,
}
impl From<SSBY_A> for bool {
    #[inline(always)]
    fn from(variant: SSBY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSBY` reader - Software Standby
pub type SSBY_R = crate::BitReader<SSBY_A>;
impl SSBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSBY_A {
        match self.bits {
            false => SSBY_A::_0,
            true => SSBY_A::_1,
        }
    }
    ///Sleep mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSBY_A::_0
    }
    ///Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSBY_A::_1
    }
}
///Field `SSBY` writer - Software Standby
pub type SSBY_W<'a, REG> = crate::BitWriter<'a, REG, SSBY_A>;
impl<'a, REG> SSBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sleep mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSBY_A::_0)
    }
    ///Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSBY_A::_1)
    }
}
impl R {
    ///Bit 15 - Software Standby
    #[inline(always)]
    pub fn ssby(&self) -> SSBY_R {
        SSBY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - Software Standby
    #[inline(always)]
    pub fn ssby(&mut self) -> SSBY_W<'_, SBYCR_SPEC> {
        SSBY_W::new(self, 15)
    }
}
/**Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`sbycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SBYCR_SPEC;
impl crate::RegisterSpec for SBYCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`sbycr::R`](R) reader structure
impl crate::Readable for SBYCR_SPEC {}
///`write(|w| ..)` method takes [`sbycr::W`](W) writer structure
impl crate::Writable for SBYCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SBYCR to value 0x4000
impl crate::Resettable for SBYCR_SPEC {
    const RESET_VALUE: u16 = 0x4000;
}
