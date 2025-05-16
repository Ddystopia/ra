///Register `DVSTCTR0` reader
pub type R = crate::R<DVSTCTR0_SPEC>;
///Register `DVSTCTR0` writer
pub type W = crate::W<DVSTCTR0_SPEC>;
/**USB Bus Reset Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RHST_A {
    ///0: Communication speed not determined
    _000 = 0,
    ///1: Low-speed connection(When the host controller function is selected) /USB bus reset in progress or low-speed connection(When the function controller function is selected)
    _001 = 1,
    ///2: Full-speed connection(When the host controller function is selected) /USB bus reset in progress or full-speed connection(When the function controller function is selected)
    _010 = 2,
    ///3: Setting prohibited
    _011 = 3,
    ///4: USB bus reset in progress(When the host controller function is selected)
    OTHERS = 4,
}
impl From<RHST_A> for u8 {
    #[inline(always)]
    fn from(variant: RHST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RHST_A {
    type Ux = u8;
}
impl crate::IsEnum for RHST_A {}
///Field `RHST` reader - USB Bus Reset Status
pub type RHST_R = crate::FieldReader<RHST_A>;
impl RHST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RHST_A {
        match self.bits {
            0 => RHST_A::_000,
            1 => RHST_A::_001,
            2 => RHST_A::_010,
            3 => RHST_A::_011,
            _ => RHST_A::OTHERS,
        }
    }
    ///Communication speed not determined
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RHST_A::_000
    }
    ///Low-speed connection(When the host controller function is selected) /USB bus reset in progress or low-speed connection(When the function controller function is selected)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RHST_A::_001
    }
    ///Full-speed connection(When the host controller function is selected) /USB bus reset in progress or full-speed connection(When the function controller function is selected)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RHST_A::_010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RHST_A::_011
    }
    ///USB bus reset in progress(When the host controller function is selected)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RHST_A::OTHERS)
    }
}
/**USB Bus Operation Enable for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UACT_A {
    ///0: Disable downstream port (disable SOF or micro-SOF transmission)
    _0 = 0,
    ///1: Enable downstream port (enable SOF or micro-SOF transmission).
    _1 = 1,
}
impl From<UACT_A> for bool {
    #[inline(always)]
    fn from(variant: UACT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UACT` reader - USB Bus Operation Enable for the Host Controller Operation
pub type UACT_R = crate::BitReader<UACT_A>;
impl UACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UACT_A {
        match self.bits {
            false => UACT_A::_0,
            true => UACT_A::_1,
        }
    }
    ///Disable downstream port (disable SOF or micro-SOF transmission)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UACT_A::_0
    }
    ///Enable downstream port (enable SOF or micro-SOF transmission).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UACT_A::_1
    }
}
///Field `UACT` writer - USB Bus Operation Enable for the Host Controller Operation
pub type UACT_W<'a, REG> = crate::BitWriter<'a, REG, UACT_A>;
impl<'a, REG> UACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable downstream port (disable SOF or micro-SOF transmission)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UACT_A::_0)
    }
    ///Enable downstream port (enable SOF or micro-SOF transmission).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UACT_A::_1)
    }
}
/**Resume Signal Output for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME_A {
    ///0: Do not output resume signal
    _0 = 0,
    ///1: Output resume signal.
    _1 = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RESUME` reader - Resume Signal Output for the Host Controller Operation
pub type RESUME_R = crate::BitReader<RESUME_A>;
impl RESUME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESUME_A {
        match self.bits {
            false => RESUME_A::_0,
            true => RESUME_A::_1,
        }
    }
    ///Do not output resume signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESUME_A::_0
    }
    ///Output resume signal.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESUME_A::_1
    }
}
///Field `RESUME` writer - Resume Signal Output for the Host Controller Operation
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG, RESUME_A>;
impl<'a, REG> RESUME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output resume signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME_A::_0)
    }
    ///Output resume signal.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME_A::_1)
    }
}
/**USB Bus Reset Output for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST_A {
    ///0: Do not output USB bus reset signal
    _0 = 0,
    ///1: Output USB bus reset signal.
    _1 = 1,
}
impl From<USBRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USBRST` reader - USB Bus Reset Output for the Host Controller Operation
pub type USBRST_R = crate::BitReader<USBRST_A>;
impl USBRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBRST_A {
        match self.bits {
            false => USBRST_A::_0,
            true => USBRST_A::_1,
        }
    }
    ///Do not output USB bus reset signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBRST_A::_0
    }
    ///Output USB bus reset signal.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBRST_A::_1
    }
}
///Field `USBRST` writer - USB Bus Reset Output for the Host Controller Operation
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG, USBRST_A>;
impl<'a, REG> USBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output USB bus reset signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::_0)
    }
    ///Output USB bus reset signal.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::_1)
    }
}
/**Remote Wakeup Detection Enable for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWUPE_A {
    ///0: Disable downstream port remote wakeup
    _0 = 0,
    ///1: Enable downstream port remote wakeup.
    _1 = 1,
}
impl From<RWUPE_A> for bool {
    #[inline(always)]
    fn from(variant: RWUPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RWUPE` reader - Remote Wakeup Detection Enable for the Host Controller Operation
pub type RWUPE_R = crate::BitReader<RWUPE_A>;
impl RWUPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWUPE_A {
        match self.bits {
            false => RWUPE_A::_0,
            true => RWUPE_A::_1,
        }
    }
    ///Disable downstream port remote wakeup
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWUPE_A::_0
    }
    ///Enable downstream port remote wakeup.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWUPE_A::_1
    }
}
///Field `RWUPE` writer - Remote Wakeup Detection Enable for the Host Controller Operation
pub type RWUPE_W<'a, REG> = crate::BitWriter<'a, REG, RWUPE_A>;
impl<'a, REG> RWUPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable downstream port remote wakeup
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RWUPE_A::_0)
    }
    ///Enable downstream port remote wakeup.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RWUPE_A::_1)
    }
}
/**Remote Wakeup Output for the Device Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_A {
    ///0: Do not output remote wakeup signal
    _0 = 0,
    ///1: Output remote wakeup signal.
    _1 = 1,
}
impl From<WKUP_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUP` reader - Remote Wakeup Output for the Device Controller Operation
pub type WKUP_R = crate::BitReader<WKUP_A>;
impl WKUP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WKUP_A {
        match self.bits {
            false => WKUP_A::_0,
            true => WKUP_A::_1,
        }
    }
    ///Do not output remote wakeup signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WKUP_A::_0
    }
    ///Output remote wakeup signal.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WKUP_A::_1
    }
}
///Field `WKUP` writer - Remote Wakeup Output for the Device Controller Operation
pub type WKUP_W<'a, REG> = crate::BitWriter<'a, REG, WKUP_A>;
impl<'a, REG> WKUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output remote wakeup signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WKUP_A::_0)
    }
    ///Output remote wakeup signal.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WKUP_A::_1)
    }
}
/**USBHS_VBUSEN Output Pin Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSEN_A {
    ///0: Output low on external USBHS_VBUSEN pin
    _0 = 0,
    ///1: Output high on external USBHS_VBUSEN pin.
    _1 = 1,
}
impl From<VBUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBUSEN` reader - USBHS_VBUSEN Output Pin Control
pub type VBUSEN_R = crate::BitReader<VBUSEN_A>;
impl VBUSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBUSEN_A {
        match self.bits {
            false => VBUSEN_A::_0,
            true => VBUSEN_A::_1,
        }
    }
    ///Output low on external USBHS_VBUSEN pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUSEN_A::_0
    }
    ///Output high on external USBHS_VBUSEN pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUSEN_A::_1
    }
}
///Field `VBUSEN` writer - USBHS_VBUSEN Output Pin Control
pub type VBUSEN_W<'a, REG> = crate::BitWriter<'a, REG, VBUSEN_A>;
impl<'a, REG> VBUSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on external USBHS_VBUSEN pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBUSEN_A::_0)
    }
    ///Output high on external USBHS_VBUSEN pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBUSEN_A::_1)
    }
}
/**USBHS_EXICEN Output Pin Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXICEN_A {
    ///0: Output low on external USBHS_EXICEN pin
    _0 = 0,
    ///1: Output high on external USBHS_EXICEN pin.
    _1 = 1,
}
impl From<EXICEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXICEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXICEN` reader - USBHS_EXICEN Output Pin Control
pub type EXICEN_R = crate::BitReader<EXICEN_A>;
impl EXICEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXICEN_A {
        match self.bits {
            false => EXICEN_A::_0,
            true => EXICEN_A::_1,
        }
    }
    ///Output low on external USBHS_EXICEN pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXICEN_A::_0
    }
    ///Output high on external USBHS_EXICEN pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXICEN_A::_1
    }
}
///Field `EXICEN` writer - USBHS_EXICEN Output Pin Control
pub type EXICEN_W<'a, REG> = crate::BitWriter<'a, REG, EXICEN_A>;
impl<'a, REG> EXICEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on external USBHS_EXICEN pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXICEN_A::_0)
    }
    ///Output high on external USBHS_EXICEN pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXICEN_A::_1)
    }
}
///Field `HNPBTOA` reader - Host Negotiation Protocol (HNP) Control Use this bit when switching from device B to device A in OTGmode. If the HNPBTOA bit is 1, the internal function controlremains in the Suspend state until the HNP processing endseven if SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set.
pub type HNPBTOA_R = crate::BitReader;
///Field `HNPBTOA` writer - Host Negotiation Protocol (HNP) Control Use this bit when switching from device B to device A in OTGmode. If the HNPBTOA bit is 1, the internal function controlremains in the Suspend state until the HNP processing endseven if SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set.
pub type HNPBTOA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - USB Bus Reset Status
    #[inline(always)]
    pub fn rhst(&self) -> RHST_R {
        RHST_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - USB Bus Operation Enable for the Host Controller Operation
    #[inline(always)]
    pub fn uact(&self) -> UACT_R {
        UACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Resume Signal Output for the Host Controller Operation
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - USB Bus Reset Output for the Host Controller Operation
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Remote Wakeup Detection Enable for the Host Controller Operation
    #[inline(always)]
    pub fn rwupe(&self) -> RWUPE_R {
        RWUPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Remote Wakeup Output for the Device Controller Operation
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - USBHS_VBUSEN Output Pin Control
    #[inline(always)]
    pub fn vbusen(&self) -> VBUSEN_R {
        VBUSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USBHS_EXICEN Output Pin Control
    #[inline(always)]
    pub fn exicen(&self) -> EXICEN_R {
        EXICEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Host Negotiation Protocol (HNP) Control Use this bit when switching from device B to device A in OTGmode. If the HNPBTOA bit is 1, the internal function controlremains in the Suspend state until the HNP processing endseven if SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set.
    #[inline(always)]
    pub fn hnpbtoa(&self) -> HNPBTOA_R {
        HNPBTOA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - USB Bus Operation Enable for the Host Controller Operation
    #[inline(always)]
    pub fn uact(&mut self) -> UACT_W<DVSTCTR0_SPEC> {
        UACT_W::new(self, 4)
    }
    ///Bit 5 - Resume Signal Output for the Host Controller Operation
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<DVSTCTR0_SPEC> {
        RESUME_W::new(self, 5)
    }
    ///Bit 6 - USB Bus Reset Output for the Host Controller Operation
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<DVSTCTR0_SPEC> {
        USBRST_W::new(self, 6)
    }
    ///Bit 7 - Remote Wakeup Detection Enable for the Host Controller Operation
    #[inline(always)]
    pub fn rwupe(&mut self) -> RWUPE_W<DVSTCTR0_SPEC> {
        RWUPE_W::new(self, 7)
    }
    ///Bit 8 - Remote Wakeup Output for the Device Controller Operation
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W<DVSTCTR0_SPEC> {
        WKUP_W::new(self, 8)
    }
    ///Bit 9 - USBHS_VBUSEN Output Pin Control
    #[inline(always)]
    pub fn vbusen(&mut self) -> VBUSEN_W<DVSTCTR0_SPEC> {
        VBUSEN_W::new(self, 9)
    }
    ///Bit 10 - USBHS_EXICEN Output Pin Control
    #[inline(always)]
    pub fn exicen(&mut self) -> EXICEN_W<DVSTCTR0_SPEC> {
        EXICEN_W::new(self, 10)
    }
    ///Bit 11 - Host Negotiation Protocol (HNP) Control Use this bit when switching from device B to device A in OTGmode. If the HNPBTOA bit is 1, the internal function controlremains in the Suspend state until the HNP processing endseven if SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set.
    #[inline(always)]
    pub fn hnpbtoa(&mut self) -> HNPBTOA_W<DVSTCTR0_SPEC> {
        HNPBTOA_W::new(self, 11)
    }
}
/**Device State Control Register 0

You can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DVSTCTR0_SPEC;
impl crate::RegisterSpec for DVSTCTR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dvstctr0::R`](R) reader structure
impl crate::Readable for DVSTCTR0_SPEC {}
///`write(|w| ..)` method takes [`dvstctr0::W`](W) writer structure
impl crate::Writable for DVSTCTR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVSTCTR0 to value 0
impl crate::Resettable for DVSTCTR0_SPEC {}
