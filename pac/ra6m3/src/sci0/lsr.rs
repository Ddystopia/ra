///Register `LSR` reader
pub type R = crate::R<LSR_SPEC>;
/**Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)

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
///Field `ORER` reader - Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
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
///Field `FNUM` reader - Framing Error CountIndicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL).
pub type FNUM_R = crate::FieldReader;
///Field `PNUM` reader - Parity Error CountIndicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL).
pub type PNUM_R = crate::FieldReader;
impl R {
    ///Bit 0 - Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
    #[inline(always)]
    pub fn orer(&self) -> ORER_R {
        ORER_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:6 - Framing Error CountIndicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL).
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 8:12 - Parity Error CountIndicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL).
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
/**Line Status Register

You can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`lsr::R`](R) reader structure
impl crate::Readable for LSR_SPEC {}
///`reset()` method sets LSR to value 0
impl crate::Resettable for LSR_SPEC {}
