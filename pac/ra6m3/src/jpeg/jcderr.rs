///Register `JCDERR` reader
pub type R = crate::R<JCDERR_SPEC>;
///Register `JCDERR` writer
pub type W = crate::W<JCDERR_SPEC>;
/**Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression.

Value on reset: 10*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERR_A {
    ///0: Normal(Decompression error codes)/Normal(Segment error codes)
    _0000 = 0,
    ///1: SOI not detected(Decompression error codes)
    _0001 = 1,
    ///2: SOF1 to SOFF detected(Decompression error codes)
    _0010 = 2,
    ///3: Unprovided pixel format detected(Decompression error codes)
    _0011 = 3,
    ///4: SOF accuracy error(Decompression error codes)
    _0100 = 4,
    ///5: DQT accuracy error(Decompression error codes)
    _0101 = 5,
    ///6: Component error 1(Decompression error codes)
    _0110 = 6,
    ///7: Component error 2(Decompression error codes)
    _0111 = 7,
    ///8: SOF0, DQT, and DHT not detected when SOS detected(Decompression error codes)
    _1000 = 8,
    ///9: SOS not detected(Decompression error codes)
    _1001 = 9,
    ///10: EOI not detected (default)(Decompression error codes)
    _1010 = 10,
    ///11: Restart interval data number error detected(Decompression error codes)/Restart interval data number error(Segment error codes)
    _1011 = 11,
    ///12: Image size error detected(Decompression error codes)/Image size error(Segment error codes)
    _1100 = 12,
    ///13: Last MCU data number error detected(Decompression error codes)/Last MCU data number error(Segment error codes)
    _1101 = 13,
    ///14: Block data number error detected(Decompression error codes)/Block data number error(Segment error codes)
    _1110 = 14,
    ///15: Setting prohibited
    OTHERS = 15,
}
impl From<ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ERR_A {
    type Ux = u8;
}
impl crate::IsEnum for ERR_A {}
///Field `ERR` reader - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression.
pub type ERR_R = crate::FieldReader<ERR_A>;
impl ERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERR_A {
        match self.bits {
            0 => ERR_A::_0000,
            1 => ERR_A::_0001,
            2 => ERR_A::_0010,
            3 => ERR_A::_0011,
            4 => ERR_A::_0100,
            5 => ERR_A::_0101,
            6 => ERR_A::_0110,
            7 => ERR_A::_0111,
            8 => ERR_A::_1000,
            9 => ERR_A::_1001,
            10 => ERR_A::_1010,
            11 => ERR_A::_1011,
            12 => ERR_A::_1100,
            13 => ERR_A::_1101,
            14 => ERR_A::_1110,
            15 => ERR_A::OTHERS,
            _ => unreachable!(),
        }
    }
    ///Normal(Decompression error codes)/Normal(Segment error codes)
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ERR_A::_0000
    }
    ///SOI not detected(Decompression error codes)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ERR_A::_0001
    }
    ///SOF1 to SOFF detected(Decompression error codes)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ERR_A::_0010
    }
    ///Unprovided pixel format detected(Decompression error codes)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ERR_A::_0011
    }
    ///SOF accuracy error(Decompression error codes)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ERR_A::_0100
    }
    ///DQT accuracy error(Decompression error codes)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ERR_A::_0101
    }
    ///Component error 1(Decompression error codes)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ERR_A::_0110
    }
    ///Component error 2(Decompression error codes)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ERR_A::_0111
    }
    ///SOF0, DQT, and DHT not detected when SOS detected(Decompression error codes)
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ERR_A::_1000
    }
    ///SOS not detected(Decompression error codes)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ERR_A::_1001
    }
    ///EOI not detected (default)(Decompression error codes)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ERR_A::_1010
    }
    ///Restart interval data number error detected(Decompression error codes)/Restart interval data number error(Segment error codes)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ERR_A::_1011
    }
    ///Image size error detected(Decompression error codes)/Image size error(Segment error codes)
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ERR_A::_1100
    }
    ///Last MCU data number error detected(Decompression error codes)/Last MCU data number error(Segment error codes)
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ERR_A::_1101
    }
    ///Block data number error detected(Decompression error codes)/Block data number error(Segment error codes)
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ERR_A::_1110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == ERR_A::OTHERS
    }
}
///Field `ERR` writer - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression.
pub type ERR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ERR_A, crate::Safe>;
impl<'a, REG> ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal(Decompression error codes)/Normal(Segment error codes)
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0000)
    }
    ///SOI not detected(Decompression error codes)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0001)
    }
    ///SOF1 to SOFF detected(Decompression error codes)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0010)
    }
    ///Unprovided pixel format detected(Decompression error codes)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0011)
    }
    ///SOF accuracy error(Decompression error codes)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0100)
    }
    ///DQT accuracy error(Decompression error codes)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0101)
    }
    ///Component error 1(Decompression error codes)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0110)
    }
    ///Component error 2(Decompression error codes)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_0111)
    }
    ///SOF0, DQT, and DHT not detected when SOS detected(Decompression error codes)
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1000)
    }
    ///SOS not detected(Decompression error codes)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1001)
    }
    ///EOI not detected (default)(Decompression error codes)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1010)
    }
    ///Restart interval data number error detected(Decompression error codes)/Restart interval data number error(Segment error codes)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1011)
    }
    ///Image size error detected(Decompression error codes)/Image size error(Segment error codes)
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1100)
    }
    ///Last MCU data number error detected(Decompression error codes)/Last MCU data number error(Segment error codes)
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1101)
    }
    ///Block data number error detected(Decompression error codes)/Block data number error(Segment error codes)
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::_1110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression.
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(self.bits & 0x0f)
    }
}
impl W {
    ///Bits 0:3 - Error Code (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression.
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<JCDERR_SPEC> {
        ERR_W::new(self, 0)
    }
}
/**JPEG Code Decode Error Register

You can [`read`](crate::Reg::read) this register and get [`jcderr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcderr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCDERR_SPEC;
impl crate::RegisterSpec for JCDERR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcderr::R`](R) reader structure
impl crate::Readable for JCDERR_SPEC {}
///`write(|w| ..)` method takes [`jcderr::W`](W) writer structure
impl crate::Writable for JCDERR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCDERR to value 0x0a
impl crate::Resettable for JCDERR_SPEC {
    const RESET_VALUE: u8 = 0x0a;
}
