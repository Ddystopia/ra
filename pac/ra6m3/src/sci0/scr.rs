///Register `SCR` reader
pub type R = crate::R<SCR_SPEC>;
///Register `SCR` writer
pub type W = crate::W<SCR_SPEC>;
/**Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKE_A {
    ///0: The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)
    _00 = 0,
    ///1: The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)
    _01 = 1,
    ///2: The clock with a frequency 16 times the bit rate should be input from the SCKn pin. (when SEMR.ABCS bit is 0) Input a clock signal with a frequency 8 times the bit rate when the SEMR.ABCS bit is 1.(Asynchronous mode) / The SCKn pin functions as the clock input pin(Clock synchronous mode)
    OTHERS = 2,
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
            _ => CKE_A::OTHERS,
        }
    }
    ///The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKE_A::_00
    }
    ///The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKE_A::_01
    }
    ///The clock with a frequency 16 times the bit rate should be input from the SCKn pin. (when SEMR.ABCS bit is 0) Input a clock signal with a frequency 8 times the bit rate when the SEMR.ABCS bit is 1.(Asynchronous mode) / The SCKn pin functions as the clock input pin(Clock synchronous mode)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CKE_A::OTHERS)
    }
}
///Field `CKE` writer - Clock Enable
pub type CKE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKE_A, crate::Safe>;
impl<'a, REG> CKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_00)
    }
    ///The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::_01)
    }
    ///The clock with a frequency 16 times the bit rate should be input from the SCKn pin. (when SEMR.ABCS bit is 0) Input a clock signal with a frequency 8 times the bit rate when the SEMR.ABCS bit is 1.(Asynchronous mode) / The SCKn pin functions as the clock input pin(Clock synchronous mode)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CKE_A::OTHERS)
    }
}
/**Transmit End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    ///0: TEI interrupt request is disabled
    _0 = 0,
    ///1: TEI interrupt request is enabled
    _1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transmit End Interrupt Enable
pub type TEIE_R = crate::BitReader<TEIE_A>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::_0,
            true => TEIE_A::_1,
        }
    }
    ///TEI interrupt request is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIE_A::_0
    }
    ///TEI interrupt request is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIE_A::_1
    }
}
///Field `TEIE` writer - Transmit End Interrupt Enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE_A>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TEI interrupt request is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::_0)
    }
    ///TEI interrupt request is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::_1)
    }
}
/**Multi-Processor Interrupt Enable(Valid in asynchronous mode when SMR.MP = 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPIE_A {
    ///0: Normal reception
    _0 = 0,
    ///1: When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed.
    _1 = 1,
}
impl From<MPIE_A> for bool {
    #[inline(always)]
    fn from(variant: MPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPIE` reader - Multi-Processor Interrupt Enable(Valid in asynchronous mode when SMR.MP = 1)
pub type MPIE_R = crate::BitReader<MPIE_A>;
impl MPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPIE_A {
        match self.bits {
            false => MPIE_A::_0,
            true => MPIE_A::_1,
        }
    }
    ///Normal reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPIE_A::_0
    }
    ///When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPIE_A::_1
    }
}
///Field `MPIE` writer - Multi-Processor Interrupt Enable(Valid in asynchronous mode when SMR.MP = 1)
pub type MPIE_W<'a, REG> = crate::BitWriter<'a, REG, MPIE_A>;
impl<'a, REG> MPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPIE_A::_0)
    }
    ///When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPIE_A::_1)
    }
}
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
    ///0: TXI interrupt request is disabled
    _0 = 0,
    ///1: TXI interrupt request is enabled
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
    ///TXI interrupt request is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    ///TXI interrupt request is enabled
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
    ///TXI interrupt request is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_0)
    }
    ///TXI interrupt request is enabled
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
    ///Bit 2 - Transmit End Interrupt Enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Multi-Processor Interrupt Enable(Valid in asynchronous mode when SMR.MP = 1)
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
    pub fn cke(&mut self) -> CKE_W<SCR_SPEC> {
        CKE_W::new(self, 0)
    }
    ///Bit 2 - Transmit End Interrupt Enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<SCR_SPEC> {
        TEIE_W::new(self, 2)
    }
    ///Bit 3 - Multi-Processor Interrupt Enable(Valid in asynchronous mode when SMR.MP = 1)
    #[inline(always)]
    pub fn mpie(&mut self) -> MPIE_W<SCR_SPEC> {
        MPIE_W::new(self, 3)
    }
    ///Bit 4 - Receive Enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<SCR_SPEC> {
        RE_W::new(self, 4)
    }
    ///Bit 5 - Transmit Enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<SCR_SPEC> {
        TE_W::new(self, 5)
    }
    ///Bit 6 - Receive Interrupt Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<SCR_SPEC> {
        RIE_W::new(self, 6)
    }
    ///Bit 7 - Transmit Interrupt Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<SCR_SPEC> {
        TIE_W::new(self, 7)
    }
}
/**Serial Control Register (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCR_SPEC {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCR_SPEC {}
