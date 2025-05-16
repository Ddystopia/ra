///Register `SCKSCR` reader
pub type R = crate::R<SCKSCR_SPEC>;
///Register `SCKSCR` writer
pub type W = crate::W<SCKSCR_SPEC>;
/**Clock Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSEL_A {
    ///0: HOCO
    _000 = 0,
    ///1: MOCO
    _001 = 1,
    ///2: LOCO
    _010 = 2,
    ///3: Main clock oscillator
    _011 = 3,
    ///4: Sub-clock oscillator
    _100 = 4,
    ///5: PLL
    _101 = 5,
    ///6: Setting prohibited
    OTHERS = 6,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CKSEL_A {}
///Field `CKSEL` reader - Clock Source Select
pub type CKSEL_R = crate::FieldReader<CKSEL_A>;
impl CKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKSEL_A {
        match self.bits {
            0 => CKSEL_A::_000,
            1 => CKSEL_A::_001,
            2 => CKSEL_A::_010,
            3 => CKSEL_A::_011,
            4 => CKSEL_A::_100,
            5 => CKSEL_A::_101,
            _ => CKSEL_A::OTHERS,
        }
    }
    ///HOCO
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKSEL_A::_000
    }
    ///MOCO
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKSEL_A::_001
    }
    ///LOCO
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKSEL_A::_010
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKSEL_A::_011
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKSEL_A::_100
    }
    ///PLL
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKSEL_A::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CKSEL_A::OTHERS)
    }
}
///Field `CKSEL` writer - Clock Source Select
pub type CKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CKSEL_A, crate::Safe>;
impl<'a, REG> CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HOCO
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_000)
    }
    ///MOCO
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_001)
    }
    ///LOCO
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_010)
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_011)
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_100)
    }
    ///PLL
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Clock Source Select
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - Clock Source Select
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<SCKSCR_SPEC> {
        CKSEL_W::new(self, 0)
    }
}
/**System Clock Source Control Register

You can [`read`](crate::Reg::read) this register and get [`sckscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCKSCR_SPEC;
impl crate::RegisterSpec for SCKSCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sckscr::R`](R) reader structure
impl crate::Readable for SCKSCR_SPEC {}
///`write(|w| ..)` method takes [`sckscr::W`](W) writer structure
impl crate::Writable for SCKSCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCKSCR to value 0x01
impl crate::Resettable for SCKSCR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
