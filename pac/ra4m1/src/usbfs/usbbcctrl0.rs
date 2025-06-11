///Register `USBBCCTRL0` reader
pub type R = crate::R<USBBCCTRL0_SPEC>;
///Register `USBBCCTRL0` writer
pub type W = crate::W<USBBCCTRL0_SPEC>;
/**D- Pin Pull-Down Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPDME0_A {
    ///0: Pull-down off
    _0 = 0,
    ///1: Pull-down on
    _1 = 1,
}
impl From<RPDME0_A> for bool {
    #[inline(always)]
    fn from(variant: RPDME0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPDME0` reader - D- Pin Pull-Down Control
pub type RPDME0_R = crate::BitReader<RPDME0_A>;
impl RPDME0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPDME0_A {
        match self.bits {
            false => RPDME0_A::_0,
            true => RPDME0_A::_1,
        }
    }
    ///Pull-down off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPDME0_A::_0
    }
    ///Pull-down on
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPDME0_A::_1
    }
}
///Field `RPDME0` writer - D- Pin Pull-Down Control
pub type RPDME0_W<'a, REG> = crate::BitWriter<'a, REG, RPDME0_A>;
impl<'a, REG> RPDME0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pull-down off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPDME0_A::_0)
    }
    ///Pull-down on
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPDME0_A::_1)
    }
}
/**D+ Pin IDPSRC Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSRCE0_A {
    ///0: Stop
    _0 = 0,
    ///1: 10uA output
    _1 = 1,
}
impl From<IDPSRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSRCE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSRCE0` reader - D+ Pin IDPSRC Output Control
pub type IDPSRCE0_R = crate::BitReader<IDPSRCE0_A>;
impl IDPSRCE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDPSRCE0_A {
        match self.bits {
            false => IDPSRCE0_A::_0,
            true => IDPSRCE0_A::_1,
        }
    }
    ///Stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSRCE0_A::_0
    }
    ///10uA output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSRCE0_A::_1
    }
}
///Field `IDPSRCE0` writer - D+ Pin IDPSRC Output Control
pub type IDPSRCE0_W<'a, REG> = crate::BitWriter<'a, REG, IDPSRCE0_A>;
impl<'a, REG> IDPSRCE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSRCE0_A::_0)
    }
    ///10uA output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSRCE0_A::_1)
    }
}
/**D- Pin 0.6 V Input Detection (Comparator and Sink) Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDMSINKE0_A {
    ///0: Detection off
    _0 = 0,
    ///1: Detection on ( Comparator and sink current on )
    _1 = 1,
}
impl From<IDMSINKE0_A> for bool {
    #[inline(always)]
    fn from(variant: IDMSINKE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDMSINKE0` reader - D- Pin 0.6 V Input Detection (Comparator and Sink) Control
pub type IDMSINKE0_R = crate::BitReader<IDMSINKE0_A>;
impl IDMSINKE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDMSINKE0_A {
        match self.bits {
            false => IDMSINKE0_A::_0,
            true => IDMSINKE0_A::_1,
        }
    }
    ///Detection off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDMSINKE0_A::_0
    }
    ///Detection on ( Comparator and sink current on )
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDMSINKE0_A::_1
    }
}
///Field `IDMSINKE0` writer - D- Pin 0.6 V Input Detection (Comparator and Sink) Control
pub type IDMSINKE0_W<'a, REG> = crate::BitWriter<'a, REG, IDMSINKE0_A>;
impl<'a, REG> IDMSINKE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDMSINKE0_A::_0)
    }
    ///Detection on ( Comparator and sink current on )
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDMSINKE0_A::_1)
    }
}
/**D+ Pin VDPSRC (0.6 V) Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDPSRCE0_A {
    ///0: Stop
    _0 = 0,
    ///1: 0.6V output
    _1 = 1,
}
impl From<VDPSRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: VDPSRCE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VDPSRCE0` reader - D+ Pin VDPSRC (0.6 V) Output Control
pub type VDPSRCE0_R = crate::BitReader<VDPSRCE0_A>;
impl VDPSRCE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDPSRCE0_A {
        match self.bits {
            false => VDPSRCE0_A::_0,
            true => VDPSRCE0_A::_1,
        }
    }
    ///Stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDPSRCE0_A::_0
    }
    ///0.6V output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDPSRCE0_A::_1
    }
}
///Field `VDPSRCE0` writer - D+ Pin VDPSRC (0.6 V) Output Control
pub type VDPSRCE0_W<'a, REG> = crate::BitWriter<'a, REG, VDPSRCE0_A>;
impl<'a, REG> VDPSRCE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDPSRCE0_A::_0)
    }
    ///0.6V output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDPSRCE0_A::_1)
    }
}
/**D+ Pin 0.6 V Input Detection (Comparator and Sink) Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSINKE0_A {
    ///0: Detection off
    _0 = 0,
    ///1: Detection on ( Comparator and sink current on )
    _1 = 1,
}
impl From<IDPSINKE0_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSINKE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSINKE0` reader - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control
pub type IDPSINKE0_R = crate::BitReader<IDPSINKE0_A>;
impl IDPSINKE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDPSINKE0_A {
        match self.bits {
            false => IDPSINKE0_A::_0,
            true => IDPSINKE0_A::_1,
        }
    }
    ///Detection off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSINKE0_A::_0
    }
    ///Detection on ( Comparator and sink current on )
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSINKE0_A::_1
    }
}
///Field `IDPSINKE0` writer - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control
pub type IDPSINKE0_W<'a, REG> = crate::BitWriter<'a, REG, IDPSINKE0_A>;
impl<'a, REG> IDPSINKE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSINKE0_A::_0)
    }
    ///Detection on ( Comparator and sink current on )
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDPSINKE0_A::_1)
    }
}
/**D- Pin VDMSRC (0.6 V) Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDMSRCE0_A {
    ///0: Stop
    _0 = 0,
    ///1: 0.6V output
    _1 = 1,
}
impl From<VDMSRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: VDMSRCE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VDMSRCE0` reader - D- Pin VDMSRC (0.6 V) Output Control
pub type VDMSRCE0_R = crate::BitReader<VDMSRCE0_A>;
impl VDMSRCE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDMSRCE0_A {
        match self.bits {
            false => VDMSRCE0_A::_0,
            true => VDMSRCE0_A::_1,
        }
    }
    ///Stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDMSRCE0_A::_0
    }
    ///0.6V output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDMSRCE0_A::_1
    }
}
///Field `VDMSRCE0` writer - D- Pin VDMSRC (0.6 V) Output Control
pub type VDMSRCE0_W<'a, REG> = crate::BitWriter<'a, REG, VDMSRCE0_A>;
impl<'a, REG> VDMSRCE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VDMSRCE0_A::_0)
    }
    ///0.6V output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VDMSRCE0_A::_1)
    }
}
/**BC (Battery Charger) Function Ch0 General Enable Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BATCHGE0_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<BATCHGE0_A> for bool {
    #[inline(always)]
    fn from(variant: BATCHGE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BATCHGE0` reader - BC (Battery Charger) Function Ch0 General Enable Control
pub type BATCHGE0_R = crate::BitReader<BATCHGE0_A>;
impl BATCHGE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BATCHGE0_A {
        match self.bits {
            false => BATCHGE0_A::_0,
            true => BATCHGE0_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BATCHGE0_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BATCHGE0_A::_1
    }
}
///Field `BATCHGE0` writer - BC (Battery Charger) Function Ch0 General Enable Control
pub type BATCHGE0_W<'a, REG> = crate::BitWriter<'a, REG, BATCHGE0_A>;
impl<'a, REG> BATCHGE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BATCHGE0_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BATCHGE0_A::_1)
    }
}
/**D- Pin 0.6 V Input Detection Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHGDETSTS0_A {
    ///0: Not detected
    _0 = 0,
    ///1: Detected
    _1 = 1,
}
impl From<CHGDETSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: CHGDETSTS0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHGDETSTS0` reader - D- Pin 0.6 V Input Detection Status
pub type CHGDETSTS0_R = crate::BitReader<CHGDETSTS0_A>;
impl CHGDETSTS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHGDETSTS0_A {
        match self.bits {
            false => CHGDETSTS0_A::_0,
            true => CHGDETSTS0_A::_1,
        }
    }
    ///Not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHGDETSTS0_A::_0
    }
    ///Detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHGDETSTS0_A::_1
    }
}
/**D+ Pin 0.6 V Input Detection Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETSTS0_A {
    ///0: Not detected
    _0 = 0,
    ///1: Detected
    _1 = 1,
}
impl From<PDDETSTS0_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETSTS0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETSTS0` reader - D+ Pin 0.6 V Input Detection Status
pub type PDDETSTS0_R = crate::BitReader<PDDETSTS0_A>;
impl PDDETSTS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDETSTS0_A {
        match self.bits {
            false => PDDETSTS0_A::_0,
            true => PDDETSTS0_A::_1,
        }
    }
    ///Not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETSTS0_A::_0
    }
    ///Detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETSTS0_A::_1
    }
}
impl R {
    ///Bit 0 - D- Pin Pull-Down Control
    #[inline(always)]
    pub fn rpdme0(&self) -> RPDME0_R {
        RPDME0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - D+ Pin IDPSRC Output Control
    #[inline(always)]
    pub fn idpsrce0(&self) -> IDPSRCE0_R {
        IDPSRCE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - D- Pin 0.6 V Input Detection (Comparator and Sink) Control
    #[inline(always)]
    pub fn idmsinke0(&self) -> IDMSINKE0_R {
        IDMSINKE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - D+ Pin VDPSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdpsrce0(&self) -> VDPSRCE0_R {
        VDPSRCE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control
    #[inline(always)]
    pub fn idpsinke0(&self) -> IDPSINKE0_R {
        IDPSINKE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - D- Pin VDMSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdmsrce0(&self) -> VDMSRCE0_R {
        VDMSRCE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - BC (Battery Charger) Function Ch0 General Enable Control
    #[inline(always)]
    pub fn batchge0(&self) -> BATCHGE0_R {
        BATCHGE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - D- Pin 0.6 V Input Detection Status
    #[inline(always)]
    pub fn chgdetsts0(&self) -> CHGDETSTS0_R {
        CHGDETSTS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - D+ Pin 0.6 V Input Detection Status
    #[inline(always)]
    pub fn pddetsts0(&self) -> PDDETSTS0_R {
        PDDETSTS0_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - D- Pin Pull-Down Control
    #[inline(always)]
    pub fn rpdme0(&mut self) -> RPDME0_W<USBBCCTRL0_SPEC> {
        RPDME0_W::new(self, 0)
    }
    ///Bit 1 - D+ Pin IDPSRC Output Control
    #[inline(always)]
    pub fn idpsrce0(&mut self) -> IDPSRCE0_W<USBBCCTRL0_SPEC> {
        IDPSRCE0_W::new(self, 1)
    }
    ///Bit 2 - D- Pin 0.6 V Input Detection (Comparator and Sink) Control
    #[inline(always)]
    pub fn idmsinke0(&mut self) -> IDMSINKE0_W<USBBCCTRL0_SPEC> {
        IDMSINKE0_W::new(self, 2)
    }
    ///Bit 3 - D+ Pin VDPSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdpsrce0(&mut self) -> VDPSRCE0_W<USBBCCTRL0_SPEC> {
        VDPSRCE0_W::new(self, 3)
    }
    ///Bit 4 - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control
    #[inline(always)]
    pub fn idpsinke0(&mut self) -> IDPSINKE0_W<USBBCCTRL0_SPEC> {
        IDPSINKE0_W::new(self, 4)
    }
    ///Bit 5 - D- Pin VDMSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdmsrce0(&mut self) -> VDMSRCE0_W<USBBCCTRL0_SPEC> {
        VDMSRCE0_W::new(self, 5)
    }
    ///Bit 7 - BC (Battery Charger) Function Ch0 General Enable Control
    #[inline(always)]
    pub fn batchge0(&mut self) -> BATCHGE0_W<USBBCCTRL0_SPEC> {
        BATCHGE0_W::new(self, 7)
    }
}
/**BC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`usbbcctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbbcctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBBCCTRL0_SPEC;
impl crate::RegisterSpec for USBBCCTRL0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbbcctrl0::R`](R) reader structure
impl crate::Readable for USBBCCTRL0_SPEC {}
///`write(|w| ..)` method takes [`usbbcctrl0::W`](W) writer structure
impl crate::Writable for USBBCCTRL0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBBCCTRL0 to value 0
impl crate::Resettable for USBBCCTRL0_SPEC {}
