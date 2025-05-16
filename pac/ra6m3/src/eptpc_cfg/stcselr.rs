///Register `STCSELR` reader
pub type R = crate::R<STCSELR_SPEC>;
///Register `STCSELR` writer
pub type W = crate::W<STCSELR_SPEC>;
/**PCLKA Clock Frequency Division

Value on reset: 6*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKDIV_A {
    ///1: 1
    _001 = 1,
    ///2: 1/2
    _010 = 2,
    ///3: 1/3
    _011 = 3,
    ///4: 1/4
    _100 = 4,
    ///5: 1/5
    _101 = 5,
    ///6: 1/6
    _110 = 6,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
}
impl From<SCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCLKDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for SCLKDIV_A {}
///Field `SCLKDIV` reader - PCLKA Clock Frequency Division
pub type SCLKDIV_R = crate::FieldReader<SCLKDIV_A>;
impl SCLKDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCLKDIV_A {
        match self.bits {
            1 => SCLKDIV_A::_001,
            2 => SCLKDIV_A::_010,
            3 => SCLKDIV_A::_011,
            4 => SCLKDIV_A::_100,
            5 => SCLKDIV_A::_101,
            6 => SCLKDIV_A::_110,
            _ => SCLKDIV_A::OTHERS,
        }
    }
    ///1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCLKDIV_A::_001
    }
    ///1/2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCLKDIV_A::_010
    }
    ///1/3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCLKDIV_A::_011
    }
    ///1/4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SCLKDIV_A::_100
    }
    ///1/5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SCLKDIV_A::_101
    }
    ///1/6
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SCLKDIV_A::_110
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SCLKDIV_A::OTHERS)
    }
}
///Field `SCLKDIV` writer - PCLKA Clock Frequency Division
pub type SCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SCLKDIV_A, crate::Safe>;
impl<'a, REG> SCLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::_001)
    }
    ///1/2
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::_010)
    }
    ///1/3
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::_011)
    }
    ///1/4
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::_100)
    }
    ///1/5
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::_101)
    }
    ///1/6
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::_110)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKDIV_A::OTHERS)
    }
}
/**STCA Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKSEL_A {
    ///0: PCLKA clock divided by 1 to 6
    _000 = 0,
    ///2: Input clock from the REF50CK0 pin
    _010 = 2,
    ///1: Settings other than above are prohibited.
    OTHERS = 1,
}
impl From<SCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SCLKSEL_A {}
///Field `SCLKSEL` reader - STCA Clock Select
pub type SCLKSEL_R = crate::FieldReader<SCLKSEL_A>;
impl SCLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCLKSEL_A {
        match self.bits {
            0 => SCLKSEL_A::_000,
            2 => SCLKSEL_A::_010,
            _ => SCLKSEL_A::OTHERS,
        }
    }
    ///PCLKA clock divided by 1 to 6
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCLKSEL_A::_000
    }
    ///Input clock from the REF50CK0 pin
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCLKSEL_A::_010
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SCLKSEL_A::OTHERS)
    }
}
///Field `SCLKSEL` writer - STCA Clock Select
pub type SCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SCLKSEL_A, crate::Safe>;
impl<'a, REG> SCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKA clock divided by 1 to 6
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::_000)
    }
    ///Input clock from the REF50CK0 pin
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::_010)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - PCLKA Clock Frequency Division
    #[inline(always)]
    pub fn sclkdiv(&self) -> SCLKDIV_R {
        SCLKDIV_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - STCA Clock Select
    #[inline(always)]
    pub fn sclksel(&self) -> SCLKSEL_R {
        SCLKSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - PCLKA Clock Frequency Division
    #[inline(always)]
    pub fn sclkdiv(&mut self) -> SCLKDIV_W<STCSELR_SPEC> {
        SCLKDIV_W::new(self, 0)
    }
    ///Bits 8:10 - STCA Clock Select
    #[inline(always)]
    pub fn sclksel(&mut self) -> SCLKSEL_W<STCSELR_SPEC> {
        SCLKSEL_W::new(self, 8)
    }
}
/**STCA Clock Select Register

You can [`read`](crate::Reg::read) this register and get [`stcselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STCSELR_SPEC;
impl crate::RegisterSpec for STCSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`stcselr::R`](R) reader structure
impl crate::Readable for STCSELR_SPEC {}
///`write(|w| ..)` method takes [`stcselr::W`](W) writer structure
impl crate::Writable for STCSELR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STCSELR to value 0x06
impl crate::Resettable for STCSELR_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
