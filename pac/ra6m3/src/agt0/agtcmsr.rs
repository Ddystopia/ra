///Register `AGTCMSR` reader
pub type R = crate::R<AGTCMSR_SPEC>;
///Register `AGTCMSR` writer
pub type W = crate::W<AGTCMSR_SPEC>;
/**Compare match A register enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMEA_A {
    ///0: Disable compare match A register
    _0 = 0,
    ///1: Enable compare match A register
    _1 = 1,
}
impl From<TCMEA_A> for bool {
    #[inline(always)]
    fn from(variant: TCMEA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCMEA` reader - Compare match A register enable
pub type TCMEA_R = crate::BitReader<TCMEA_A>;
impl TCMEA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCMEA_A {
        match self.bits {
            false => TCMEA_A::_0,
            true => TCMEA_A::_1,
        }
    }
    ///Disable compare match A register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMEA_A::_0
    }
    ///Enable compare match A register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMEA_A::_1
    }
}
///Field `TCMEA` writer - Compare match A register enable
pub type TCMEA_W<'a, REG> = crate::BitWriter<'a, REG, TCMEA_A>;
impl<'a, REG> TCMEA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare match A register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCMEA_A::_0)
    }
    ///Enable compare match A register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCMEA_A::_1)
    }
}
/**AGTOA output enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOEA_A {
    ///0: AGTOA output disabled (port)
    _0 = 0,
    ///1: AGTOA output enabled
    _1 = 1,
}
impl From<TOEA_A> for bool {
    #[inline(always)]
    fn from(variant: TOEA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOEA` reader - AGTOA output enable
pub type TOEA_R = crate::BitReader<TOEA_A>;
impl TOEA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOEA_A {
        match self.bits {
            false => TOEA_A::_0,
            true => TOEA_A::_1,
        }
    }
    ///AGTOA output disabled (port)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOEA_A::_0
    }
    ///AGTOA output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOEA_A::_1
    }
}
///Field `TOEA` writer - AGTOA output enable
pub type TOEA_W<'a, REG> = crate::BitWriter<'a, REG, TOEA_A>;
impl<'a, REG> TOEA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOA output disabled (port)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOEA_A::_0)
    }
    ///AGTOA output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOEA_A::_1)
    }
}
/**AGTOA polarity select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOPOLA_A {
    ///0: AGTOA Output is started at low
    _0 = 0,
    ///1: AGTOA Output is started at high
    _1 = 1,
}
impl From<TOPOLA_A> for bool {
    #[inline(always)]
    fn from(variant: TOPOLA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOPOLA` reader - AGTOA polarity select
pub type TOPOLA_R = crate::BitReader<TOPOLA_A>;
impl TOPOLA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOPOLA_A {
        match self.bits {
            false => TOPOLA_A::_0,
            true => TOPOLA_A::_1,
        }
    }
    ///AGTOA Output is started at low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOPOLA_A::_0
    }
    ///AGTOA Output is started at high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOPOLA_A::_1
    }
}
///Field `TOPOLA` writer - AGTOA polarity select
pub type TOPOLA_W<'a, REG> = crate::BitWriter<'a, REG, TOPOLA_A>;
impl<'a, REG> TOPOLA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOA Output is started at low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOPOLA_A::_0)
    }
    ///AGTOA Output is started at high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOPOLA_A::_1)
    }
}
/**Compare match B register enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMEB_A {
    ///0: Disable compare match B register
    _0 = 0,
    ///1: Enable compare match B register
    _1 = 1,
}
impl From<TCMEB_A> for bool {
    #[inline(always)]
    fn from(variant: TCMEB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCMEB` reader - Compare match B register enable
pub type TCMEB_R = crate::BitReader<TCMEB_A>;
impl TCMEB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCMEB_A {
        match self.bits {
            false => TCMEB_A::_0,
            true => TCMEB_A::_1,
        }
    }
    ///Disable compare match B register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCMEB_A::_0
    }
    ///Enable compare match B register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCMEB_A::_1
    }
}
///Field `TCMEB` writer - Compare match B register enable
pub type TCMEB_W<'a, REG> = crate::BitWriter<'a, REG, TCMEB_A>;
impl<'a, REG> TCMEB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare match B register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCMEB_A::_0)
    }
    ///Enable compare match B register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCMEB_A::_1)
    }
}
/**AGTOB output enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOEB_A {
    ///0: AGTOB output disabled (port)
    _0 = 0,
    ///1: AGTOB output enabled
    _1 = 1,
}
impl From<TOEB_A> for bool {
    #[inline(always)]
    fn from(variant: TOEB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOEB` reader - AGTOB output enable
pub type TOEB_R = crate::BitReader<TOEB_A>;
impl TOEB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOEB_A {
        match self.bits {
            false => TOEB_A::_0,
            true => TOEB_A::_1,
        }
    }
    ///AGTOB output disabled (port)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOEB_A::_0
    }
    ///AGTOB output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOEB_A::_1
    }
}
///Field `TOEB` writer - AGTOB output enable
pub type TOEB_W<'a, REG> = crate::BitWriter<'a, REG, TOEB_A>;
impl<'a, REG> TOEB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOB output disabled (port)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOEB_A::_0)
    }
    ///AGTOB output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOEB_A::_1)
    }
}
/**AGTOB polarity select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOPOLB_A {
    ///0: AGTOB Output is started at low
    _0 = 0,
    ///1: AGTOB Output is started at high
    _1 = 1,
}
impl From<TOPOLB_A> for bool {
    #[inline(always)]
    fn from(variant: TOPOLB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOPOLB` reader - AGTOB polarity select
pub type TOPOLB_R = crate::BitReader<TOPOLB_A>;
impl TOPOLB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOPOLB_A {
        match self.bits {
            false => TOPOLB_A::_0,
            true => TOPOLB_A::_1,
        }
    }
    ///AGTOB Output is started at low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOPOLB_A::_0
    }
    ///AGTOB Output is started at high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOPOLB_A::_1
    }
}
///Field `TOPOLB` writer - AGTOB polarity select
pub type TOPOLB_W<'a, REG> = crate::BitWriter<'a, REG, TOPOLB_A>;
impl<'a, REG> TOPOLB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOB Output is started at low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOPOLB_A::_0)
    }
    ///AGTOB Output is started at high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOPOLB_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare match A register enable
    #[inline(always)]
    pub fn tcmea(&self) -> TCMEA_R {
        TCMEA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AGTOA output enable
    #[inline(always)]
    pub fn toea(&self) -> TOEA_R {
        TOEA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGTOA polarity select
    #[inline(always)]
    pub fn topola(&self) -> TOPOLA_R {
        TOPOLA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Compare match B register enable
    #[inline(always)]
    pub fn tcmeb(&self) -> TCMEB_R {
        TCMEB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AGTOB output enable
    #[inline(always)]
    pub fn toeb(&self) -> TOEB_R {
        TOEB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AGTOB polarity select
    #[inline(always)]
    pub fn topolb(&self) -> TOPOLB_R {
        TOPOLB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare match A register enable
    #[inline(always)]
    pub fn tcmea(&mut self) -> TCMEA_W<AGTCMSR_SPEC> {
        TCMEA_W::new(self, 0)
    }
    ///Bit 1 - AGTOA output enable
    #[inline(always)]
    pub fn toea(&mut self) -> TOEA_W<AGTCMSR_SPEC> {
        TOEA_W::new(self, 1)
    }
    ///Bit 2 - AGTOA polarity select
    #[inline(always)]
    pub fn topola(&mut self) -> TOPOLA_W<AGTCMSR_SPEC> {
        TOPOLA_W::new(self, 2)
    }
    ///Bit 4 - Compare match B register enable
    #[inline(always)]
    pub fn tcmeb(&mut self) -> TCMEB_W<AGTCMSR_SPEC> {
        TCMEB_W::new(self, 4)
    }
    ///Bit 5 - AGTOB output enable
    #[inline(always)]
    pub fn toeb(&mut self) -> TOEB_W<AGTCMSR_SPEC> {
        TOEB_W::new(self, 5)
    }
    ///Bit 6 - AGTOB polarity select
    #[inline(always)]
    pub fn topolb(&mut self) -> TOPOLB_W<AGTCMSR_SPEC> {
        TOPOLB_W::new(self, 6)
    }
}
/**AGT Compare Match Function Select Register

You can [`read`](crate::Reg::read) this register and get [`agtcmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTCMSR_SPEC;
impl crate::RegisterSpec for AGTCMSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtcmsr::R`](R) reader structure
impl crate::Readable for AGTCMSR_SPEC {}
///`write(|w| ..)` method takes [`agtcmsr::W`](W) writer structure
impl crate::Writable for AGTCMSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCMSR to value 0
impl crate::Resettable for AGTCMSR_SPEC {}
