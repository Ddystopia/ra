///Register `SDIO_INFO1_MASK` reader
pub type R = crate::R<SDIO_INFO1_MASK_SPEC>;
///Register `SDIO_INFO1_MASK` writer
pub type W = crate::W<SDIO_INFO1_MASK_SPEC>;
/**IOIRQ Interrupt Mask Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOIRQM_A {
    ///0: IOIRQ interrupt not masked
    _0 = 0,
    ///1: IOIRQ interrupt masked
    _1 = 1,
}
impl From<IOIRQM_A> for bool {
    #[inline(always)]
    fn from(variant: IOIRQM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IOIRQM` reader - IOIRQ Interrupt Mask Control
pub type IOIRQM_R = crate::BitReader<IOIRQM_A>;
impl IOIRQM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOIRQM_A {
        match self.bits {
            false => IOIRQM_A::_0,
            true => IOIRQM_A::_1,
        }
    }
    ///IOIRQ interrupt not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOIRQM_A::_0
    }
    ///IOIRQ interrupt masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOIRQM_A::_1
    }
}
///Field `IOIRQM` writer - IOIRQ Interrupt Mask Control
pub type IOIRQM_W<'a, REG> = crate::BitWriter<'a, REG, IOIRQM_A>;
impl<'a, REG> IOIRQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IOIRQ interrupt not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOIRQM_A::_0)
    }
    ///IOIRQ interrupt masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOIRQM_A::_1)
    }
}
/**EXPUB52 Interrupt Request Mask Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXPUB52M_A {
    ///0: EXPUB52 interrupt request not masked
    _0 = 0,
    ///1: EXPUB52 interrupt request masked
    _1 = 1,
}
impl From<EXPUB52M_A> for bool {
    #[inline(always)]
    fn from(variant: EXPUB52M_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXPUB52M` reader - EXPUB52 Interrupt Request Mask Control
pub type EXPUB52M_R = crate::BitReader<EXPUB52M_A>;
impl EXPUB52M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXPUB52M_A {
        match self.bits {
            false => EXPUB52M_A::_0,
            true => EXPUB52M_A::_1,
        }
    }
    ///EXPUB52 interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXPUB52M_A::_0
    }
    ///EXPUB52 interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXPUB52M_A::_1
    }
}
///Field `EXPUB52M` writer - EXPUB52 Interrupt Request Mask Control
pub type EXPUB52M_W<'a, REG> = crate::BitWriter<'a, REG, EXPUB52M_A>;
impl<'a, REG> EXPUB52M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EXPUB52 interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXPUB52M_A::_0)
    }
    ///EXPUB52 interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXPUB52M_A::_1)
    }
}
/**EXWT Interrupt Request Mask Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXWTM_A {
    ///0: EXWT interrupt request not masked
    _0 = 0,
    ///1: EXWT interrupt request masked
    _1 = 1,
}
impl From<EXWTM_A> for bool {
    #[inline(always)]
    fn from(variant: EXWTM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXWTM` reader - EXWT Interrupt Request Mask Control
pub type EXWTM_R = crate::BitReader<EXWTM_A>;
impl EXWTM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXWTM_A {
        match self.bits {
            false => EXWTM_A::_0,
            true => EXWTM_A::_1,
        }
    }
    ///EXWT interrupt request not masked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXWTM_A::_0
    }
    ///EXWT interrupt request masked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXWTM_A::_1
    }
}
///Field `EXWTM` writer - EXWT Interrupt Request Mask Control
pub type EXWTM_W<'a, REG> = crate::BitWriter<'a, REG, EXWTM_A>;
impl<'a, REG> EXWTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EXWT interrupt request not masked
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXWTM_A::_0)
    }
    ///EXWT interrupt request masked
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXWTM_A::_1)
    }
}
impl R {
    ///Bit 0 - IOIRQ Interrupt Mask Control
    #[inline(always)]
    pub fn ioirqm(&self) -> IOIRQM_R {
        IOIRQM_R::new((self.bits & 1) != 0)
    }
    ///Bit 14 - EXPUB52 Interrupt Request Mask Control
    #[inline(always)]
    pub fn expub52m(&self) -> EXPUB52M_R {
        EXPUB52M_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EXWT Interrupt Request Mask Control
    #[inline(always)]
    pub fn exwtm(&self) -> EXWTM_R {
        EXWTM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IOIRQ Interrupt Mask Control
    #[inline(always)]
    pub fn ioirqm(&mut self) -> IOIRQM_W<SDIO_INFO1_MASK_SPEC> {
        IOIRQM_W::new(self, 0)
    }
    ///Bit 14 - EXPUB52 Interrupt Request Mask Control
    #[inline(always)]
    pub fn expub52m(&mut self) -> EXPUB52M_W<SDIO_INFO1_MASK_SPEC> {
        EXPUB52M_W::new(self, 14)
    }
    ///Bit 15 - EXWT Interrupt Request Mask Control
    #[inline(always)]
    pub fn exwtm(&mut self) -> EXWTM_W<SDIO_INFO1_MASK_SPEC> {
        EXWTM_W::new(self, 15)
    }
}
/**SDIO_INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sdio_info1_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_INFO1_MASK_SPEC;
impl crate::RegisterSpec for SDIO_INFO1_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_info1_mask::R`](R) reader structure
impl crate::Readable for SDIO_INFO1_MASK_SPEC {}
///`write(|w| ..)` method takes [`sdio_info1_mask::W`](W) writer structure
impl crate::Writable for SDIO_INFO1_MASK_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIO_INFO1_MASK to value 0xc007
impl crate::Resettable for SDIO_INFO1_MASK_SPEC {
    const RESET_VALUE: u32 = 0xc007;
}
