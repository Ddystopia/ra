///Register `ADPGAGS0` reader
pub type R = crate::R<ADPGAGS0_SPEC>;
///Register `ADPGAGS0` writer
pub type W = crate::W<ADPGAGS0_SPEC>;
/**PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P000GAIN_A {
    ///0: x 2.000 (ADPGADDCR0.P000DEN=0)
    _0000 = 0,
    ///1: x 2.500 (ADPGADDCR0.P000DEN=0) / x 1.500 (ADPGADDCR0.P000DEN=1)
    _0001 = 1,
    ///2: x 2.667 (ADPGADDCR0.P000DEN=0)
    _0010 = 2,
    ///3: x 2.857 (ADPGADDCR0.P000DEN=0)
    _0011 = 3,
    ///4: x 3.077 (ADPGADDCR0.P000DEN=0)
    _0100 = 4,
    ///5: x 3.333 (ADPGADDCR0.P000DEN=0) / x 2.333 (ADPGADDCR0.P000DEN=1)
    _0101 = 5,
    ///6: x 3.636 (ADPGADDCR0.P000DEN=0)
    _0110 = 6,
    ///7: x 4.000 (ADPGADDCR0.P000DEN=0)
    _0111 = 7,
    ///8: x 4.444 (ADPGADDCR0.P000DEN=0)
    _1000 = 8,
    ///9: x 5.000 (ADPGADDCR0.P000DEN=0) / x 4.00 (ADPGADDCR0.P000DEN=1)
    _1001 = 9,
    ///10: x 5.714 (ADPGADDCR0.P000DEN=0)
    _1010 = 10,
    ///11: x 6.667 (ADPGADDCR0.P000DEN=0) / x 5.667 (ADPGADDCR0.P000DEN=1)
    _1011 = 11,
    ///12: x 8.000 (ADPGADDCR0.P000DEN=0)
    _1100 = 12,
    ///13: x 10.000 (ADPGADDCR0.P000DEN=0)
    _1101 = 13,
    ///14: x 13.333 (ADPGADDCR0.P000DEN=0)
    _1110 = 14,
    ///15: x 1.000 (for offset measurement) (ADPGADDCR0.P000DEN=0)
    _1111 = 15,
}
impl From<P000GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: P000GAIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P000GAIN_A {
    type Ux = u8;
}
impl crate::IsEnum for P000GAIN_A {}
///Field `P000GAIN` reader - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0.
pub type P000GAIN_R = crate::FieldReader<P000GAIN_A>;
impl P000GAIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000GAIN_A {
        match self.bits {
            0 => P000GAIN_A::_0000,
            1 => P000GAIN_A::_0001,
            2 => P000GAIN_A::_0010,
            3 => P000GAIN_A::_0011,
            4 => P000GAIN_A::_0100,
            5 => P000GAIN_A::_0101,
            6 => P000GAIN_A::_0110,
            7 => P000GAIN_A::_0111,
            8 => P000GAIN_A::_1000,
            9 => P000GAIN_A::_1001,
            10 => P000GAIN_A::_1010,
            11 => P000GAIN_A::_1011,
            12 => P000GAIN_A::_1100,
            13 => P000GAIN_A::_1101,
            14 => P000GAIN_A::_1110,
            15 => P000GAIN_A::_1111,
            _ => unreachable!(),
        }
    }
    ///x 2.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == P000GAIN_A::_0000
    }
    ///x 2.500 (ADPGADDCR0.P000DEN=0) / x 1.500 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == P000GAIN_A::_0001
    }
    ///x 2.667 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == P000GAIN_A::_0010
    }
    ///x 2.857 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == P000GAIN_A::_0011
    }
    ///x 3.077 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == P000GAIN_A::_0100
    }
    ///x 3.333 (ADPGADDCR0.P000DEN=0) / x 2.333 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == P000GAIN_A::_0101
    }
    ///x 3.636 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == P000GAIN_A::_0110
    }
    ///x 4.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == P000GAIN_A::_0111
    }
    ///x 4.444 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == P000GAIN_A::_1000
    }
    ///x 5.000 (ADPGADDCR0.P000DEN=0) / x 4.00 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == P000GAIN_A::_1001
    }
    ///x 5.714 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == P000GAIN_A::_1010
    }
    ///x 6.667 (ADPGADDCR0.P000DEN=0) / x 5.667 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == P000GAIN_A::_1011
    }
    ///x 8.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == P000GAIN_A::_1100
    }
    ///x 10.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == P000GAIN_A::_1101
    }
    ///x 13.333 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == P000GAIN_A::_1110
    }
    ///x 1.000 (for offset measurement) (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == P000GAIN_A::_1111
    }
}
///Field `P000GAIN` writer - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0.
pub type P000GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, P000GAIN_A, crate::Safe>;
impl<'a, REG> P000GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 2.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0000)
    }
    ///x 2.500 (ADPGADDCR0.P000DEN=0) / x 1.500 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0001)
    }
    ///x 2.667 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0010)
    }
    ///x 2.857 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0011)
    }
    ///x 3.077 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0100)
    }
    ///x 3.333 (ADPGADDCR0.P000DEN=0) / x 2.333 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0101)
    }
    ///x 3.636 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0110)
    }
    ///x 4.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_0111)
    }
    ///x 4.444 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1000)
    }
    ///x 5.000 (ADPGADDCR0.P000DEN=0) / x 4.00 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1001)
    }
    ///x 5.714 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1010)
    }
    ///x 6.667 (ADPGADDCR0.P000DEN=0) / x 5.667 (ADPGADDCR0.P000DEN=1)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1011)
    }
    ///x 8.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1100)
    }
    ///x 10.000 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1101)
    }
    ///x 13.333 (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1110)
    }
    ///x 1.000 (for offset measurement) (ADPGADDCR0.P000DEN=0)
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(P000GAIN_A::_1111)
    }
}
/**PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P001GAIN_A {
    ///0: x 2.000 (ADPGADDCR0.P001DEN=0)
    _0000 = 0,
    ///1: x 2.500 (ADPGADDCR0.P001DEN=0) / x 1.500 (ADPGADDCR0.P001DEN=1)
    _0001 = 1,
    ///2: x 2.667 (ADPGADDCR0.P001DEN=0)
    _0010 = 2,
    ///3: x 2.857 (ADPGADDCR0.P001DEN=0)
    _0011 = 3,
    ///4: x 3.077 (ADPGADDCR0.P001DEN=0)
    _0100 = 4,
    ///5: x 3.333 (ADPGADDCR0.P001DEN=0) / x 2.333 (ADPGADDCR0.P001DEN=1)
    _0101 = 5,
    ///6: x 3.636 (ADPGADDCR0.P001DEN=0)
    _0110 = 6,
    ///7: x 4.000 (ADPGADDCR0.P001DEN=0)
    _0111 = 7,
    ///8: x 4.444 (ADPGADDCR0.P001DEN=0)
    _1000 = 8,
    ///9: x 5.000 (ADPGADDCR0.P001DEN=0) / x 4.00 (ADPGADDCR0.P001DEN=1)
    _1001 = 9,
    ///10: x 5.714 (ADPGADDCR0.P001DEN=0)
    _1010 = 10,
    ///11: x 6.667 (ADPGADDCR0.P001DEN=0) / x 5.667 (ADPGADDCR0.P001DEN=1)
    _1011 = 11,
    ///12: x 8.000 (ADPGADDCR0.P001DEN=0)
    _1100 = 12,
    ///13: x 10.000 (ADPGADDCR0.P001DEN=0)
    _1101 = 13,
    ///14: x 13.333 (ADPGADDCR0.P001DEN=0)
    _1110 = 14,
    ///15: x 1.000 (for offset measurement) (ADPGADDCR0.P001DEN=0)
    _1111 = 15,
}
impl From<P001GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: P001GAIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P001GAIN_A {
    type Ux = u8;
}
impl crate::IsEnum for P001GAIN_A {}
///Field `P001GAIN` reader - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0.
pub type P001GAIN_R = crate::FieldReader<P001GAIN_A>;
impl P001GAIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001GAIN_A {
        match self.bits {
            0 => P001GAIN_A::_0000,
            1 => P001GAIN_A::_0001,
            2 => P001GAIN_A::_0010,
            3 => P001GAIN_A::_0011,
            4 => P001GAIN_A::_0100,
            5 => P001GAIN_A::_0101,
            6 => P001GAIN_A::_0110,
            7 => P001GAIN_A::_0111,
            8 => P001GAIN_A::_1000,
            9 => P001GAIN_A::_1001,
            10 => P001GAIN_A::_1010,
            11 => P001GAIN_A::_1011,
            12 => P001GAIN_A::_1100,
            13 => P001GAIN_A::_1101,
            14 => P001GAIN_A::_1110,
            15 => P001GAIN_A::_1111,
            _ => unreachable!(),
        }
    }
    ///x 2.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == P001GAIN_A::_0000
    }
    ///x 2.500 (ADPGADDCR0.P001DEN=0) / x 1.500 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == P001GAIN_A::_0001
    }
    ///x 2.667 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == P001GAIN_A::_0010
    }
    ///x 2.857 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == P001GAIN_A::_0011
    }
    ///x 3.077 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == P001GAIN_A::_0100
    }
    ///x 3.333 (ADPGADDCR0.P001DEN=0) / x 2.333 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == P001GAIN_A::_0101
    }
    ///x 3.636 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == P001GAIN_A::_0110
    }
    ///x 4.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == P001GAIN_A::_0111
    }
    ///x 4.444 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == P001GAIN_A::_1000
    }
    ///x 5.000 (ADPGADDCR0.P001DEN=0) / x 4.00 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == P001GAIN_A::_1001
    }
    ///x 5.714 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == P001GAIN_A::_1010
    }
    ///x 6.667 (ADPGADDCR0.P001DEN=0) / x 5.667 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == P001GAIN_A::_1011
    }
    ///x 8.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == P001GAIN_A::_1100
    }
    ///x 10.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == P001GAIN_A::_1101
    }
    ///x 13.333 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == P001GAIN_A::_1110
    }
    ///x 1.000 (for offset measurement) (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == P001GAIN_A::_1111
    }
}
///Field `P001GAIN` writer - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0.
pub type P001GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, P001GAIN_A, crate::Safe>;
impl<'a, REG> P001GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 2.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0000)
    }
    ///x 2.500 (ADPGADDCR0.P001DEN=0) / x 1.500 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0001)
    }
    ///x 2.667 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0010)
    }
    ///x 2.857 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0011)
    }
    ///x 3.077 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0100)
    }
    ///x 3.333 (ADPGADDCR0.P001DEN=0) / x 2.333 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0101)
    }
    ///x 3.636 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0110)
    }
    ///x 4.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_0111)
    }
    ///x 4.444 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1000)
    }
    ///x 5.000 (ADPGADDCR0.P001DEN=0) / x 4.00 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1001)
    }
    ///x 5.714 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1010)
    }
    ///x 6.667 (ADPGADDCR0.P001DEN=0) / x 5.667 (ADPGADDCR0.P001DEN=1)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1011)
    }
    ///x 8.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1100)
    }
    ///x 10.000 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1101)
    }
    ///x 13.333 (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1110)
    }
    ///x 1.000 (for offset measurement) (ADPGADDCR0.P001DEN=0)
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(P001GAIN_A::_1111)
    }
}
/**PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P002GAIN_A {
    ///0: x 2.000 (ADPGADDCR0.P002DEN=0)
    _0000 = 0,
    ///1: x 2.500 (ADPGADDCR0.P002DEN=0) / x 1.500 (ADPGADDCR0.P002DEN=1)
    _0001 = 1,
    ///2: x 2.667 (ADPGADDCR0.P002DEN=0)
    _0010 = 2,
    ///3: x 2.857 (ADPGADDCR0.P002DEN=0)
    _0011 = 3,
    ///4: x 3.077 (ADPGADDCR0.P002DEN=0)
    _0100 = 4,
    ///5: x 3.333 (ADPGADDCR0.P002DEN=0) / x 2.333 (ADPGADDCR0.P002DEN=1)
    _0101 = 5,
    ///6: x 3.636 (ADPGADDCR0.P002DEN=0)
    _0110 = 6,
    ///7: x 4.000 (ADPGADDCR0.P002DEN=0)
    _0111 = 7,
    ///8: x 4.444 (ADPGADDCR0.P002DEN=0)
    _1000 = 8,
    ///9: x 5.000 (ADPGADDCR0.P002DEN=0) / x 4.00 (ADPGADDCR0.P002DEN=1)
    _1001 = 9,
    ///10: x 5.714 (ADPGADDCR0.P002DEN=0)
    _1010 = 10,
    ///11: x 6.667 (ADPGADDCR0.P002DEN=0) / x 5.667 (ADPGADDCR0.P002DEN=1)
    _1011 = 11,
    ///12: x 8.000 (ADPGADDCR0.P002DEN=0)
    _1100 = 12,
    ///13: x 10.000 (ADPGADDCR0.P002DEN=0)
    _1101 = 13,
    ///14: x 13.333 (ADPGADDCR0.P002DEN=0)
    _1110 = 14,
    ///15: x 1.000 (for offset measurement) (ADPGADDCR0.P002DEN=0)
    _1111 = 15,
}
impl From<P002GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: P002GAIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P002GAIN_A {
    type Ux = u8;
}
impl crate::IsEnum for P002GAIN_A {}
///Field `P002GAIN` reader - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0.
pub type P002GAIN_R = crate::FieldReader<P002GAIN_A>;
impl P002GAIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002GAIN_A {
        match self.bits {
            0 => P002GAIN_A::_0000,
            1 => P002GAIN_A::_0001,
            2 => P002GAIN_A::_0010,
            3 => P002GAIN_A::_0011,
            4 => P002GAIN_A::_0100,
            5 => P002GAIN_A::_0101,
            6 => P002GAIN_A::_0110,
            7 => P002GAIN_A::_0111,
            8 => P002GAIN_A::_1000,
            9 => P002GAIN_A::_1001,
            10 => P002GAIN_A::_1010,
            11 => P002GAIN_A::_1011,
            12 => P002GAIN_A::_1100,
            13 => P002GAIN_A::_1101,
            14 => P002GAIN_A::_1110,
            15 => P002GAIN_A::_1111,
            _ => unreachable!(),
        }
    }
    ///x 2.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == P002GAIN_A::_0000
    }
    ///x 2.500 (ADPGADDCR0.P002DEN=0) / x 1.500 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == P002GAIN_A::_0001
    }
    ///x 2.667 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == P002GAIN_A::_0010
    }
    ///x 2.857 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == P002GAIN_A::_0011
    }
    ///x 3.077 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == P002GAIN_A::_0100
    }
    ///x 3.333 (ADPGADDCR0.P002DEN=0) / x 2.333 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == P002GAIN_A::_0101
    }
    ///x 3.636 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == P002GAIN_A::_0110
    }
    ///x 4.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == P002GAIN_A::_0111
    }
    ///x 4.444 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == P002GAIN_A::_1000
    }
    ///x 5.000 (ADPGADDCR0.P002DEN=0) / x 4.00 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == P002GAIN_A::_1001
    }
    ///x 5.714 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == P002GAIN_A::_1010
    }
    ///x 6.667 (ADPGADDCR0.P002DEN=0) / x 5.667 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == P002GAIN_A::_1011
    }
    ///x 8.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == P002GAIN_A::_1100
    }
    ///x 10.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == P002GAIN_A::_1101
    }
    ///x 13.333 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == P002GAIN_A::_1110
    }
    ///x 1.000 (for offset measurement) (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == P002GAIN_A::_1111
    }
}
///Field `P002GAIN` writer - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0.
pub type P002GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, P002GAIN_A, crate::Safe>;
impl<'a, REG> P002GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 2.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0000)
    }
    ///x 2.500 (ADPGADDCR0.P002DEN=0) / x 1.500 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0001)
    }
    ///x 2.667 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0010)
    }
    ///x 2.857 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0011)
    }
    ///x 3.077 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0100)
    }
    ///x 3.333 (ADPGADDCR0.P002DEN=0) / x 2.333 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0101)
    }
    ///x 3.636 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0110)
    }
    ///x 4.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_0111)
    }
    ///x 4.444 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1000)
    }
    ///x 5.000 (ADPGADDCR0.P002DEN=0) / x 4.00 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1001)
    }
    ///x 5.714 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1010)
    }
    ///x 6.667 (ADPGADDCR0.P002DEN=0) / x 5.667 (ADPGADDCR0.P002DEN=1)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1011)
    }
    ///x 8.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1100)
    }
    ///x 10.000 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1101)
    }
    ///x 13.333 (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1110)
    }
    ///x 1.000 (for offset measurement) (ADPGADDCR0.P002DEN=0)
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(P002GAIN_A::_1111)
    }
}
impl R {
    ///Bits 0:3 - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0.
    #[inline(always)]
    pub fn p000gain(&self) -> P000GAIN_R {
        P000GAIN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0.
    #[inline(always)]
    pub fn p001gain(&self) -> P001GAIN_R {
        P001GAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0.
    #[inline(always)]
    pub fn p002gain(&self) -> P002GAIN_R {
        P002GAIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - PGA P000 gain setting bit.The gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. When the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0.
    #[inline(always)]
    pub fn p000gain(&mut self) -> P000GAIN_W<ADPGAGS0_SPEC> {
        P000GAIN_W::new(self, 0)
    }
    ///Bits 4:7 - PGA P001 gain setting bit.The gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. When the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0.
    #[inline(always)]
    pub fn p001gain(&mut self) -> P001GAIN_W<ADPGAGS0_SPEC> {
        P001GAIN_W::new(self, 4)
    }
    ///Bits 8:11 - PGA P002 gain setting bit.The gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. When the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0.
    #[inline(always)]
    pub fn p002gain(&mut self) -> P002GAIN_W<ADPGAGS0_SPEC> {
        P002GAIN_W::new(self, 8)
    }
}
/**A/D Programmable Gain Amplifier Gain Setting Register 0

You can [`read`](crate::Reg::read) this register and get [`adpgags0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adpgags0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADPGAGS0_SPEC;
impl crate::RegisterSpec for ADPGAGS0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adpgags0::R`](R) reader structure
impl crate::Readable for ADPGAGS0_SPEC {}
///`write(|w| ..)` method takes [`adpgags0::W`](W) writer structure
impl crate::Writable for ADPGAGS0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADPGAGS0 to value 0
impl crate::Resettable for ADPGAGS0_SPEC {}
