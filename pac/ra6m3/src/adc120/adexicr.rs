///Register `ADEXICR` reader
pub type R = crate::R<ADEXICR_SPEC>;
///Register `ADEXICR` writer
pub type W = crate::W<ADEXICR_SPEC>;
/**Temperature Sensor Output A/D converted Value Addition/Average Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSAD_A {
    ///0: Temperature sensor output A/D-converted value addition/average mode is not selected.
    _0 = 0,
    ///1: Temperature sensor output A/D-converted value addition/average mode is selected.
    _1 = 1,
}
impl From<TSSAD_A> for bool {
    #[inline(always)]
    fn from(variant: TSSAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSAD` reader - Temperature Sensor Output A/D converted Value Addition/Average Mode Select
pub type TSSAD_R = crate::BitReader<TSSAD_A>;
impl TSSAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSSAD_A {
        match self.bits {
            false => TSSAD_A::_0,
            true => TSSAD_A::_1,
        }
    }
    ///Temperature sensor output A/D-converted value addition/average mode is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSAD_A::_0
    }
    ///Temperature sensor output A/D-converted value addition/average mode is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSAD_A::_1
    }
}
///Field `TSSAD` writer - Temperature Sensor Output A/D converted Value Addition/Average Mode Select
pub type TSSAD_W<'a, REG> = crate::BitWriter<'a, REG, TSSAD_A>;
impl<'a, REG> TSSAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor output A/D-converted value addition/average mode is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSSAD_A::_0)
    }
    ///Temperature sensor output A/D-converted value addition/average mode is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSSAD_A::_1)
    }
}
/**Internal Reference Voltage A/D converted Value Addition/Average Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCSAD_A {
    ///0: Internal reference voltage A/D-converted value addition/average mode is not selected.
    _0 = 0,
    ///1: Internal reference voltage A/D-converted value addition/average mode is selected.
    _1 = 1,
}
impl From<OCSAD_A> for bool {
    #[inline(always)]
    fn from(variant: OCSAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OCSAD` reader - Internal Reference Voltage A/D converted Value Addition/Average Mode Select
pub type OCSAD_R = crate::BitReader<OCSAD_A>;
impl OCSAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCSAD_A {
        match self.bits {
            false => OCSAD_A::_0,
            true => OCSAD_A::_1,
        }
    }
    ///Internal reference voltage A/D-converted value addition/average mode is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSAD_A::_0
    }
    ///Internal reference voltage A/D-converted value addition/average mode is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSAD_A::_1
    }
}
///Field `OCSAD` writer - Internal Reference Voltage A/D converted Value Addition/Average Mode Select
pub type OCSAD_W<'a, REG> = crate::BitWriter<'a, REG, OCSAD_A>;
impl<'a, REG> OCSAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal reference voltage A/D-converted value addition/average mode is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OCSAD_A::_0)
    }
    ///Internal reference voltage A/D-converted value addition/average mode is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OCSAD_A::_1)
    }
}
/**Temperature Sensor Output A/D Conversion Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSA_A {
    ///0: The temperature sensor output is not selected.
    _0 = 0,
    ///1: The temperature sensor output is selected.
    _1 = 1,
}
impl From<TSSA_A> for bool {
    #[inline(always)]
    fn from(variant: TSSA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSA` reader - Temperature Sensor Output A/D Conversion Select
pub type TSSA_R = crate::BitReader<TSSA_A>;
impl TSSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSSA_A {
        match self.bits {
            false => TSSA_A::_0,
            true => TSSA_A::_1,
        }
    }
    ///The temperature sensor output is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSA_A::_0
    }
    ///The temperature sensor output is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSA_A::_1
    }
}
///Field `TSSA` writer - Temperature Sensor Output A/D Conversion Select
pub type TSSA_W<'a, REG> = crate::BitWriter<'a, REG, TSSA_A>;
impl<'a, REG> TSSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The temperature sensor output is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSSA_A::_0)
    }
    ///The temperature sensor output is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSSA_A::_1)
    }
}
/**Internal Reference Voltage A/D Conversion Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCSA_A {
    ///0: The internal reference voltage is not selected.
    _0 = 0,
    ///1: The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode.
    _1 = 1,
}
impl From<OCSA_A> for bool {
    #[inline(always)]
    fn from(variant: OCSA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OCSA` reader - Internal Reference Voltage A/D Conversion Select
pub type OCSA_R = crate::BitReader<OCSA_A>;
impl OCSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCSA_A {
        match self.bits {
            false => OCSA_A::_0,
            true => OCSA_A::_1,
        }
    }
    ///The internal reference voltage is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSA_A::_0
    }
    ///The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSA_A::_1
    }
}
///Field `OCSA` writer - Internal Reference Voltage A/D Conversion Select
pub type OCSA_W<'a, REG> = crate::BitWriter<'a, REG, OCSA_A>;
impl<'a, REG> OCSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The internal reference voltage is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OCSA_A::_0)
    }
    ///The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OCSA_A::_1)
    }
}
/**Temperature Sensor Output A/D Conversion Select for Group B

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSSB_A {
    ///0: The temperature sensor output is not selected.
    _0 = 0,
    ///1: The temperature sensor output is not selected for group B in group scan mode.
    _1 = 1,
}
impl From<TSSB_A> for bool {
    #[inline(always)]
    fn from(variant: TSSB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSB` reader - Temperature Sensor Output A/D Conversion Select for Group B
pub type TSSB_R = crate::BitReader<TSSB_A>;
impl TSSB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSSB_A {
        match self.bits {
            false => TSSB_A::_0,
            true => TSSB_A::_1,
        }
    }
    ///The temperature sensor output is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSSB_A::_0
    }
    ///The temperature sensor output is not selected for group B in group scan mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSSB_A::_1
    }
}
///Field `TSSB` writer - Temperature Sensor Output A/D Conversion Select for Group B
pub type TSSB_W<'a, REG> = crate::BitWriter<'a, REG, TSSB_A>;
impl<'a, REG> TSSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The temperature sensor output is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSSB_A::_0)
    }
    ///The temperature sensor output is not selected for group B in group scan mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSSB_A::_1)
    }
}
/**Internal Reference Voltage A/D Conversion Select for Group B

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCSB_A {
    ///0: The internal reference voltage is not selected.
    _0 = 0,
    ///1: The internal reference voltage is selected for group B in group scan mode.
    _1 = 1,
}
impl From<OCSB_A> for bool {
    #[inline(always)]
    fn from(variant: OCSB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OCSB` reader - Internal Reference Voltage A/D Conversion Select for Group B
pub type OCSB_R = crate::BitReader<OCSB_A>;
impl OCSB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCSB_A {
        match self.bits {
            false => OCSB_A::_0,
            true => OCSB_A::_1,
        }
    }
    ///The internal reference voltage is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCSB_A::_0
    }
    ///The internal reference voltage is selected for group B in group scan mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCSB_A::_1
    }
}
///Field `OCSB` writer - Internal Reference Voltage A/D Conversion Select for Group B
pub type OCSB_W<'a, REG> = crate::BitWriter<'a, REG, OCSB_A>;
impl<'a, REG> OCSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The internal reference voltage is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OCSB_A::_0)
    }
    ///The internal reference voltage is selected for group B in group scan mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OCSB_A::_1)
    }
}
impl R {
    ///Bit 0 - Temperature Sensor Output A/D converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn tssad(&self) -> TSSAD_R {
        TSSAD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal Reference Voltage A/D converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn ocsad(&self) -> OCSAD_R {
        OCSAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Temperature Sensor Output A/D Conversion Select
    #[inline(always)]
    pub fn tssa(&self) -> TSSA_R {
        TSSA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Internal Reference Voltage A/D Conversion Select
    #[inline(always)]
    pub fn ocsa(&self) -> OCSA_R {
        OCSA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Temperature Sensor Output A/D Conversion Select for Group B
    #[inline(always)]
    pub fn tssb(&self) -> TSSB_R {
        TSSB_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Internal Reference Voltage A/D Conversion Select for Group B
    #[inline(always)]
    pub fn ocsb(&self) -> OCSB_R {
        OCSB_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Temperature Sensor Output A/D converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn tssad(&mut self) -> TSSAD_W<ADEXICR_SPEC> {
        TSSAD_W::new(self, 0)
    }
    ///Bit 1 - Internal Reference Voltage A/D converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn ocsad(&mut self) -> OCSAD_W<ADEXICR_SPEC> {
        OCSAD_W::new(self, 1)
    }
    ///Bit 8 - Temperature Sensor Output A/D Conversion Select
    #[inline(always)]
    pub fn tssa(&mut self) -> TSSA_W<ADEXICR_SPEC> {
        TSSA_W::new(self, 8)
    }
    ///Bit 9 - Internal Reference Voltage A/D Conversion Select
    #[inline(always)]
    pub fn ocsa(&mut self) -> OCSA_W<ADEXICR_SPEC> {
        OCSA_W::new(self, 9)
    }
    ///Bit 10 - Temperature Sensor Output A/D Conversion Select for Group B
    #[inline(always)]
    pub fn tssb(&mut self) -> TSSB_W<ADEXICR_SPEC> {
        TSSB_W::new(self, 10)
    }
    ///Bit 11 - Internal Reference Voltage A/D Conversion Select for Group B
    #[inline(always)]
    pub fn ocsb(&mut self) -> OCSB_W<ADEXICR_SPEC> {
        OCSB_W::new(self, 11)
    }
}
/**A/D Conversion Extended Input Control Register

You can [`read`](crate::Reg::read) this register and get [`adexicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adexicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADEXICR_SPEC;
impl crate::RegisterSpec for ADEXICR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adexicr::R`](R) reader structure
impl crate::Readable for ADEXICR_SPEC {}
///`write(|w| ..)` method takes [`adexicr::W`](W) writer structure
impl crate::Writable for ADEXICR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADEXICR to value 0
impl crate::Resettable for ADEXICR_SPEC {}
