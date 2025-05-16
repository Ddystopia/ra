///Register `FFLTR` reader
pub type R = crate::R<FFLTR_SPEC>;
///Register `FFLTR` writer
pub type W = crate::W<FFLTR_SPEC>;
/**Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    ///0: Only receive multicast frames matching the MAC address setting in FMAC0R(U/L).
    _0 = 0,
    ///1: Only receive multicast frames matching the MAC address setting in FMAC1R(U/L).
    _1 = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEL` reader - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1.
pub type SEL_R = crate::BitReader<SEL_A>;
impl SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::_0,
            true => SEL_A::_1,
        }
    }
    ///Only receive multicast frames matching the MAC address setting in FMAC0R(U/L).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEL_A::_0
    }
    ///Only receive multicast frames matching the MAC address setting in FMAC1R(U/L).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEL_A::_1
    }
}
///Field `SEL` writer - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1.
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG, SEL_A>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Only receive multicast frames matching the MAC address setting in FMAC0R(U/L).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_0)
    }
    ///Only receive multicast frames matching the MAC address setting in FMAC1R(U/L).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_1)
    }
}
/**Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_A {
    ///0: Do not receive multicast frames.
    _0 = 0,
    ///1: See SEL bit.
    _1 = 1,
}
impl From<PRT_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRT` reader - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1.
pub type PRT_R = crate::BitReader<PRT_A>;
impl PRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRT_A {
        match self.bits {
            false => PRT_A::_0,
            true => PRT_A::_1,
        }
    }
    ///Do not receive multicast frames.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRT_A::_0
    }
    ///See SEL bit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRT_A::_1
    }
}
///Field `PRT` writer - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1.
pub type PRT_W<'a, REG> = crate::BitWriter<'a, REG, PRT_A>;
impl<'a, REG> PRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not receive multicast frames.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_A::_0)
    }
    ///See SEL bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRT_A::_1)
    }
}
/**Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: Filtering is disabled (all multicast frames are received).
    _0 = 0,
    ///1: See PRT and SEL bit.
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENB` reader - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0.
pub type ENB_R = crate::BitReader<ENB_A>;
impl ENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    ///Filtering is disabled (all multicast frames are received).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///See PRT and SEL bit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
///Field `ENB` writer - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0.
pub type ENB_W<'a, REG> = crate::BitWriter<'a, REG, ENB_A>;
impl<'a, REG> ENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Filtering is disabled (all multicast frames are received).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///See PRT and SEL bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
/**Extended Promiscuous ModeSetting

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTPRM_A {
    ///0: Normal operation (unicast frames addressed to the EPTPC are received, filtering of PTP frames is applied, multicast filtering is applied, and all broadcast frames are received).
    _0 = 0,
    ///1: Extended promiscuous mode (all frames are received)
    _1 = 1,
}
impl From<EXTPRM_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPRM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTPRM` reader - Extended Promiscuous ModeSetting
pub type EXTPRM_R = crate::BitReader<EXTPRM_A>;
impl EXTPRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTPRM_A {
        match self.bits {
            false => EXTPRM_A::_0,
            true => EXTPRM_A::_1,
        }
    }
    ///Normal operation (unicast frames addressed to the EPTPC are received, filtering of PTP frames is applied, multicast filtering is applied, and all broadcast frames are received).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTPRM_A::_0
    }
    ///Extended promiscuous mode (all frames are received)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTPRM_A::_1
    }
}
///Field `EXTPRM` writer - Extended Promiscuous ModeSetting
pub type EXTPRM_W<'a, REG> = crate::BitWriter<'a, REG, EXTPRM_A>;
impl<'a, REG> EXTPRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation (unicast frames addressed to the EPTPC are received, filtering of PTP frames is applied, multicast filtering is applied, and all broadcast frames are received).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPRM_A::_0)
    }
    ///Extended promiscuous mode (all frames are received)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPRM_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1.
    #[inline(always)]
    pub fn prt(&self) -> PRT_R {
        PRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0.
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Extended Promiscuous ModeSetting
    #[inline(always)]
    pub fn extprm(&self) -> EXTPRM_R {
        EXTPRM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1.
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<FFLTR_SPEC> {
        SEL_W::new(self, 0)
    }
    ///Bit 1 - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1.
    #[inline(always)]
    pub fn prt(&mut self) -> PRT_W<FFLTR_SPEC> {
        PRT_W::new(self, 1)
    }
    ///Bit 2 - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0.
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<FFLTR_SPEC> {
        ENB_W::new(self, 2)
    }
    ///Bit 16 - Extended Promiscuous ModeSetting
    #[inline(always)]
    pub fn extprm(&mut self) -> EXTPRM_W<FFLTR_SPEC> {
        EXTPRM_W::new(self, 16)
    }
}
/**Frame Reception Filter Setting Register

You can [`read`](crate::Reg::read) this register and get [`ffltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FFLTR_SPEC;
impl crate::RegisterSpec for FFLTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ffltr::R`](R) reader structure
impl crate::Readable for FFLTR_SPEC {}
///`write(|w| ..)` method takes [`ffltr::W`](W) writer structure
impl crate::Writable for FFLTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FFLTR to value 0x0001_0000
impl crate::Resettable for FFLTR_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
