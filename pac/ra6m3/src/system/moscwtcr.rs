///Register `MOSCWTCR` reader
pub type R = crate::R<MOSCWTCR_SPEC>;
///Register `MOSCWTCR` writer
pub type W = crate::W<MOSCWTCR_SPEC>;
/**Main clock oscillator wait time setting

Value on reset: 5*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTS_A {
    ///1: Wait time = 35 cycles (133.5 μs)
    _0001 = 1,
    ///2: Wait time = 67 cycles (255.6 μs)
    _0010 = 2,
    ///3: Wait time = 131 cycles (499.7 μs)
    _0011 = 3,
    ///4: Wait time = 259 cycles (988.0 μs)
    _0100 = 4,
    ///5: Wait time = 547 cycles (2086.6 μs) (value after reset)
    _0101 = 5,
    ///6: Wait time = 1059 cycles (4039.8 μs)
    _0110 = 6,
    ///7: Wait time = 2147 cycles (8190.2 μs)
    _0111 = 7,
    ///8: Wait time = 4291 cycles (16368.9 μs)
    _1000 = 8,
    ///9: Wait time = 8163 cycles (31139.4 μs).
    _1001 = 9,
    ///0: settings prohibited.
    OTHERS = 0,
}
impl From<MSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSTS_A {
    type Ux = u8;
}
impl crate::IsEnum for MSTS_A {}
///Field `MSTS` reader - Main clock oscillator wait time setting
pub type MSTS_R = crate::FieldReader<MSTS_A>;
impl MSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTS_A {
        match self.bits {
            1 => MSTS_A::_0001,
            2 => MSTS_A::_0010,
            3 => MSTS_A::_0011,
            4 => MSTS_A::_0100,
            5 => MSTS_A::_0101,
            6 => MSTS_A::_0110,
            7 => MSTS_A::_0111,
            8 => MSTS_A::_1000,
            9 => MSTS_A::_1001,
            _ => MSTS_A::OTHERS,
        }
    }
    ///Wait time = 35 cycles (133.5 μs)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == MSTS_A::_0001
    }
    ///Wait time = 67 cycles (255.6 μs)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == MSTS_A::_0010
    }
    ///Wait time = 131 cycles (499.7 μs)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == MSTS_A::_0011
    }
    ///Wait time = 259 cycles (988.0 μs)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == MSTS_A::_0100
    }
    ///Wait time = 547 cycles (2086.6 μs) (value after reset)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == MSTS_A::_0101
    }
    ///Wait time = 1059 cycles (4039.8 μs)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == MSTS_A::_0110
    }
    ///Wait time = 2147 cycles (8190.2 μs)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == MSTS_A::_0111
    }
    ///Wait time = 4291 cycles (16368.9 μs)
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == MSTS_A::_1000
    }
    ///Wait time = 8163 cycles (31139.4 μs).
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == MSTS_A::_1001
    }
    ///settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), MSTS_A::OTHERS)
    }
}
///Field `MSTS` writer - Main clock oscillator wait time setting
pub type MSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSTS_A, crate::Safe>;
impl<'a, REG> MSTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Wait time = 35 cycles (133.5 μs)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0001)
    }
    ///Wait time = 67 cycles (255.6 μs)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0010)
    }
    ///Wait time = 131 cycles (499.7 μs)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0011)
    }
    ///Wait time = 259 cycles (988.0 μs)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0100)
    }
    ///Wait time = 547 cycles (2086.6 μs) (value after reset)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0101)
    }
    ///Wait time = 1059 cycles (4039.8 μs)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0110)
    }
    ///Wait time = 2147 cycles (8190.2 μs)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_0111)
    }
    ///Wait time = 4291 cycles (16368.9 μs)
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_1000)
    }
    ///Wait time = 8163 cycles (31139.4 μs).
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::_1001)
    }
    ///settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(MSTS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Main clock oscillator wait time setting
    #[inline(always)]
    pub fn msts(&self) -> MSTS_R {
        MSTS_R::new(self.bits & 0x0f)
    }
}
impl W {
    ///Bits 0:3 - Main clock oscillator wait time setting
    #[inline(always)]
    pub fn msts(&mut self) -> MSTS_W<MOSCWTCR_SPEC> {
        MSTS_W::new(self, 0)
    }
}
/**Main Clock Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MOSCWTCR_SPEC;
impl crate::RegisterSpec for MOSCWTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`moscwtcr::R`](R) reader structure
impl crate::Readable for MOSCWTCR_SPEC {}
///`write(|w| ..)` method takes [`moscwtcr::W`](W) writer structure
impl crate::Writable for MOSCWTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOSCWTCR to value 0x05
impl crate::Resettable for MOSCWTCR_SPEC {
    const RESET_VALUE: u8 = 0x05;
}
