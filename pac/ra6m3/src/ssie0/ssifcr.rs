///Register `SSIFCR` reader
pub type R = crate::R<SSIFCR_SPEC>;
///Register `SSIFCR` writer
pub type W = crate::W<SSIFCR_SPEC>;
/**Receive FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    ///0: Clears the receive data FIFO reset.
    _0 = 0,
    ///1: Initiates the receive data FIFO reset.
    _1 = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRST` reader - Receive FIFO Data Register Reset
pub type RFRST_R = crate::BitReader<RFRST_A>;
impl RFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::_0,
            true => RFRST_A::_1,
        }
    }
    ///Clears the receive data FIFO reset.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRST_A::_0
    }
    ///Initiates the receive data FIFO reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRST_A::_1
    }
}
///Field `RFRST` writer - Receive FIFO Data Register Reset
pub type RFRST_W<'a, REG> = crate::BitWriter<'a, REG, RFRST_A>;
impl<'a, REG> RFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the receive data FIFO reset.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_0)
    }
    ///Initiates the receive data FIFO reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_1)
    }
}
/**Transmit FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    ///0: Clears the transmit data FIFO reset.
    _0 = 0,
    ///1: Initiates the transmit data FIFO reset.
    _1 = 1,
}
impl From<TFRST_A> for bool {
    #[inline(always)]
    fn from(variant: TFRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFRST` reader - Transmit FIFO Data Register Reset
pub type TFRST_R = crate::BitReader<TFRST_A>;
impl TFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFRST_A {
        match self.bits {
            false => TFRST_A::_0,
            true => TFRST_A::_1,
        }
    }
    ///Clears the transmit data FIFO reset.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFRST_A::_0
    }
    ///Initiates the transmit data FIFO reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFRST_A::_1
    }
}
///Field `TFRST` writer - Transmit FIFO Data Register Reset
pub type TFRST_W<'a, REG> = crate::BitWriter<'a, REG, TFRST_A>;
impl<'a, REG> TFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the transmit data FIFO reset.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_0)
    }
    ///Initiates the transmit data FIFO reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_1)
    }
}
/**Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    ///0: Receive data full interrupt (RXI) request is disabled
    _0 = 0,
    ///1: Receive data full interrupt (RXI) request is enabled
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit.
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
    ///Receive data full interrupt (RXI) request is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    ///Receive data full interrupt (RXI) request is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
///Field `RIE` writer - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit.
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG, RIE_A>;
impl<'a, REG> RIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive data full interrupt (RXI) request is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    ///Receive data full interrupt (RXI) request is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
/**Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    ///0: Transmit data empty interrupt (TXI) request is disabled
    _0 = 0,
    ///1: Transmit data empty interrupt (TXI) request is enabled
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit.
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
    ///Transmit data empty interrupt (TXI) request is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    ///Transmit data empty interrupt (TXI) request is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
///Field `TIE` writer - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit.
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit data empty interrupt (TXI) request is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_0)
    }
    ///Transmit data empty interrupt (TXI) request is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_1)
    }
}
/**Receive Data Trigger Number

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTRG_A {
    ///0: 1
    _00 = 0,
    ///1: 2
    _01 = 1,
    ///2: 4
    _10 = 2,
    ///3: 6
    _11 = 3,
}
impl From<RTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for RTRG_A {}
///Field `RTRG` reader - Receive Data Trigger Number
pub type RTRG_R = crate::FieldReader<RTRG_A>;
impl RTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTRG_A {
        match self.bits {
            0 => RTRG_A::_00,
            1 => RTRG_A::_01,
            2 => RTRG_A::_10,
            3 => RTRG_A::_11,
            _ => unreachable!(),
        }
    }
    ///1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTRG_A::_00
    }
    ///2
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTRG_A::_01
    }
    ///4
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTRG_A::_10
    }
    ///6
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTRG_A::_11
    }
}
///Field `RTRG` writer - Receive Data Trigger Number
pub type RTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTRG_A, crate::Safe>;
impl<'a, REG> RTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::_00)
    }
    ///2
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::_01)
    }
    ///4
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::_10)
    }
    ///6
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::_11)
    }
}
/**Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTRG_A {
    ///0: 7 (1)
    _00 = 0,
    ///1: 6 (2)
    _01 = 1,
    ///2: 4 (4)
    _10 = 2,
    ///3: 2 (6)
    _11 = 3,
}
impl From<TTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: TTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for TTRG_A {}
///Field `TTRG` reader - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set.
pub type TTRG_R = crate::FieldReader<TTRG_A>;
impl TTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TTRG_A {
        match self.bits {
            0 => TTRG_A::_00,
            1 => TTRG_A::_01,
            2 => TTRG_A::_10,
            3 => TTRG_A::_11,
            _ => unreachable!(),
        }
    }
    ///7 (1)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TTRG_A::_00
    }
    ///6 (2)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TTRG_A::_01
    }
    ///4 (4)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TTRG_A::_10
    }
    ///2 (6)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TTRG_A::_11
    }
}
///Field `TTRG` writer - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set.
pub type TTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TTRG_A, crate::Safe>;
impl<'a, REG> TTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///7 (1)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::_00)
    }
    ///6 (2)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::_01)
    }
    ///4 (4)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::_10)
    }
    ///2 (6)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::_11)
    }
}
/**SSI soft ware reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIRST_A {
    ///0: Clears the SSI software reset.
    _0 = 0,
    ///1: initiates the SSI software reset.
    _1 = 1,
}
impl From<SSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: SSIRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSIRST` reader - SSI soft ware reset
pub type SSIRST_R = crate::BitReader<SSIRST_A>;
impl SSIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSIRST_A {
        match self.bits {
            false => SSIRST_A::_0,
            true => SSIRST_A::_1,
        }
    }
    ///Clears the SSI software reset.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIRST_A::_0
    }
    ///initiates the SSI software reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIRST_A::_1
    }
}
///Field `SSIRST` writer - SSI soft ware reset
pub type SSIRST_W<'a, REG> = crate::BitWriter<'a, REG, SSIRST_A>;
impl<'a, REG> SSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the SSI software reset.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSIRST_A::_0)
    }
    ///initiates the SSI software reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSIRST_A::_1)
    }
}
/**Oversampling Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUCKE_A {
    ///0: The oversampling clock is disabled.
    _0 = 0,
    ///1: The oversampling clock is enabled.
    _1 = 1,
}
impl From<AUCKE_A> for bool {
    #[inline(always)]
    fn from(variant: AUCKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUCKE` reader - Oversampling Clock Enable
pub type AUCKE_R = crate::BitReader<AUCKE_A>;
impl AUCKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUCKE_A {
        match self.bits {
            false => AUCKE_A::_0,
            true => AUCKE_A::_1,
        }
    }
    ///The oversampling clock is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUCKE_A::_0
    }
    ///The oversampling clock is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUCKE_A::_1
    }
}
///Field `AUCKE` writer - Oversampling Clock Enable
pub type AUCKE_W<'a, REG> = crate::BitWriter<'a, REG, AUCKE_A>;
impl<'a, REG> AUCKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The oversampling clock is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AUCKE_A::_0)
    }
    ///The oversampling clock is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AUCKE_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit.
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit.
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Receive Data Trigger Number
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set.
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 16 - SSI soft ware reset
    #[inline(always)]
    pub fn ssirst(&self) -> SSIRST_R {
        SSIRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Oversampling Clock Enable
    #[inline(always)]
    pub fn aucke(&self) -> AUCKE_R {
        AUCKE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W<SSIFCR_SPEC> {
        RFRST_W::new(self, 0)
    }
    ///Bit 1 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&mut self) -> TFRST_W<SSIFCR_SPEC> {
        TFRST_W::new(self, 1)
    }
    ///Bit 2 - Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit.
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<SSIFCR_SPEC> {
        RIE_W::new(self, 2)
    }
    ///Bit 3 - Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit.
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<SSIFCR_SPEC> {
        TIE_W::new(self, 3)
    }
    ///Bits 4:5 - Receive Data Trigger Number
    #[inline(always)]
    pub fn rtrg(&mut self) -> RTRG_W<SSIFCR_SPEC> {
        RTRG_W::new(self, 4)
    }
    ///Bits 6:7 - Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set.
    #[inline(always)]
    pub fn ttrg(&mut self) -> TTRG_W<SSIFCR_SPEC> {
        TTRG_W::new(self, 6)
    }
    ///Bit 16 - SSI soft ware reset
    #[inline(always)]
    pub fn ssirst(&mut self) -> SSIRST_W<SSIFCR_SPEC> {
        SSIRST_W::new(self, 16)
    }
    ///Bit 31 - Oversampling Clock Enable
    #[inline(always)]
    pub fn aucke(&mut self) -> AUCKE_W<SSIFCR_SPEC> {
        AUCKE_W::new(self, 31)
    }
}
/**FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`ssifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSIFCR_SPEC;
impl crate::RegisterSpec for SSIFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssifcr::R`](R) reader structure
impl crate::Readable for SSIFCR_SPEC {}
///`write(|w| ..)` method takes [`ssifcr::W`](W) writer structure
impl crate::Writable for SSIFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIFCR to value 0
impl crate::Resettable for SSIFCR_SPEC {}
