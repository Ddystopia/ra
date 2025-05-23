///Register `FRDRH` reader
pub type R = crate::R<FRDRH_SPEC>;
///Field `RDATH` reader - Serial receive data(b8)(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
pub type RDATH_R = crate::BitReader;
/**Multi-processor bit flag(Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\[8:0\])

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPB_A {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
    _1 = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPB` reader - Multi-processor bit flag(Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\[8:0\])
pub type MPB_R = crate::BitReader<MPB_A>;
impl MPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::_0,
            true => MPB_A::_1,
        }
    }
    ///Data transmission cycles
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPB_A::_0
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPB_A::_1
    }
}
/**Receive data ready flag(It is same as SSR.DR)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DR_A {
    ///0: Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving.
    _0 = 0,
    ///1: Next receive data has not been received for a period after normal completed receiving.
    _1 = 1,
}
impl From<DR_A> for bool {
    #[inline(always)]
    fn from(variant: DR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DR` reader - Receive data ready flag(It is same as SSR.DR)
pub type DR_R = crate::BitReader<DR_A>;
impl DR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DR_A {
        match self.bits {
            false => DR_A::_0,
            true => DR_A::_1,
        }
    }
    ///Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DR_A::_0
    }
    ///Next receive data has not been received for a period after normal completed receiving.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DR_A::_1
    }
}
/**Parity error flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    ///0: No parity error occurred at the first data of FRDRH and FRDRL
    _0 = 0,
    ///1: A parity error has occurred at the first data of FRDRH and FRDRL
    _1 = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PER` reader - Parity error flag
pub type PER_R = crate::BitReader<PER_A>;
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::_0,
            true => PER_A::_1,
        }
    }
    ///No parity error occurred at the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    ///A parity error has occurred at the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER_A::_1
    }
}
/**Framing error flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FER_A {
    ///0: No framing error occurred at the first data of FRDRH and FRDRL
    _0 = 0,
    ///1: A framing error has occurred at the first data of FRDRH and FRDRL
    _1 = 1,
}
impl From<FER_A> for bool {
    #[inline(always)]
    fn from(variant: FER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FER` reader - Framing error flag
pub type FER_R = crate::BitReader<FER_A>;
impl FER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FER_A {
        match self.bits {
            false => FER_A::_0,
            true => FER_A::_1,
        }
    }
    ///No framing error occurred at the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FER_A::_0
    }
    ///A framing error has occurred at the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FER_A::_1
    }
}
/**Overrun error flag(It is same as SSR.ORER)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORER_A {
    ///0: No overrun error occurred
    _0 = 0,
    ///1: An overrun error has occurred
    _1 = 1,
}
impl From<ORER_A> for bool {
    #[inline(always)]
    fn from(variant: ORER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ORER` reader - Overrun error flag(It is same as SSR.ORER)
pub type ORER_R = crate::BitReader<ORER_A>;
impl ORER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORER_A {
        match self.bits {
            false => ORER_A::_0,
            true => ORER_A::_1,
        }
    }
    ///No overrun error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORER_A::_0
    }
    ///An overrun error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORER_A::_1
    }
}
/**Receive FIFO data full flag(It is same as SSR.RDF)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    ///0: The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number.
    _0 = 0,
    ///1: The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number.
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDF` reader - Receive FIFO data full flag(It is same as SSR.RDF)
pub type RDF_R = crate::BitReader<RDF_A>;
impl RDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    ///The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    ///The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
impl R {
    ///Bit 0 - Serial receive data(b8)(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
    #[inline(always)]
    pub fn rdath(&self) -> RDATH_R {
        RDATH_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Multi-processor bit flag(Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\[8:0\])
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive data ready flag(It is same as SSR.DR)
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity error flag
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Framing error flag
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun error flag(It is same as SSR.ORER)
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive FIFO data full flag(It is same as SSR.RDF)
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
/**Receive FIFO Data Register H

You can [`read`](crate::Reg::read) this register and get [`frdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FRDRH_SPEC;
impl crate::RegisterSpec for FRDRH_SPEC {
    type Ux = u8;
}
///`read()` method returns [`frdrh::R`](R) reader structure
impl crate::Readable for FRDRH_SPEC {}
///`reset()` method sets FRDRH to value 0
impl crate::Resettable for FRDRH_SPEC {}
