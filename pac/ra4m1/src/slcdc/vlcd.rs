///Register `VLCD` reader
pub type R = crate::R<VLCD_SPEC>;
///Register `VLCD` writer
pub type W = crate::W<VLCD_SPEC>;
/**Reference Voltage(Contrast Adjustment) Select

Value on reset: 4*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VLCD_A {
    ///4: Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)
    _00100 = 4,
    ///5: Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)
    _00101 = 5,
    ///6: Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)
    _00110 = 6,
    ///7: Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)
    _00111 = 7,
    ///8: Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)
    _01000 = 8,
    ///9: Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)
    _01001 = 9,
    ///10: Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)
    _01010 = 10,
    ///11: Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _01011 = 11,
    ///12: Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _01100 = 12,
    ///13: Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _01101 = 13,
    ///14: Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _01110 = 14,
    ///15: Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _01111 = 15,
    ///16: Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _10000 = 16,
    ///17: Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _10001 = 17,
    ///18: Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _10010 = 18,
    ///19: Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    _10011 = 19,
    ///0: Setting prohibited
    OTHERS = 0,
}
impl From<VLCD_A> for u8 {
    #[inline(always)]
    fn from(variant: VLCD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VLCD_A {
    type Ux = u8;
}
impl crate::IsEnum for VLCD_A {}
///Field `VLCD` reader - Reference Voltage(Contrast Adjustment) Select
pub type VLCD_R = crate::FieldReader<VLCD_A>;
impl VLCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VLCD_A {
        match self.bits {
            4 => VLCD_A::_00100,
            5 => VLCD_A::_00101,
            6 => VLCD_A::_00110,
            7 => VLCD_A::_00111,
            8 => VLCD_A::_01000,
            9 => VLCD_A::_01001,
            10 => VLCD_A::_01010,
            11 => VLCD_A::_01011,
            12 => VLCD_A::_01100,
            13 => VLCD_A::_01101,
            14 => VLCD_A::_01110,
            15 => VLCD_A::_01111,
            16 => VLCD_A::_10000,
            17 => VLCD_A::_10001,
            18 => VLCD_A::_10010,
            19 => VLCD_A::_10011,
            _ => VLCD_A::OTHERS,
        }
    }
    ///Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == VLCD_A::_00100
    }
    ///Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == VLCD_A::_00101
    }
    ///Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == VLCD_A::_00110
    }
    ///Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == VLCD_A::_00111
    }
    ///Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == VLCD_A::_01000
    }
    ///Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == VLCD_A::_01001
    }
    ///Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == VLCD_A::_01010
    }
    ///Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == VLCD_A::_01011
    }
    ///Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == VLCD_A::_01100
    }
    ///Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == VLCD_A::_01101
    }
    ///Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == VLCD_A::_01110
    }
    ///Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == VLCD_A::_01111
    }
    ///Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == VLCD_A::_10000
    }
    ///Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == VLCD_A::_10001
    }
    ///Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == VLCD_A::_10010
    }
    ///Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == VLCD_A::_10011
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), VLCD_A::OTHERS)
    }
}
///Field `VLCD` writer - Reference Voltage(Contrast Adjustment) Select
pub type VLCD_W<'a, REG> = crate::FieldWriter<'a, REG, 5, VLCD_A, crate::Safe>;
impl<'a, REG> VLCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reference voltageselection(contrast adjustment): 1.00 V (default) VL4 voltage: 3.00 V(1/3 bias method)/4.00 V(1/4 bias method)
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00100)
    }
    ///Reference voltageselection(contrast adjustment): 1.05 V VL4 voltage: 3.15 V(1/3 bias method)/4.20 V(1/4 bias method)
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00101)
    }
    ///Reference voltageselection(contrast adjustment): 1.10 V VL4 voltage: 3.30 V(1/3 bias method)/4.40 V(1/4 bias method)
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00110)
    }
    ///Reference voltageselection(contrast adjustment): 1.15 V VL4 voltage: 3.45 V(1/3 bias method)/4.60 V(1/4 bias method)
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_00111)
    }
    ///Reference voltageselection(contrast adjustment): 1.20 V VL4 voltage: 3.60 V(1/3 bias method)/4.80 V(1/4 bias method)
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01000)
    }
    ///Reference voltageselection(contrast adjustment): 1.25 V VL4 voltage: 3.75 V(1/3 bias method)/5.00 V(1/4 bias method)
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01001)
    }
    ///Reference voltageselection(contrast adjustment): 1.30 V VL4 voltage: 3.90 V(1/3 bias method)/5.20 V(1/4 bias method)
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01010)
    }
    ///Reference voltageselection(contrast adjustment): 1.35 V VL4 voltage: 4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01011)
    }
    ///Reference voltageselection(contrast adjustment): 1.40 V VL4 voltage: 4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01100)
    }
    ///Reference voltageselection(contrast adjustment): 1.45 V VL4 voltage: 4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01101)
    }
    ///Reference voltageselection(contrast adjustment): 1.50 V VL4 voltage: 4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01110)
    }
    ///Reference voltageselection(contrast adjustment): 1.55 V VL4 voltage: 4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_01111)
    }
    ///Reference voltageselection(contrast adjustment): 1.60 V VL4 voltage: 4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10000)
    }
    ///Reference voltageselection(contrast adjustment): 1.65 V VL4 voltage: 4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10001)
    }
    ///Reference voltageselection(contrast adjustment): 1.70 V VL4 voltage: 5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10010)
    }
    ///Reference voltageselection(contrast adjustment): 1.75 V VL4 voltage: 5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::_10011)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(VLCD_A::OTHERS)
    }
}
impl R {
    ///Bits 0:4 - Reference Voltage(Contrast Adjustment) Select
    #[inline(always)]
    pub fn vlcd(&self) -> VLCD_R {
        VLCD_R::new(self.bits & 0x1f)
    }
}
impl W {
    ///Bits 0:4 - Reference Voltage(Contrast Adjustment) Select
    #[inline(always)]
    pub fn vlcd(&mut self) -> VLCD_W<VLCD_SPEC> {
        VLCD_W::new(self, 0)
    }
}
/**LCD Boost Level Control Register

You can [`read`](crate::Reg::read) this register and get [`vlcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VLCD_SPEC;
impl crate::RegisterSpec for VLCD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vlcd::R`](R) reader structure
impl crate::Readable for VLCD_SPEC {}
///`write(|w| ..)` method takes [`vlcd::W`](W) writer structure
impl crate::Writable for VLCD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VLCD to value 0x04
impl crate::Resettable for VLCD_SPEC {
    const RESET_VALUE: u8 = 0x04;
}
