///Register `JIFDCNT` reader
pub type R = crate::R<JIFDCNT_SPEC>;
///Register `JIFDCNT` writer
pub type W = crate::W<JIFDCNT_SPEC>;
/**Byte/Word Swap Output image data in decompression is swapped.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOUTSWAP_A {
    ///0: (1) (2) (3) (4) (5) (6) (7) (8)
    _000 = 0,
    ///1: (2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    _001 = 1,
    ///2: (3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    _010 = 2,
    ///3: (4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    _011 = 3,
    ///4: (5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    _100 = 4,
    ///5: (6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    _101 = 5,
    ///6: (7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    _110 = 6,
    ///7: (8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    _111 = 7,
}
impl From<DOUTSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: DOUTSWAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DOUTSWAP_A {
    type Ux = u8;
}
impl crate::IsEnum for DOUTSWAP_A {}
///Field `DOUTSWAP` reader - Byte/Word Swap Output image data in decompression is swapped.
pub type DOUTSWAP_R = crate::FieldReader<DOUTSWAP_A>;
impl DOUTSWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOUTSWAP_A {
        match self.bits {
            0 => DOUTSWAP_A::_000,
            1 => DOUTSWAP_A::_001,
            2 => DOUTSWAP_A::_010,
            3 => DOUTSWAP_A::_011,
            4 => DOUTSWAP_A::_100,
            5 => DOUTSWAP_A::_101,
            6 => DOUTSWAP_A::_110,
            7 => DOUTSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DOUTSWAP_A::_000
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DOUTSWAP_A::_001
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DOUTSWAP_A::_010
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DOUTSWAP_A::_011
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DOUTSWAP_A::_100
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DOUTSWAP_A::_101
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DOUTSWAP_A::_110
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DOUTSWAP_A::_111
    }
}
///Field `DOUTSWAP` writer - Byte/Word Swap Output image data in decompression is swapped.
pub type DOUTSWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DOUTSWAP_A, crate::Safe>;
impl<'a, REG> DOUTSWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_000)
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_001)
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_010)
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_011)
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_100)
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_101)
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_110)
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTSWAP_A::_111)
    }
}
/**Count Mode for Stopping Output Image Data Lines

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUTLC_A {
    ///0: Count mode for stopping the output of image data lines is off.
    _0 = 0,
    ///1: Count mode for stopping the output of image data lines is on
    _1 = 1,
}
impl From<DOUTLC_A> for bool {
    #[inline(always)]
    fn from(variant: DOUTLC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOUTLC` reader - Count Mode for Stopping Output Image Data Lines
pub type DOUTLC_R = crate::BitReader<DOUTLC_A>;
impl DOUTLC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOUTLC_A {
        match self.bits {
            false => DOUTLC_A::_0,
            true => DOUTLC_A::_1,
        }
    }
    ///Count mode for stopping the output of image data lines is off.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUTLC_A::_0
    }
    ///Count mode for stopping the output of image data lines is on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUTLC_A::_1
    }
}
///Field `DOUTLC` writer - Count Mode for Stopping Output Image Data Lines
pub type DOUTLC_W<'a, REG> = crate::BitWriter<'a, REG, DOUTLC_A>;
impl<'a, REG> DOUTLC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count mode for stopping the output of image data lines is off.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTLC_A::_0)
    }
    ///Count mode for stopping the output of image data lines is on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTLC_A::_1)
    }
}
///Field `DOUTRCMD` writer - Output Image Data Lines Resume Command This bit is valid only when the count mode for stopping the output of image data lines is on. Setting this bit to 1 resumes writing image data. This bit is always read as 0.
pub type DOUTRCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUTRINI_A {
    ///0: The transfer address is not initialized when the output of lines of image data is restarted.
    _0 = 0,
    ///1: The transfer address is initialized when the output of lines of image data is restarted
    _1 = 1,
}
impl From<DOUTRINI_A> for bool {
    #[inline(always)]
    fn from(variant: DOUTRINI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOUTRINI` reader - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
pub type DOUTRINI_R = crate::BitReader<DOUTRINI_A>;
impl DOUTRINI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOUTRINI_A {
        match self.bits {
            false => DOUTRINI_A::_0,
            true => DOUTRINI_A::_1,
        }
    }
    ///The transfer address is not initialized when the output of lines of image data is restarted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUTRINI_A::_0
    }
    ///The transfer address is initialized when the output of lines of image data is restarted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUTRINI_A::_1
    }
}
///Field `DOUTRINI` writer - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
pub type DOUTRINI_W<'a, REG> = crate::BitWriter<'a, REG, DOUTRINI_A>;
impl<'a, REG> DOUTRINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The transfer address is not initialized when the output of lines of image data is restarted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTRINI_A::_0)
    }
    ///The transfer address is initialized when the output of lines of image data is restarted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTRINI_A::_1)
    }
}
/**Byte/Word/Longword Swap Input coded data in decompression is swapped.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JINSWAP_A {
    ///0: (1) (2) (3) (4) (5) (6) (7) (8)
    _000 = 0,
    ///1: (2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    _001 = 1,
    ///2: (3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    _010 = 2,
    ///3: (4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    _011 = 3,
    ///4: (5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    _100 = 4,
    ///5: (6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    _101 = 5,
    ///6: (7) (8) (5) (6) (3) (4) (1) (2) \[Word -Halfword swap\]
    _110 = 6,
    ///7: (8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    _111 = 7,
}
impl From<JINSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: JINSWAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JINSWAP_A {
    type Ux = u8;
}
impl crate::IsEnum for JINSWAP_A {}
///Field `JINSWAP` reader - Byte/Word/Longword Swap Input coded data in decompression is swapped.
pub type JINSWAP_R = crate::FieldReader<JINSWAP_A>;
impl JINSWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JINSWAP_A {
        match self.bits {
            0 => JINSWAP_A::_000,
            1 => JINSWAP_A::_001,
            2 => JINSWAP_A::_010,
            3 => JINSWAP_A::_011,
            4 => JINSWAP_A::_100,
            5 => JINSWAP_A::_101,
            6 => JINSWAP_A::_110,
            7 => JINSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == JINSWAP_A::_000
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == JINSWAP_A::_001
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == JINSWAP_A::_010
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == JINSWAP_A::_011
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == JINSWAP_A::_100
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == JINSWAP_A::_101
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word -Halfword swap\]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == JINSWAP_A::_110
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == JINSWAP_A::_111
    }
}
///Field `JINSWAP` writer - Byte/Word/Longword Swap Input coded data in decompression is swapped.
pub type JINSWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, JINSWAP_A, crate::Safe>;
impl<'a, REG> JINSWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_000)
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_001)
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_010)
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_011)
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_100)
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_101)
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word -Halfword swap\]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_110)
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(JINSWAP_A::_111)
    }
}
/**Count Mode Setting for Stopping Input Coded Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JINC_A {
    ///0: Count mode for stopping the input of coded data is off.
    _0 = 0,
    ///1: Count mode for stopping the input of coded data is on
    _1 = 1,
}
impl From<JINC_A> for bool {
    #[inline(always)]
    fn from(variant: JINC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `JINC` reader - Count Mode Setting for Stopping Input Coded Data
pub type JINC_R = crate::BitReader<JINC_A>;
impl JINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JINC_A {
        match self.bits {
            false => JINC_A::_0,
            true => JINC_A::_1,
        }
    }
    ///Count mode for stopping the input of coded data is off.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JINC_A::_0
    }
    ///Count mode for stopping the input of coded data is on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JINC_A::_1
    }
}
///Field `JINC` writer - Count Mode Setting for Stopping Input Coded Data
pub type JINC_W<'a, REG> = crate::BitWriter<'a, REG, JINC_A>;
impl<'a, REG> JINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count mode for stopping the input of coded data is off.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(JINC_A::_0)
    }
    ///Count mode for stopping the input of coded data is on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(JINC_A::_1)
    }
}
///Field `JINRCMD` writer - Input Coded Data Resume CommandThis bit is valid only when the count mode for stopping the input of coded data is on. Setting this bit to 1 resumes reading input coded data. This bit is always read as 0.
pub type JINRCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JINRINI_A {
    ///0: The transfer address is not initialized when the input of coded data is restarted.
    _0 = 0,
    ///1: The transfer address is initialized when the input of coded data is restarted.
    _1 = 1,
}
impl From<JINRINI_A> for bool {
    #[inline(always)]
    fn from(variant: JINRINI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `JINRINI` reader - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit.
pub type JINRINI_R = crate::BitReader<JINRINI_A>;
impl JINRINI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JINRINI_A {
        match self.bits {
            false => JINRINI_A::_0,
            true => JINRINI_A::_1,
        }
    }
    ///The transfer address is not initialized when the input of coded data is restarted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JINRINI_A::_0
    }
    ///The transfer address is initialized when the input of coded data is restarted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JINRINI_A::_1
    }
}
///Field `JINRINI` writer - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit.
pub type JINRINI_W<'a, REG> = crate::BitWriter<'a, REG, JINRINI_A>;
impl<'a, REG> JINRINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The transfer address is not initialized when the input of coded data is restarted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(JINRINI_A::_0)
    }
    ///The transfer address is initialized when the input of coded data is restarted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(JINRINI_A::_1)
    }
}
/**Specifies output image data pixel format.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPF_A {
    ///1: ARGB8888
    _01 = 1,
    ///2: RGB565
    _10 = 2,
    ///0: Setting prohibited
    OTHERS = 0,
}
impl From<OPF_A> for u8 {
    #[inline(always)]
    fn from(variant: OPF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPF_A {
    type Ux = u8;
}
impl crate::IsEnum for OPF_A {}
///Field `OPF` reader - Specifies output image data pixel format.
pub type OPF_R = crate::FieldReader<OPF_A>;
impl OPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPF_A {
        match self.bits {
            1 => OPF_A::_01,
            2 => OPF_A::_10,
            _ => OPF_A::OTHERS,
        }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OPF_A::_01
    }
    ///RGB565
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OPF_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), OPF_A::OTHERS)
    }
}
///Field `OPF` writer - Specifies output image data pixel format.
pub type OPF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OPF_A, crate::Safe>;
impl<'a, REG> OPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ARGB8888
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OPF_A::_01)
    }
    ///RGB565
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OPF_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(OPF_A::OTHERS)
    }
}
/**Horizontal Subsampling Subsamples horizontal output image data.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HINTER_A {
    ///0: No subsampling
    _00 = 0,
    ///1: Subsamples output data into 1/2.
    _01 = 1,
    ///2: Subsamples output data into 1/4.
    _10 = 2,
    ///3: Subsamples output data into 1/8.
    _11 = 3,
}
impl From<HINTER_A> for u8 {
    #[inline(always)]
    fn from(variant: HINTER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HINTER_A {
    type Ux = u8;
}
impl crate::IsEnum for HINTER_A {}
///Field `HINTER` reader - Horizontal Subsampling Subsamples horizontal output image data.
pub type HINTER_R = crate::FieldReader<HINTER_A>;
impl HINTER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HINTER_A {
        match self.bits {
            0 => HINTER_A::_00,
            1 => HINTER_A::_01,
            2 => HINTER_A::_10,
            3 => HINTER_A::_11,
            _ => unreachable!(),
        }
    }
    ///No subsampling
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HINTER_A::_00
    }
    ///Subsamples output data into 1/2.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HINTER_A::_01
    }
    ///Subsamples output data into 1/4.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HINTER_A::_10
    }
    ///Subsamples output data into 1/8.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HINTER_A::_11
    }
}
///Field `HINTER` writer - Horizontal Subsampling Subsamples horizontal output image data.
pub type HINTER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HINTER_A, crate::Safe>;
impl<'a, REG> HINTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No subsampling
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(HINTER_A::_00)
    }
    ///Subsamples output data into 1/2.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(HINTER_A::_01)
    }
    ///Subsamples output data into 1/4.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(HINTER_A::_10)
    }
    ///Subsamples output data into 1/8.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(HINTER_A::_11)
    }
}
/**Vertical SubsamplingSubsamples vertical output image data.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VINTER_A {
    ///0: No subsampling
    _00 = 0,
    ///1: Subsamples output data into 1/2.
    _01 = 1,
    ///2: Subsamples output data into 1/4.
    _10 = 2,
    ///3: Subsamples output data into 1/8.
    _11 = 3,
}
impl From<VINTER_A> for u8 {
    #[inline(always)]
    fn from(variant: VINTER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VINTER_A {
    type Ux = u8;
}
impl crate::IsEnum for VINTER_A {}
///Field `VINTER` reader - Vertical SubsamplingSubsamples vertical output image data.
pub type VINTER_R = crate::FieldReader<VINTER_A>;
impl VINTER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VINTER_A {
        match self.bits {
            0 => VINTER_A::_00,
            1 => VINTER_A::_01,
            2 => VINTER_A::_10,
            3 => VINTER_A::_11,
            _ => unreachable!(),
        }
    }
    ///No subsampling
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VINTER_A::_00
    }
    ///Subsamples output data into 1/2.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VINTER_A::_01
    }
    ///Subsamples output data into 1/4.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VINTER_A::_10
    }
    ///Subsamples output data into 1/8.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VINTER_A::_11
    }
}
///Field `VINTER` writer - Vertical SubsamplingSubsamples vertical output image data.
pub type VINTER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VINTER_A, crate::Safe>;
impl<'a, REG> VINTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No subsampling
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(VINTER_A::_00)
    }
    ///Subsamples output data into 1/2.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(VINTER_A::_01)
    }
    ///Subsamples output data into 1/4.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(VINTER_A::_10)
    }
    ///Subsamples output data into 1/8.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(VINTER_A::_11)
    }
}
impl R {
    ///Bits 0:2 - Byte/Word Swap Output image data in decompression is swapped.
    #[inline(always)]
    pub fn doutswap(&self) -> DOUTSWAP_R {
        DOUTSWAP_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Count Mode for Stopping Output Image Data Lines
    #[inline(always)]
    pub fn doutlc(&self) -> DOUTLC_R {
        DOUTLC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
    #[inline(always)]
    pub fn doutrini(&self) -> DOUTRINI_R {
        DOUTRINI_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:10 - Byte/Word/Longword Swap Input coded data in decompression is swapped.
    #[inline(always)]
    pub fn jinswap(&self) -> JINSWAP_R {
        JINSWAP_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - Count Mode Setting for Stopping Input Coded Data
    #[inline(always)]
    pub fn jinc(&self) -> JINC_R {
        JINC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit.
    #[inline(always)]
    pub fn jinrini(&self) -> JINRINI_R {
        JINRINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 24:25 - Specifies output image data pixel format.
    #[inline(always)]
    pub fn opf(&self) -> OPF_R {
        OPF_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Horizontal Subsampling Subsamples horizontal output image data.
    #[inline(always)]
    pub fn hinter(&self) -> HINTER_R {
        HINTER_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Vertical SubsamplingSubsamples vertical output image data.
    #[inline(always)]
    pub fn vinter(&self) -> VINTER_R {
        VINTER_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - Byte/Word Swap Output image data in decompression is swapped.
    #[inline(always)]
    pub fn doutswap(&mut self) -> DOUTSWAP_W<JIFDCNT_SPEC> {
        DOUTSWAP_W::new(self, 0)
    }
    ///Bit 4 - Count Mode for Stopping Output Image Data Lines
    #[inline(always)]
    pub fn doutlc(&mut self) -> DOUTLC_W<JIFDCNT_SPEC> {
        DOUTLC_W::new(self, 4)
    }
    ///Bit 5 - Output Image Data Lines Resume Command This bit is valid only when the count mode for stopping the output of image data lines is on. Setting this bit to 1 resumes writing image data. This bit is always read as 0.
    #[inline(always)]
    pub fn doutrcmd(&mut self) -> DOUTRCMD_W<JIFDCNT_SPEC> {
        DOUTRCMD_W::new(self, 5)
    }
    ///Bit 6 - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
    #[inline(always)]
    pub fn doutrini(&mut self) -> DOUTRINI_W<JIFDCNT_SPEC> {
        DOUTRINI_W::new(self, 6)
    }
    ///Bits 8:10 - Byte/Word/Longword Swap Input coded data in decompression is swapped.
    #[inline(always)]
    pub fn jinswap(&mut self) -> JINSWAP_W<JIFDCNT_SPEC> {
        JINSWAP_W::new(self, 8)
    }
    ///Bit 12 - Count Mode Setting for Stopping Input Coded Data
    #[inline(always)]
    pub fn jinc(&mut self) -> JINC_W<JIFDCNT_SPEC> {
        JINC_W::new(self, 12)
    }
    ///Bit 13 - Input Coded Data Resume CommandThis bit is valid only when the count mode for stopping the input of coded data is on. Setting this bit to 1 resumes reading input coded data. This bit is always read as 0.
    #[inline(always)]
    pub fn jinrcmd(&mut self) -> JINRCMD_W<JIFDCNT_SPEC> {
        JINRCMD_W::new(self, 13)
    }
    ///Bit 14 - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit.
    #[inline(always)]
    pub fn jinrini(&mut self) -> JINRINI_W<JIFDCNT_SPEC> {
        JINRINI_W::new(self, 14)
    }
    ///Bits 24:25 - Specifies output image data pixel format.
    #[inline(always)]
    pub fn opf(&mut self) -> OPF_W<JIFDCNT_SPEC> {
        OPF_W::new(self, 24)
    }
    ///Bits 26:27 - Horizontal Subsampling Subsamples horizontal output image data.
    #[inline(always)]
    pub fn hinter(&mut self) -> HINTER_W<JIFDCNT_SPEC> {
        HINTER_W::new(self, 26)
    }
    ///Bits 28:29 - Vertical SubsamplingSubsamples vertical output image data.
    #[inline(always)]
    pub fn vinter(&mut self) -> VINTER_W<JIFDCNT_SPEC> {
        VINTER_W::new(self, 28)
    }
}
/**JPEG Interface Decompression Control Register

You can [`read`](crate::Reg::read) this register and get [`jifdcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDCNT_SPEC;
impl crate::RegisterSpec for JIFDCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifdcnt::R`](R) reader structure
impl crate::Readable for JIFDCNT_SPEC {}
///`write(|w| ..)` method takes [`jifdcnt::W`](W) writer structure
impl crate::Writable for JIFDCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDCNT to value 0x0100_0000
impl crate::Resettable for JIFDCNT_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
