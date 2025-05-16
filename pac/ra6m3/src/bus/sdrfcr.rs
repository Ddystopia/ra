///Register `SDRFCR` reader
pub type R = crate::R<SDRFCR_SPEC>;
///Register `SDRFCR` writer
pub type W = crate::W<SDRFCR_SPEC>;
/**Auto-Refresh Request Interval Setting

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum RFC_A {
    ///0: Setting prohibited
    _0X0 = 0,
    ///1: RFC+1 cycles inserted
    OTHERS = 1,
}
impl From<RFC_A> for u16 {
    #[inline(always)]
    fn from(variant: RFC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RFC_A {
    type Ux = u16;
}
impl crate::IsEnum for RFC_A {}
///Field `RFC` reader - Auto-Refresh Request Interval Setting
pub type RFC_R = crate::FieldReader<RFC_A>;
impl RFC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFC_A {
        match self.bits {
            0 => RFC_A::_0X0,
            _ => RFC_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RFC_A::_0X0
    }
    ///RFC+1 cycles inserted
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RFC_A::OTHERS)
    }
}
///Field `RFC` writer - Auto-Refresh Request Interval Setting
pub type RFC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, RFC_A, crate::Safe>;
impl<'a, REG> RFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RFC_A::_0X0)
    }
    ///RFC+1 cycles inserted
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RFC_A::OTHERS)
    }
}
///Field `REFW` reader - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )
pub type REFW_R = crate::FieldReader;
///Field `REFW` writer - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )
pub type REFW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:11 - Auto-Refresh Request Interval Setting
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(self.bits & 0x0fff)
    }
    ///Bits 12:15 - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )
    #[inline(always)]
    pub fn refw(&self) -> REFW_R {
        REFW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:11 - Auto-Refresh Request Interval Setting
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W<SDRFCR_SPEC> {
        RFC_W::new(self, 0)
    }
    ///Bits 12:15 - Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )
    #[inline(always)]
    pub fn refw(&mut self) -> REFW_W<SDRFCR_SPEC> {
        REFW_W::new(self, 12)
    }
}
/**SDRAM Refresh Control Register

You can [`read`](crate::Reg::read) this register and get [`sdrfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDRFCR_SPEC;
impl crate::RegisterSpec for SDRFCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`sdrfcr::R`](R) reader structure
impl crate::Readable for SDRFCR_SPEC {}
///`write(|w| ..)` method takes [`sdrfcr::W`](W) writer structure
impl crate::Writable for SDRFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDRFCR to value 0x01
impl crate::Resettable for SDRFCR_SPEC {
    const RESET_VALUE: u16 = 0x01;
}
