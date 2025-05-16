///Register `JCMOD` reader
pub type R = crate::R<JCMOD_SPEC>;
///Register `JCMOD` writer
pub type W = crate::W<JCMOD_SPEC>;
/**Pixel FormatNOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REDU_A {
    ///1: YCbCr422(Compression) / YCbCr422(Decompression)
    _001 = 1,
    ///0: Setting prohibited(Compression) / YCbCr444(Decompression)
    _000 = 0,
    ///6: Setting prohibited(Compression) / YCbCr411/\[Decompression\]
    _110 = 6,
    ///2: Setting prohibited(Compression) / YCbCr420/\[Decompression\]
    _010 = 2,
    ///3: Setting prohibited(Compression) / Error (this module cannot process normally.)(Decompression])
    OTHERS = 3,
}
impl From<REDU_A> for u8 {
    #[inline(always)]
    fn from(variant: REDU_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REDU_A {
    type Ux = u8;
}
impl crate::IsEnum for REDU_A {}
///Field `REDU` reader - Pixel FormatNOTE: Read-only in Decompression.
pub type REDU_R = crate::FieldReader<REDU_A>;
impl REDU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REDU_A {
        match self.bits {
            1 => REDU_A::_001,
            0 => REDU_A::_000,
            6 => REDU_A::_110,
            2 => REDU_A::_010,
            _ => REDU_A::OTHERS,
        }
    }
    ///YCbCr422(Compression) / YCbCr422(Decompression)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == REDU_A::_001
    }
    ///Setting prohibited(Compression) / YCbCr444(Decompression)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == REDU_A::_000
    }
    ///Setting prohibited(Compression) / YCbCr411/\[Decompression\]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == REDU_A::_110
    }
    ///Setting prohibited(Compression) / YCbCr420/\[Decompression\]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == REDU_A::_010
    }
    ///Setting prohibited(Compression) / Error (this module cannot process normally.)(Decompression])
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), REDU_A::OTHERS)
    }
}
///Field `REDU` writer - Pixel FormatNOTE: Read-only in Decompression.
pub type REDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3, REDU_A, crate::Safe>;
impl<'a, REG> REDU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///YCbCr422(Compression) / YCbCr422(Decompression)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(REDU_A::_001)
    }
    ///Setting prohibited(Compression) / YCbCr444(Decompression)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(REDU_A::_000)
    }
    ///Setting prohibited(Compression) / YCbCr411/\[Decompression\]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(REDU_A::_110)
    }
    ///Setting prohibited(Compression) / YCbCr420/\[Decompression\]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(REDU_A::_010)
    }
    ///Setting prohibited(Compression) / Error (this module cannot process normally.)(Decompression])
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(REDU_A::OTHERS)
    }
}
/**Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_A {
    ///0: Compression process
    _0 = 0,
    ///1: Decompression process
    _1 = 1,
}
impl From<DSP_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSP` reader - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes.
pub type DSP_R = crate::BitReader<DSP_A>;
impl DSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSP_A {
        match self.bits {
            false => DSP_A::_0,
            true => DSP_A::_1,
        }
    }
    ///Compression process
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSP_A::_0
    }
    ///Decompression process
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSP_A::_1
    }
}
///Field `DSP` writer - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes.
pub type DSP_W<'a, REG> = crate::BitWriter<'a, REG, DSP_A>;
impl<'a, REG> DSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compression process
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSP_A::_0)
    }
    ///Decompression process
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSP_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Pixel FormatNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn redu(&self) -> REDU_R {
        REDU_R::new(self.bits & 7)
    }
    ///Bit 3 - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes.
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Pixel FormatNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn redu(&mut self) -> REDU_W<JCMOD_SPEC> {
        REDU_W::new(self, 0)
    }
    ///Bit 3 - Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes.
    #[inline(always)]
    pub fn dsp(&mut self) -> DSP_W<JCMOD_SPEC> {
        DSP_W::new(self, 3)
    }
}
/**JPEG Code Mode Register

You can [`read`](crate::Reg::read) this register and get [`jcmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCMOD_SPEC;
impl crate::RegisterSpec for JCMOD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcmod::R`](R) reader structure
impl crate::Readable for JCMOD_SPEC {}
///`write(|w| ..)` method takes [`jcmod::W`](W) writer structure
impl crate::Writable for JCMOD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCMOD to value 0
impl crate::Resettable for JCMOD_SPEC {}
