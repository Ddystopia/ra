///Register `HOCOWTCR` reader
pub type R = crate::R<HOCOWTCR_SPEC>;
///Register `HOCOWTCR` writer
pub type W = crate::W<HOCOWTCR_SPEC>;
/**HOCO wait time setting

Value on reset: 5*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSTS_A {
    ///5: If HOCO frequency is other than 64MHz, should set the value to 101b.
    _101 = 5,
    ///6: If HOCO frequency = 64MHz, should set the value to 110b.
    _110 = 6,
    ///0: Setting prohibited
    OTHERS = 0,
}
impl From<HSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSTS_A {
    type Ux = u8;
}
impl crate::IsEnum for HSTS_A {}
///Field `HSTS` reader - HOCO wait time setting
pub type HSTS_R = crate::FieldReader<HSTS_A>;
impl HSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSTS_A {
        match self.bits {
            5 => HSTS_A::_101,
            6 => HSTS_A::_110,
            _ => HSTS_A::OTHERS,
        }
    }
    ///If HOCO frequency is other than 64MHz, should set the value to 101b.
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == HSTS_A::_101
    }
    ///If HOCO frequency = 64MHz, should set the value to 110b.
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == HSTS_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), HSTS_A::OTHERS)
    }
}
///Field `HSTS` writer - HOCO wait time setting
pub type HSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HSTS_A, crate::Safe>;
impl<'a, REG> HSTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///If HOCO frequency is other than 64MHz, should set the value to 101b.
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(HSTS_A::_101)
    }
    ///If HOCO frequency = 64MHz, should set the value to 110b.
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(HSTS_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(HSTS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - HOCO wait time setting
    #[inline(always)]
    pub fn hsts(&self) -> HSTS_R {
        HSTS_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - HOCO wait time setting
    #[inline(always)]
    pub fn hsts(&mut self) -> HSTS_W<'_, HOCOWTCR_SPEC> {
        HSTS_W::new(self, 0)
    }
}
/**High-Speed On-Chip Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`hocowtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocowtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOCOWTCR_SPEC;
impl crate::RegisterSpec for HOCOWTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`hocowtcr::R`](R) reader structure
impl crate::Readable for HOCOWTCR_SPEC {}
///`write(|w| ..)` method takes [`hocowtcr::W`](W) writer structure
impl crate::Writable for HOCOWTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HOCOWTCR to value 0x05
impl crate::Resettable for HOCOWTCR_SPEC {
    const RESET_VALUE: u8 = 0x05;
}
