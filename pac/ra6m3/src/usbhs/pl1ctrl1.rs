///Register `PL1CTRL1` reader
pub type R = crate::R<PL1CTRL1_SPEC>;
///Register `PL1CTRL1` writer
pub type W = crate::W<PL1CTRL1_SPEC>;
/**L1 Response Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RESPEN_A {
    ///0: LPM is not supported.
    _0 = 0,
    ///1: LPM is supported.
    _1 = 1,
}
impl From<L1RESPEN_A> for bool {
    #[inline(always)]
    fn from(variant: L1RESPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1RESPEN` reader - L1 Response Enable
pub type L1RESPEN_R = crate::BitReader<L1RESPEN_A>;
impl L1RESPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1RESPEN_A {
        match self.bits {
            false => L1RESPEN_A::_0,
            true => L1RESPEN_A::_1,
        }
    }
    ///LPM is not supported.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1RESPEN_A::_0
    }
    ///LPM is supported.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1RESPEN_A::_1
    }
}
///Field `L1RESPEN` writer - L1 Response Enable
pub type L1RESPEN_W<'a, REG> = crate::BitWriter<'a, REG, L1RESPEN_A>;
impl<'a, REG> L1RESPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPM is not supported.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESPEN_A::_0)
    }
    ///LPM is supported.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESPEN_A::_1)
    }
}
/**L1 Response Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1RESPMD_A {
    ///0: NYET
    _00 = 0,
    ///1: ACK
    _01 = 1,
    ///2: STALL
    _10 = 2,
    ///3: According to the L1NEGOMD bit
    _11 = 3,
}
impl From<L1RESPMD_A> for u8 {
    #[inline(always)]
    fn from(variant: L1RESPMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for L1RESPMD_A {
    type Ux = u8;
}
impl crate::IsEnum for L1RESPMD_A {}
///Field `L1RESPMD` reader - L1 Response Mode
pub type L1RESPMD_R = crate::FieldReader<L1RESPMD_A>;
impl L1RESPMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1RESPMD_A {
        match self.bits {
            0 => L1RESPMD_A::_00,
            1 => L1RESPMD_A::_01,
            2 => L1RESPMD_A::_10,
            3 => L1RESPMD_A::_11,
            _ => unreachable!(),
        }
    }
    ///NYET
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == L1RESPMD_A::_00
    }
    ///ACK
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == L1RESPMD_A::_01
    }
    ///STALL
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == L1RESPMD_A::_10
    }
    ///According to the L1NEGOMD bit
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == L1RESPMD_A::_11
    }
}
///Field `L1RESPMD` writer - L1 Response Mode
pub type L1RESPMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, L1RESPMD_A, crate::Safe>;
impl<'a, REG> L1RESPMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///NYET
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESPMD_A::_00)
    }
    ///ACK
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESPMD_A::_01)
    }
    ///STALL
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESPMD_A::_10)
    }
    ///According to the L1NEGOMD bit
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESPMD_A::_11)
    }
}
/**L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\[1:0\] value is 2'b11.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1NEGOMD_A {
    ///0: When receive HIRD is larger than HIRDTHR\[3:0\], ACK response is returned. In other cases (including HIRD = HIRDTHR\[3:0\]), NYET response is returned.
    _0 = 0,
    ///1: When receive HIRD is smaller than HIRDTHR\[3:0\], ACK response is returned. In other cases (including HIRD = HIRDTHR\[3:0\]), NYET response is returned.
    _1 = 1,
}
impl From<L1NEGOMD_A> for bool {
    #[inline(always)]
    fn from(variant: L1NEGOMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1NEGOMD` reader - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\[1:0\] value is 2'b11.
pub type L1NEGOMD_R = crate::BitReader<L1NEGOMD_A>;
impl L1NEGOMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1NEGOMD_A {
        match self.bits {
            false => L1NEGOMD_A::_0,
            true => L1NEGOMD_A::_1,
        }
    }
    ///When receive HIRD is larger than HIRDTHR\[3:0\], ACK response is returned. In other cases (including HIRD = HIRDTHR\[3:0\]), NYET response is returned.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1NEGOMD_A::_0
    }
    ///When receive HIRD is smaller than HIRDTHR\[3:0\], ACK response is returned. In other cases (including HIRD = HIRDTHR\[3:0\]), NYET response is returned.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1NEGOMD_A::_1
    }
}
///Field `L1NEGOMD` writer - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\[1:0\] value is 2'b11.
pub type L1NEGOMD_W<'a, REG> = crate::BitWriter<'a, REG, L1NEGOMD_A>;
impl<'a, REG> L1NEGOMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When receive HIRD is larger than HIRDTHR\[3:0\], ACK response is returned. In other cases (including HIRD = HIRDTHR\[3:0\]), NYET response is returned.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1NEGOMD_A::_0)
    }
    ///When receive HIRD is smaller than HIRDTHR\[3:0\], ACK response is returned. In other cases (including HIRD = HIRDTHR\[3:0\]), NYET response is returned.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1NEGOMD_A::_1)
    }
}
/**DVSQ Extension.DVSQ\[3\] is Mirror of DVSQ\[2:0\] in INTSTS0.Indicates the L1 state together with the device state bits DVSQ\[2:0\].

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVSQ_A {
    ///0: Powered state
    _0000 = 0,
    ///1: Default state
    _0001 = 1,
    ///2: Address state
    _0010 = 2,
    ///3: Configured state
    _0011 = 3,
    ///4: Suspended state
    _0100 = 4,
    ///5: Suspended state
    _0101 = 5,
    ///6: Suspended state
    _0110 = 6,
    ///7: Suspended state
    _0111 = 7,
    ///8: L1 state
    _1000 = 8,
    ///9: L1 state
    _1001 = 9,
    ///10: L1 state
    _1010 = 10,
    ///11: L1 state
    _1011 = 11,
    ///12: setting prohibited
    OTHERS = 12,
}
impl From<DVSQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DVSQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DVSQ_A {
    type Ux = u8;
}
impl crate::IsEnum for DVSQ_A {}
///Field `DVSQ` reader - DVSQ Extension.DVSQ\[3\] is Mirror of DVSQ\[2:0\] in INTSTS0.Indicates the L1 state together with the device state bits DVSQ\[2:0\].
pub type DVSQ_R = crate::FieldReader<DVSQ_A>;
impl DVSQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVSQ_A {
        match self.bits {
            0 => DVSQ_A::_0000,
            1 => DVSQ_A::_0001,
            2 => DVSQ_A::_0010,
            3 => DVSQ_A::_0011,
            4 => DVSQ_A::_0100,
            5 => DVSQ_A::_0101,
            6 => DVSQ_A::_0110,
            7 => DVSQ_A::_0111,
            8 => DVSQ_A::_1000,
            9 => DVSQ_A::_1001,
            10 => DVSQ_A::_1010,
            11 => DVSQ_A::_1011,
            _ => DVSQ_A::OTHERS,
        }
    }
    ///Powered state
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DVSQ_A::_0000
    }
    ///Default state
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DVSQ_A::_0001
    }
    ///Address state
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DVSQ_A::_0010
    }
    ///Configured state
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DVSQ_A::_0011
    }
    ///Suspended state
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DVSQ_A::_0100
    }
    ///Suspended state
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DVSQ_A::_0101
    }
    ///Suspended state
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DVSQ_A::_0110
    }
    ///Suspended state
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DVSQ_A::_0111
    }
    ///L1 state
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == DVSQ_A::_1000
    }
    ///L1 state
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == DVSQ_A::_1001
    }
    ///L1 state
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == DVSQ_A::_1010
    }
    ///L1 state
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == DVSQ_A::_1011
    }
    ///setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DVSQ_A::OTHERS)
    }
}
///Field `HIRDTHR` reader - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL.
pub type HIRDTHR_R = crate::FieldReader;
///Field `HIRDTHR` writer - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL.
pub type HIRDTHR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**PHY Control Mode at L1 Return

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1EXTMD_A {
    ///0: SUSPENDM is not set by hardware when Host K is received.
    _0 = 0,
    ///1: SUSPENDM is set by hardware when Host K is received.
    _1 = 1,
}
impl From<L1EXTMD_A> for bool {
    #[inline(always)]
    fn from(variant: L1EXTMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1EXTMD` reader - PHY Control Mode at L1 Return
pub type L1EXTMD_R = crate::BitReader<L1EXTMD_A>;
impl L1EXTMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1EXTMD_A {
        match self.bits {
            false => L1EXTMD_A::_0,
            true => L1EXTMD_A::_1,
        }
    }
    ///SUSPENDM is not set by hardware when Host K is received.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1EXTMD_A::_0
    }
    ///SUSPENDM is set by hardware when Host K is received.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1EXTMD_A::_1
    }
}
///Field `L1EXTMD` writer - PHY Control Mode at L1 Return
pub type L1EXTMD_W<'a, REG> = crate::BitWriter<'a, REG, L1EXTMD_A>;
impl<'a, REG> L1EXTMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SUSPENDM is not set by hardware when Host K is received.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1EXTMD_A::_0)
    }
    ///SUSPENDM is set by hardware when Host K is received.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1EXTMD_A::_1)
    }
}
impl R {
    ///Bit 0 - L1 Response Enable
    #[inline(always)]
    pub fn l1respen(&self) -> L1RESPEN_R {
        L1RESPEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - L1 Response Mode
    #[inline(always)]
    pub fn l1respmd(&self) -> L1RESPMD_R {
        L1RESPMD_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\[1:0\] value is 2'b11.
    #[inline(always)]
    pub fn l1negomd(&self) -> L1NEGOMD_R {
        L1NEGOMD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - DVSQ Extension.DVSQ\[3\] is Mirror of DVSQ\[2:0\] in INTSTS0.Indicates the L1 state together with the device state bits DVSQ\[2:0\].
    #[inline(always)]
    pub fn dvsq(&self) -> DVSQ_R {
        DVSQ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL.
    #[inline(always)]
    pub fn hirdthr(&self) -> HIRDTHR_R {
        HIRDTHR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 14 - PHY Control Mode at L1 Return
    #[inline(always)]
    pub fn l1extmd(&self) -> L1EXTMD_R {
        L1EXTMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - L1 Response Enable
    #[inline(always)]
    pub fn l1respen(&mut self) -> L1RESPEN_W<PL1CTRL1_SPEC> {
        L1RESPEN_W::new(self, 0)
    }
    ///Bits 1:2 - L1 Response Mode
    #[inline(always)]
    pub fn l1respmd(&mut self) -> L1RESPMD_W<PL1CTRL1_SPEC> {
        L1RESPMD_W::new(self, 1)
    }
    ///Bit 3 - L1 Response Negotiation Control.NOTE: This bit is valid only when the L1RESPMD\[1:0\] value is 2'b11.
    #[inline(always)]
    pub fn l1negomd(&mut self) -> L1NEGOMD_W<PL1CTRL1_SPEC> {
        L1NEGOMD_W::new(self, 3)
    }
    ///Bits 8:11 - L1 Response Negotiation Threshold ValueHIRD threshold value used for L1NEGOMD.The format is the same as the HIRD field in HL1CTRL.
    #[inline(always)]
    pub fn hirdthr(&mut self) -> HIRDTHR_W<PL1CTRL1_SPEC> {
        HIRDTHR_W::new(self, 8)
    }
    ///Bit 14 - PHY Control Mode at L1 Return
    #[inline(always)]
    pub fn l1extmd(&mut self) -> L1EXTMD_W<PL1CTRL1_SPEC> {
        L1EXTMD_W::new(self, 14)
    }
}
/**Function L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PL1CTRL1_SPEC;
impl crate::RegisterSpec for PL1CTRL1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pl1ctrl1::R`](R) reader structure
impl crate::Readable for PL1CTRL1_SPEC {}
///`write(|w| ..)` method takes [`pl1ctrl1::W`](W) writer structure
impl crate::Writable for PL1CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PL1CTRL1 to value 0
impl crate::Resettable for PL1CTRL1_SPEC {}
