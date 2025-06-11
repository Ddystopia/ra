///Register `USBMC` reader
pub type R = crate::R<USBMC_SPEC>;
///Register `USBMC` writer
pub type W = crate::W<USBMC_SPEC>;
/**USB Reference Power Supply Circuit On/Off Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDUSBE_A {
    ///0: USB reference power supply circuit off
    _0 = 0,
    ///1: USB reference power supply circuit on
    _1 = 1,
}
impl From<VDDUSBE_A> for bool {
    #[inline(always)]
    fn from(variant: VDDUSBE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDUSBE` reader - USB Reference Power Supply Circuit On/Off Control
pub type VDDUSBE_R = crate::BitReader<VDDUSBE_A>;
impl VDDUSBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDUSBE_A {
        match self.bits {
            false => VDDUSBE_A::_0,
            true => VDDUSBE_A::_1,
        }
    }
    ///USB reference power supply circuit off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDDUSBE_A::_0
    }
    ///USB reference power supply circuit on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDDUSBE_A::_1
    }
}
///Field `VDDUSBE` writer - USB Reference Power Supply Circuit On/Off Control
pub type VDDUSBE_W<'a, REG> = crate::BitWriter<'a, REG, VDDUSBE_A>;
impl<'a, REG> VDDUSBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB reference power supply circuit off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDDUSBE_A::_0)
    }
    ///USB reference power supply circuit on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDDUSBE_A::_1)
    }
}
/**USB Regulator On/Off Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDCEN_A {
    ///0: USB regulator off
    _0 = 0,
    ///1: USB regulator on
    _1 = 1,
}
impl From<VDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VDCEN` reader - USB Regulator On/Off Control
pub type VDCEN_R = crate::BitReader<VDCEN_A>;
impl VDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDCEN_A {
        match self.bits {
            false => VDCEN_A::_0,
            true => VDCEN_A::_1,
        }
    }
    ///USB regulator off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDCEN_A::_0
    }
    ///USB regulator on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDCEN_A::_1
    }
}
///Field `VDCEN` writer - USB Regulator On/Off Control
pub type VDCEN_W<'a, REG> = crate::BitWriter<'a, REG, VDCEN_A>;
impl<'a, REG> VDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB regulator off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDCEN_A::_0)
    }
    ///USB regulator on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDCEN_A::_1)
    }
}
impl R {
    ///Bit 0 - USB Reference Power Supply Circuit On/Off Control
    #[inline(always)]
    pub fn vddusbe(&self) -> VDDUSBE_R {
        VDDUSBE_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - USB Regulator On/Off Control
    #[inline(always)]
    pub fn vdcen(&self) -> VDCEN_R {
        VDCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USB Reference Power Supply Circuit On/Off Control
    #[inline(always)]
    pub fn vddusbe(&mut self) -> VDDUSBE_W<USBMC_SPEC> {
        VDDUSBE_W::new(self, 0)
    }
    ///Bit 7 - USB Regulator On/Off Control
    #[inline(always)]
    pub fn vdcen(&mut self) -> VDCEN_W<USBMC_SPEC> {
        VDCEN_W::new(self, 7)
    }
}
/**USB Module Control Register

You can [`read`](crate::Reg::read) this register and get [`usbmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBMC_SPEC;
impl crate::RegisterSpec for USBMC_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbmc::R`](R) reader structure
impl crate::Readable for USBMC_SPEC {}
///`write(|w| ..)` method takes [`usbmc::W`](W) writer structure
impl crate::Writable for USBMC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBMC to value 0x02
impl crate::Resettable for USBMC_SPEC {
    const RESET_VALUE: u16 = 0x02;
}
