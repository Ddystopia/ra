///Register `R64CNT` reader
pub type R = crate::R<R64CNT_SPEC>;
///Field `F64HZ` reader - 64Hz
pub type F64HZ_R = crate::BitReader;
///Field `F32HZ` reader - 32Hz
pub type F32HZ_R = crate::BitReader;
///Field `F16HZ` reader - 16Hz
pub type F16HZ_R = crate::BitReader;
///Field `F8HZ` reader - 8Hz
pub type F8HZ_R = crate::BitReader;
///Field `F4HZ` reader - 4Hz
pub type F4HZ_R = crate::BitReader;
///Field `F2HZ` reader - 2Hz
pub type F2HZ_R = crate::BitReader;
///Field `F1HZ` reader - 1Hz
pub type F1HZ_R = crate::BitReader;
impl R {
    ///Bit 0 - 64Hz
    #[inline(always)]
    pub fn f64hz(&self) -> F64HZ_R {
        F64HZ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 32Hz
    #[inline(always)]
    pub fn f32hz(&self) -> F32HZ_R {
        F32HZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 16Hz
    #[inline(always)]
    pub fn f16hz(&self) -> F16HZ_R {
        F16HZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 8Hz
    #[inline(always)]
    pub fn f8hz(&self) -> F8HZ_R {
        F8HZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 4Hz
    #[inline(always)]
    pub fn f4hz(&self) -> F4HZ_R {
        F4HZ_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 2Hz
    #[inline(always)]
    pub fn f2hz(&self) -> F2HZ_R {
        F2HZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1Hz
    #[inline(always)]
    pub fn f1hz(&self) -> F1HZ_R {
        F1HZ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
/**64-Hz Counter

You can [`read`](crate::Reg::read) this register and get [`r64cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct R64CNT_SPEC;
impl crate::RegisterSpec for R64CNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`r64cnt::R`](R) reader structure
impl crate::Readable for R64CNT_SPEC {}
///`reset()` method sets R64CNT to value 0
impl crate::Resettable for R64CNT_SPEC {}
