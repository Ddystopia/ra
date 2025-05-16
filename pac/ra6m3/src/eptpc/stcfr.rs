///Register `STCFR` reader
pub type R = crate::R<STCFR_SPEC>;
///Register `STCFR` writer
pub type W = crate::W<STCFR_SPEC>;
/**STCA Clock Frequency

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STCF_A {
    ///0: 20MHz
    _00 = 0,
    ///1: 25MHz
    _01 = 1,
    ///2: 50MHz
    _10 = 2,
    ///3: 100 MHz
    _11 = 3,
}
impl From<STCF_A> for u8 {
    #[inline(always)]
    fn from(variant: STCF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STCF_A {
    type Ux = u8;
}
impl crate::IsEnum for STCF_A {}
///Field `STCF` reader - STCA Clock Frequency
pub type STCF_R = crate::FieldReader<STCF_A>;
impl STCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STCF_A {
        match self.bits {
            0 => STCF_A::_00,
            1 => STCF_A::_01,
            2 => STCF_A::_10,
            3 => STCF_A::_11,
            _ => unreachable!(),
        }
    }
    ///20MHz
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == STCF_A::_00
    }
    ///25MHz
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == STCF_A::_01
    }
    ///50MHz
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == STCF_A::_10
    }
    ///100 MHz
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == STCF_A::_11
    }
}
///Field `STCF` writer - STCA Clock Frequency
pub type STCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STCF_A, crate::Safe>;
impl<'a, REG> STCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///20MHz
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(STCF_A::_00)
    }
    ///25MHz
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(STCF_A::_01)
    }
    ///50MHz
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(STCF_A::_10)
    }
    ///100 MHz
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(STCF_A::_11)
    }
}
impl R {
    ///Bits 0:1 - STCA Clock Frequency
    #[inline(always)]
    pub fn stcf(&self) -> STCF_R {
        STCF_R::new((self.bits & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - STCA Clock Frequency
    #[inline(always)]
    pub fn stcf(&mut self) -> STCF_W<STCFR_SPEC> {
        STCF_W::new(self, 0)
    }
}
/**STCA Clock Frequency Setting Register

You can [`read`](crate::Reg::read) this register and get [`stcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STCFR_SPEC;
impl crate::RegisterSpec for STCFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`stcfr::R`](R) reader structure
impl crate::Readable for STCFR_SPEC {}
///`write(|w| ..)` method takes [`stcfr::W`](W) writer structure
impl crate::Writable for STCFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STCFR to value 0
impl crate::Resettable for STCFR_SPEC {}
