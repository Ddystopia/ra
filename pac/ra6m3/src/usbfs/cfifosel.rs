///Register `CFIFOSEL` reader
pub type R = crate::R<CFIFOSEL_SPEC>;
///Register `CFIFOSEL` writer
pub type W = crate::W<CFIFOSEL_SPEC>;
/**CFIFO Port Access Pipe Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURPIPE_A {
    ///0: DCP (Default control pipe)
    _0000 = 0,
    ///1: Pipe 1
    _0001 = 1,
    ///2: Pipe 2
    _0010 = 2,
    ///3: Pipe 3
    _0011 = 3,
    ///4: Pipe 4
    _0100 = 4,
    ///5: Pipe 5
    _0101 = 5,
    ///6: Pipe 6
    _0110 = 6,
    ///7: Pipe 7
    _0111 = 7,
    ///8: Pipe 8
    _1000 = 8,
    ///9: Pipe 9
    _1001 = 9,
    ///10: Setting prohibited
    OTHERS = 10,
}
impl From<CURPIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CURPIPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CURPIPE_A {
    type Ux = u8;
}
impl crate::IsEnum for CURPIPE_A {}
///Field `CURPIPE` reader - CFIFO Port Access Pipe Specification
pub type CURPIPE_R = crate::FieldReader<CURPIPE_A>;
impl CURPIPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CURPIPE_A {
        match self.bits {
            0 => CURPIPE_A::_0000,
            1 => CURPIPE_A::_0001,
            2 => CURPIPE_A::_0010,
            3 => CURPIPE_A::_0011,
            4 => CURPIPE_A::_0100,
            5 => CURPIPE_A::_0101,
            6 => CURPIPE_A::_0110,
            7 => CURPIPE_A::_0111,
            8 => CURPIPE_A::_1000,
            9 => CURPIPE_A::_1001,
            _ => CURPIPE_A::OTHERS,
        }
    }
    ///DCP (Default control pipe)
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CURPIPE_A::_0000
    }
    ///Pipe 1
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CURPIPE_A::_0001
    }
    ///Pipe 2
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CURPIPE_A::_0010
    }
    ///Pipe 3
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CURPIPE_A::_0011
    }
    ///Pipe 4
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CURPIPE_A::_0100
    }
    ///Pipe 5
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CURPIPE_A::_0101
    }
    ///Pipe 6
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CURPIPE_A::_0110
    }
    ///Pipe 7
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CURPIPE_A::_0111
    }
    ///Pipe 8
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CURPIPE_A::_1000
    }
    ///Pipe 9
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CURPIPE_A::_1001
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CURPIPE_A::OTHERS)
    }
}
///Field `CURPIPE` writer - CFIFO Port Access Pipe Specification
pub type CURPIPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CURPIPE_A, crate::Safe>;
impl<'a, REG> CURPIPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DCP (Default control pipe)
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0000)
    }
    ///Pipe 1
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0001)
    }
    ///Pipe 2
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0010)
    }
    ///Pipe 3
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0011)
    }
    ///Pipe 4
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0100)
    }
    ///Pipe 5
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0101)
    }
    ///Pipe 6
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0110)
    }
    ///Pipe 7
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_0111)
    }
    ///Pipe 8
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_1000)
    }
    ///Pipe 9
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::_1001)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CURPIPE_A::OTHERS)
    }
}
/**CFIFO Port Access Direction When DCP is Selected

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    ///0: Reading from the buffer memory is selected
    _0 = 0,
    ///1: Writing to the buffer memory is selected
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ISEL` reader - CFIFO Port Access Direction When DCP is Selected
pub type ISEL_R = crate::BitReader<ISEL_A>;
impl ISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    ///Reading from the buffer memory is selected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    ///Writing to the buffer memory is selected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
///Field `ISEL` writer - CFIFO Port Access Direction When DCP is Selected
pub type ISEL_W<'a, REG> = crate::BitWriter<'a, REG, ISEL_A>;
impl<'a, REG> ISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reading from the buffer memory is selected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_0)
    }
    ///Writing to the buffer memory is selected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_1)
    }
}
/**CFIFO Port Endian Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIGEND_A {
    ///0: Little endian
    _0 = 0,
    ///1: Big endian
    _1 = 1,
}
impl From<BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BIGEND` reader - CFIFO Port Endian Control
pub type BIGEND_R = crate::BitReader<BIGEND_A>;
impl BIGEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIGEND_A {
        match self.bits {
            false => BIGEND_A::_0,
            true => BIGEND_A::_1,
        }
    }
    ///Little endian
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIGEND_A::_0
    }
    ///Big endian
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIGEND_A::_1
    }
}
///Field `BIGEND` writer - CFIFO Port Endian Control
pub type BIGEND_W<'a, REG> = crate::BitWriter<'a, REG, BIGEND_A>;
impl<'a, REG> BIGEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little endian
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BIGEND_A::_0)
    }
    ///Big endian
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BIGEND_A::_1)
    }
}
/**CFIFO Port Access Bit Width

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBW_A {
    ///0: 8-bit width
    _0 = 0,
    ///1: 16-bit width
    _1 = 1,
}
impl From<MBW_A> for bool {
    #[inline(always)]
    fn from(variant: MBW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MBW` reader - CFIFO Port Access Bit Width
pub type MBW_R = crate::BitReader<MBW_A>;
impl MBW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MBW_A {
        match self.bits {
            false => MBW_A::_0,
            true => MBW_A::_1,
        }
    }
    ///8-bit width
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MBW_A::_0
    }
    ///16-bit width
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MBW_A::_1
    }
}
///Field `MBW` writer - CFIFO Port Access Bit Width
pub type MBW_W<'a, REG> = crate::BitWriter<'a, REG, MBW_A>;
impl<'a, REG> MBW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///8-bit width
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MBW_A::_0)
    }
    ///16-bit width
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MBW_A::_1)
    }
}
/**Buffer Pointer Rewind

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REW_A {
    ///0: The buffer pointer is not rewound.
    _0 = 0,
    ///1: The buffer pointer is rewound.
    _1 = 1,
}
impl From<REW_A> for bool {
    #[inline(always)]
    fn from(variant: REW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REW` reader - Buffer Pointer Rewind
pub type REW_R = crate::BitReader<REW_A>;
impl REW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REW_A {
        match self.bits {
            false => REW_A::_0,
            true => REW_A::_1,
        }
    }
    ///The buffer pointer is not rewound.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REW_A::_0
    }
    ///The buffer pointer is rewound.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REW_A::_1
    }
}
/**Read Count Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCNT_A {
    ///0: The DTLN\[8:0\] bits (CFIFOCRT.DTLN\[8:0\], D0FIFOCRT.DTLN\[8:0\], D1FIFOCRT.DTLN\[8:0\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\[8:0\] bit value is cleared when all the data has been read from only a single plane.)
    _0 = 0,
    ///1: The DTLN\[8:0\] bits are decremented each time the receive data is read from the CFIFO.
    _1 = 1,
}
impl From<RCNT_A> for bool {
    #[inline(always)]
    fn from(variant: RCNT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCNT` reader - Read Count Mode
pub type RCNT_R = crate::BitReader<RCNT_A>;
impl RCNT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCNT_A {
        match self.bits {
            false => RCNT_A::_0,
            true => RCNT_A::_1,
        }
    }
    ///The DTLN\[8:0\] bits (CFIFOCRT.DTLN\[8:0\], D0FIFOCRT.DTLN\[8:0\], D1FIFOCRT.DTLN\[8:0\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\[8:0\] bit value is cleared when all the data has been read from only a single plane.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCNT_A::_0
    }
    ///The DTLN\[8:0\] bits are decremented each time the receive data is read from the CFIFO.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCNT_A::_1
    }
}
///Field `RCNT` writer - Read Count Mode
pub type RCNT_W<'a, REG> = crate::BitWriter<'a, REG, RCNT_A>;
impl<'a, REG> RCNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The DTLN\[8:0\] bits (CFIFOCRT.DTLN\[8:0\], D0FIFOCRT.DTLN\[8:0\], D1FIFOCRT.DTLN\[8:0\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\[8:0\] bit value is cleared when all the data has been read from only a single plane.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCNT_A::_0)
    }
    ///The DTLN\[8:0\] bits are decremented each time the receive data is read from the CFIFO.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCNT_A::_1)
    }
}
impl R {
    ///Bits 0:3 - CFIFO Port Access Pipe Specification
    #[inline(always)]
    pub fn curpipe(&self) -> CURPIPE_R {
        CURPIPE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - CFIFO Port Access Direction When DCP is Selected
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - CFIFO Port Endian Control
    #[inline(always)]
    pub fn bigend(&self) -> BIGEND_R {
        BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - CFIFO Port Access Bit Width
    #[inline(always)]
    pub fn mbw(&self) -> MBW_R {
        MBW_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Buffer Pointer Rewind
    #[inline(always)]
    pub fn rew(&self) -> REW_R {
        REW_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Read Count Mode
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - CFIFO Port Access Pipe Specification
    #[inline(always)]
    pub fn curpipe(&mut self) -> CURPIPE_W<CFIFOSEL_SPEC> {
        CURPIPE_W::new(self, 0)
    }
    ///Bit 5 - CFIFO Port Access Direction When DCP is Selected
    #[inline(always)]
    pub fn isel(&mut self) -> ISEL_W<CFIFOSEL_SPEC> {
        ISEL_W::new(self, 5)
    }
    ///Bit 8 - CFIFO Port Endian Control
    #[inline(always)]
    pub fn bigend(&mut self) -> BIGEND_W<CFIFOSEL_SPEC> {
        BIGEND_W::new(self, 8)
    }
    ///Bit 10 - CFIFO Port Access Bit Width
    #[inline(always)]
    pub fn mbw(&mut self) -> MBW_W<CFIFOSEL_SPEC> {
        MBW_W::new(self, 10)
    }
    ///Bit 15 - Read Count Mode
    #[inline(always)]
    pub fn rcnt(&mut self) -> RCNT_W<CFIFOSEL_SPEC> {
        RCNT_W::new(self, 15)
    }
}
/**CFIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`cfifosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFIFOSEL_SPEC;
impl crate::RegisterSpec for CFIFOSEL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`cfifosel::R`](R) reader structure
impl crate::Readable for CFIFOSEL_SPEC {}
///`write(|w| ..)` method takes [`cfifosel::W`](W) writer structure
impl crate::Writable for CFIFOSEL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOSEL to value 0
impl crate::Resettable for CFIFOSEL_SPEC {}
