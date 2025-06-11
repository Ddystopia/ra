///Register `MSSR` reader
pub type R = crate::R<MSSR_SPEC>;
///Field `MBNST` reader - Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR.
pub type MBNST_R = crate::FieldReader;
/**Search Result Status

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEST_A {
    ///0: Search result found
    _0 = 0,
    ///1: No search result
    _1 = 1,
}
impl From<SEST_A> for bool {
    #[inline(always)]
    fn from(variant: SEST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEST` reader - Search Result Status
pub type SEST_R = crate::BitReader<SEST_A>;
impl SEST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEST_A {
        match self.bits {
            false => SEST_A::_0,
            true => SEST_A::_1,
        }
    }
    ///Search result found
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEST_A::_0
    }
    ///No search result
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEST_A::_1
    }
}
impl R {
    ///Bits 0:4 - Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR.
    #[inline(always)]
    pub fn mbnst(&self) -> MBNST_R {
        MBNST_R::new(self.bits & 0x1f)
    }
    ///Bit 7 - Search Result Status
    #[inline(always)]
    pub fn sest(&self) -> SEST_R {
        SEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
/**Mailbox Search Status Register

You can [`read`](crate::Reg::read) this register and get [`mssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSSR_SPEC;
impl crate::RegisterSpec for MSSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mssr::R`](R) reader structure
impl crate::Readable for MSSR_SPEC {}
///`reset()` method sets MSSR to value 0x80
impl crate::Resettable for MSSR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
