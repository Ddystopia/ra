///Register `CTSUSSC` reader
pub type R = crate::R<CTSUSSC_SPEC>;
///Register `CTSUSSC` writer
pub type W = crate::W<CTSUSSC_SPEC>;
/**CTSU Spectrum Diffusion Frequency Division Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUSSDIV_A {
    ///0: 4.00 <= fb
    _0000 = 0,
    ///1: 2.00 <= fb < 4.00
    _0001 = 1,
    ///2: 1.33 <= fb < 2.00
    _0010 = 2,
    ///3: 1.00 <= fb < 1.33
    _0011 = 3,
    ///4: 0.80 <= fb < 1.00
    _0100 = 4,
    ///5: 0.67 <= fb < 0.80
    _0101 = 5,
    ///6: 0.57 <= fb < 0.67
    _0110 = 6,
    ///7: 0.50 <= fb < 0.57
    _0111 = 7,
    ///8: 0.44 <= fb < 0.50
    _1000 = 8,
    ///9: 0.40 <= fb < 0.44
    _1001 = 9,
    ///10: 0.36 <= fb < 0.40
    _1010 = 10,
    ///11: 0.33 <= fb < 0.36
    _1011 = 11,
    ///12: 0.31 <= fb < 0.33
    _1100 = 12,
    ///13: 0.29 <= fb < 0.31
    _1101 = 13,
    ///14: 0.27 <= fb < 0.29
    _1110 = 14,
    ///15: fb < 0.27
    _1111 = 15,
}
impl From<CTSUSSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUSSDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUSSDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUSSDIV_A {}
///Field `CTSUSSDIV` reader - CTSU Spectrum Diffusion Frequency Division Setting
pub type CTSUSSDIV_R = crate::FieldReader<CTSUSSDIV_A>;
impl CTSUSSDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSSDIV_A {
        match self.bits {
            0 => CTSUSSDIV_A::_0000,
            1 => CTSUSSDIV_A::_0001,
            2 => CTSUSSDIV_A::_0010,
            3 => CTSUSSDIV_A::_0011,
            4 => CTSUSSDIV_A::_0100,
            5 => CTSUSSDIV_A::_0101,
            6 => CTSUSSDIV_A::_0110,
            7 => CTSUSSDIV_A::_0111,
            8 => CTSUSSDIV_A::_1000,
            9 => CTSUSSDIV_A::_1001,
            10 => CTSUSSDIV_A::_1010,
            11 => CTSUSSDIV_A::_1011,
            12 => CTSUSSDIV_A::_1100,
            13 => CTSUSSDIV_A::_1101,
            14 => CTSUSSDIV_A::_1110,
            15 => CTSUSSDIV_A::_1111,
            _ => unreachable!(),
        }
    }
    ///4.00 <= fb
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CTSUSSDIV_A::_0000
    }
    ///2.00 <= fb < 4.00
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CTSUSSDIV_A::_0001
    }
    ///1.33 <= fb < 2.00
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CTSUSSDIV_A::_0010
    }
    ///1.00 <= fb < 1.33
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CTSUSSDIV_A::_0011
    }
    ///0.80 <= fb < 1.00
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CTSUSSDIV_A::_0100
    }
    ///0.67 <= fb < 0.80
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CTSUSSDIV_A::_0101
    }
    ///0.57 <= fb < 0.67
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CTSUSSDIV_A::_0110
    }
    ///0.50 <= fb < 0.57
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CTSUSSDIV_A::_0111
    }
    ///0.44 <= fb < 0.50
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CTSUSSDIV_A::_1000
    }
    ///0.40 <= fb < 0.44
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CTSUSSDIV_A::_1001
    }
    ///0.36 <= fb < 0.40
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == CTSUSSDIV_A::_1010
    }
    ///0.33 <= fb < 0.36
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == CTSUSSDIV_A::_1011
    }
    ///0.31 <= fb < 0.33
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == CTSUSSDIV_A::_1100
    }
    ///0.29 <= fb < 0.31
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == CTSUSSDIV_A::_1101
    }
    ///0.27 <= fb < 0.29
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == CTSUSSDIV_A::_1110
    }
    ///fb < 0.27
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CTSUSSDIV_A::_1111
    }
}
///Field `CTSUSSDIV` writer - CTSU Spectrum Diffusion Frequency Division Setting
pub type CTSUSSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CTSUSSDIV_A, crate::Safe>;
impl<'a, REG> CTSUSSDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4.00 <= fb
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0000)
    }
    ///2.00 <= fb < 4.00
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0001)
    }
    ///1.33 <= fb < 2.00
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0010)
    }
    ///1.00 <= fb < 1.33
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0011)
    }
    ///0.80 <= fb < 1.00
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0100)
    }
    ///0.67 <= fb < 0.80
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0101)
    }
    ///0.57 <= fb < 0.67
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0110)
    }
    ///0.50 <= fb < 0.57
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_0111)
    }
    ///0.44 <= fb < 0.50
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1000)
    }
    ///0.40 <= fb < 0.44
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1001)
    }
    ///0.36 <= fb < 0.40
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1010)
    }
    ///0.33 <= fb < 0.36
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1011)
    }
    ///0.31 <= fb < 0.33
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1100)
    }
    ///0.29 <= fb < 0.31
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1101)
    }
    ///0.27 <= fb < 0.29
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1110)
    }
    ///fb < 0.27
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSSDIV_A::_1111)
    }
}
impl R {
    ///Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting
    #[inline(always)]
    pub fn ctsussdiv(&self) -> CTSUSSDIV_R {
        CTSUSSDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting
    #[inline(always)]
    pub fn ctsussdiv(&mut self) -> CTSUSSDIV_W<CTSUSSC_SPEC> {
        CTSUSSDIV_W::new(self, 8)
    }
}
/**CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsussc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsussc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUSSC_SPEC;
impl crate::RegisterSpec for CTSUSSC_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctsussc::R`](R) reader structure
impl crate::Readable for CTSUSSC_SPEC {}
///`write(|w| ..)` method takes [`ctsussc::W`](W) writer structure
impl crate::Writable for CTSUSSC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSSC to value 0
impl crate::Resettable for CTSUSSC_SPEC {}
