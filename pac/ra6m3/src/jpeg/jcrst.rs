///Register `JCRST` reader
pub type R = crate::R<JCRST_SPEC>;
/**Operating State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    ///0: State other than below
    _0 = 0,
    ///1: Suspended state caused by interrupt sources of JINTE0
    _1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RST` reader - Operating State
pub type RST_R = crate::BitReader<RST_A>;
impl RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::_0,
            true => RST_A::_1,
        }
    }
    ///State other than below
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RST_A::_0
    }
    ///Suspended state caused by interrupt sources of JINTE0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RST_A::_1
    }
}
impl R {
    ///Bit 0 - Operating State
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
}
/**JPEG Code Reset Register

You can [`read`](crate::Reg::read) this register and get [`jcrst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCRST_SPEC;
impl crate::RegisterSpec for JCRST_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcrst::R`](R) reader structure
impl crate::Readable for JCRST_SPEC {}
///`reset()` method sets JCRST to value 0
impl crate::Resettable for JCRST_SPEC {}
