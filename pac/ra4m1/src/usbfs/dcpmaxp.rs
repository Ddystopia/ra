///Register `DCPMAXP` reader
pub type R = crate::R<DCPMAXP_SPEC>;
///Register `DCPMAXP` writer
pub type W = crate::W<DCPMAXP_SPEC>;
/**Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.

Value on reset: 64*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MXPS_A {
    ///8: 8 bytes
    _0X08 = 8,
    ///16: 16 bytes
    _0X10 = 16,
    ///24: 24 bytes
    _0X18 = 24,
    ///32: 32 bytes
    _0X20 = 32,
    ///40: 40 bytes
    _0X28 = 40,
    ///48: 48 bytes
    _0X30 = 48,
    ///56: 56 bytes
    _0X38 = 56,
    ///64: 64 bytes
    _0X40 = 64,
    ///72: 72 bytes
    _0X48 = 72,
    ///80: 80 bytes
    _0X50 = 80,
    ///88: 88 bytes
    _0X58 = 88,
    ///96: 96 bytes
    _0X60 = 96,
    ///104: 104 bytes
    _0X68 = 104,
    ///112: 112 bytes
    _0X70 = 112,
    ///120: 120 bytes
    _0X78 = 120,
    ///0: Setting prohibited
    OTHERS = 0,
}
impl From<MXPS_A> for u8 {
    #[inline(always)]
    fn from(variant: MXPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MXPS_A {
    type Ux = u8;
}
impl crate::IsEnum for MXPS_A {}
///Field `MXPS` reader - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.
pub type MXPS_R = crate::FieldReader<MXPS_A>;
impl MXPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MXPS_A {
        match self.bits {
            8 => MXPS_A::_0X08,
            16 => MXPS_A::_0X10,
            24 => MXPS_A::_0X18,
            32 => MXPS_A::_0X20,
            40 => MXPS_A::_0X28,
            48 => MXPS_A::_0X30,
            56 => MXPS_A::_0X38,
            64 => MXPS_A::_0X40,
            72 => MXPS_A::_0X48,
            80 => MXPS_A::_0X50,
            88 => MXPS_A::_0X58,
            96 => MXPS_A::_0X60,
            104 => MXPS_A::_0X68,
            112 => MXPS_A::_0X70,
            120 => MXPS_A::_0X78,
            _ => MXPS_A::OTHERS,
        }
    }
    ///8 bytes
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == MXPS_A::_0X08
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == MXPS_A::_0X10
    }
    ///24 bytes
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == MXPS_A::_0X18
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == MXPS_A::_0X20
    }
    ///40 bytes
    #[inline(always)]
    pub fn is_0x28(&self) -> bool {
        *self == MXPS_A::_0X28
    }
    ///48 bytes
    #[inline(always)]
    pub fn is_0x30(&self) -> bool {
        *self == MXPS_A::_0X30
    }
    ///56 bytes
    #[inline(always)]
    pub fn is_0x38(&self) -> bool {
        *self == MXPS_A::_0X38
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_0x40(&self) -> bool {
        *self == MXPS_A::_0X40
    }
    ///72 bytes
    #[inline(always)]
    pub fn is_0x48(&self) -> bool {
        *self == MXPS_A::_0X48
    }
    ///80 bytes
    #[inline(always)]
    pub fn is_0x50(&self) -> bool {
        *self == MXPS_A::_0X50
    }
    ///88 bytes
    #[inline(always)]
    pub fn is_0x58(&self) -> bool {
        *self == MXPS_A::_0X58
    }
    ///96 bytes
    #[inline(always)]
    pub fn is_0x60(&self) -> bool {
        *self == MXPS_A::_0X60
    }
    ///104 bytes
    #[inline(always)]
    pub fn is_0x68(&self) -> bool {
        *self == MXPS_A::_0X68
    }
    ///112 bytes
    #[inline(always)]
    pub fn is_0x70(&self) -> bool {
        *self == MXPS_A::_0X70
    }
    ///120 bytes
    #[inline(always)]
    pub fn is_0x78(&self) -> bool {
        *self == MXPS_A::_0X78
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), MXPS_A::OTHERS)
    }
}
///Field `MXPS` writer - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.
pub type MXPS_W<'a, REG> = crate::FieldWriter<'a, REG, 7, MXPS_A, crate::Safe>;
impl<'a, REG> MXPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bytes
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X08)
    }
    ///16 bytes
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X10)
    }
    ///24 bytes
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X18)
    }
    ///32 bytes
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X20)
    }
    ///40 bytes
    #[inline(always)]
    pub fn _0x28(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X28)
    }
    ///48 bytes
    #[inline(always)]
    pub fn _0x30(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X30)
    }
    ///56 bytes
    #[inline(always)]
    pub fn _0x38(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X38)
    }
    ///64 bytes
    #[inline(always)]
    pub fn _0x40(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X40)
    }
    ///72 bytes
    #[inline(always)]
    pub fn _0x48(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X48)
    }
    ///80 bytes
    #[inline(always)]
    pub fn _0x50(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X50)
    }
    ///88 bytes
    #[inline(always)]
    pub fn _0x58(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X58)
    }
    ///96 bytes
    #[inline(always)]
    pub fn _0x60(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X60)
    }
    ///104 bytes
    #[inline(always)]
    pub fn _0x68(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X68)
    }
    ///112 bytes
    #[inline(always)]
    pub fn _0x70(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X70)
    }
    ///120 bytes
    #[inline(always)]
    pub fn _0x78(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::_0X78)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::OTHERS)
    }
}
/**Device Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSEL_A {
    ///0: Address 0000
    _0000 = 0,
    ///1: Address 0001
    _0001 = 1,
    ///2: Address 0010
    _0010 = 2,
    ///3: Address 0011
    _0011 = 3,
    ///4: Address 0100
    _0100 = 4,
    ///5: Address 0101
    _0101 = 5,
    ///6: Settings prohibited.
    OTHERS = 6,
}
impl From<DEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEVSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for DEVSEL_A {}
///Field `DEVSEL` reader - Device Select
pub type DEVSEL_R = crate::FieldReader<DEVSEL_A>;
impl DEVSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEVSEL_A {
        match self.bits {
            0 => DEVSEL_A::_0000,
            1 => DEVSEL_A::_0001,
            2 => DEVSEL_A::_0010,
            3 => DEVSEL_A::_0011,
            4 => DEVSEL_A::_0100,
            5 => DEVSEL_A::_0101,
            _ => DEVSEL_A::OTHERS,
        }
    }
    ///Address 0000
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DEVSEL_A::_0000
    }
    ///Address 0001
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DEVSEL_A::_0001
    }
    ///Address 0010
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DEVSEL_A::_0010
    }
    ///Address 0011
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DEVSEL_A::_0011
    }
    ///Address 0100
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DEVSEL_A::_0100
    }
    ///Address 0101
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DEVSEL_A::_0101
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DEVSEL_A::OTHERS)
    }
}
///Field `DEVSEL` writer - Device Select
pub type DEVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DEVSEL_A, crate::Safe>;
impl<'a, REG> DEVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Address 0000
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0000)
    }
    ///Address 0001
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0001)
    }
    ///Address 0010
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0010)
    }
    ///Address 0011
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0011)
    }
    ///Address 0100
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0100)
    }
    ///Address 0101
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0101)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:6 - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:6 - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.
    #[inline(always)]
    pub fn mxps(&mut self) -> MXPS_W<DCPMAXP_SPEC> {
        MXPS_W::new(self, 0)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&mut self) -> DEVSEL_W<DCPMAXP_SPEC> {
        DEVSEL_W::new(self, 12)
    }
}
/**DCP Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`dcpmaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpmaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCPMAXP_SPEC;
impl crate::RegisterSpec for DCPMAXP_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dcpmaxp::R`](R) reader structure
impl crate::Readable for DCPMAXP_SPEC {}
///`write(|w| ..)` method takes [`dcpmaxp::W`](W) writer structure
impl crate::Writable for DCPMAXP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCPMAXP to value 0x40
impl crate::Resettable for DCPMAXP_SPEC {
    const RESET_VALUE: u16 = 0x40;
}
