///Register `SDIR` reader
pub type R = crate::R<SDIR_SPEC>;
///Register `SDIR` writer
pub type W = crate::W<SDIR_SPEC>;
///Field `ARFI` reader - Initialization Auto-Refresh Interval (ARFI+3 cycles )
pub type ARFI_R = crate::FieldReader;
///Field `ARFI` writer - Initialization Auto-Refresh Interval (ARFI+3 cycles )
pub type ARFI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Initialization Auto-Refresh Count

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARFC_A {
    ///0: Setting prohibited
    _0X0 = 0,
    ///1: ARFC times
    OTHERS = 1,
}
impl From<ARFC_A> for u8 {
    #[inline(always)]
    fn from(variant: ARFC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARFC_A {
    type Ux = u8;
}
impl crate::IsEnum for ARFC_A {}
///Field `ARFC` reader - Initialization Auto-Refresh Count
pub type ARFC_R = crate::FieldReader<ARFC_A>;
impl ARFC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARFC_A {
        match self.bits {
            0 => ARFC_A::_0X0,
            _ => ARFC_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ARFC_A::_0X0
    }
    ///ARFC times
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), ARFC_A::OTHERS)
    }
}
///Field `ARFC` writer - Initialization Auto-Refresh Count
pub type ARFC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ARFC_A, crate::Safe>;
impl<'a, REG> ARFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARFC_A::_0X0)
    }
    ///ARFC times
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ARFC_A::OTHERS)
    }
}
///Field `PRC` reader - Initialization Precharge Cycle Count (PRC+3 cycles)
pub type PRC_R = crate::FieldReader;
///Field `PRC` writer - Initialization Precharge Cycle Count (PRC+3 cycles)
pub type PRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - Initialization Auto-Refresh Interval (ARFI+3 cycles )
    #[inline(always)]
    pub fn arfi(&self) -> ARFI_R {
        ARFI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Initialization Auto-Refresh Count
    #[inline(always)]
    pub fn arfc(&self) -> ARFC_R {
        ARFC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - Initialization Precharge Cycle Count (PRC+3 cycles)
    #[inline(always)]
    pub fn prc(&self) -> PRC_R {
        PRC_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    ///Bits 0:3 - Initialization Auto-Refresh Interval (ARFI+3 cycles )
    #[inline(always)]
    pub fn arfi(&mut self) -> ARFI_W<SDIR_SPEC> {
        ARFI_W::new(self, 0)
    }
    ///Bits 4:7 - Initialization Auto-Refresh Count
    #[inline(always)]
    pub fn arfc(&mut self) -> ARFC_W<SDIR_SPEC> {
        ARFC_W::new(self, 4)
    }
    ///Bits 8:10 - Initialization Precharge Cycle Count (PRC+3 cycles)
    #[inline(always)]
    pub fn prc(&mut self) -> PRC_W<SDIR_SPEC> {
        PRC_W::new(self, 8)
    }
}
/**SDRAM Initialization Register

You can [`read`](crate::Reg::read) this register and get [`sdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIR_SPEC;
impl crate::RegisterSpec for SDIR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`sdir::R`](R) reader structure
impl crate::Readable for SDIR_SPEC {}
///`write(|w| ..)` method takes [`sdir::W`](W) writer structure
impl crate::Writable for SDIR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIR to value 0x10
impl crate::Resettable for SDIR_SPEC {
    const RESET_VALUE: u16 = 0x10;
}
