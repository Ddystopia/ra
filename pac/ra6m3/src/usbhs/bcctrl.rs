///Register `BCCTRL` reader
pub type R = crate::R<BCCTRL_SPEC>;
///Register `BCCTRL` writer
pub type W = crate::W<BCCTRL_SPEC>;
/**IDPSRC Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSRCE_A {
    ///0: The IDP_SRC circuit is disabled. (Initial value)
    _0 = 0,
    ///1: The IDP_SRC circuit is enabled.
    _1 = 1,
}
impl From<IDPSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSRCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSRCE` reader - IDPSRC Control
pub type IDPSRCE_R = crate::BitReader<IDPSRCE_A>;
impl IDPSRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDPSRCE_A {
        match self.bits {
            false => IDPSRCE_A::_0,
            true => IDPSRCE_A::_1,
        }
    }
    ///The IDP_SRC circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSRCE_A::_0
    }
    ///The IDP_SRC circuit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSRCE_A::_1
    }
}
///Field `IDPSRCE` writer - IDPSRC Control
pub type IDPSRCE_W<'a, REG> = crate::BitWriter<'a, REG, IDPSRCE_A>;
impl<'a, REG> IDPSRCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The IDP_SRC circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSRCE_A::_0)
    }
    ///The IDP_SRC circuit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSRCE_A::_1)
    }
}
/**IDMSINK Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMSINKE_A {
    ///0: The IDM_SINK circuit is disabled. (Initial value)
    _0 = 0,
    ///1: The IDM_SINK circuit is enabled.
    _1 = 1,
}
impl From<IDMSINKE_A> for bool {
    #[inline(always)]
    fn from(variant: IDMSINKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDMSINKE` reader - IDMSINK Control
pub type IDMSINKE_R = crate::BitReader<IDMSINKE_A>;
impl IDMSINKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDMSINKE_A {
        match self.bits {
            false => IDMSINKE_A::_0,
            true => IDMSINKE_A::_1,
        }
    }
    ///The IDM_SINK circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDMSINKE_A::_0
    }
    ///The IDM_SINK circuit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDMSINKE_A::_1
    }
}
///Field `IDMSINKE` writer - IDMSINK Control
pub type IDMSINKE_W<'a, REG> = crate::BitWriter<'a, REG, IDMSINKE_A>;
impl<'a, REG> IDMSINKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The IDM_SINK circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDMSINKE_A::_0)
    }
    ///The IDM_SINK circuit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDMSINKE_A::_1)
    }
}
/**VDPSRC Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDPSRCE_A {
    ///0: The VDP_SRC circuit is disabled. (Initial value)
    _0 = 0,
    ///1: The VDP_SRC circuit is enabled.
    _1 = 1,
}
impl From<VDPSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: VDPSRCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VDPSRCE` reader - VDPSRC Control
pub type VDPSRCE_R = crate::BitReader<VDPSRCE_A>;
impl VDPSRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDPSRCE_A {
        match self.bits {
            false => VDPSRCE_A::_0,
            true => VDPSRCE_A::_1,
        }
    }
    ///The VDP_SRC circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDPSRCE_A::_0
    }
    ///The VDP_SRC circuit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDPSRCE_A::_1
    }
}
///Field `VDPSRCE` writer - VDPSRC Control
pub type VDPSRCE_W<'a, REG> = crate::BitWriter<'a, REG, VDPSRCE_A>;
impl<'a, REG> VDPSRCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The VDP_SRC circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDPSRCE_A::_0)
    }
    ///The VDP_SRC circuit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDPSRCE_A::_1)
    }
}
/**IDPSINK Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSINKE_A {
    ///0: The IDP_SINK circuit is disabled. (Initial value)
    _0 = 0,
    ///1: The IDP_SINK circuit is enabled.
    _1 = 1,
}
impl From<IDPSINKE_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSINKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSINKE` reader - IDPSINK Control
pub type IDPSINKE_R = crate::BitReader<IDPSINKE_A>;
impl IDPSINKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDPSINKE_A {
        match self.bits {
            false => IDPSINKE_A::_0,
            true => IDPSINKE_A::_1,
        }
    }
    ///The IDP_SINK circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSINKE_A::_0
    }
    ///The IDP_SINK circuit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSINKE_A::_1
    }
}
///Field `IDPSINKE` writer - IDPSINK Control
pub type IDPSINKE_W<'a, REG> = crate::BitWriter<'a, REG, IDPSINKE_A>;
impl<'a, REG> IDPSINKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The IDP_SINK circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSINKE_A::_0)
    }
    ///The IDP_SINK circuit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSINKE_A::_1)
    }
}
/**VDMSRC Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDMSRCE_A {
    ///0: The VDM_SRC circuit is disabled. (Initial value)
    _0 = 0,
    ///1: The VDM_SRC circuit is enabled.
    _1 = 1,
}
impl From<VDMSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: VDMSRCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VDMSRCE` reader - VDMSRC Control
pub type VDMSRCE_R = crate::BitReader<VDMSRCE_A>;
impl VDMSRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDMSRCE_A {
        match self.bits {
            false => VDMSRCE_A::_0,
            true => VDMSRCE_A::_1,
        }
    }
    ///The VDM_SRC circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDMSRCE_A::_0
    }
    ///The VDM_SRC circuit is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDMSRCE_A::_1
    }
}
///Field `VDMSRCE` writer - VDMSRC Control
pub type VDMSRCE_W<'a, REG> = crate::BitWriter<'a, REG, VDMSRCE_A>;
impl<'a, REG> VDMSRCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The VDM_SRC circuit is disabled. (Initial value)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDMSRCE_A::_0)
    }
    ///The VDM_SRC circuit is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDMSRCE_A::_1)
    }
}
/**DCP Mode Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCPMODE_A {
    ///0: The RDCP_DAT resistor is disabled
    _0 = 0,
    ///1: The RDCP_DAT resistor is enabled.
    _1 = 1,
}
impl From<DCPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCPMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DCPMODE` reader - DCP Mode Control
pub type DCPMODE_R = crate::BitReader<DCPMODE_A>;
impl DCPMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCPMODE_A {
        match self.bits {
            false => DCPMODE_A::_0,
            true => DCPMODE_A::_1,
        }
    }
    ///The RDCP_DAT resistor is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCPMODE_A::_0
    }
    ///The RDCP_DAT resistor is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCPMODE_A::_1
    }
}
///Field `DCPMODE` writer - DCP Mode Control
pub type DCPMODE_W<'a, REG> = crate::BitWriter<'a, REG, DCPMODE_A>;
impl<'a, REG> DCPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RDCP_DAT resistor is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCPMODE_A::_0)
    }
    ///The RDCP_DAT resistor is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCPMODE_A::_1)
    }
}
/**CHGDET Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHGDETSTS_A {
    ///0: The CHGDET pin is at low level.
    _0 = 0,
    ///1: The CHGDET pin is at high level.
    _1 = 1,
}
impl From<CHGDETSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CHGDETSTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHGDETSTS` reader - CHGDET Status
pub type CHGDETSTS_R = crate::BitReader<CHGDETSTS_A>;
impl CHGDETSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHGDETSTS_A {
        match self.bits {
            false => CHGDETSTS_A::_0,
            true => CHGDETSTS_A::_1,
        }
    }
    ///The CHGDET pin is at low level.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHGDETSTS_A::_0
    }
    ///The CHGDET pin is at high level.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHGDETSTS_A::_1
    }
}
/**PDDET Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETSTS_A {
    ///0: The PDDET pin is at low level.
    _0 = 0,
    ///1: The PDDET pin is at high level.
    _1 = 1,
}
impl From<PDDETSTS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETSTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETSTS` reader - PDDET Status
pub type PDDETSTS_R = crate::BitReader<PDDETSTS_A>;
impl PDDETSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDETSTS_A {
        match self.bits {
            false => PDDETSTS_A::_0,
            true => PDDETSTS_A::_1,
        }
    }
    ///The PDDET pin is at low level.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETSTS_A::_0
    }
    ///The PDDET pin is at high level.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETSTS_A::_1
    }
}
impl R {
    ///Bit 0 - IDPSRC Control
    #[inline(always)]
    pub fn idpsrce(&self) -> IDPSRCE_R {
        IDPSRCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IDMSINK Control
    #[inline(always)]
    pub fn idmsinke(&self) -> IDMSINKE_R {
        IDMSINKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VDPSRC Control
    #[inline(always)]
    pub fn vdpsrce(&self) -> VDPSRCE_R {
        VDPSRCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IDPSINK Control
    #[inline(always)]
    pub fn idpsinke(&self) -> IDPSINKE_R {
        IDPSINKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VDMSRC Control
    #[inline(always)]
    pub fn vdmsrce(&self) -> VDMSRCE_R {
        VDMSRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DCP Mode Control
    #[inline(always)]
    pub fn dcpmode(&self) -> DCPMODE_R {
        DCPMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - CHGDET Status
    #[inline(always)]
    pub fn chgdetsts(&self) -> CHGDETSTS_R {
        CHGDETSTS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PDDET Status
    #[inline(always)]
    pub fn pddetsts(&self) -> PDDETSTS_R {
        PDDETSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IDPSRC Control
    #[inline(always)]
    pub fn idpsrce(&mut self) -> IDPSRCE_W<BCCTRL_SPEC> {
        IDPSRCE_W::new(self, 0)
    }
    ///Bit 1 - IDMSINK Control
    #[inline(always)]
    pub fn idmsinke(&mut self) -> IDMSINKE_W<BCCTRL_SPEC> {
        IDMSINKE_W::new(self, 1)
    }
    ///Bit 2 - VDPSRC Control
    #[inline(always)]
    pub fn vdpsrce(&mut self) -> VDPSRCE_W<BCCTRL_SPEC> {
        VDPSRCE_W::new(self, 2)
    }
    ///Bit 3 - IDPSINK Control
    #[inline(always)]
    pub fn idpsinke(&mut self) -> IDPSINKE_W<BCCTRL_SPEC> {
        IDPSINKE_W::new(self, 3)
    }
    ///Bit 4 - VDMSRC Control
    #[inline(always)]
    pub fn vdmsrce(&mut self) -> VDMSRCE_W<BCCTRL_SPEC> {
        VDMSRCE_W::new(self, 4)
    }
    ///Bit 5 - DCP Mode Control
    #[inline(always)]
    pub fn dcpmode(&mut self) -> DCPMODE_W<BCCTRL_SPEC> {
        DCPMODE_W::new(self, 5)
    }
}
/**Battery Charging Control Register

You can [`read`](crate::Reg::read) this register and get [`bcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCCTRL_SPEC;
impl crate::RegisterSpec for BCCTRL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`bcctrl::R`](R) reader structure
impl crate::Readable for BCCTRL_SPEC {}
///`write(|w| ..)` method takes [`bcctrl::W`](W) writer structure
impl crate::Writable for BCCTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCTRL to value 0
impl crate::Resettable for BCCTRL_SPEC {}
