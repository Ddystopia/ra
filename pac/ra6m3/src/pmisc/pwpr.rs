///Register `PWPR` reader
pub type R = crate::R<PWPR_SPEC>;
///Register `PWPR` writer
pub type W = crate::W<PWPR_SPEC>;
/**PFS Register Write Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFSWE_A {
    ///0: Writing to the PFS register is disabled
    _0 = 0,
    ///1: Writing to the PFS register is enabled
    _1 = 1,
}
impl From<PFSWE_A> for bool {
    #[inline(always)]
    fn from(variant: PFSWE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PFSWE` reader - PFS Register Write Enable
pub type PFSWE_R = crate::BitReader<PFSWE_A>;
impl PFSWE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFSWE_A {
        match self.bits {
            false => PFSWE_A::_0,
            true => PFSWE_A::_1,
        }
    }
    ///Writing to the PFS register is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFSWE_A::_0
    }
    ///Writing to the PFS register is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFSWE_A::_1
    }
}
///Field `PFSWE` writer - PFS Register Write Enable
pub type PFSWE_W<'a, REG> = crate::BitWriter<'a, REG, PFSWE_A>;
impl<'a, REG> PFSWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing to the PFS register is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PFSWE_A::_0)
    }
    ///Writing to the PFS register is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PFSWE_A::_1)
    }
}
/**PFSWE Bit Write Disable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOWI_A {
    ///0: Writing to the PFSWE bit is enabled
    _0 = 0,
    ///1: Writing to the PFSWE bit is disabled
    _1 = 1,
}
impl From<BOWI_A> for bool {
    #[inline(always)]
    fn from(variant: BOWI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BOWI` reader - PFSWE Bit Write Disable
pub type BOWI_R = crate::BitReader<BOWI_A>;
impl BOWI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOWI_A {
        match self.bits {
            false => BOWI_A::_0,
            true => BOWI_A::_1,
        }
    }
    ///Writing to the PFSWE bit is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOWI_A::_0
    }
    ///Writing to the PFSWE bit is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOWI_A::_1
    }
}
///Field `BOWI` writer - PFSWE Bit Write Disable
pub type BOWI_W<'a, REG> = crate::BitWriter<'a, REG, BOWI_A>;
impl<'a, REG> BOWI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing to the PFSWE bit is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BOWI_A::_0)
    }
    ///Writing to the PFSWE bit is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BOWI_A::_1)
    }
}
impl R {
    ///Bit 6 - PFS Register Write Enable
    #[inline(always)]
    pub fn pfswe(&self) -> PFSWE_R {
        PFSWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PFSWE Bit Write Disable
    #[inline(always)]
    pub fn bowi(&self) -> BOWI_R {
        BOWI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - PFS Register Write Enable
    #[inline(always)]
    pub fn pfswe(&mut self) -> PFSWE_W<PWPR_SPEC> {
        PFSWE_W::new(self, 6)
    }
    ///Bit 7 - PFSWE Bit Write Disable
    #[inline(always)]
    pub fn bowi(&mut self) -> BOWI_W<PWPR_SPEC> {
        BOWI_W::new(self, 7)
    }
}
/**Write-Protect Register

You can [`read`](crate::Reg::read) this register and get [`pwpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PWPR_SPEC;
impl crate::RegisterSpec for PWPR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`pwpr::R`](R) reader structure
impl crate::Readable for PWPR_SPEC {}
///`write(|w| ..)` method takes [`pwpr::W`](W) writer structure
impl crate::Writable for PWPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWPR to value 0x80
impl crate::Resettable for PWPR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
