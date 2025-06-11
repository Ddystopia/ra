///Register `DAADSCR` reader
pub type R = crate::R<DAADSCR_SPEC>;
///Register `DAADSCR` writer
pub type W = crate::W<DAADSCR_SPEC>;
/**D/A-A/D Synchronous Conversion

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAADST_A {
    ///0: D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled).
    _0 = 0,
    ///1: D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled).
    _1 = 1,
}
impl From<DAADST_A> for bool {
    #[inline(always)]
    fn from(variant: DAADST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAADST` reader - D/A-A/D Synchronous Conversion
pub type DAADST_R = crate::BitReader<DAADST_A>;
impl DAADST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAADST_A {
        match self.bits {
            false => DAADST_A::_0,
            true => DAADST_A::_1,
        }
    }
    ///D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAADST_A::_0
    }
    ///D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAADST_A::_1
    }
}
///Field `DAADST` writer - D/A-A/D Synchronous Conversion
pub type DAADST_W<'a, REG> = crate::BitWriter<'a, REG, DAADST_A>;
impl<'a, REG> DAADST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///D/A converter operation does not synchronize with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is disabled).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAADST_A::_0)
    }
    ///D/A converter operation synchronizes with A/D converter operation (unit 1) (countermeasure against interference between D/A and A/D conversions is enabled).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAADST_A::_1)
    }
}
impl R {
    ///Bit 7 - D/A-A/D Synchronous Conversion
    #[inline(always)]
    pub fn daadst(&self) -> DAADST_R {
        DAADST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - D/A-A/D Synchronous Conversion
    #[inline(always)]
    pub fn daadst(&mut self) -> DAADST_W<DAADSCR_SPEC> {
        DAADST_W::new(self, 7)
    }
}
/**D/A-A/D Synchronous Start Control Register

You can [`read`](crate::Reg::read) this register and get [`daadscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAADSCR_SPEC;
impl crate::RegisterSpec for DAADSCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`daadscr::R`](R) reader structure
impl crate::Readable for DAADSCR_SPEC {}
///`write(|w| ..)` method takes [`daadscr::W`](W) writer structure
impl crate::Writable for DAADSCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAADSCR to value 0
impl crate::Resettable for DAADSCR_SPEC {}
