///Register `DACR` reader
pub type R = crate::R<DACR_SPEC>;
///Register `DACR` writer
pub type W = crate::W<DACR_SPEC>;
/**D/A Output Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAOE0_A {
    ///0: Analog output of channel 0 (DA0) is disabled.
    _0 = 0,
    ///1: D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled.
    _1 = 1,
}
impl From<DAOE0_A> for bool {
    #[inline(always)]
    fn from(variant: DAOE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAOE0` reader - D/A Output Enable 0
pub type DAOE0_R = crate::BitReader<DAOE0_A>;
impl DAOE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAOE0_A {
        match self.bits {
            false => DAOE0_A::_0,
            true => DAOE0_A::_1,
        }
    }
    ///Analog output of channel 0 (DA0) is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAOE0_A::_0
    }
    ///D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAOE0_A::_1
    }
}
///Field `DAOE0` writer - D/A Output Enable 0
pub type DAOE0_W<'a, REG> = crate::BitWriter<'a, REG, DAOE0_A>;
impl<'a, REG> DAOE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog output of channel 0 (DA0) is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAOE0_A::_0)
    }
    ///D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAOE0_A::_1)
    }
}
impl R {
    ///Bit 6 - D/A Output Enable 0
    #[inline(always)]
    pub fn daoe0(&self) -> DAOE0_R {
        DAOE0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - D/A Output Enable 0
    #[inline(always)]
    pub fn daoe0(&mut self) -> DAOE0_W<'_, DACR_SPEC> {
        DAOE0_W::new(self, 6)
    }
}
/**D/A Control Register

You can [`read`](crate::Reg::read) this register and get [`dacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DACR_SPEC;
impl crate::RegisterSpec for DACR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dacr::R`](R) reader structure
impl crate::Readable for DACR_SPEC {}
///`write(|w| ..)` method takes [`dacr::W`](W) writer structure
impl crate::Writable for DACR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DACR to value 0x1f
impl crate::Resettable for DACR_SPEC {
    const RESET_VALUE: u8 = 0x1f;
}
