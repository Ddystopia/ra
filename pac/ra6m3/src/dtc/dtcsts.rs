///Register `DTCSTS` reader
pub type R = crate::R<DTCSTS_SPEC>;
///Field `VECN` reader - DTC-Activating Vector Number MonitoringThese bits indicate the vector number for the activating source when DTC transfer is in progress.The value is only valid if DTC transfer is in progress (the value of the ACT flag is 1)
pub type VECN_R = crate::FieldReader;
/**DTC Active Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACT_A {
    ///0: DTC transfer operation is not in progress.
    _0 = 0,
    ///1: DTC transfer operation is in progress.
    _1 = 1,
}
impl From<ACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACT` reader - DTC Active Flag
pub type ACT_R = crate::BitReader<ACT_A>;
impl ACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACT_A {
        match self.bits {
            false => ACT_A::_0,
            true => ACT_A::_1,
        }
    }
    ///DTC transfer operation is not in progress.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACT_A::_0
    }
    ///DTC transfer operation is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACT_A::_1
    }
}
impl R {
    ///Bits 0:7 - DTC-Activating Vector Number MonitoringThese bits indicate the vector number for the activating source when DTC transfer is in progress.The value is only valid if DTC transfer is in progress (the value of the ACT flag is 1)
    #[inline(always)]
    pub fn vecn(&self) -> VECN_R {
        VECN_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 15 - DTC Active Flag
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
/**DTC Status Register

You can [`read`](crate::Reg::read) this register and get [`dtcsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DTCSTS_SPEC;
impl crate::RegisterSpec for DTCSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dtcsts::R`](R) reader structure
impl crate::Readable for DTCSTS_SPEC {}
///`reset()` method sets DTCSTS to value 0
impl crate::Resettable for DTCSTS_SPEC {}
