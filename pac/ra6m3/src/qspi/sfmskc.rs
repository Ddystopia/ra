///Register `SFMSKC` reader
pub type R = crate::R<SFMSKC_SPEC>;
///Register `SFMSKC` writer
pub type W = crate::W<SFMSKC_SPEC>;
/**Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction.

Value on reset: 8*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMDV_A {
    ///16: 18 x PCLKA
    _10000 = 16,
    ///17: 20 x PCLKA
    _10001 = 17,
    ///18: 22 x PCLKA
    _10010 = 18,
    ///19: 24 x PCLKA
    _10011 = 19,
    ///20: 26 x PCLKA
    _10100 = 20,
    ///21: 28 x PCLKA
    _10101 = 21,
    ///22: 30 x PCLKA
    _10110 = 22,
    ///23: 32 x PCLKA
    _10111 = 23,
    ///24: 34 x PCLKA
    _11000 = 24,
    ///25: 36 x PCLKA
    _11001 = 25,
    ///26: 38 x PCLKA
    _11010 = 26,
    ///27: 40 x PCLKA
    _11011 = 27,
    ///28: 42 x PCLKA
    _11100 = 28,
    ///29: 44 x PCLKA
    _11101 = 29,
    ///30: 46 x PCLKA
    _11110 = 30,
    ///31: 48 x PCLKA
    _11111 = 31,
    ///0: ( SFMDV + 2 ) x PCLKA
    OTHERS = 0,
}
impl From<SFMDV_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMDV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMDV_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMDV_A {}
///Field `SFMDV` reader - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction.
pub type SFMDV_R = crate::FieldReader<SFMDV_A>;
impl SFMDV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMDV_A {
        match self.bits {
            16 => SFMDV_A::_10000,
            17 => SFMDV_A::_10001,
            18 => SFMDV_A::_10010,
            19 => SFMDV_A::_10011,
            20 => SFMDV_A::_10100,
            21 => SFMDV_A::_10101,
            22 => SFMDV_A::_10110,
            23 => SFMDV_A::_10111,
            24 => SFMDV_A::_11000,
            25 => SFMDV_A::_11001,
            26 => SFMDV_A::_11010,
            27 => SFMDV_A::_11011,
            28 => SFMDV_A::_11100,
            29 => SFMDV_A::_11101,
            30 => SFMDV_A::_11110,
            31 => SFMDV_A::_11111,
            _ => SFMDV_A::OTHERS,
        }
    }
    ///18 x PCLKA
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == SFMDV_A::_10000
    }
    ///20 x PCLKA
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == SFMDV_A::_10001
    }
    ///22 x PCLKA
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == SFMDV_A::_10010
    }
    ///24 x PCLKA
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == SFMDV_A::_10011
    }
    ///26 x PCLKA
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == SFMDV_A::_10100
    }
    ///28 x PCLKA
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == SFMDV_A::_10101
    }
    ///30 x PCLKA
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == SFMDV_A::_10110
    }
    ///32 x PCLKA
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == SFMDV_A::_10111
    }
    ///34 x PCLKA
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == SFMDV_A::_11000
    }
    ///36 x PCLKA
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == SFMDV_A::_11001
    }
    ///38 x PCLKA
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == SFMDV_A::_11010
    }
    ///40 x PCLKA
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == SFMDV_A::_11011
    }
    ///42 x PCLKA
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == SFMDV_A::_11100
    }
    ///44 x PCLKA
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == SFMDV_A::_11101
    }
    ///46 x PCLKA
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == SFMDV_A::_11110
    }
    ///48 x PCLKA
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == SFMDV_A::_11111
    }
    ///( SFMDV + 2 ) x PCLKA
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SFMDV_A::OTHERS)
    }
}
///Field `SFMDV` writer - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction.
pub type SFMDV_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SFMDV_A, crate::Safe>;
impl<'a, REG> SFMDV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///18 x PCLKA
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10000)
    }
    ///20 x PCLKA
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10001)
    }
    ///22 x PCLKA
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10010)
    }
    ///24 x PCLKA
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10011)
    }
    ///26 x PCLKA
    #[inline(always)]
    pub fn _10100(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10100)
    }
    ///28 x PCLKA
    #[inline(always)]
    pub fn _10101(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10101)
    }
    ///30 x PCLKA
    #[inline(always)]
    pub fn _10110(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10110)
    }
    ///32 x PCLKA
    #[inline(always)]
    pub fn _10111(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_10111)
    }
    ///34 x PCLKA
    #[inline(always)]
    pub fn _11000(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11000)
    }
    ///36 x PCLKA
    #[inline(always)]
    pub fn _11001(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11001)
    }
    ///38 x PCLKA
    #[inline(always)]
    pub fn _11010(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11010)
    }
    ///40 x PCLKA
    #[inline(always)]
    pub fn _11011(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11011)
    }
    ///42 x PCLKA
    #[inline(always)]
    pub fn _11100(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11100)
    }
    ///44 x PCLKA
    #[inline(always)]
    pub fn _11101(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11101)
    }
    ///46 x PCLKA
    #[inline(always)]
    pub fn _11110(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11110)
    }
    ///48 x PCLKA
    #[inline(always)]
    pub fn _11111(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::_11111)
    }
    ///( SFMDV + 2 ) x PCLKA
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDV_A::OTHERS)
    }
}
/**Selection of a duty ratio correction function for the SCK signal

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMDTY_A {
    ///0: Serial interface reference cycle selection (* Pay attention to the irregularity.)
    _0 = 0,
    ///1: Delays the rising of the SCK signal by 0.5*PCLKA.(* Valid with PCLKA multiplied by an odd number)
    _1 = 1,
}
impl From<SFMDTY_A> for bool {
    #[inline(always)]
    fn from(variant: SFMDTY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMDTY` reader - Selection of a duty ratio correction function for the SCK signal
pub type SFMDTY_R = crate::BitReader<SFMDTY_A>;
impl SFMDTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMDTY_A {
        match self.bits {
            false => SFMDTY_A::_0,
            true => SFMDTY_A::_1,
        }
    }
    ///Serial interface reference cycle selection (* Pay attention to the irregularity.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMDTY_A::_0
    }
    ///Delays the rising of the SCK signal by 0.5*PCLKA.(* Valid with PCLKA multiplied by an odd number)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMDTY_A::_1
    }
}
///Field `SFMDTY` writer - Selection of a duty ratio correction function for the SCK signal
pub type SFMDTY_W<'a, REG> = crate::BitWriter<'a, REG, SFMDTY_A>;
impl<'a, REG> SFMDTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Serial interface reference cycle selection (* Pay attention to the irregularity.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDTY_A::_0)
    }
    ///Delays the rising of the SCK signal by 0.5*PCLKA.(* Valid with PCLKA multiplied by an odd number)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDTY_A::_1)
    }
}
impl R {
    ///Bits 0:4 - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction.
    #[inline(always)]
    pub fn sfmdv(&self) -> SFMDV_R {
        SFMDV_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Selection of a duty ratio correction function for the SCK signal
    #[inline(always)]
    pub fn sfmdty(&self) -> SFMDTY_R {
        SFMDTY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction.
    #[inline(always)]
    pub fn sfmdv(&mut self) -> SFMDV_W<SFMSKC_SPEC> {
        SFMDV_W::new(self, 0)
    }
    ///Bit 5 - Selection of a duty ratio correction function for the SCK signal
    #[inline(always)]
    pub fn sfmdty(&mut self) -> SFMDTY_W<SFMSKC_SPEC> {
        SFMDTY_W::new(self, 5)
    }
}
/**Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmskc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmskc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSKC_SPEC;
impl crate::RegisterSpec for SFMSKC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmskc::R`](R) reader structure
impl crate::Readable for SFMSKC_SPEC {}
///`write(|w| ..)` method takes [`sfmskc::W`](W) writer structure
impl crate::Writable for SFMSKC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSKC to value 0x08
impl crate::Resettable for SFMSKC_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
