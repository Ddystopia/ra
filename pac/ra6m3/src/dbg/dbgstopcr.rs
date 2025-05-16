///Register `DBGSTOPCR` reader
pub type R = crate::R<DBGSTOPCR_SPEC>;
///Register `DBGSTOPCR` writer
pub type W = crate::W<DBGSTOPCR_SPEC>;
/**Mask bit for IWDT reset/interrupt

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_IWDT_A {
    ///0: Mask IWDT reset/interrupt
    _0 = 0,
    ///1: Enable IWDT reset
    _1 = 1,
}
impl From<DBGSTOP_IWDT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_IWDT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_IWDT` reader - Mask bit for IWDT reset/interrupt
pub type DBGSTOP_IWDT_R = crate::BitReader<DBGSTOP_IWDT_A>;
impl DBGSTOP_IWDT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGSTOP_IWDT_A {
        match self.bits {
            false => DBGSTOP_IWDT_A::_0,
            true => DBGSTOP_IWDT_A::_1,
        }
    }
    ///Mask IWDT reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_IWDT_A::_0
    }
    ///Enable IWDT reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_IWDT_A::_1
    }
}
///Field `DBGSTOP_IWDT` writer - Mask bit for IWDT reset/interrupt
pub type DBGSTOP_IWDT_W<'a, REG> = crate::BitWriter<'a, REG, DBGSTOP_IWDT_A>;
impl<'a, REG> DBGSTOP_IWDT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mask IWDT reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_IWDT_A::_0)
    }
    ///Enable IWDT reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_IWDT_A::_1)
    }
}
/**Mask bit for WDT reset/interrupt

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_WDT_A {
    ///0: Mask WDT reset/interrupt
    _0 = 0,
    ///1: Enable WDT reset
    _1 = 1,
}
impl From<DBGSTOP_WDT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_WDT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_WDT` reader - Mask bit for WDT reset/interrupt
pub type DBGSTOP_WDT_R = crate::BitReader<DBGSTOP_WDT_A>;
impl DBGSTOP_WDT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGSTOP_WDT_A {
        match self.bits {
            false => DBGSTOP_WDT_A::_0,
            true => DBGSTOP_WDT_A::_1,
        }
    }
    ///Mask WDT reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_WDT_A::_0
    }
    ///Enable WDT reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_WDT_A::_1
    }
}
///Field `DBGSTOP_WDT` writer - Mask bit for WDT reset/interrupt
pub type DBGSTOP_WDT_W<'a, REG> = crate::BitWriter<'a, REG, DBGSTOP_WDT_A>;
impl<'a, REG> DBGSTOP_WDT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mask WDT reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_WDT_A::_0)
    }
    ///Enable WDT reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_WDT_A::_1)
    }
}
///Field `DBGSTOP_LVD` reader - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask)b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask)b16: Mask bit for LVD0 reset (0:enable / 1:Mask)
pub type DBGSTOP_LVD_R = crate::FieldReader;
///Field `DBGSTOP_LVD` writer - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask)b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask)b16: Mask bit for LVD0 reset (0:enable / 1:Mask)
pub type DBGSTOP_LVD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Mask bit for RAM parity error reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_RPER_A {
    ///0: Enable RAM parity error reset/interrupt
    _0 = 0,
    ///1: Mask RAM parity error reset/interrupt
    _1 = 1,
}
impl From<DBGSTOP_RPER_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_RPER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_RPER` reader - Mask bit for RAM parity error reset/interrupt
pub type DBGSTOP_RPER_R = crate::BitReader<DBGSTOP_RPER_A>;
impl DBGSTOP_RPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGSTOP_RPER_A {
        match self.bits {
            false => DBGSTOP_RPER_A::_0,
            true => DBGSTOP_RPER_A::_1,
        }
    }
    ///Enable RAM parity error reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_RPER_A::_0
    }
    ///Mask RAM parity error reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_RPER_A::_1
    }
}
///Field `DBGSTOP_RPER` writer - Mask bit for RAM parity error reset/interrupt
pub type DBGSTOP_RPER_W<'a, REG> = crate::BitWriter<'a, REG, DBGSTOP_RPER_A>;
impl<'a, REG> DBGSTOP_RPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable RAM parity error reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_RPER_A::_0)
    }
    ///Mask RAM parity error reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_RPER_A::_1)
    }
}
/**Mask bit for RAM ECC error reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_RECCR_A {
    ///0: Enable RAM ECC error reset/interrupt
    _0 = 0,
    ///1: Mask RAM ECC error reset/interrupt
    _1 = 1,
}
impl From<DBGSTOP_RECCR_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_RECCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_RECCR` reader - Mask bit for RAM ECC error reset/interrupt
pub type DBGSTOP_RECCR_R = crate::BitReader<DBGSTOP_RECCR_A>;
impl DBGSTOP_RECCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGSTOP_RECCR_A {
        match self.bits {
            false => DBGSTOP_RECCR_A::_0,
            true => DBGSTOP_RECCR_A::_1,
        }
    }
    ///Enable RAM ECC error reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_RECCR_A::_0
    }
    ///Mask RAM ECC error reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_RECCR_A::_1
    }
}
///Field `DBGSTOP_RECCR` writer - Mask bit for RAM ECC error reset/interrupt
pub type DBGSTOP_RECCR_W<'a, REG> = crate::BitWriter<'a, REG, DBGSTOP_RECCR_A>;
impl<'a, REG> DBGSTOP_RECCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable RAM ECC error reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_RECCR_A::_0)
    }
    ///Mask RAM ECC error reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSTOP_RECCR_A::_1)
    }
}
impl R {
    ///Bit 0 - Mask bit for IWDT reset/interrupt
    #[inline(always)]
    pub fn dbgstop_iwdt(&self) -> DBGSTOP_IWDT_R {
        DBGSTOP_IWDT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mask bit for WDT reset/interrupt
    #[inline(always)]
    pub fn dbgstop_wdt(&self) -> DBGSTOP_WDT_R {
        DBGSTOP_WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:18 - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask)b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask)b16: Mask bit for LVD0 reset (0:enable / 1:Mask)
    #[inline(always)]
    pub fn dbgstop_lvd(&self) -> DBGSTOP_LVD_R {
        DBGSTOP_LVD_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 24 - Mask bit for RAM parity error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_rper(&self) -> DBGSTOP_RPER_R {
        DBGSTOP_RPER_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Mask bit for RAM ECC error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_reccr(&self) -> DBGSTOP_RECCR_R {
        DBGSTOP_RECCR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Mask bit for IWDT reset/interrupt
    #[inline(always)]
    pub fn dbgstop_iwdt(&mut self) -> DBGSTOP_IWDT_W<DBGSTOPCR_SPEC> {
        DBGSTOP_IWDT_W::new(self, 0)
    }
    ///Bit 1 - Mask bit for WDT reset/interrupt
    #[inline(always)]
    pub fn dbgstop_wdt(&mut self) -> DBGSTOP_WDT_W<DBGSTOPCR_SPEC> {
        DBGSTOP_WDT_W::new(self, 1)
    }
    ///Bits 16:18 - b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask)b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask)b16: Mask bit for LVD0 reset (0:enable / 1:Mask)
    #[inline(always)]
    pub fn dbgstop_lvd(&mut self) -> DBGSTOP_LVD_W<DBGSTOPCR_SPEC> {
        DBGSTOP_LVD_W::new(self, 16)
    }
    ///Bit 24 - Mask bit for RAM parity error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_rper(&mut self) -> DBGSTOP_RPER_W<DBGSTOPCR_SPEC> {
        DBGSTOP_RPER_W::new(self, 24)
    }
    ///Bit 25 - Mask bit for RAM ECC error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_reccr(&mut self) -> DBGSTOP_RECCR_W<DBGSTOPCR_SPEC> {
        DBGSTOP_RECCR_W::new(self, 25)
    }
}
/**Debug Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBGSTOPCR_SPEC;
impl crate::RegisterSpec for DBGSTOPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbgstopcr::R`](R) reader structure
impl crate::Readable for DBGSTOPCR_SPEC {}
///`write(|w| ..)` method takes [`dbgstopcr::W`](W) writer structure
impl crate::Writable for DBGSTOPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSTOPCR to value 0x03
impl crate::Resettable for DBGSTOPCR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
