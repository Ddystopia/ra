///Register `ADCMPSER` reader
pub type R = crate::R<ADCMPSER_SPEC>;
///Register `ADCMPSER` writer
pub type W = crate::W<ADCMPSER_SPEC>;
/**Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTTSA_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTTSA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTTSA_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTTSA` reader - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTTSA_R = crate::BitReader<CMPSTTSA_A>;
impl CMPSTTSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTTSA_A {
        match self.bits {
            false => CMPSTTSA_A::_0,
            true => CMPSTTSA_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTTSA_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTTSA_A::_1
    }
}
///Field `CMPSTTSA` writer - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.
pub type CMPSTTSA_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTTSA_A>;
impl<'a, REG> CMPSTTSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTTSA_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTTSA_A::_1)
    }
}
/**Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTOCA_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTOCA_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTOCA_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTOCA` reader - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTOCA_R = crate::BitReader<CMPSTOCA_A>;
impl CMPSTOCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTOCA_A {
        match self.bits {
            false => CMPSTOCA_A::_0,
            true => CMPSTOCA_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTOCA_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTOCA_A::_1
    }
}
///Field `CMPSTOCA` writer - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.
pub type CMPSTOCA_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTOCA_A>;
impl<'a, REG> CMPSTOCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTOCA_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTOCA_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.
    #[inline(always)]
    pub fn cmpsttsa(&self) -> CMPSTTSA_R {
        CMPSTTSA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.
    #[inline(always)]
    pub fn cmpstoca(&self) -> CMPSTOCA_R {
        CMPSTOCA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.
    #[inline(always)]
    pub fn cmpsttsa(&mut self) -> CMPSTTSA_W<ADCMPSER_SPEC> {
        CMPSTTSA_W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time.
    #[inline(always)]
    pub fn cmpstoca(&mut self) -> CMPSTOCA_W<ADCMPSER_SPEC> {
        CMPSTOCA_W::new(self, 1)
    }
}
/**A/D Compare Function Window A Extended Input Channel Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPSER_SPEC;
impl crate::RegisterSpec for ADCMPSER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adcmpser::R`](R) reader structure
impl crate::Readable for ADCMPSER_SPEC {}
///`write(|w| ..)` method takes [`adcmpser::W`](W) writer structure
impl crate::Writable for ADCMPSER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
///`reset()` method sets ADCMPSER to value 0
impl crate::Resettable for ADCMPSER_SPEC {}
