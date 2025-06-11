///Register `COMPSEL0` reader
pub type R = crate::R<COMPSEL0_SPEC>;
///Register `COMPSEL0` writer
pub type W = crate::W<COMPSEL0_SPEC>;
/**ACMPLP0 Input(IVCMP0) Selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL20_A {
    ///0: No input
    _000 = 0,
    ///1: CMPIN0 (P100)
    _001 = 1,
    ///4: CMPIN0 (P503)
    _100 = 4,
    ///2: settings prohibited
    OTHERS = 2,
}
impl From<CMPSEL20_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL20_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPSEL20_A {
    type Ux = u8;
}
impl crate::IsEnum for CMPSEL20_A {}
///Field `CMPSEL20` reader - ACMPLP0 Input(IVCMP0) Selection
pub type CMPSEL20_R = crate::FieldReader<CMPSEL20_A>;
impl CMPSEL20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSEL20_A {
        match self.bits {
            0 => CMPSEL20_A::_000,
            1 => CMPSEL20_A::_001,
            4 => CMPSEL20_A::_100,
            _ => CMPSEL20_A::OTHERS,
        }
    }
    ///No input
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CMPSEL20_A::_000
    }
    ///CMPIN0 (P100)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CMPSEL20_A::_001
    }
    ///CMPIN0 (P503)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMPSEL20_A::_100
    }
    ///settings prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CMPSEL20_A::OTHERS)
    }
}
///Field `CMPSEL20` writer - ACMPLP0 Input(IVCMP0) Selection
pub type CMPSEL20_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CMPSEL20_A, crate::Safe>;
impl<'a, REG> CMPSEL20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No input
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::_000)
    }
    ///CMPIN0 (P100)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::_001)
    }
    ///CMPIN0 (P503)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::_100)
    }
    ///settings prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL20_A::OTHERS)
    }
}
/**ACMPLP1 Input (IVCMP1) Selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL64_A {
    ///0: No input
    _000 = 0,
    ///1: CMPIN1 (P102)
    _001 = 1,
    ///4: CMPIN1 (P501)
    _100 = 4,
    ///2: settings prohibited
    OTHERS = 2,
}
impl From<CMPSEL64_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL64_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPSEL64_A {
    type Ux = u8;
}
impl crate::IsEnum for CMPSEL64_A {}
///Field `CMPSEL64` reader - ACMPLP1 Input (IVCMP1) Selection
pub type CMPSEL64_R = crate::FieldReader<CMPSEL64_A>;
impl CMPSEL64_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSEL64_A {
        match self.bits {
            0 => CMPSEL64_A::_000,
            1 => CMPSEL64_A::_001,
            4 => CMPSEL64_A::_100,
            _ => CMPSEL64_A::OTHERS,
        }
    }
    ///No input
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CMPSEL64_A::_000
    }
    ///CMPIN1 (P102)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CMPSEL64_A::_001
    }
    ///CMPIN1 (P501)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CMPSEL64_A::_100
    }
    ///settings prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CMPSEL64_A::OTHERS)
    }
}
///Field `CMPSEL64` writer - ACMPLP1 Input (IVCMP1) Selection
pub type CMPSEL64_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CMPSEL64_A, crate::Safe>;
impl<'a, REG> CMPSEL64_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No input
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::_000)
    }
    ///CMPIN1 (P102)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::_001)
    }
    ///CMPIN1 (P501)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::_100)
    }
    ///settings prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL64_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection
    #[inline(always)]
    pub fn cmpsel20(&self) -> CMPSEL20_R {
        CMPSEL20_R::new(self.bits & 7)
    }
    ///Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection
    #[inline(always)]
    pub fn cmpsel64(&self) -> CMPSEL64_R {
        CMPSEL64_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    ///Bits 0:2 - ACMPLP0 Input(IVCMP0) Selection
    #[inline(always)]
    pub fn cmpsel20(&mut self) -> CMPSEL20_W<COMPSEL0_SPEC> {
        CMPSEL20_W::new(self, 0)
    }
    ///Bits 4:6 - ACMPLP1 Input (IVCMP1) Selection
    #[inline(always)]
    pub fn cmpsel64(&mut self) -> CMPSEL64_W<COMPSEL0_SPEC> {
        CMPSEL64_W::new(self, 4)
    }
}
/**Comparator Input Select Register

You can [`read`](crate::Reg::read) this register and get [`compsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMPSEL0_SPEC;
impl crate::RegisterSpec for COMPSEL0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`compsel0::R`](R) reader structure
impl crate::Readable for COMPSEL0_SPEC {}
///`write(|w| ..)` method takes [`compsel0::W`](W) writer structure
impl crate::Writable for COMPSEL0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMPSEL0 to value 0x11
impl crate::Resettable for COMPSEL0_SPEC {
    const RESET_VALUE: u8 = 0x11;
}
