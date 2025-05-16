///Register `JIFECNT` reader
pub type R = crate::R<JIFECNT_SPEC>;
///Register `JIFECNT` writer
pub type W = crate::W<JIFECNT_SPEC>;
/**Byte/Halfword Swap

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DINSWAP_A {
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
impl From<DINSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: DINSWAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DINSWAP_A {
    type Ux = u8;
}
impl crate::IsEnum for DINSWAP_A {}
///Field `DINSWAP` reader - Byte/Halfword Swap
pub type DINSWAP_R = crate::FieldReader<DINSWAP_A>;
impl DINSWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DINSWAP_A {
        match self.bits {
            0 => DINSWAP_A::_000,
            1 => DINSWAP_A::_001,
            2 => DINSWAP_A::_010,
            3 => DINSWAP_A::_011,
            4 => DINSWAP_A::_100,
            5 => DINSWAP_A::_101,
            6 => DINSWAP_A::_110,
            7 => DINSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DINSWAP_A::_000
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DINSWAP_A::_001
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DINSWAP_A::_010
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DINSWAP_A::_011
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DINSWAP_A::_100
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DINSWAP_A::_101
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DINSWAP_A::_110
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DINSWAP_A::_111
    }
}
///Field `DINSWAP` writer - Byte/Halfword Swap
pub type DINSWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DINSWAP_A, crate::Safe>;
impl<'a, REG> DINSWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_000)
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_001)
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_010)
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_011)
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_100)
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_101)
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_110)
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Halfword - byte swap\]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(DINSWAP_A::_111)
    }
}
/**Count Mode Setting for Stopping Input Image Data Lines

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINLC_A {
    ///0: Count mode for stopping the input of image data lines is off
    _0 = 0,
    ///1: Count mode for stopping the input of image data lines is on
    _1 = 1,
}
impl From<DINLC_A> for bool {
    #[inline(always)]
    fn from(variant: DINLC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DINLC` reader - Count Mode Setting for Stopping Input Image Data Lines
pub type DINLC_R = crate::BitReader<DINLC_A>;
impl DINLC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DINLC_A {
        match self.bits {
            false => DINLC_A::_0,
            true => DINLC_A::_1,
        }
    }
    ///Count mode for stopping the input of image data lines is off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINLC_A::_0
    }
    ///Count mode for stopping the input of image data lines is on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINLC_A::_1
    }
}
///Field `DINLC` writer - Count Mode Setting for Stopping Input Image Data Lines
pub type DINLC_W<'a, REG> = crate::BitWriter<'a, REG, DINLC_A>;
impl<'a, REG> DINLC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count mode for stopping the input of image data lines is off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DINLC_A::_0)
    }
    ///Count mode for stopping the input of image data lines is on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DINLC_A::_1)
    }
}
///Field `DINRCMD` writer - Input Image Data Lines Resume Command This bit is valid only when the count mode for stopping the input of image data lines is on. Setting this bit to 1 resumes reading input image data. This bit is always read as 0.
pub type DINRCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINRINI_A {
    ///0: The transfer address is not initialized when the input of image data lines is restarted
    _0 = 0,
    ///1: The transfer address is initialized when the input of image data lines is restarted
    _1 = 1,
}
impl From<DINRINI_A> for bool {
    #[inline(always)]
    fn from(variant: DINRINI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DINRINI` reader - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
pub type DINRINI_R = crate::BitReader<DINRINI_A>;
impl DINRINI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DINRINI_A {
        match self.bits {
            false => DINRINI_A::_0,
            true => DINRINI_A::_1,
        }
    }
    ///The transfer address is not initialized when the input of image data lines is restarted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINRINI_A::_0
    }
    ///The transfer address is initialized when the input of image data lines is restarted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINRINI_A::_1
    }
}
///Field `DINRINI` writer - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
pub type DINRINI_W<'a, REG> = crate::BitWriter<'a, REG, DINRINI_A>;
impl<'a, REG> DINRINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The transfer address is not initialized when the input of image data lines is restarted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DINRINI_A::_0)
    }
    ///The transfer address is initialized when the input of image data lines is restarted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DINRINI_A::_1)
    }
}
/**Byte/Halfword/Word Swap Output coded data in compression is swapped.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JOUTSWAP_A {
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
    ///7: (8) (7) (6) (5) (4) (3) (2) (1) \[Word - Word - byte swap\]
    _111 = 7,
}
impl From<JOUTSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: JOUTSWAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JOUTSWAP_A {
    type Ux = u8;
}
impl crate::IsEnum for JOUTSWAP_A {}
///Field `JOUTSWAP` reader - Byte/Halfword/Word Swap Output coded data in compression is swapped.
pub type JOUTSWAP_R = crate::FieldReader<JOUTSWAP_A>;
impl JOUTSWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JOUTSWAP_A {
        match self.bits {
            0 => JOUTSWAP_A::_000,
            1 => JOUTSWAP_A::_001,
            2 => JOUTSWAP_A::_010,
            3 => JOUTSWAP_A::_011,
            4 => JOUTSWAP_A::_100,
            5 => JOUTSWAP_A::_101,
            6 => JOUTSWAP_A::_110,
            7 => JOUTSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == JOUTSWAP_A::_000
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == JOUTSWAP_A::_001
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == JOUTSWAP_A::_010
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == JOUTSWAP_A::_011
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == JOUTSWAP_A::_100
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == JOUTSWAP_A::_101
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == JOUTSWAP_A::_110
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Word - byte swap\]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == JOUTSWAP_A::_111
    }
}
///Field `JOUTSWAP` writer - Byte/Halfword/Word Swap Output coded data in compression is swapped.
pub type JOUTSWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, JOUTSWAP_A, crate::Safe>;
impl<'a, REG> JOUTSWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///(1) (2) (3) (4) (5) (6) (7) (8)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_000)
    }
    ///(2) (1) (4) (3) (6) (5) (8) (7) \[Byte swap\]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_001)
    }
    ///(3) (4) (1) (2) (7) (8) (5) (6) \[Halfword swap\]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_010)
    }
    ///(4) (3) (2) (1) (8) (7) (6) (5) \[Halfword - byte swap\]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_011)
    }
    ///(5) (6) (7) (8) (1) (2) (3) (4) \[Word swap\]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_100)
    }
    ///(6) (5) (8) (7) (2) (1) (4) (3) \[Word - byte swap\]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_101)
    }
    ///(7) (8) (5) (6) (3) (4) (1) (2) \[Word - Halfword swap\]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_110)
    }
    ///(8) (7) (6) (5) (4) (3) (2) (1) \[Word - Word - byte swap\]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(JOUTSWAP_A::_111)
    }
}
impl R {
    ///Bits 0:2 - Byte/Halfword Swap
    #[inline(always)]
    pub fn dinswap(&self) -> DINSWAP_R {
        DINSWAP_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Count Mode Setting for Stopping Input Image Data Lines
    #[inline(always)]
    pub fn dinlc(&self) -> DINLC_R {
        DINLC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
    #[inline(always)]
    pub fn dinrini(&self) -> DINRINI_R {
        DINRINI_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:10 - Byte/Halfword/Word Swap Output coded data in compression is swapped.
    #[inline(always)]
    pub fn joutswap(&self) -> JOUTSWAP_R {
        JOUTSWAP_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Byte/Halfword Swap
    #[inline(always)]
    pub fn dinswap(&mut self) -> DINSWAP_W<JIFECNT_SPEC> {
        DINSWAP_W::new(self, 0)
    }
    ///Bit 4 - Count Mode Setting for Stopping Input Image Data Lines
    #[inline(always)]
    pub fn dinlc(&mut self) -> DINLC_W<JIFECNT_SPEC> {
        DINLC_W::new(self, 4)
    }
    ///Bit 5 - Input Image Data Lines Resume Command This bit is valid only when the count mode for stopping the input of image data lines is on. Setting this bit to 1 resumes reading input image data. This bit is always read as 0.
    #[inline(always)]
    pub fn dinrcmd(&mut self) -> DINRCMD_W<JIFECNT_SPEC> {
        DINRCMD_W::new(self, 5)
    }
    ///Bit 6 - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.
    #[inline(always)]
    pub fn dinrini(&mut self) -> DINRINI_W<JIFECNT_SPEC> {
        DINRINI_W::new(self, 6)
    }
    ///Bits 8:10 - Byte/Halfword/Word Swap Output coded data in compression is swapped.
    #[inline(always)]
    pub fn joutswap(&mut self) -> JOUTSWAP_W<JIFECNT_SPEC> {
        JOUTSWAP_W::new(self, 8)
    }
}
/**JPEG Interface Compression Control Register

You can [`read`](crate::Reg::read) this register and get [`jifecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFECNT_SPEC;
impl crate::RegisterSpec for JIFECNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifecnt::R`](R) reader structure
impl crate::Readable for JIFECNT_SPEC {}
///`write(|w| ..)` method takes [`jifecnt::W`](W) writer structure
impl crate::Writable for JIFECNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFECNT to value 0
impl crate::Resettable for JIFECNT_SPEC {}
