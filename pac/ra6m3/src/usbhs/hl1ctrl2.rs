///Register `HL1CTRL2` reader
pub type R = crate::R<HL1CTRL2_SPEC>;
///Register `HL1CTRL2` writer
pub type W = crate::W<HL1CTRL2_SPEC>;
///Field `L1ADDR` reader - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token.
pub type L1ADDR_R = crate::FieldReader;
///Field `L1ADDR` writer - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token.
pub type L1ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**LPM Token HIRD

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIRD_A {
    ///0: 50 us(Setting prohibited(BESL = 0)) / 75 us(BESL = 1)
    _0000 = 0,
    ///1: 125 us(BESL = 0) / 100 us(BESL = 1)
    _0001 = 1,
    ///2: 200 us(BESL = 0) / 150 us(BESL = 1)
    _0010 = 2,
    ///3: 275 us(BESL = 0) / 250 us(BESL = 1)
    _0011 = 3,
    ///4: 350 us(BESL = 0) / 350 us(BESL = 1)
    _0100 = 4,
    ///5: 425 us(BESL = 0) / 450 us(BESL = 1)
    _0101 = 5,
    ///6: 500 us(BESL = 0) / 950 us(BESL = 1)
    _0110 = 6,
    ///7: 575 us(BESL = 0) / 1950 us(BESL = 1)
    _0111 = 7,
    ///8: 650 us(BESL = 0) / 2950 us(BESL = 1)
    _1000 = 8,
    ///9: 725 us(BESL = 0) / 3950 us(BESL = 1)
    _1001 = 9,
    ///10: 800 us(BESL = 0) / 4950 us(BESL = 1)
    _1010 = 10,
    ///11: 875 us(BESL = 0) / 5950 us(BESL = 1)
    _1011 = 11,
    ///12: 950 us(BESL = 0) / 6950 us(BESL = 1)
    _1100 = 12,
    ///13: 1025 us(Setting prohibited(BESL = 0)) / 7950 us(BESL = 1)
    _1101 = 13,
    ///14: 1100 us(Setting prohibited(BESL = 0)) / 8950 us(BESL = 1)
    _1110 = 14,
    ///15: 1175 us(Setting prohibited(BESL = 0)) / 9950 us(BESL = 1)
    _1111 = 15,
}
impl From<HIRD_A> for u8 {
    #[inline(always)]
    fn from(variant: HIRD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIRD_A {
    type Ux = u8;
}
impl crate::IsEnum for HIRD_A {}
///Field `HIRD` reader - LPM Token HIRD
pub type HIRD_R = crate::FieldReader<HIRD_A>;
impl HIRD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HIRD_A {
        match self.bits {
            0 => HIRD_A::_0000,
            1 => HIRD_A::_0001,
            2 => HIRD_A::_0010,
            3 => HIRD_A::_0011,
            4 => HIRD_A::_0100,
            5 => HIRD_A::_0101,
            6 => HIRD_A::_0110,
            7 => HIRD_A::_0111,
            8 => HIRD_A::_1000,
            9 => HIRD_A::_1001,
            10 => HIRD_A::_1010,
            11 => HIRD_A::_1011,
            12 => HIRD_A::_1100,
            13 => HIRD_A::_1101,
            14 => HIRD_A::_1110,
            15 => HIRD_A::_1111,
            _ => unreachable!(),
        }
    }
    ///50 us(Setting prohibited(BESL = 0)) / 75 us(BESL = 1)
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == HIRD_A::_0000
    }
    ///125 us(BESL = 0) / 100 us(BESL = 1)
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == HIRD_A::_0001
    }
    ///200 us(BESL = 0) / 150 us(BESL = 1)
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == HIRD_A::_0010
    }
    ///275 us(BESL = 0) / 250 us(BESL = 1)
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == HIRD_A::_0011
    }
    ///350 us(BESL = 0) / 350 us(BESL = 1)
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == HIRD_A::_0100
    }
    ///425 us(BESL = 0) / 450 us(BESL = 1)
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == HIRD_A::_0101
    }
    ///500 us(BESL = 0) / 950 us(BESL = 1)
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == HIRD_A::_0110
    }
    ///575 us(BESL = 0) / 1950 us(BESL = 1)
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == HIRD_A::_0111
    }
    ///650 us(BESL = 0) / 2950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == HIRD_A::_1000
    }
    ///725 us(BESL = 0) / 3950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == HIRD_A::_1001
    }
    ///800 us(BESL = 0) / 4950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == HIRD_A::_1010
    }
    ///875 us(BESL = 0) / 5950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == HIRD_A::_1011
    }
    ///950 us(BESL = 0) / 6950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == HIRD_A::_1100
    }
    ///1025 us(Setting prohibited(BESL = 0)) / 7950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == HIRD_A::_1101
    }
    ///1100 us(Setting prohibited(BESL = 0)) / 8950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == HIRD_A::_1110
    }
    ///1175 us(Setting prohibited(BESL = 0)) / 9950 us(BESL = 1)
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == HIRD_A::_1111
    }
}
///Field `HIRD` writer - LPM Token HIRD
pub type HIRD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HIRD_A, crate::Safe>;
impl<'a, REG> HIRD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///50 us(Setting prohibited(BESL = 0)) / 75 us(BESL = 1)
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0000)
    }
    ///125 us(BESL = 0) / 100 us(BESL = 1)
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0001)
    }
    ///200 us(BESL = 0) / 150 us(BESL = 1)
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0010)
    }
    ///275 us(BESL = 0) / 250 us(BESL = 1)
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0011)
    }
    ///350 us(BESL = 0) / 350 us(BESL = 1)
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0100)
    }
    ///425 us(BESL = 0) / 450 us(BESL = 1)
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0101)
    }
    ///500 us(BESL = 0) / 950 us(BESL = 1)
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0110)
    }
    ///575 us(BESL = 0) / 1950 us(BESL = 1)
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_0111)
    }
    ///650 us(BESL = 0) / 2950 us(BESL = 1)
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1000)
    }
    ///725 us(BESL = 0) / 3950 us(BESL = 1)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1001)
    }
    ///800 us(BESL = 0) / 4950 us(BESL = 1)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1010)
    }
    ///875 us(BESL = 0) / 5950 us(BESL = 1)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1011)
    }
    ///950 us(BESL = 0) / 6950 us(BESL = 1)
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1100)
    }
    ///1025 us(Setting prohibited(BESL = 0)) / 7950 us(BESL = 1)
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1101)
    }
    ///1100 us(Setting prohibited(BESL = 0)) / 8950 us(BESL = 1)
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1110)
    }
    ///1175 us(Setting prohibited(BESL = 0)) / 9950 us(BESL = 1)
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(HIRD_A::_1111)
    }
}
///Field `L1RWE` reader - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token.
pub type L1RWE_R = crate::BitReader;
///Field `L1RWE` writer - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token.
pub type L1RWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BESL` reader - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume.
pub type BESL_R = crate::BitReader;
///Field `BESL` writer - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume.
pub type BESL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token.
    #[inline(always)]
    pub fn l1addr(&self) -> L1ADDR_R {
        L1ADDR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - LPM Token HIRD
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token.
    #[inline(always)]
    pub fn l1rwe(&self) -> L1RWE_R {
        L1RWE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume.
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - LPM Token DeviceAddressThese bits specify the value to be set in the ADDR field of LPM token.
    #[inline(always)]
    pub fn l1addr(&mut self) -> L1ADDR_W<HL1CTRL2_SPEC> {
        L1ADDR_W::new(self, 0)
    }
    ///Bits 8:11 - LPM Token HIRD
    #[inline(always)]
    pub fn hird(&mut self) -> HIRD_W<HL1CTRL2_SPEC> {
        HIRD_W::new(self, 8)
    }
    ///Bit 12 - LPM Token L1 RemoteWake EnableThese bits specify the value to be set in the RWE field of LPM token.
    #[inline(always)]
    pub fn l1rwe(&mut self) -> L1RWE_W<HL1CTRL2_SPEC> {
        L1RWE_W::new(self, 12)
    }
    ///Bit 15 - BESL & Alternate HIRDThis bit selects the K-State drive period at the time of L1 Resume.
    #[inline(always)]
    pub fn besl(&mut self) -> BESL_W<HL1CTRL2_SPEC> {
        BESL_W::new(self, 15)
    }
}
/**Host L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HL1CTRL2_SPEC;
impl crate::RegisterSpec for HL1CTRL2_SPEC {
    type Ux = u16;
}
///`read()` method returns [`hl1ctrl2::R`](R) reader structure
impl crate::Readable for HL1CTRL2_SPEC {}
///`write(|w| ..)` method takes [`hl1ctrl2::W`](W) writer structure
impl crate::Writable for HL1CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HL1CTRL2 to value 0
impl crate::Resettable for HL1CTRL2_SPEC {}
