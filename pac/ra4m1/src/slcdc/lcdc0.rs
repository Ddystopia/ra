///Register `LCDC0` reader
pub type R = crate::R<LCDC0_SPEC>;
///Register `LCDC0` writer
pub type W = crate::W<LCDC0_SPEC>;
/**LCD clock (LCDCL)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDC_A {
    ///1: (Sub clock)/22 or (LOCO clock)/22
    _000001 = 1,
    ///2: (Sub clock)/23 or (LOCO clock)/23
    _000010 = 2,
    ///3: (Sub clock)/24 or (LOCO clock)/24
    _000011 = 3,
    ///4: (Sub clock)/25 or (LOCO clock)/25
    _000100 = 4,
    ///5: (Sub clock)/26 or (LOCO clock)/26
    _000101 = 5,
    ///6: (Sub clock)/27 or (LOCO clock)/27
    _000110 = 6,
    ///7: (Sub clock)/28 or (LOCO clock)/28
    _000111 = 7,
    ///8: (Sub clock)/29 or (LOCO clock)/29
    _001000 = 8,
    ///9: (Sub clock)/210 or (LOCO clock)/210
    _001001 = 9,
    ///17: (Main clock)/28 or (HOCO clock)/28
    _010001 = 17,
    ///18: (Main clock)/29 or (HOCO clock)/29
    _010010 = 18,
    ///19: (Main clock)/210 or (HOCO clock)/210
    _010011 = 19,
    ///20: (Main clock)/211 or (HOCO clock)/211
    _010100 = 20,
    ///21: (Main clock)/212 or (HOCO clock)/212
    _010101 = 21,
    ///22: (Main clock)/213 or (HOCO clock)/213
    _010110 = 22,
    ///23: (Main clock)/214 or (HOCO clock)/214
    _010111 = 23,
    ///24: (Main clock)/215 or (HOCO clock)/215
    _011000 = 24,
    ///25: (Main clock)/216 or (HOCO clock)/216
    _011001 = 25,
    ///26: (Main clock)/217 or (HOCO clock)/217
    _011010 = 26,
    ///27: (Main clock)/218 or (HOCO clock)/218
    _011011 = 27,
    ///43: (Main clock)/219 or (HOCO clock)/219
    _101011 = 43,
    ///0: Other than above Setting prohibited
    OTHERS = 0,
}
impl From<LCDC_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LCDC_A {
    type Ux = u8;
}
impl crate::IsEnum for LCDC_A {}
///Field `LCDC` reader - LCD clock (LCDCL)
pub type LCDC_R = crate::FieldReader<LCDC_A>;
impl LCDC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCDC_A {
        match self.bits {
            1 => LCDC_A::_000001,
            2 => LCDC_A::_000010,
            3 => LCDC_A::_000011,
            4 => LCDC_A::_000100,
            5 => LCDC_A::_000101,
            6 => LCDC_A::_000110,
            7 => LCDC_A::_000111,
            8 => LCDC_A::_001000,
            9 => LCDC_A::_001001,
            17 => LCDC_A::_010001,
            18 => LCDC_A::_010010,
            19 => LCDC_A::_010011,
            20 => LCDC_A::_010100,
            21 => LCDC_A::_010101,
            22 => LCDC_A::_010110,
            23 => LCDC_A::_010111,
            24 => LCDC_A::_011000,
            25 => LCDC_A::_011001,
            26 => LCDC_A::_011010,
            27 => LCDC_A::_011011,
            43 => LCDC_A::_101011,
            _ => LCDC_A::OTHERS,
        }
    }
    ///(Sub clock)/22 or (LOCO clock)/22
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        *self == LCDC_A::_000001
    }
    ///(Sub clock)/23 or (LOCO clock)/23
    #[inline(always)]
    pub fn is_000010(&self) -> bool {
        *self == LCDC_A::_000010
    }
    ///(Sub clock)/24 or (LOCO clock)/24
    #[inline(always)]
    pub fn is_000011(&self) -> bool {
        *self == LCDC_A::_000011
    }
    ///(Sub clock)/25 or (LOCO clock)/25
    #[inline(always)]
    pub fn is_000100(&self) -> bool {
        *self == LCDC_A::_000100
    }
    ///(Sub clock)/26 or (LOCO clock)/26
    #[inline(always)]
    pub fn is_000101(&self) -> bool {
        *self == LCDC_A::_000101
    }
    ///(Sub clock)/27 or (LOCO clock)/27
    #[inline(always)]
    pub fn is_000110(&self) -> bool {
        *self == LCDC_A::_000110
    }
    ///(Sub clock)/28 or (LOCO clock)/28
    #[inline(always)]
    pub fn is_000111(&self) -> bool {
        *self == LCDC_A::_000111
    }
    ///(Sub clock)/29 or (LOCO clock)/29
    #[inline(always)]
    pub fn is_001000(&self) -> bool {
        *self == LCDC_A::_001000
    }
    ///(Sub clock)/210 or (LOCO clock)/210
    #[inline(always)]
    pub fn is_001001(&self) -> bool {
        *self == LCDC_A::_001001
    }
    ///(Main clock)/28 or (HOCO clock)/28
    #[inline(always)]
    pub fn is_010001(&self) -> bool {
        *self == LCDC_A::_010001
    }
    ///(Main clock)/29 or (HOCO clock)/29
    #[inline(always)]
    pub fn is_010010(&self) -> bool {
        *self == LCDC_A::_010010
    }
    ///(Main clock)/210 or (HOCO clock)/210
    #[inline(always)]
    pub fn is_010011(&self) -> bool {
        *self == LCDC_A::_010011
    }
    ///(Main clock)/211 or (HOCO clock)/211
    #[inline(always)]
    pub fn is_010100(&self) -> bool {
        *self == LCDC_A::_010100
    }
    ///(Main clock)/212 or (HOCO clock)/212
    #[inline(always)]
    pub fn is_010101(&self) -> bool {
        *self == LCDC_A::_010101
    }
    ///(Main clock)/213 or (HOCO clock)/213
    #[inline(always)]
    pub fn is_010110(&self) -> bool {
        *self == LCDC_A::_010110
    }
    ///(Main clock)/214 or (HOCO clock)/214
    #[inline(always)]
    pub fn is_010111(&self) -> bool {
        *self == LCDC_A::_010111
    }
    ///(Main clock)/215 or (HOCO clock)/215
    #[inline(always)]
    pub fn is_011000(&self) -> bool {
        *self == LCDC_A::_011000
    }
    ///(Main clock)/216 or (HOCO clock)/216
    #[inline(always)]
    pub fn is_011001(&self) -> bool {
        *self == LCDC_A::_011001
    }
    ///(Main clock)/217 or (HOCO clock)/217
    #[inline(always)]
    pub fn is_011010(&self) -> bool {
        *self == LCDC_A::_011010
    }
    ///(Main clock)/218 or (HOCO clock)/218
    #[inline(always)]
    pub fn is_011011(&self) -> bool {
        *self == LCDC_A::_011011
    }
    ///(Main clock)/219 or (HOCO clock)/219
    #[inline(always)]
    pub fn is_101011(&self) -> bool {
        *self == LCDC_A::_101011
    }
    ///Other than above Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LCDC_A::OTHERS)
    }
}
///Field `LCDC` writer - LCD clock (LCDCL)
pub type LCDC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, LCDC_A, crate::Safe>;
impl<'a, REG> LCDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///(Sub clock)/22 or (LOCO clock)/22
    #[inline(always)]
    pub fn _000001(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000001)
    }
    ///(Sub clock)/23 or (LOCO clock)/23
    #[inline(always)]
    pub fn _000010(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000010)
    }
    ///(Sub clock)/24 or (LOCO clock)/24
    #[inline(always)]
    pub fn _000011(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000011)
    }
    ///(Sub clock)/25 or (LOCO clock)/25
    #[inline(always)]
    pub fn _000100(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000100)
    }
    ///(Sub clock)/26 or (LOCO clock)/26
    #[inline(always)]
    pub fn _000101(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000101)
    }
    ///(Sub clock)/27 or (LOCO clock)/27
    #[inline(always)]
    pub fn _000110(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000110)
    }
    ///(Sub clock)/28 or (LOCO clock)/28
    #[inline(always)]
    pub fn _000111(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_000111)
    }
    ///(Sub clock)/29 or (LOCO clock)/29
    #[inline(always)]
    pub fn _001000(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_001000)
    }
    ///(Sub clock)/210 or (LOCO clock)/210
    #[inline(always)]
    pub fn _001001(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_001001)
    }
    ///(Main clock)/28 or (HOCO clock)/28
    #[inline(always)]
    pub fn _010001(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010001)
    }
    ///(Main clock)/29 or (HOCO clock)/29
    #[inline(always)]
    pub fn _010010(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010010)
    }
    ///(Main clock)/210 or (HOCO clock)/210
    #[inline(always)]
    pub fn _010011(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010011)
    }
    ///(Main clock)/211 or (HOCO clock)/211
    #[inline(always)]
    pub fn _010100(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010100)
    }
    ///(Main clock)/212 or (HOCO clock)/212
    #[inline(always)]
    pub fn _010101(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010101)
    }
    ///(Main clock)/213 or (HOCO clock)/213
    #[inline(always)]
    pub fn _010110(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010110)
    }
    ///(Main clock)/214 or (HOCO clock)/214
    #[inline(always)]
    pub fn _010111(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_010111)
    }
    ///(Main clock)/215 or (HOCO clock)/215
    #[inline(always)]
    pub fn _011000(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_011000)
    }
    ///(Main clock)/216 or (HOCO clock)/216
    #[inline(always)]
    pub fn _011001(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_011001)
    }
    ///(Main clock)/217 or (HOCO clock)/217
    #[inline(always)]
    pub fn _011010(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_011010)
    }
    ///(Main clock)/218 or (HOCO clock)/218
    #[inline(always)]
    pub fn _011011(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_011011)
    }
    ///(Main clock)/219 or (HOCO clock)/219
    #[inline(always)]
    pub fn _101011(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::_101011)
    }
    ///Other than above Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LCDC_A::OTHERS)
    }
}
impl R {
    ///Bits 0:5 - LCD clock (LCDCL)
    #[inline(always)]
    pub fn lcdc(&self) -> LCDC_R {
        LCDC_R::new(self.bits & 0x3f)
    }
}
impl W {
    ///Bits 0:5 - LCD clock (LCDCL)
    #[inline(always)]
    pub fn lcdc(&mut self) -> LCDC_W<LCDC0_SPEC> {
        LCDC_W::new(self, 0)
    }
}
/**LCD Clock Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lcdc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCDC0_SPEC;
impl crate::RegisterSpec for LCDC0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lcdc0::R`](R) reader structure
impl crate::Readable for LCDC0_SPEC {}
///`write(|w| ..)` method takes [`lcdc0::W`](W) writer structure
impl crate::Writable for LCDC0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCDC0 to value 0
impl crate::Resettable for LCDC0_SPEC {}
