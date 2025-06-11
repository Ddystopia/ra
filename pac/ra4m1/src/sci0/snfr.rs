///Register `SNFR` reader
pub type R = crate::R<SNFR_SPEC>;
///Register `SNFR` writer
pub type W = crate::W<SNFR_SPEC>;
/**Noise Filter Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    ///0: The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)
    _000 = 0,
    ///1: The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)
    _001 = 1,
    ///2: The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)
    _010 = 2,
    ///3: The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)
    _011 = 3,
    ///4: The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)
    _100 = 4,
    ///5: Settings prohibited.
    OTHERS = 5,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCS_A {
    type Ux = u8;
}
impl crate::IsEnum for NFCS_A {}
///Field `NFCS` reader - Noise Filter Clock Select
pub type NFCS_R = crate::FieldReader<NFCS_A>;
impl NFCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_000,
            1 => NFCS_A::_001,
            2 => NFCS_A::_010,
            3 => NFCS_A::_011,
            4 => NFCS_A::_100,
            _ => NFCS_A::OTHERS,
        }
    }
    ///The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == NFCS_A::_000
    }
    ///The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == NFCS_A::_001
    }
    ///The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == NFCS_A::_010
    }
    ///The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == NFCS_A::_011
    }
    ///The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == NFCS_A::_100
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), NFCS_A::OTHERS)
    }
}
///Field `NFCS` writer - Noise Filter Clock Select
pub type NFCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, NFCS_A, crate::Safe>;
impl<'a, REG> NFCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_000)
    }
    ///The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_001)
    }
    ///The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_010)
    }
    ///The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_011)
    }
    ///The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_100)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Noise Filter Clock Select
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - Noise Filter Clock Select
    #[inline(always)]
    pub fn nfcs(&mut self) -> NFCS_W<SNFR_SPEC> {
        NFCS_W::new(self, 0)
    }
}
/**Noise Filter Setting Register

You can [`read`](crate::Reg::read) this register and get [`snfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SNFR_SPEC;
impl crate::RegisterSpec for SNFR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`snfr::R`](R) reader structure
impl crate::Readable for SNFR_SPEC {}
///`write(|w| ..)` method takes [`snfr::W`](W) writer structure
impl crate::Writable for SNFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNFR to value 0
impl crate::Resettable for SNFR_SPEC {}
