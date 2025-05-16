///Register `SCR_SMCI` reader
pub type R = crate::R<SCR_SMCI_SPEC>;
///Register `SCR_SMCI` writer
pub type W = crate::W<SCR_SMCI_SPEC>;
/**Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKE_A {
    ///0: Output disabled(SMR_SMCI.GM=0) / Output fixed low(SMR_SMCI.GM=1)
    _00 = 0,
    ///1: Clock Output
    _01 = 1,
    ///2: Setting prohibited(SMR_SMCI.GM=0) / Output fixed High(SMR_SMCI.GM=1)
    _10 = 2,
    ///3: Setting prohibited(SMR_SMCI.GM=0) / Clock Output(SMR_SMCI.GM=1)
    _11 = 3,
}
impl From<CKE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKE_A {
    type Ux = u8;
}
impl crate::IsEnum for CKE_A {}
///Field `CKE` reader - Clock Enable
pub type CKE_R = crate::FieldReader<CKE_A>;
impl CKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKE_A {
        match self.bits {
            0 => CKE_A::_00,
            1 => CKE_A::_01,
            2 => CKE_A::_10,
            3 => CKE_A::_11,
            _ => unreachable!(),
        }
    }
    ///Output disabled(SMR_SMCI.GM=0) / Output fixed low(SMR_SMCI.GM=1)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKE_A::_00
    }
    ///Clock Output
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKE_A::_01
    }
    ///Setting prohibited(SMR_SMCI.GM=0) / Output fixed High(SMR_SMCI.GM=1)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKE_A::_10
    }
    ///Setting prohibited(SMR_SMCI.GM=0) / Clock Output(SMR_SMCI.GM=1)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKE_A::_11
    }
}
///Field `CKE` writer - Clock Enable
pub type CKE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKE_A, crate::Safe>;
impl<'a, REG> CKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output disabled(SMR_SMCI.GM=0) / Output fixed low(SMR_SMCI.GM=1)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_00)
    }
    ///Clock Output
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_01)
    }
    ///Setting prohibited(SMR_SMCI.GM=0) / Output fixed High(SMR_SMCI.GM=1)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_10)
    }
    ///Setting prohibited(SMR_SMCI.GM=0) / Clock Output(SMR_SMCI.GM=1)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_11)
    }
}
///Field `TEIE` reader - Transmit End Interrupt EnableSet this bit to 0 in smart card interface mode.
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transmit End Interrupt EnableSet this bit to 0 in smart card interface mode.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPIE` reader - Multi-Processor Interrupt EnableSet this bit to 0 in smart card interface mode.
pub type MPIE_R = crate::BitReader;
///Field `MPIE` writer - Multi-Processor Interrupt EnableSet this bit to 0 in smart card interface mode.
pub type MPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Receive Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    ///0: Serial reception is disabled
    _0 = 0,
    ///1: Serial reception is enabled
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Receive Enable
pub type RE_R = crate::BitReader<RE_A>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    ///Serial reception is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    ///Serial reception is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
///Field `RE` writer - Receive Enable
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE_A>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Serial reception is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::_0)
    }
    ///Serial reception is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::_1)
    }
}
/**Transmit Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    ///0: Serial transmission is disabled
    _0 = 0,
    ///1: Serial transmission is enabled
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmit Enable
pub type TE_R = crate::BitReader<TE_A>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    ///Serial transmission is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    ///Serial transmission is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
///Field `TE` writer - Transmit Enable
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE_A>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Serial transmission is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::_0)
    }
    ///Serial transmission is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::_1)
    }
}
/**Receive Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    ///0: RXI and ERI interrupt requests are disabled
    _0 = 0,
    ///1: RXI and ERI interrupt requests are enabled
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Interrupt Enable
pub type RIE_R = crate::BitReader<RIE_A>;
impl RIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    ///RXI and ERI interrupt requests are disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    ///RXI and ERI interrupt requests are enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
///Field `RIE` writer - Receive Interrupt Enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG, RIE_A>;
impl<'a, REG> RIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXI and ERI interrupt requests are disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    ///RXI and ERI interrupt requests are enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
/**Transmit Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    ///0: A TXI interrupt request is disabled
    _0 = 0,
    ///1: A TXI interrupt request is enabled
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Interrupt Enable
pub type TIE_R = crate::BitReader<TIE_A>;
impl TIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    ///A TXI interrupt request is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    ///A TXI interrupt request is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
///Field `TIE` writer - Transmit Interrupt Enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A TXI interrupt request is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_0)
    }
    ///A TXI interrupt request is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Clock Enable
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(self.bits & 3)
    }
    ///Bit 2 - Transmit End Interrupt EnableSet this bit to 0 in smart card interface mode.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Multi-Processor Interrupt EnableSet this bit to 0 in smart card interface mode.
    #[inline(always)]
    pub fn mpie(&self) -> MPIE_R {
        MPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive Enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit Enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Clock Enable
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W<SCR_SMCI_SPEC> {
        CKE_W::new(self, 0)
    }
    ///Bit 2 - Transmit End Interrupt EnableSet this bit to 0 in smart card interface mode.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<SCR_SMCI_SPEC> {
        TEIE_W::new(self, 2)
    }
    ///Bit 3 - Multi-Processor Interrupt EnableSet this bit to 0 in smart card interface mode.
    #[inline(always)]
    pub fn mpie(&mut self) -> MPIE_W<SCR_SMCI_SPEC> {
        MPIE_W::new(self, 3)
    }
    ///Bit 4 - Receive Enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<SCR_SMCI_SPEC> {
        RE_W::new(self, 4)
    }
    ///Bit 5 - Transmit Enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<SCR_SMCI_SPEC> {
        TE_W::new(self, 5)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<SCR_SMCI_SPEC> {
        RIE_W::new(self, 6)
    }
    ///Bit 7 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<SCR_SMCI_SPEC> {
        TIE_W::new(self, 7)
    }
}
/**Serial Control Register (SCMR.SMIF =1)

You can [`read`](crate::Reg::read) this register and get [`scr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCR_SMCI_SPEC;
impl crate::RegisterSpec for SCR_SMCI_SPEC {
    type Ux = u8;
}
///`read()` method returns [`scr_smci::R`](R) reader structure
impl crate::Readable for SCR_SMCI_SPEC {}
///`write(|w| ..)` method takes [`scr_smci::W`](W) writer structure
impl crate::Writable for SCR_SMCI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR_SMCI to value 0
impl crate::Resettable for SCR_SMCI_SPEC {}
