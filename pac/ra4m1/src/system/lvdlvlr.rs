///Register `LVDLVLR` reader
pub type R = crate::R<LVDLVLR_SPEC>;
///Register `LVDLVLR` writer
pub type W = crate::W<LVDLVLR_SPEC>;
/**Voltage Detection 1 Level Select (Standard voltage during drop in voltage)

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD1LVL_A {
    ///0: 4.29V (Vdet1_0)
    _00000 = 0,
    ///1: 4.14V (Vdet1_1)
    _00001 = 1,
    ///2: 4.02V (Vdet1_2)
    _00010 = 2,
    ///3: 3.84V (Vdet1_3)
    _00011 = 3,
    ///4: 3.10V (Vdet1_4)
    _00100 = 4,
    ///5: 3.00V (Vdet1_5)
    _00101 = 5,
    ///6: 2.90V (Vdet1_6)
    _00110 = 6,
    ///7: 2.79V (Vdet1_7)
    _00111 = 7,
    ///8: 2.68V (Vdet1_8)
    _01000 = 8,
    ///9: 2.58V (Vdet1_9)
    _01001 = 9,
    ///10: 2.48V (Vdet1_A)
    _01010 = 10,
    ///11: 2.20V (Vdet1_B)
    _01011 = 11,
    ///12: 1.96V (Vdet1_C)
    _01100 = 12,
    ///13: 1.86V (Vdet1_D)
    _01101 = 13,
    ///14: 1.75V (Vdet1_E)
    _01110 = 14,
    ///15: 1.65V (Vdet1_F)
    _01111 = 15,
    ///16: Setting prohibited
    OTHERS = 16,
}
impl From<LVD1LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVD1LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVD1LVL_A {
    type Ux = u8;
}
impl crate::IsEnum for LVD1LVL_A {}
///Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
pub type LVD1LVL_R = crate::FieldReader<LVD1LVL_A>;
impl LVD1LVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1LVL_A {
        match self.bits {
            0 => LVD1LVL_A::_00000,
            1 => LVD1LVL_A::_00001,
            2 => LVD1LVL_A::_00010,
            3 => LVD1LVL_A::_00011,
            4 => LVD1LVL_A::_00100,
            5 => LVD1LVL_A::_00101,
            6 => LVD1LVL_A::_00110,
            7 => LVD1LVL_A::_00111,
            8 => LVD1LVL_A::_01000,
            9 => LVD1LVL_A::_01001,
            10 => LVD1LVL_A::_01010,
            11 => LVD1LVL_A::_01011,
            12 => LVD1LVL_A::_01100,
            13 => LVD1LVL_A::_01101,
            14 => LVD1LVL_A::_01110,
            15 => LVD1LVL_A::_01111,
            _ => LVD1LVL_A::OTHERS,
        }
    }
    ///4.29V (Vdet1_0)
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == LVD1LVL_A::_00000
    }
    ///4.14V (Vdet1_1)
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == LVD1LVL_A::_00001
    }
    ///4.02V (Vdet1_2)
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == LVD1LVL_A::_00010
    }
    ///3.84V (Vdet1_3)
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == LVD1LVL_A::_00011
    }
    ///3.10V (Vdet1_4)
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == LVD1LVL_A::_00100
    }
    ///3.00V (Vdet1_5)
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == LVD1LVL_A::_00101
    }
    ///2.90V (Vdet1_6)
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == LVD1LVL_A::_00110
    }
    ///2.79V (Vdet1_7)
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == LVD1LVL_A::_00111
    }
    ///2.68V (Vdet1_8)
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == LVD1LVL_A::_01000
    }
    ///2.58V (Vdet1_9)
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == LVD1LVL_A::_01001
    }
    ///2.48V (Vdet1_A)
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == LVD1LVL_A::_01010
    }
    ///2.20V (Vdet1_B)
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == LVD1LVL_A::_01011
    }
    ///1.96V (Vdet1_C)
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == LVD1LVL_A::_01100
    }
    ///1.86V (Vdet1_D)
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == LVD1LVL_A::_01101
    }
    ///1.75V (Vdet1_E)
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == LVD1LVL_A::_01110
    }
    ///1.65V (Vdet1_F)
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == LVD1LVL_A::_01111
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LVD1LVL_A::OTHERS)
    }
}
///Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
pub type LVD1LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, LVD1LVL_A, crate::Safe>;
impl<'a, REG> LVD1LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4.29V (Vdet1_0)
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00000)
    }
    ///4.14V (Vdet1_1)
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00001)
    }
    ///4.02V (Vdet1_2)
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00010)
    }
    ///3.84V (Vdet1_3)
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00011)
    }
    ///3.10V (Vdet1_4)
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00100)
    }
    ///3.00V (Vdet1_5)
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00101)
    }
    ///2.90V (Vdet1_6)
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00110)
    }
    ///2.79V (Vdet1_7)
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_00111)
    }
    ///2.68V (Vdet1_8)
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01000)
    }
    ///2.58V (Vdet1_9)
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01001)
    }
    ///2.48V (Vdet1_A)
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01010)
    }
    ///2.20V (Vdet1_B)
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01011)
    }
    ///1.96V (Vdet1_C)
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01100)
    }
    ///1.86V (Vdet1_D)
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01101)
    }
    ///1.75V (Vdet1_E)
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01110)
    }
    ///1.65V (Vdet1_F)
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_01111)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::OTHERS)
    }
}
/**Voltage Detection 2 Level Select (Standard voltage during drop in voltage)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD2LVL_A {
    ///0: 4.29V (Vdet2_0)
    _000 = 0,
    ///1: 4.14V (Vdet2_1)
    _001 = 1,
    ///2: 4.02V (Vdet2_2)
    _010 = 2,
    ///3: 3.84V (Vdet2_3)
    _011 = 3,
    ///4: Setting prohibited.
    OTHERS = 4,
}
impl From<LVD2LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVD2LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVD2LVL_A {
    type Ux = u8;
}
impl crate::IsEnum for LVD2LVL_A {}
///Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
pub type LVD2LVL_R = crate::FieldReader<LVD2LVL_A>;
impl LVD2LVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2LVL_A {
        match self.bits {
            0 => LVD2LVL_A::_000,
            1 => LVD2LVL_A::_001,
            2 => LVD2LVL_A::_010,
            3 => LVD2LVL_A::_011,
            _ => LVD2LVL_A::OTHERS,
        }
    }
    ///4.29V (Vdet2_0)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LVD2LVL_A::_000
    }
    ///4.14V (Vdet2_1)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LVD2LVL_A::_001
    }
    ///4.02V (Vdet2_2)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LVD2LVL_A::_010
    }
    ///3.84V (Vdet2_3)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == LVD2LVL_A::_011
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LVD2LVL_A::OTHERS)
    }
}
///Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
pub type LVD2LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LVD2LVL_A, crate::Safe>;
impl<'a, REG> LVD2LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4.29V (Vdet2_0)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_000)
    }
    ///4.14V (Vdet2_1)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_001)
    }
    ///4.02V (Vdet2_2)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_010)
    }
    ///3.84V (Vdet2_3)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_011)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd1lvl(&self) -> LVD1LVL_R {
        LVD1LVL_R::new(self.bits & 0x1f)
    }
    ///Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd2lvl(&self) -> LVD2LVL_R {
        LVD2LVL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    ///Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd1lvl(&mut self) -> LVD1LVL_W<LVDLVLR_SPEC> {
        LVD1LVL_W::new(self, 0)
    }
    ///Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during drop in voltage)
    #[inline(always)]
    pub fn lvd2lvl(&mut self) -> LVD2LVL_W<LVDLVLR_SPEC> {
        LVD2LVL_W::new(self, 5)
    }
}
/**Voltage Detection Level Select Register

You can [`read`](crate::Reg::read) this register and get [`lvdlvlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdlvlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVDLVLR_SPEC;
impl crate::RegisterSpec for LVDLVLR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lvdlvlr::R`](R) reader structure
impl crate::Readable for LVDLVLR_SPEC {}
///`write(|w| ..)` method takes [`lvdlvlr::W`](W) writer structure
impl crate::Writable for LVDLVLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVDLVLR to value 0x07
impl crate::Resettable for LVDLVLR_SPEC {
    const RESET_VALUE: u8 = 0x07;
}
