///Register `GTIOR` reader
pub type R = crate::R<GTIOR_SPEC>;
///Register `GTIOR` writer
pub type W = crate::W<GTIOR_SPEC>;
/**GTIOCA Pin Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GTIOA_A {
    ///0: Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match.
    _00000 = 0,
    ///1: Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match.
    _00001 = 1,
    ///2: Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match.
    _00010 = 2,
    ///3: Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match.
    _00011 = 3,
    ///4: Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match.
    _00100 = 4,
    ///5: Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match.
    _00101 = 5,
    ///6: Initial output is Low. Low output at cycle end. High output at GTCCRA compare match.
    _00110 = 6,
    ///7: Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match.
    _00111 = 7,
    ///8: Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match.
    _01000 = 8,
    ///9: Initial output is Low. High output at cycle end. Low output at GTCCRA compare match.
    _01001 = 9,
    ///10: Initial output is Low. High output at cycle end. High output at GTCCRA compare match.
    _01010 = 10,
    ///11: Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match.
    _01011 = 11,
    ///12: Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match.
    _01100 = 12,
    ///13: Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match.
    _01101 = 13,
    ///14: Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match.
    _01110 = 14,
    ///15: Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match.
    _01111 = 15,
    ///16: Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match.
    _10000 = 16,
    ///17: Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match.
    _10001 = 17,
    ///18: Initial output is High. Output retained at cycle end. High output at GTCCRA compare match.
    _10010 = 18,
    ///19: Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match.
    _10011 = 19,
    ///20: Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match.
    _10100 = 20,
    ///21: Initial output is High. Low output at cycle end. Low output at GTCCRA compare match.
    _10101 = 21,
    ///22: Initial output is High. Low output at cycle end. High output at GTCCRA compare match.
    _10110 = 22,
    ///23: Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match.
    _10111 = 23,
    ///24: Initial output is High. High output at cycle end. Output retained at GTCCRA compare match.
    _11000 = 24,
    ///25: Initial output is High. High output at cycle end. Low output at GTCCRA compare match.
    _11001 = 25,
    ///26: Initial output is High. High output at cycle end. High output at GTCCRA compare match.
    _11010 = 26,
    ///27: Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match.
    _11011 = 27,
    ///28: Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match.
    _11100 = 28,
    ///29: Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match.
    _11101 = 29,
    ///30: Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match.
    _11110 = 30,
    ///31: Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match.
    _11111 = 31,
}
impl From<GTIOA_A> for u8 {
    #[inline(always)]
    fn from(variant: GTIOA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GTIOA_A {
    type Ux = u8;
}
impl crate::IsEnum for GTIOA_A {}
///Field `GTIOA` reader - GTIOCA Pin Function Select
pub type GTIOA_R = crate::FieldReader<GTIOA_A>;
impl GTIOA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GTIOA_A {
        match self.bits {
            0 => GTIOA_A::_00000,
            1 => GTIOA_A::_00001,
            2 => GTIOA_A::_00010,
            3 => GTIOA_A::_00011,
            4 => GTIOA_A::_00100,
            5 => GTIOA_A::_00101,
            6 => GTIOA_A::_00110,
            7 => GTIOA_A::_00111,
            8 => GTIOA_A::_01000,
            9 => GTIOA_A::_01001,
            10 => GTIOA_A::_01010,
            11 => GTIOA_A::_01011,
            12 => GTIOA_A::_01100,
            13 => GTIOA_A::_01101,
            14 => GTIOA_A::_01110,
            15 => GTIOA_A::_01111,
            16 => GTIOA_A::_10000,
            17 => GTIOA_A::_10001,
            18 => GTIOA_A::_10010,
            19 => GTIOA_A::_10011,
            20 => GTIOA_A::_10100,
            21 => GTIOA_A::_10101,
            22 => GTIOA_A::_10110,
            23 => GTIOA_A::_10111,
            24 => GTIOA_A::_11000,
            25 => GTIOA_A::_11001,
            26 => GTIOA_A::_11010,
            27 => GTIOA_A::_11011,
            28 => GTIOA_A::_11100,
            29 => GTIOA_A::_11101,
            30 => GTIOA_A::_11110,
            31 => GTIOA_A::_11111,
            _ => unreachable!(),
        }
    }
    ///Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == GTIOA_A::_00000
    }
    ///Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == GTIOA_A::_00001
    }
    ///Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == GTIOA_A::_00010
    }
    ///Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == GTIOA_A::_00011
    }
    ///Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == GTIOA_A::_00100
    }
    ///Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == GTIOA_A::_00101
    }
    ///Initial output is Low. Low output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == GTIOA_A::_00110
    }
    ///Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == GTIOA_A::_00111
    }
    ///Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == GTIOA_A::_01000
    }
    ///Initial output is Low. High output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == GTIOA_A::_01001
    }
    ///Initial output is Low. High output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == GTIOA_A::_01010
    }
    ///Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == GTIOA_A::_01011
    }
    ///Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == GTIOA_A::_01100
    }
    ///Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == GTIOA_A::_01101
    }
    ///Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == GTIOA_A::_01110
    }
    ///Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == GTIOA_A::_01111
    }
    ///Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == GTIOA_A::_10000
    }
    ///Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == GTIOA_A::_10001
    }
    ///Initial output is High. Output retained at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == GTIOA_A::_10010
    }
    ///Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == GTIOA_A::_10011
    }
    ///Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == GTIOA_A::_10100
    }
    ///Initial output is High. Low output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == GTIOA_A::_10101
    }
    ///Initial output is High. Low output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == GTIOA_A::_10110
    }
    ///Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == GTIOA_A::_10111
    }
    ///Initial output is High. High output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == GTIOA_A::_11000
    }
    ///Initial output is High. High output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == GTIOA_A::_11001
    }
    ///Initial output is High. High output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == GTIOA_A::_11010
    }
    ///Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == GTIOA_A::_11011
    }
    ///Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == GTIOA_A::_11100
    }
    ///Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == GTIOA_A::_11101
    }
    ///Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == GTIOA_A::_11110
    }
    ///Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == GTIOA_A::_11111
    }
}
///Field `GTIOA` writer - GTIOCA Pin Function Select
pub type GTIOA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, GTIOA_A, crate::Safe>;
impl<'a, REG> GTIOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00000)
    }
    ///Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00001)
    }
    ///Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00010)
    }
    ///Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00011)
    }
    ///Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00100)
    }
    ///Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00101)
    }
    ///Initial output is Low. Low output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00110)
    }
    ///Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_00111)
    }
    ///Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01000)
    }
    ///Initial output is Low. High output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01001)
    }
    ///Initial output is Low. High output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01010)
    }
    ///Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01011)
    }
    ///Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01100)
    }
    ///Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01101)
    }
    ///Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01110)
    }
    ///Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_01111)
    }
    ///Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10000)
    }
    ///Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10001)
    }
    ///Initial output is High. Output retained at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10010)
    }
    ///Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10011)
    }
    ///Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _10100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10100)
    }
    ///Initial output is High. Low output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _10101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10101)
    }
    ///Initial output is High. Low output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _10110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10110)
    }
    ///Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _10111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_10111)
    }
    ///Initial output is High. High output at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _11000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11000)
    }
    ///Initial output is High. High output at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _11001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11001)
    }
    ///Initial output is High. High output at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _11010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11010)
    }
    ///Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _11011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11011)
    }
    ///Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match.
    #[inline(always)]
    pub fn _11100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11100)
    }
    ///Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match.
    #[inline(always)]
    pub fn _11101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11101)
    }
    ///Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match.
    #[inline(always)]
    pub fn _11110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11110)
    }
    ///Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match.
    #[inline(always)]
    pub fn _11111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOA_A::_11111)
    }
}
/**GTIOCA Pin Output Value Setting at the Count Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADFLT_A {
    ///0: Output low on GTIOCA pin when counting stops
    _0 = 0,
    ///1: Output high on GTIOCA pin when counting stops.
    _1 = 1,
}
impl From<OADFLT_A> for bool {
    #[inline(always)]
    fn from(variant: OADFLT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OADFLT` reader - GTIOCA Pin Output Value Setting at the Count Stop
pub type OADFLT_R = crate::BitReader<OADFLT_A>;
impl OADFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OADFLT_A {
        match self.bits {
            false => OADFLT_A::_0,
            true => OADFLT_A::_1,
        }
    }
    ///Output low on GTIOCA pin when counting stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADFLT_A::_0
    }
    ///Output high on GTIOCA pin when counting stops.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADFLT_A::_1
    }
}
///Field `OADFLT` writer - GTIOCA Pin Output Value Setting at the Count Stop
pub type OADFLT_W<'a, REG> = crate::BitWriter<'a, REG, OADFLT_A>;
impl<'a, REG> OADFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on GTIOCA pin when counting stops
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OADFLT_A::_0)
    }
    ///Output high on GTIOCA pin when counting stops.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OADFLT_A::_1)
    }
}
/**GTIOCA Pin Output Setting at the Start/Stop Count

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAHLD_A {
    ///0: Set GTIOCA pin output level on counting start and stop based on the register setting.
    _0 = 0,
    ///1: Retain GTIOCA pin output level on counting start and stop
    _1 = 1,
}
impl From<OAHLD_A> for bool {
    #[inline(always)]
    fn from(variant: OAHLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OAHLD` reader - GTIOCA Pin Output Setting at the Start/Stop Count
pub type OAHLD_R = crate::BitReader<OAHLD_A>;
impl OAHLD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OAHLD_A {
        match self.bits {
            false => OAHLD_A::_0,
            true => OAHLD_A::_1,
        }
    }
    ///Set GTIOCA pin output level on counting start and stop based on the register setting.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAHLD_A::_0
    }
    ///Retain GTIOCA pin output level on counting start and stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAHLD_A::_1
    }
}
///Field `OAHLD` writer - GTIOCA Pin Output Setting at the Start/Stop Count
pub type OAHLD_W<'a, REG> = crate::BitWriter<'a, REG, OAHLD_A>;
impl<'a, REG> OAHLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set GTIOCA pin output level on counting start and stop based on the register setting.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAHLD_A::_0)
    }
    ///Retain GTIOCA pin output level on counting start and stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAHLD_A::_1)
    }
}
/**GTIOCA Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAE_A {
    ///0: Disable output
    _0 = 0,
    ///1: Enable output.
    _1 = 1,
}
impl From<OAE_A> for bool {
    #[inline(always)]
    fn from(variant: OAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OAE` reader - GTIOCA Pin Output Enable
pub type OAE_R = crate::BitReader<OAE_A>;
impl OAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OAE_A {
        match self.bits {
            false => OAE_A::_0,
            true => OAE_A::_1,
        }
    }
    ///Disable output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAE_A::_0
    }
    ///Enable output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAE_A::_1
    }
}
///Field `OAE` writer - GTIOCA Pin Output Enable
pub type OAE_W<'a, REG> = crate::BitWriter<'a, REG, OAE_A>;
impl<'a, REG> OAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAE_A::_0)
    }
    ///Enable output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAE_A::_1)
    }
}
/**GTIOCA Pin Disable Value Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OADF_A {
    ///0: Prohibit output disable
    _00 = 0,
    ///1: Set GTIOCA pin to Hi-Z on output disable
    _01 = 1,
    ///2: Set GTIOCA pin to 0 on output disable
    _10 = 2,
    ///3: Set GTIOCA pin to 1 on output disable.
    _11 = 3,
}
impl From<OADF_A> for u8 {
    #[inline(always)]
    fn from(variant: OADF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OADF_A {
    type Ux = u8;
}
impl crate::IsEnum for OADF_A {}
///Field `OADF` reader - GTIOCA Pin Disable Value Setting
pub type OADF_R = crate::FieldReader<OADF_A>;
impl OADF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OADF_A {
        match self.bits {
            0 => OADF_A::_00,
            1 => OADF_A::_01,
            2 => OADF_A::_10,
            3 => OADF_A::_11,
            _ => unreachable!(),
        }
    }
    ///Prohibit output disable
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OADF_A::_00
    }
    ///Set GTIOCA pin to Hi-Z on output disable
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OADF_A::_01
    }
    ///Set GTIOCA pin to 0 on output disable
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OADF_A::_10
    }
    ///Set GTIOCA pin to 1 on output disable.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OADF_A::_11
    }
}
///Field `OADF` writer - GTIOCA Pin Disable Value Setting
pub type OADF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OADF_A, crate::Safe>;
impl<'a, REG> OADF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prohibit output disable
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OADF_A::_00)
    }
    ///Set GTIOCA pin to Hi-Z on output disable
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OADF_A::_01)
    }
    ///Set GTIOCA pin to 0 on output disable
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OADF_A::_10)
    }
    ///Set GTIOCA pin to 1 on output disable.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OADF_A::_11)
    }
}
/**Noise Filter A Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFAEN_A {
    ///0: Disable noise filter for GTIOCA pin
    _0 = 0,
    ///1: Enable noise filter for GTIOCA pin.
    _1 = 1,
}
impl From<NFAEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFAEN` reader - Noise Filter A Enable
pub type NFAEN_R = crate::BitReader<NFAEN_A>;
impl NFAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFAEN_A {
        match self.bits {
            false => NFAEN_A::_0,
            true => NFAEN_A::_1,
        }
    }
    ///Disable noise filter for GTIOCA pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFAEN_A::_0
    }
    ///Enable noise filter for GTIOCA pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFAEN_A::_1
    }
}
///Field `NFAEN` writer - Noise Filter A Enable
pub type NFAEN_W<'a, REG> = crate::BitWriter<'a, REG, NFAEN_A>;
impl<'a, REG> NFAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable noise filter for GTIOCA pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFAEN_A::_0)
    }
    ///Enable noise filter for GTIOCA pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFAEN_A::_1)
    }
}
/**Noise Filter A Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCSA_A {
    ///0: PCLK/1
    _00 = 0,
    ///1: PCLK/4
    _01 = 1,
    ///2: PCLK/16
    _10 = 2,
    ///3: PCLK/64
    _11 = 3,
}
impl From<NFCSA_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCSA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCSA_A {
    type Ux = u8;
}
impl crate::IsEnum for NFCSA_A {}
///Field `NFCSA` reader - Noise Filter A Sampling Clock Select
pub type NFCSA_R = crate::FieldReader<NFCSA_A>;
impl NFCSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFCSA_A {
        match self.bits {
            0 => NFCSA_A::_00,
            1 => NFCSA_A::_01,
            2 => NFCSA_A::_10,
            3 => NFCSA_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK/1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCSA_A::_00
    }
    ///PCLK/4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCSA_A::_01
    }
    ///PCLK/16
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCSA_A::_10
    }
    ///PCLK/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCSA_A::_11
    }
}
///Field `NFCSA` writer - Noise Filter A Sampling Clock Select
pub type NFCSA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NFCSA_A, crate::Safe>;
impl<'a, REG> NFCSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK/1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSA_A::_00)
    }
    ///PCLK/4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSA_A::_01)
    }
    ///PCLK/16
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSA_A::_10)
    }
    ///PCLK/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSA_A::_11)
    }
}
/**GTIOCB Pin Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GTIOB_A {
    ///0: Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match.
    _00000 = 0,
    ///1: Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match.
    _00001 = 1,
    ///2: Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match.
    _00010 = 2,
    ///3: Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match.
    _00011 = 3,
    ///4: Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match.
    _00100 = 4,
    ///5: Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match.
    _00101 = 5,
    ///6: Initial output is Low. Low output at cycle end. High output at GTCCRB compare match.
    _00110 = 6,
    ///7: Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match.
    _00111 = 7,
    ///8: Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match.
    _01000 = 8,
    ///9: Initial output is Low. High output at cycle end. Low output at GTCCRB compare match.
    _01001 = 9,
    ///10: Initial output is Low. High output at cycle end. High output at GTCCRB compare match.
    _01010 = 10,
    ///11: Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match.
    _01011 = 11,
    ///12: Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match.
    _01100 = 12,
    ///13: Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match.
    _01101 = 13,
    ///14: Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match.
    _01110 = 14,
    ///15: Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match.
    _01111 = 15,
    ///16: Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match.
    _10000 = 16,
    ///17: Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match.
    _10001 = 17,
    ///18: Initial output is High. Output retained at cycle end. High output at GTCCRB compare match.
    _10010 = 18,
    ///19: Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match.
    _10011 = 19,
    ///20: Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match.
    _10100 = 20,
    ///21: Initial output is High. Low output at cycle end. Low output at GTCCRB compare match.
    _10101 = 21,
    ///22: Initial output is High. Low output at cycle end. High output at GTCCRB compare match.
    _10110 = 22,
    ///23: Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match.
    _10111 = 23,
    ///24: Initial output is High. High output at cycle end. Output retained at GTCCRB compare match.
    _11000 = 24,
    ///25: Initial output is High. High output at cycle end. Low output at GTCCRB compare match.
    _11001 = 25,
    ///26: Initial output is High. High output at cycle end. High output at GTCCRB compare match.
    _11010 = 26,
    ///27: Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match.
    _11011 = 27,
    ///28: Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match.
    _11100 = 28,
    ///29: Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match.
    _11101 = 29,
    ///30: Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match.
    _11110 = 30,
    ///31: Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match.
    _11111 = 31,
}
impl From<GTIOB_A> for u8 {
    #[inline(always)]
    fn from(variant: GTIOB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GTIOB_A {
    type Ux = u8;
}
impl crate::IsEnum for GTIOB_A {}
///Field `GTIOB` reader - GTIOCB Pin Function Select
pub type GTIOB_R = crate::FieldReader<GTIOB_A>;
impl GTIOB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GTIOB_A {
        match self.bits {
            0 => GTIOB_A::_00000,
            1 => GTIOB_A::_00001,
            2 => GTIOB_A::_00010,
            3 => GTIOB_A::_00011,
            4 => GTIOB_A::_00100,
            5 => GTIOB_A::_00101,
            6 => GTIOB_A::_00110,
            7 => GTIOB_A::_00111,
            8 => GTIOB_A::_01000,
            9 => GTIOB_A::_01001,
            10 => GTIOB_A::_01010,
            11 => GTIOB_A::_01011,
            12 => GTIOB_A::_01100,
            13 => GTIOB_A::_01101,
            14 => GTIOB_A::_01110,
            15 => GTIOB_A::_01111,
            16 => GTIOB_A::_10000,
            17 => GTIOB_A::_10001,
            18 => GTIOB_A::_10010,
            19 => GTIOB_A::_10011,
            20 => GTIOB_A::_10100,
            21 => GTIOB_A::_10101,
            22 => GTIOB_A::_10110,
            23 => GTIOB_A::_10111,
            24 => GTIOB_A::_11000,
            25 => GTIOB_A::_11001,
            26 => GTIOB_A::_11010,
            27 => GTIOB_A::_11011,
            28 => GTIOB_A::_11100,
            29 => GTIOB_A::_11101,
            30 => GTIOB_A::_11110,
            31 => GTIOB_A::_11111,
            _ => unreachable!(),
        }
    }
    ///Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == GTIOB_A::_00000
    }
    ///Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == GTIOB_A::_00001
    }
    ///Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == GTIOB_A::_00010
    }
    ///Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == GTIOB_A::_00011
    }
    ///Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == GTIOB_A::_00100
    }
    ///Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == GTIOB_A::_00101
    }
    ///Initial output is Low. Low output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == GTIOB_A::_00110
    }
    ///Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == GTIOB_A::_00111
    }
    ///Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == GTIOB_A::_01000
    }
    ///Initial output is Low. High output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == GTIOB_A::_01001
    }
    ///Initial output is Low. High output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == GTIOB_A::_01010
    }
    ///Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == GTIOB_A::_01011
    }
    ///Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == GTIOB_A::_01100
    }
    ///Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == GTIOB_A::_01101
    }
    ///Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == GTIOB_A::_01110
    }
    ///Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == GTIOB_A::_01111
    }
    ///Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == GTIOB_A::_10000
    }
    ///Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == GTIOB_A::_10001
    }
    ///Initial output is High. Output retained at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == GTIOB_A::_10010
    }
    ///Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == GTIOB_A::_10011
    }
    ///Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == GTIOB_A::_10100
    }
    ///Initial output is High. Low output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == GTIOB_A::_10101
    }
    ///Initial output is High. Low output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == GTIOB_A::_10110
    }
    ///Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == GTIOB_A::_10111
    }
    ///Initial output is High. High output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == GTIOB_A::_11000
    }
    ///Initial output is High. High output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == GTIOB_A::_11001
    }
    ///Initial output is High. High output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == GTIOB_A::_11010
    }
    ///Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == GTIOB_A::_11011
    }
    ///Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == GTIOB_A::_11100
    }
    ///Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == GTIOB_A::_11101
    }
    ///Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == GTIOB_A::_11110
    }
    ///Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == GTIOB_A::_11111
    }
}
///Field `GTIOB` writer - GTIOCB Pin Function Select
pub type GTIOB_W<'a, REG> = crate::FieldWriter<'a, REG, 5, GTIOB_A, crate::Safe>;
impl<'a, REG> GTIOB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00000)
    }
    ///Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00001)
    }
    ///Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00010)
    }
    ///Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00011)
    }
    ///Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00100)
    }
    ///Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00101)
    }
    ///Initial output is Low. Low output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00110)
    }
    ///Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_00111)
    }
    ///Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01000)
    }
    ///Initial output is Low. High output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01001)
    }
    ///Initial output is Low. High output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01010)
    }
    ///Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01011)
    }
    ///Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01100)
    }
    ///Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01101)
    }
    ///Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01110)
    }
    ///Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_01111)
    }
    ///Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10000)
    }
    ///Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10001)
    }
    ///Initial output is High. Output retained at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10010)
    }
    ///Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10011)
    }
    ///Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _10100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10100)
    }
    ///Initial output is High. Low output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _10101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10101)
    }
    ///Initial output is High. Low output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _10110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10110)
    }
    ///Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _10111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_10111)
    }
    ///Initial output is High. High output at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _11000(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11000)
    }
    ///Initial output is High. High output at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _11001(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11001)
    }
    ///Initial output is High. High output at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _11010(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11010)
    }
    ///Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _11011(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11011)
    }
    ///Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match.
    #[inline(always)]
    pub fn _11100(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11100)
    }
    ///Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match.
    #[inline(always)]
    pub fn _11101(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11101)
    }
    ///Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match.
    #[inline(always)]
    pub fn _11110(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11110)
    }
    ///Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match.
    #[inline(always)]
    pub fn _11111(self) -> &'a mut crate::W<REG> {
        self.variant(GTIOB_A::_11111)
    }
}
/**GTIOCB Pin Output Value Setting at the Count Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDFLT_A {
    ///0: Output low on GTIOCB pin when counting stops
    _0 = 0,
    ///1: Output high on GTIOCB pin when counting stops
    _1 = 1,
}
impl From<OBDFLT_A> for bool {
    #[inline(always)]
    fn from(variant: OBDFLT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OBDFLT` reader - GTIOCB Pin Output Value Setting at the Count Stop
pub type OBDFLT_R = crate::BitReader<OBDFLT_A>;
impl OBDFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBDFLT_A {
        match self.bits {
            false => OBDFLT_A::_0,
            true => OBDFLT_A::_1,
        }
    }
    ///Output low on GTIOCB pin when counting stops
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDFLT_A::_0
    }
    ///Output high on GTIOCB pin when counting stops
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDFLT_A::_1
    }
}
///Field `OBDFLT` writer - GTIOCB Pin Output Value Setting at the Count Stop
pub type OBDFLT_W<'a, REG> = crate::BitWriter<'a, REG, OBDFLT_A>;
impl<'a, REG> OBDFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on GTIOCB pin when counting stops
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OBDFLT_A::_0)
    }
    ///Output high on GTIOCB pin when counting stops
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OBDFLT_A::_1)
    }
}
/**GTIOCB Pin Output Setting at the Start/Stop Count

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBHLD_A {
    ///0: Set GTIOCB pin output level on counting start and stop based on the register setting
    _0 = 0,
    ///1: Retain GTIOCB pin output level on counting start and stop
    _1 = 1,
}
impl From<OBHLD_A> for bool {
    #[inline(always)]
    fn from(variant: OBHLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OBHLD` reader - GTIOCB Pin Output Setting at the Start/Stop Count
pub type OBHLD_R = crate::BitReader<OBHLD_A>;
impl OBHLD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBHLD_A {
        match self.bits {
            false => OBHLD_A::_0,
            true => OBHLD_A::_1,
        }
    }
    ///Set GTIOCB pin output level on counting start and stop based on the register setting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBHLD_A::_0
    }
    ///Retain GTIOCB pin output level on counting start and stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBHLD_A::_1
    }
}
///Field `OBHLD` writer - GTIOCB Pin Output Setting at the Start/Stop Count
pub type OBHLD_W<'a, REG> = crate::BitWriter<'a, REG, OBHLD_A>;
impl<'a, REG> OBHLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set GTIOCB pin output level on counting start and stop based on the register setting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OBHLD_A::_0)
    }
    ///Retain GTIOCB pin output level on counting start and stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OBHLD_A::_1)
    }
}
/**GTIOCB Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBE_A {
    ///0: Disable output
    _0 = 0,
    ///1: Enable output
    _1 = 1,
}
impl From<OBE_A> for bool {
    #[inline(always)]
    fn from(variant: OBE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OBE` reader - GTIOCB Pin Output Enable
pub type OBE_R = crate::BitReader<OBE_A>;
impl OBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBE_A {
        match self.bits {
            false => OBE_A::_0,
            true => OBE_A::_1,
        }
    }
    ///Disable output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBE_A::_0
    }
    ///Enable output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBE_A::_1
    }
}
///Field `OBE` writer - GTIOCB Pin Output Enable
pub type OBE_W<'a, REG> = crate::BitWriter<'a, REG, OBE_A>;
impl<'a, REG> OBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OBE_A::_0)
    }
    ///Enable output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OBE_A::_1)
    }
}
/**GTIOCB Pin Disable Value Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OBDF_A {
    ///0: Prohibit output disable
    _00 = 0,
    ///1: Set GTIOCB pin to Hi-Z on output disable
    _01 = 1,
    ///2: Set GTIOCB pin to 0 on output disable
    _10 = 2,
    ///3: Set GTIOCB pin to 1 on output disable.
    _11 = 3,
}
impl From<OBDF_A> for u8 {
    #[inline(always)]
    fn from(variant: OBDF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OBDF_A {
    type Ux = u8;
}
impl crate::IsEnum for OBDF_A {}
///Field `OBDF` reader - GTIOCB Pin Disable Value Setting
pub type OBDF_R = crate::FieldReader<OBDF_A>;
impl OBDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBDF_A {
        match self.bits {
            0 => OBDF_A::_00,
            1 => OBDF_A::_01,
            2 => OBDF_A::_10,
            3 => OBDF_A::_11,
            _ => unreachable!(),
        }
    }
    ///Prohibit output disable
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OBDF_A::_00
    }
    ///Set GTIOCB pin to Hi-Z on output disable
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OBDF_A::_01
    }
    ///Set GTIOCB pin to 0 on output disable
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OBDF_A::_10
    }
    ///Set GTIOCB pin to 1 on output disable.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OBDF_A::_11
    }
}
///Field `OBDF` writer - GTIOCB Pin Disable Value Setting
pub type OBDF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OBDF_A, crate::Safe>;
impl<'a, REG> OBDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prohibit output disable
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OBDF_A::_00)
    }
    ///Set GTIOCB pin to Hi-Z on output disable
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OBDF_A::_01)
    }
    ///Set GTIOCB pin to 0 on output disable
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OBDF_A::_10)
    }
    ///Set GTIOCB pin to 1 on output disable.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OBDF_A::_11)
    }
}
/**Noise Filter B Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFBEN_A {
    ///0: Disable noise filter for GTIOCB pin
    _0 = 0,
    ///1: Enable noise filter for GTIOCB pin
    _1 = 1,
}
impl From<NFBEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFBEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFBEN` reader - Noise Filter B Enable
pub type NFBEN_R = crate::BitReader<NFBEN_A>;
impl NFBEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFBEN_A {
        match self.bits {
            false => NFBEN_A::_0,
            true => NFBEN_A::_1,
        }
    }
    ///Disable noise filter for GTIOCB pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFBEN_A::_0
    }
    ///Enable noise filter for GTIOCB pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFBEN_A::_1
    }
}
///Field `NFBEN` writer - Noise Filter B Enable
pub type NFBEN_W<'a, REG> = crate::BitWriter<'a, REG, NFBEN_A>;
impl<'a, REG> NFBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable noise filter for GTIOCB pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFBEN_A::_0)
    }
    ///Enable noise filter for GTIOCB pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFBEN_A::_1)
    }
}
/**Noise Filter B Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCSB_A {
    ///0: PCLK/1
    _00 = 0,
    ///1: PCLK/4
    _01 = 1,
    ///2: PCLK/16
    _10 = 2,
    ///3: PCLK/64
    _11 = 3,
}
impl From<NFCSB_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCSB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCSB_A {
    type Ux = u8;
}
impl crate::IsEnum for NFCSB_A {}
///Field `NFCSB` reader - Noise Filter B Sampling Clock Select
pub type NFCSB_R = crate::FieldReader<NFCSB_A>;
impl NFCSB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFCSB_A {
        match self.bits {
            0 => NFCSB_A::_00,
            1 => NFCSB_A::_01,
            2 => NFCSB_A::_10,
            3 => NFCSB_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK/1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCSB_A::_00
    }
    ///PCLK/4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCSB_A::_01
    }
    ///PCLK/16
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCSB_A::_10
    }
    ///PCLK/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCSB_A::_11
    }
}
///Field `NFCSB` writer - Noise Filter B Sampling Clock Select
pub type NFCSB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NFCSB_A, crate::Safe>;
impl<'a, REG> NFCSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK/1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSB_A::_00)
    }
    ///PCLK/4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSB_A::_01)
    }
    ///PCLK/16
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSB_A::_10)
    }
    ///PCLK/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NFCSB_A::_11)
    }
}
impl R {
    ///Bits 0:4 - GTIOCA Pin Function Select
    #[inline(always)]
    pub fn gtioa(&self) -> GTIOA_R {
        GTIOA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - GTIOCA Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn oadflt(&self) -> OADFLT_R {
        OADFLT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTIOCA Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn oahld(&self) -> OAHLD_R {
        OAHLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Output Enable
    #[inline(always)]
    pub fn oae(&self) -> OAE_R {
        OAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - GTIOCA Pin Disable Value Setting
    #[inline(always)]
    pub fn oadf(&self) -> OADF_R {
        OADF_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 13 - Noise Filter A Enable
    #[inline(always)]
    pub fn nfaen(&self) -> NFAEN_R {
        NFAEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Noise Filter A Sampling Clock Select
    #[inline(always)]
    pub fn nfcsa(&self) -> NFCSA_R {
        NFCSA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:20 - GTIOCB Pin Function Select
    #[inline(always)]
    pub fn gtiob(&self) -> GTIOB_R {
        GTIOB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - GTIOCB Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn obdflt(&self) -> OBDFLT_R {
        OBDFLT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GTIOCB Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn obhld(&self) -> OBHLD_R {
        OBHLD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GTIOCB Pin Output Enable
    #[inline(always)]
    pub fn obe(&self) -> OBE_R {
        OBE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - GTIOCB Pin Disable Value Setting
    #[inline(always)]
    pub fn obdf(&self) -> OBDF_R {
        OBDF_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 29 - Noise Filter B Enable
    #[inline(always)]
    pub fn nfben(&self) -> NFBEN_R {
        NFBEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Noise Filter B Sampling Clock Select
    #[inline(always)]
    pub fn nfcsb(&self) -> NFCSB_R {
        NFCSB_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:4 - GTIOCA Pin Function Select
    #[inline(always)]
    pub fn gtioa(&mut self) -> GTIOA_W<GTIOR_SPEC> {
        GTIOA_W::new(self, 0)
    }
    ///Bit 6 - GTIOCA Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn oadflt(&mut self) -> OADFLT_W<GTIOR_SPEC> {
        OADFLT_W::new(self, 6)
    }
    ///Bit 7 - GTIOCA Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn oahld(&mut self) -> OAHLD_W<GTIOR_SPEC> {
        OAHLD_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Output Enable
    #[inline(always)]
    pub fn oae(&mut self) -> OAE_W<GTIOR_SPEC> {
        OAE_W::new(self, 8)
    }
    ///Bits 9:10 - GTIOCA Pin Disable Value Setting
    #[inline(always)]
    pub fn oadf(&mut self) -> OADF_W<GTIOR_SPEC> {
        OADF_W::new(self, 9)
    }
    ///Bit 13 - Noise Filter A Enable
    #[inline(always)]
    pub fn nfaen(&mut self) -> NFAEN_W<GTIOR_SPEC> {
        NFAEN_W::new(self, 13)
    }
    ///Bits 14:15 - Noise Filter A Sampling Clock Select
    #[inline(always)]
    pub fn nfcsa(&mut self) -> NFCSA_W<GTIOR_SPEC> {
        NFCSA_W::new(self, 14)
    }
    ///Bits 16:20 - GTIOCB Pin Function Select
    #[inline(always)]
    pub fn gtiob(&mut self) -> GTIOB_W<GTIOR_SPEC> {
        GTIOB_W::new(self, 16)
    }
    ///Bit 22 - GTIOCB Pin Output Value Setting at the Count Stop
    #[inline(always)]
    pub fn obdflt(&mut self) -> OBDFLT_W<GTIOR_SPEC> {
        OBDFLT_W::new(self, 22)
    }
    ///Bit 23 - GTIOCB Pin Output Setting at the Start/Stop Count
    #[inline(always)]
    pub fn obhld(&mut self) -> OBHLD_W<GTIOR_SPEC> {
        OBHLD_W::new(self, 23)
    }
    ///Bit 24 - GTIOCB Pin Output Enable
    #[inline(always)]
    pub fn obe(&mut self) -> OBE_W<GTIOR_SPEC> {
        OBE_W::new(self, 24)
    }
    ///Bits 25:26 - GTIOCB Pin Disable Value Setting
    #[inline(always)]
    pub fn obdf(&mut self) -> OBDF_W<GTIOR_SPEC> {
        OBDF_W::new(self, 25)
    }
    ///Bit 29 - Noise Filter B Enable
    #[inline(always)]
    pub fn nfben(&mut self) -> NFBEN_W<GTIOR_SPEC> {
        NFBEN_W::new(self, 29)
    }
    ///Bits 30:31 - Noise Filter B Sampling Clock Select
    #[inline(always)]
    pub fn nfcsb(&mut self) -> NFCSB_W<GTIOR_SPEC> {
        NFCSB_W::new(self, 30)
    }
}
/**General PWM Timer I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`gtior::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtior::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTIOR_SPEC;
impl crate::RegisterSpec for GTIOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtior::R`](R) reader structure
impl crate::Readable for GTIOR_SPEC {}
///`write(|w| ..)` method takes [`gtior::W`](W) writer structure
impl crate::Writable for GTIOR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTIOR to value 0
impl crate::Resettable for GTIOR_SPEC {}
