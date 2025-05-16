///Register `TESTMODE` reader
pub type R = crate::R<TESTMODE_SPEC>;
///Register `TESTMODE` writer
pub type W = crate::W<TESTMODE_SPEC>;
/**Test Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTST_A {
    ///0: Normal operation
    _0000 = 0,
    ///1: Test_J TestMode(When the Function Controller Function is Selected)
    _0001 = 1,
    ///2: Test_K TestMode(When the Function Controller Function is Selected)
    _0010 = 2,
    ///3: Test_SE0_NAK TestMode(When the Function Controller Function is Selected)
    _0011 = 3,
    ///4: Test_Packet TestMode(When the Function Controller Function is Selected)
    _0100 = 4,
    ///5: Reserved TestMode(When the Function Controller Function is Selected)
    _0101 = 5,
    ///6: Reserved TestMode(When the Function Controller Function is Selected)
    _0110 = 6,
    ///7: Reserved TestMode(When the Function Controller Function is Selected)
    _0111 = 7,
    ///9: Test_J TestMode(When the Host Controller Function is Selected)
    _1001 = 9,
    ///10: Test_K TestMode(When the Host Controller Function is Selected)
    _1010 = 10,
    ///11: Test_SE0_NAK TestMode(When the Host Controller Function is Selected)
    _1011 = 11,
    ///12: Test_Packet TestMode(When the Host Controller Function is Selected)
    _1100 = 12,
    ///13: Test_Force_EnableTestMode(When the Host Controller Function is Selected)
    _1101 = 13,
    ///14: Reserved TestMode(When the Host Controller Function is Selected)
    _1110 = 14,
    ///15: Reserved TestMode(When the Host Controller Function is Selected)
    _1111 = 15,
}
impl From<UTST_A> for u8 {
    #[inline(always)]
    fn from(variant: UTST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UTST_A {
    type Ux = u8;
}
impl crate::IsEnum for UTST_A {}
///Field `UTST` reader - Test Mode
pub type UTST_R = crate::FieldReader<UTST_A>;
impl UTST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UTST_A> {
        match self.bits {
            0 => Some(UTST_A::_0000),
            1 => Some(UTST_A::_0001),
            2 => Some(UTST_A::_0010),
            3 => Some(UTST_A::_0011),
            4 => Some(UTST_A::_0100),
            5 => Some(UTST_A::_0101),
            6 => Some(UTST_A::_0110),
            7 => Some(UTST_A::_0111),
            9 => Some(UTST_A::_1001),
            10 => Some(UTST_A::_1010),
            11 => Some(UTST_A::_1011),
            12 => Some(UTST_A::_1100),
            13 => Some(UTST_A::_1101),
            14 => Some(UTST_A::_1110),
            15 => Some(UTST_A::_1111),
            _ => None,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == UTST_A::_0000
    }
    ///Test_J TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == UTST_A::_0001
    }
    ///Test_K TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == UTST_A::_0010
    }
    ///Test_SE0_NAK TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == UTST_A::_0011
    }
    ///Test_Packet TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == UTST_A::_0100
    }
    ///Reserved TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == UTST_A::_0101
    }
    ///Reserved TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == UTST_A::_0110
    }
    ///Reserved TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == UTST_A::_0111
    }
    ///Test_J TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == UTST_A::_1001
    }
    ///Test_K TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == UTST_A::_1010
    }
    ///Test_SE0_NAK TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == UTST_A::_1011
    }
    ///Test_Packet TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == UTST_A::_1100
    }
    ///Test_Force_EnableTestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == UTST_A::_1101
    }
    ///Reserved TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == UTST_A::_1110
    }
    ///Reserved TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == UTST_A::_1111
    }
}
///Field `UTST` writer - Test Mode
pub type UTST_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UTST_A>;
impl<'a, REG> UTST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal operation
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0000)
    }
    ///Test_J TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0001)
    }
    ///Test_K TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0010)
    }
    ///Test_SE0_NAK TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0011)
    }
    ///Test_Packet TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0100)
    }
    ///Reserved TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0101)
    }
    ///Reserved TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0110)
    }
    ///Reserved TestMode(When the Function Controller Function is Selected)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_0111)
    }
    ///Test_J TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1001)
    }
    ///Test_K TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1010)
    }
    ///Test_SE0_NAK TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1011)
    }
    ///Test_Packet TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1100)
    }
    ///Test_Force_EnableTestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1101)
    }
    ///Reserved TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1110)
    }
    ///Reserved TestMode(When the Host Controller Function is Selected)
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(UTST_A::_1111)
    }
}
impl R {
    ///Bits 0:3 - Test Mode
    #[inline(always)]
    pub fn utst(&self) -> UTST_R {
        UTST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Test Mode
    #[inline(always)]
    pub fn utst(&mut self) -> UTST_W<TESTMODE_SPEC> {
        UTST_W::new(self, 0)
    }
}
/**USB Test Mode Register

You can [`read`](crate::Reg::read) this register and get [`testmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TESTMODE_SPEC;
impl crate::RegisterSpec for TESTMODE_SPEC {
    type Ux = u16;
}
///`read()` method returns [`testmode::R`](R) reader structure
impl crate::Readable for TESTMODE_SPEC {}
///`write(|w| ..)` method takes [`testmode::W`](W) writer structure
impl crate::Writable for TESTMODE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TESTMODE to value 0
impl crate::Resettable for TESTMODE_SPEC {}
