///Register `ADCMPANSER` reader
pub type R = crate::R<ADCMPANSER_SPEC>;
///Register `ADCMPANSER` writer
pub type W = crate::W<ADCMPANSER_SPEC>;
/**Temperature sensor output Compare selection bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPTSA_A {
    ///0: Excludes the temperature sensor output from the compare window A target range.
    _0 = 0,
    ///1: Includes the temperature sensor output in the compare window A target range.
    _1 = 1,
}
impl From<CMPTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTSA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPTSA` reader - Temperature sensor output Compare selection bit.
pub type CMPTSA_R = crate::BitReader<CMPTSA_A>;
impl CMPTSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPTSA_A {
        match self.bits {
            false => CMPTSA_A::_0,
            true => CMPTSA_A::_1,
        }
    }
    ///Excludes the temperature sensor output from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPTSA_A::_0
    }
    ///Includes the temperature sensor output in the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPTSA_A::_1
    }
}
///Field `CMPTSA` writer - Temperature sensor output Compare selection bit.
pub type CMPTSA_W<'a, REG> = crate::BitWriter<'a, REG, CMPTSA_A>;
impl<'a, REG> CMPTSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes the temperature sensor output from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPTSA_A::_0)
    }
    ///Includes the temperature sensor output in the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPTSA_A::_1)
    }
}
/**Internal reference voltage Compare selection bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOCA_A {
    ///0: Excludes the internal reference voltage from the compare window A target range.
    _0 = 0,
    ///1: Includes the internal reference voltage in the compare window A target range.
    _1 = 1,
}
impl From<CMPOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOCA` reader - Internal reference voltage Compare selection bit.
pub type CMPOCA_R = crate::BitReader<CMPOCA_A>;
impl CMPOCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPOCA_A {
        match self.bits {
            false => CMPOCA_A::_0,
            true => CMPOCA_A::_1,
        }
    }
    ///Excludes the internal reference voltage from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPOCA_A::_0
    }
    ///Includes the internal reference voltage in the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPOCA_A::_1
    }
}
///Field `CMPOCA` writer - Internal reference voltage Compare selection bit.
pub type CMPOCA_W<'a, REG> = crate::BitWriter<'a, REG, CMPOCA_A>;
impl<'a, REG> CMPOCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes the internal reference voltage from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOCA_A::_0)
    }
    ///Includes the internal reference voltage in the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOCA_A::_1)
    }
}
impl R {
    ///Bit 0 - Temperature sensor output Compare selection bit.
    #[inline(always)]
    pub fn cmptsa(&self) -> CMPTSA_R {
        CMPTSA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal reference voltage Compare selection bit.
    #[inline(always)]
    pub fn cmpoca(&self) -> CMPOCA_R {
        CMPOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Temperature sensor output Compare selection bit.
    #[inline(always)]
    pub fn cmptsa(&mut self) -> CMPTSA_W<ADCMPANSER_SPEC> {
        CMPTSA_W::new(self, 0)
    }
    ///Bit 1 - Internal reference voltage Compare selection bit.
    #[inline(always)]
    pub fn cmpoca(&mut self) -> CMPOCA_W<ADCMPANSER_SPEC> {
        CMPOCA_W::new(self, 1)
    }
}
/**A/D Compare Function Window A Extended Input Select Register

You can [`read`](crate::Reg::read) this register and get [`adcmpanser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpanser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPANSER_SPEC;
impl crate::RegisterSpec for ADCMPANSER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adcmpanser::R`](R) reader structure
impl crate::Readable for ADCMPANSER_SPEC {}
///`write(|w| ..)` method takes [`adcmpanser::W`](W) writer structure
impl crate::Writable for ADCMPANSER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPANSER to value 0
impl crate::Resettable for ADCMPANSER_SPEC {}
