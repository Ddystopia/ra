///Register `SISR` reader
pub type R = crate::R<SISR_SPEC>;
/**ACK Reception Data Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICACKR_A {
    ///0: ACK received
    _0 = 0,
    ///1: NACK received
    _1 = 1,
}
impl From<IICACKR_A> for bool {
    #[inline(always)]
    fn from(variant: IICACKR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICACKR` reader - ACK Reception Data Flag
pub type IICACKR_R = crate::BitReader<IICACKR_A>;
impl IICACKR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICACKR_A {
        match self.bits {
            false => IICACKR_A::_0,
            true => IICACKR_A::_1,
        }
    }
    ///ACK received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICACKR_A::_0
    }
    ///NACK received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICACKR_A::_1
    }
}
impl R {
    ///Bit 0 - ACK Reception Data Flag
    #[inline(always)]
    pub fn iicackr(&self) -> IICACKR_R {
        IICACKR_R::new((self.bits & 1) != 0)
    }
}
/**I2C Status Register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SISR_SPEC;
impl crate::RegisterSpec for SISR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sisr::R`](R) reader structure
impl crate::Readable for SISR_SPEC {}
///`reset()` method sets SISR to value 0
impl crate::Resettable for SISR_SPEC {}
