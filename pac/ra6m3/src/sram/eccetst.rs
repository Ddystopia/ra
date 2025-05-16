///Register `ECCETST` reader
pub type R = crate::R<ECCETST_SPEC>;
///Register `ECCETST` writer
pub type W = crate::W<ECCETST_SPEC>;
/**ECC Bypass Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTBYP_A {
    ///0: ECC bypass disabled
    _0 = 0,
    ///1: ECC bypass enabled.
    _1 = 1,
}
impl From<TSTBYP_A> for bool {
    #[inline(always)]
    fn from(variant: TSTBYP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTBYP` reader - ECC Bypass Select
pub type TSTBYP_R = crate::BitReader<TSTBYP_A>;
impl TSTBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSTBYP_A {
        match self.bits {
            false => TSTBYP_A::_0,
            true => TSTBYP_A::_1,
        }
    }
    ///ECC bypass disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTBYP_A::_0
    }
    ///ECC bypass enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTBYP_A::_1
    }
}
///Field `TSTBYP` writer - ECC Bypass Select
pub type TSTBYP_W<'a, REG> = crate::BitWriter<'a, REG, TSTBYP_A>;
impl<'a, REG> TSTBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECC bypass disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTBYP_A::_0)
    }
    ///ECC bypass enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTBYP_A::_1)
    }
}
impl R {
    ///Bit 0 - ECC Bypass Select
    #[inline(always)]
    pub fn tstbyp(&self) -> TSTBYP_R {
        TSTBYP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ECC Bypass Select
    #[inline(always)]
    pub fn tstbyp(&mut self) -> TSTBYP_W<ECCETST_SPEC> {
        TSTBYP_W::new(self, 0)
    }
}
/**ECC Test Control Register

You can [`read`](crate::Reg::read) this register and get [`eccetst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccetst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECCETST_SPEC;
impl crate::RegisterSpec for ECCETST_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eccetst::R`](R) reader structure
impl crate::Readable for ECCETST_SPEC {}
///`write(|w| ..)` method takes [`eccetst::W`](W) writer structure
impl crate::Writable for ECCETST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCETST to value 0
impl crate::Resettable for ECCETST_SPEC {}
