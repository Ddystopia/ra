///Register `LVDLVLR` reader
pub type R = crate::R<LVDLVLR_SPEC>;
///Register `LVDLVLR` writer
pub type W = crate::W<LVDLVLR_SPEC>;
/**Voltage Detection 1 Level Select (Standard voltage during fall in voltage)

Value on reset: 19*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD1LVL_A {
    ///17: 2.99V (Vdet1_1)
    _10001 = 17,
    ///18: 2.92V (Vdet1_2)
    _10010 = 18,
    ///19: 2.85V (Vdet1_3)
    _10011 = 19,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
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
///Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)
pub type LVD1LVL_R = crate::FieldReader<LVD1LVL_A>;
impl LVD1LVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1LVL_A {
        match self.bits {
            17 => LVD1LVL_A::_10001,
            18 => LVD1LVL_A::_10010,
            19 => LVD1LVL_A::_10011,
            _ => LVD1LVL_A::OTHERS,
        }
    }
    ///2.99V (Vdet1_1)
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == LVD1LVL_A::_10001
    }
    ///2.92V (Vdet1_2)
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == LVD1LVL_A::_10010
    }
    ///2.85V (Vdet1_3)
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == LVD1LVL_A::_10011
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LVD1LVL_A::OTHERS)
    }
}
///Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)
pub type LVD1LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, LVD1LVL_A, crate::Safe>;
impl<'a, REG> LVD1LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2.99V (Vdet1_1)
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_10001)
    }
    ///2.92V (Vdet1_2)
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_10010)
    }
    ///2.85V (Vdet1_3)
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::_10011)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1LVL_A::OTHERS)
    }
}
/**Voltage Detection 2 Level Select (Standard voltage during fall in voltage)

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVD2LVL_A {
    ///5: 2.99V (Vdet2_1)
    _101 = 5,
    ///6: 2.92V (Vdet2_2)
    _110 = 6,
    ///7: 2.85V (Vdet2_3)
    _111 = 7,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
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
///Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)
pub type LVD2LVL_R = crate::FieldReader<LVD2LVL_A>;
impl LVD2LVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2LVL_A {
        match self.bits {
            5 => LVD2LVL_A::_101,
            6 => LVD2LVL_A::_110,
            7 => LVD2LVL_A::_111,
            _ => LVD2LVL_A::OTHERS,
        }
    }
    ///2.99V (Vdet2_1)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == LVD2LVL_A::_101
    }
    ///2.92V (Vdet2_2)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == LVD2LVL_A::_110
    }
    ///2.85V (Vdet2_3)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == LVD2LVL_A::_111
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LVD2LVL_A::OTHERS)
    }
}
///Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)
pub type LVD2LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LVD2LVL_A, crate::Safe>;
impl<'a, REG> LVD2LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2.99V (Vdet2_1)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_101)
    }
    ///2.92V (Vdet2_2)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_110)
    }
    ///2.85V (Vdet2_3)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::_111)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2LVL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)
    #[inline(always)]
    pub fn lvd1lvl(&self) -> LVD1LVL_R {
        LVD1LVL_R::new(self.bits & 0x1f)
    }
    ///Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)
    #[inline(always)]
    pub fn lvd2lvl(&self) -> LVD2LVL_R {
        LVD2LVL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    ///Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)
    #[inline(always)]
    pub fn lvd1lvl(&mut self) -> LVD1LVL_W<LVDLVLR_SPEC> {
        LVD1LVL_W::new(self, 0)
    }
    ///Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)
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
///`reset()` method sets LVDLVLR to value 0xf3
impl crate::Resettable for LVDLVLR_SPEC {
    const RESET_VALUE: u8 = 0xf3;
}
