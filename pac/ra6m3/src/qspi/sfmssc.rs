///Register `SFMSSC` reader
pub type R = crate::R<SFMSSC_SPEC>;
///Register `SFMSSC` writer
pub type W = crate::W<SFMSSC_SPEC>;
/**Selection of a minimum high-level width of the QSSL signal

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSW_A {
    ///0: 1 x QSPCLK
    _0000 = 0,
    ///1: 2 x QSPCLK
    _0001 = 1,
    ///2: 3 x QSPCLK
    _0010 = 2,
    ///3: 4 x QSPCLK
    _0011 = 3,
    ///4: 5 x QSPCLK
    _0100 = 4,
    ///5: 6 x QSPCLK
    _0101 = 5,
    ///6: 7 x QSPCLK
    _0110 = 6,
    ///7: 8 x QSPCLK
    _0111 = 7,
    ///8: 9 x QSPCLK
    _1000 = 8,
    ///9: 10 x QSPCLK
    _1001 = 9,
    ///10: 11 x QSPCLK
    _1010 = 10,
    ///11: 12 x QSPCLK
    _1011 = 11,
    ///12: 13 x QSPCLK
    _1100 = 12,
    ///13: 14 x QSPCLK
    _1101 = 13,
    ///14: 15 x QSPCLK
    _1110 = 14,
    ///15: 16 x QSPCLK
    _1111 = 15,
}
impl From<SFMSW_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMSW_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMSW_A {}
///Field `SFMSW` reader - Selection of a minimum high-level width of the QSSL signal
pub type SFMSW_R = crate::FieldReader<SFMSW_A>;
impl SFMSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMSW_A {
        match self.bits {
            0 => SFMSW_A::_0000,
            1 => SFMSW_A::_0001,
            2 => SFMSW_A::_0010,
            3 => SFMSW_A::_0011,
            4 => SFMSW_A::_0100,
            5 => SFMSW_A::_0101,
            6 => SFMSW_A::_0110,
            7 => SFMSW_A::_0111,
            8 => SFMSW_A::_1000,
            9 => SFMSW_A::_1001,
            10 => SFMSW_A::_1010,
            11 => SFMSW_A::_1011,
            12 => SFMSW_A::_1100,
            13 => SFMSW_A::_1101,
            14 => SFMSW_A::_1110,
            15 => SFMSW_A::_1111,
            _ => unreachable!(),
        }
    }
    ///1 x QSPCLK
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SFMSW_A::_0000
    }
    ///2 x QSPCLK
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SFMSW_A::_0001
    }
    ///3 x QSPCLK
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SFMSW_A::_0010
    }
    ///4 x QSPCLK
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SFMSW_A::_0011
    }
    ///5 x QSPCLK
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SFMSW_A::_0100
    }
    ///6 x QSPCLK
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SFMSW_A::_0101
    }
    ///7 x QSPCLK
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SFMSW_A::_0110
    }
    ///8 x QSPCLK
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == SFMSW_A::_0111
    }
    ///9 x QSPCLK
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SFMSW_A::_1000
    }
    ///10 x QSPCLK
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == SFMSW_A::_1001
    }
    ///11 x QSPCLK
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == SFMSW_A::_1010
    }
    ///12 x QSPCLK
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == SFMSW_A::_1011
    }
    ///13 x QSPCLK
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == SFMSW_A::_1100
    }
    ///14 x QSPCLK
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == SFMSW_A::_1101
    }
    ///15 x QSPCLK
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == SFMSW_A::_1110
    }
    ///16 x QSPCLK
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == SFMSW_A::_1111
    }
}
///Field `SFMSW` writer - Selection of a minimum high-level width of the QSSL signal
pub type SFMSW_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SFMSW_A, crate::Safe>;
impl<'a, REG> SFMSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 x QSPCLK
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0000)
    }
    ///2 x QSPCLK
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0001)
    }
    ///3 x QSPCLK
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0010)
    }
    ///4 x QSPCLK
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0011)
    }
    ///5 x QSPCLK
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0100)
    }
    ///6 x QSPCLK
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0101)
    }
    ///7 x QSPCLK
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0110)
    }
    ///8 x QSPCLK
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_0111)
    }
    ///9 x QSPCLK
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1000)
    }
    ///10 x QSPCLK
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1001)
    }
    ///11 x QSPCLK
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1010)
    }
    ///12 x QSPCLK
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1011)
    }
    ///13 x QSPCLK
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1100)
    }
    ///14 x QSPCLK
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1101)
    }
    ///15 x QSPCLK
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1110)
    }
    ///16 x QSPCLK
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSW_A::_1111)
    }
}
/**QSSL signal release timing selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSHD_A {
    ///0: Releases QSSL 0.5*SCK after the last rising edge of QSPCLK
    _0 = 0,
    ///1: Releases QSSL 1.5*SCK after the last rising edge of QSPCLK
    _1 = 1,
}
impl From<SFMSHD_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSHD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMSHD` reader - QSSL signal release timing selection
pub type SFMSHD_R = crate::BitReader<SFMSHD_A>;
impl SFMSHD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMSHD_A {
        match self.bits {
            false => SFMSHD_A::_0,
            true => SFMSHD_A::_1,
        }
    }
    ///Releases QSSL 0.5*SCK after the last rising edge of QSPCLK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSHD_A::_0
    }
    ///Releases QSSL 1.5*SCK after the last rising edge of QSPCLK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSHD_A::_1
    }
}
///Field `SFMSHD` writer - QSSL signal release timing selection
pub type SFMSHD_W<'a, REG> = crate::BitWriter<'a, REG, SFMSHD_A>;
impl<'a, REG> SFMSHD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Releases QSSL 0.5*SCK after the last rising edge of QSPCLK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSHD_A::_0)
    }
    ///Releases QSSL 1.5*SCK after the last rising edge of QSPCLK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSHD_A::_1)
    }
}
/**QSSL signal output timing selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSLD_A {
    ///0: Outputs QSSL 0.5*SCK before the first rising edge of QSPCLK
    _0 = 0,
    ///1: Outputs QSSL 1.5*SCK before the first rising edge of QSPCLK
    _1 = 1,
}
impl From<SFMSLD_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMSLD` reader - QSSL signal output timing selection
pub type SFMSLD_R = crate::BitReader<SFMSLD_A>;
impl SFMSLD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMSLD_A {
        match self.bits {
            false => SFMSLD_A::_0,
            true => SFMSLD_A::_1,
        }
    }
    ///Outputs QSSL 0.5*SCK before the first rising edge of QSPCLK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSLD_A::_0
    }
    ///Outputs QSSL 1.5*SCK before the first rising edge of QSPCLK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSLD_A::_1
    }
}
///Field `SFMSLD` writer - QSSL signal output timing selection
pub type SFMSLD_W<'a, REG> = crate::BitWriter<'a, REG, SFMSLD_A>;
impl<'a, REG> SFMSLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Outputs QSSL 0.5*SCK before the first rising edge of QSPCLK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSLD_A::_0)
    }
    ///Outputs QSSL 1.5*SCK before the first rising edge of QSPCLK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSLD_A::_1)
    }
}
impl R {
    ///Bits 0:3 - Selection of a minimum high-level width of the QSSL signal
    #[inline(always)]
    pub fn sfmsw(&self) -> SFMSW_R {
        SFMSW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - QSSL signal release timing selection
    #[inline(always)]
    pub fn sfmshd(&self) -> SFMSHD_R {
        SFMSHD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - QSSL signal output timing selection
    #[inline(always)]
    pub fn sfmsld(&self) -> SFMSLD_R {
        SFMSLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Selection of a minimum high-level width of the QSSL signal
    #[inline(always)]
    pub fn sfmsw(&mut self) -> SFMSW_W<SFMSSC_SPEC> {
        SFMSW_W::new(self, 0)
    }
    ///Bit 4 - QSSL signal release timing selection
    #[inline(always)]
    pub fn sfmshd(&mut self) -> SFMSHD_W<SFMSSC_SPEC> {
        SFMSHD_W::new(self, 4)
    }
    ///Bit 5 - QSSL signal output timing selection
    #[inline(always)]
    pub fn sfmsld(&mut self) -> SFMSLD_W<SFMSSC_SPEC> {
        SFMSLD_W::new(self, 5)
    }
}
/**Chip Selection Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmssc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmssc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSSC_SPEC;
impl crate::RegisterSpec for SFMSSC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmssc::R`](R) reader structure
impl crate::Readable for SFMSSC_SPEC {}
///`write(|w| ..)` method takes [`sfmssc::W`](W) writer structure
impl crate::Writable for SFMSSC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSSC to value 0x37
impl crate::Resettable for SFMSSC_SPEC {
    const RESET_VALUE: u32 = 0x37;
}
