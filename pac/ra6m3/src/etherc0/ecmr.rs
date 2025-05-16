///Register `ECMR` reader
pub type R = crate::R<ECMR_SPEC>;
///Register `ECMR` writer
pub type W = crate::W<ECMR_SPEC>;
/**Promiscuous Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRM_A {
    ///0: Promiscuous mode is disabled.
    _0 = 0,
    ///1: Promiscuous mode is enabled.
    _1 = 1,
}
impl From<PRM_A> for bool {
    #[inline(always)]
    fn from(variant: PRM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRM` reader - Promiscuous Mode
pub type PRM_R = crate::BitReader<PRM_A>;
impl PRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRM_A {
        match self.bits {
            false => PRM_A::_0,
            true => PRM_A::_1,
        }
    }
    ///Promiscuous mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRM_A::_0
    }
    ///Promiscuous mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRM_A::_1
    }
}
///Field `PRM` writer - Promiscuous Mode
pub type PRM_W<'a, REG> = crate::BitWriter<'a, REG, PRM_A>;
impl<'a, REG> PRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Promiscuous mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRM_A::_0)
    }
    ///Promiscuous mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRM_A::_1)
    }
}
/**Duplex Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM_A {
    ///0: Half-duplex mode
    _0 = 0,
    ///1: Full-duplex mode
    _1 = 1,
}
impl From<DM_A> for bool {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DM` reader - Duplex Mode
pub type DM_R = crate::BitReader<DM_A>;
impl DM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DM_A {
        match self.bits {
            false => DM_A::_0,
            true => DM_A::_1,
        }
    }
    ///Half-duplex mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DM_A::_0
    }
    ///Full-duplex mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DM_A::_1
    }
}
///Field `DM` writer - Duplex Mode
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG, DM_A>;
impl<'a, REG> DM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half-duplex mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::_0)
    }
    ///Full-duplex mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::_1)
    }
}
/**Bit Rate

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTM_A {
    ///0: 10 Mbps
    _0 = 0,
    ///1: 100 Mbps
    _1 = 1,
}
impl From<RTM_A> for bool {
    #[inline(always)]
    fn from(variant: RTM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTM` reader - Bit Rate
pub type RTM_R = crate::BitReader<RTM_A>;
impl RTM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTM_A {
        match self.bits {
            false => RTM_A::_0,
            true => RTM_A::_1,
        }
    }
    ///10 Mbps
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTM_A::_0
    }
    ///100 Mbps
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTM_A::_1
    }
}
///Field `RTM` writer - Bit Rate
pub type RTM_W<'a, REG> = crate::BitWriter<'a, REG, RTM_A>;
impl<'a, REG> RTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///10 Mbps
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTM_A::_0)
    }
    ///100 Mbps
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTM_A::_1)
    }
}
/**Internal Loopback Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILB_A {
    ///0: Normal data transmission or reception is performed.
    _0 = 0,
    ///1: Data is looped back in the ETHERC when full-duplex mode is selected.
    _1 = 1,
}
impl From<ILB_A> for bool {
    #[inline(always)]
    fn from(variant: ILB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ILB` reader - Internal Loopback Mode
pub type ILB_R = crate::BitReader<ILB_A>;
impl ILB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILB_A {
        match self.bits {
            false => ILB_A::_0,
            true => ILB_A::_1,
        }
    }
    ///Normal data transmission or reception is performed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILB_A::_0
    }
    ///Data is looped back in the ETHERC when full-duplex mode is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILB_A::_1
    }
}
///Field `ILB` writer - Internal Loopback Mode
pub type ILB_W<'a, REG> = crate::BitWriter<'a, REG, ILB_A>;
impl<'a, REG> ILB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal data transmission or reception is performed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILB_A::_0)
    }
    ///Data is looped back in the ETHERC when full-duplex mode is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILB_A::_1)
    }
}
/**Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    ///0: Transmit function is disabled.
    _0 = 0,
    ///1: Transmit function is enabled.
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmission Enable
pub type TE_R = crate::BitReader<TE_A>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    ///Transmit function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    ///Transmit function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
///Field `TE` writer - Transmission Enable
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE_A>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::_0)
    }
    ///Transmit function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::_1)
    }
}
/**Reception Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    ///0: Receive function is disabled.
    _0 = 0,
    ///1: Receive function is enabled.
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Reception Enable
pub type RE_R = crate::BitReader<RE_A>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    ///Receive function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    ///Receive function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
///Field `RE` writer - Reception Enable
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE_A>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::_0)
    }
    ///Receive function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::_1)
    }
}
/**Magic Packet Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDE_A {
    ///0: Magic Packet detection is disabled.
    _0 = 0,
    ///1: Magic Packet detection is enabled.
    _1 = 1,
}
impl From<MPDE_A> for bool {
    #[inline(always)]
    fn from(variant: MPDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPDE` reader - Magic Packet Detection Enable
pub type MPDE_R = crate::BitReader<MPDE_A>;
impl MPDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPDE_A {
        match self.bits {
            false => MPDE_A::_0,
            true => MPDE_A::_1,
        }
    }
    ///Magic Packet detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDE_A::_0
    }
    ///Magic Packet detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDE_A::_1
    }
}
///Field `MPDE` writer - Magic Packet Detection Enable
pub type MPDE_W<'a, REG> = crate::BitWriter<'a, REG, MPDE_A>;
impl<'a, REG> MPDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Magic Packet detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPDE_A::_0)
    }
    ///Magic Packet detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPDE_A::_1)
    }
}
/**CRC Error Frame Receive Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRCEF_A {
    ///0: EDMAC is notified of a CRC error.
    _0 = 0,
    ///1: EDMAC is not notified of a CRC error.
    _1 = 1,
}
impl From<PRCEF_A> for bool {
    #[inline(always)]
    fn from(variant: PRCEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRCEF` reader - CRC Error Frame Receive Mode
pub type PRCEF_R = crate::BitReader<PRCEF_A>;
impl PRCEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRCEF_A {
        match self.bits {
            false => PRCEF_A::_0,
            true => PRCEF_A::_1,
        }
    }
    ///EDMAC is notified of a CRC error.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRCEF_A::_0
    }
    ///EDMAC is not notified of a CRC error.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRCEF_A::_1
    }
}
///Field `PRCEF` writer - CRC Error Frame Receive Mode
pub type PRCEF_W<'a, REG> = crate::BitWriter<'a, REG, PRCEF_A>;
impl<'a, REG> PRCEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EDMAC is notified of a CRC error.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRCEF_A::_0)
    }
    ///EDMAC is not notified of a CRC error.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRCEF_A::_1)
    }
}
/**Transmit Flow Control Operating Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXF_A {
    ///0: Automatic PAUSE frame transmission is disabled.(PAUSE frame is not automatically transmitted.)
    _0 = 0,
    ///1: Automatic PAUSE frame transmission is enabled.(PAUSE frame is automatically transmitted as required.)
    _1 = 1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXF` reader - Transmit Flow Control Operating Mode
pub type TXF_R = crate::BitReader<TXF_A>;
impl TXF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::_0,
            true => TXF_A::_1,
        }
    }
    ///Automatic PAUSE frame transmission is disabled.(PAUSE frame is not automatically transmitted.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXF_A::_0
    }
    ///Automatic PAUSE frame transmission is enabled.(PAUSE frame is automatically transmitted as required.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXF_A::_1
    }
}
///Field `TXF` writer - Transmit Flow Control Operating Mode
pub type TXF_W<'a, REG> = crate::BitWriter<'a, REG, TXF_A>;
impl<'a, REG> TXF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic PAUSE frame transmission is disabled.(PAUSE frame is not automatically transmitted.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TXF_A::_0)
    }
    ///Automatic PAUSE frame transmission is enabled.(PAUSE frame is automatically transmitted as required.)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TXF_A::_1)
    }
}
/**Receive Flow Control Operating Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXF_A {
    ///0: PAUSE frame detection is disabled.
    _0 = 0,
    ///1: PAUSE frame detection is enabled.
    _1 = 1,
}
impl From<RXF_A> for bool {
    #[inline(always)]
    fn from(variant: RXF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXF` reader - Receive Flow Control Operating Mode
pub type RXF_R = crate::BitReader<RXF_A>;
impl RXF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXF_A {
        match self.bits {
            false => RXF_A::_0,
            true => RXF_A::_1,
        }
    }
    ///PAUSE frame detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXF_A::_0
    }
    ///PAUSE frame detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXF_A::_1
    }
}
///Field `RXF` writer - Receive Flow Control Operating Mode
pub type RXF_W<'a, REG> = crate::BitWriter<'a, REG, RXF_A>;
impl<'a, REG> RXF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PAUSE frame detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RXF_A::_0)
    }
    ///PAUSE frame detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RXF_A::_1)
    }
}
/**PAUSE Frame Receive Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFR_A {
    ///0: PAUSE frame is not transferred to the EDMAC.
    _0 = 0,
    ///1: PAUSE frame is transferred to the EDMAC.
    _1 = 1,
}
impl From<PFR_A> for bool {
    #[inline(always)]
    fn from(variant: PFR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PFR` reader - PAUSE Frame Receive Mode
pub type PFR_R = crate::BitReader<PFR_A>;
impl PFR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFR_A {
        match self.bits {
            false => PFR_A::_0,
            true => PFR_A::_1,
        }
    }
    ///PAUSE frame is not transferred to the EDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFR_A::_0
    }
    ///PAUSE frame is transferred to the EDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFR_A::_1
    }
}
///Field `PFR` writer - PAUSE Frame Receive Mode
pub type PFR_W<'a, REG> = crate::BitWriter<'a, REG, PFR_A>;
impl<'a, REG> PFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PAUSE frame is not transferred to the EDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PFR_A::_0)
    }
    ///PAUSE frame is transferred to the EDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PFR_A::_1)
    }
}
/**0 Time PAUSE Frame Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZPF_A {
    ///0: PAUSE frame that contains the pause_time parameter of 0 is not used.
    _0 = 0,
    ///1: PAUSE frame that contains the pause_time parameter of 0 is used.
    _1 = 1,
}
impl From<ZPF_A> for bool {
    #[inline(always)]
    fn from(variant: ZPF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ZPF` reader - 0 Time PAUSE Frame Enable
pub type ZPF_R = crate::BitReader<ZPF_A>;
impl ZPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ZPF_A {
        match self.bits {
            false => ZPF_A::_0,
            true => ZPF_A::_1,
        }
    }
    ///PAUSE frame that contains the pause_time parameter of 0 is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ZPF_A::_0
    }
    ///PAUSE frame that contains the pause_time parameter of 0 is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ZPF_A::_1
    }
}
///Field `ZPF` writer - 0 Time PAUSE Frame Enable
pub type ZPF_W<'a, REG> = crate::BitWriter<'a, REG, ZPF_A>;
impl<'a, REG> ZPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PAUSE frame that contains the pause_time parameter of 0 is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ZPF_A::_0)
    }
    ///PAUSE frame that contains the pause_time parameter of 0 is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ZPF_A::_1)
    }
}
/**PAUSE Frame Transmit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPC_A {
    ///0: PAUSE frame is transmitted even during a PAUSE period.
    _0 = 0,
    ///1: PAUSE frame is not transmitted during a PAUSE period.
    _1 = 1,
}
impl From<TPC_A> for bool {
    #[inline(always)]
    fn from(variant: TPC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TPC` reader - PAUSE Frame Transmit
pub type TPC_R = crate::BitReader<TPC_A>;
impl TPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPC_A {
        match self.bits {
            false => TPC_A::_0,
            true => TPC_A::_1,
        }
    }
    ///PAUSE frame is transmitted even during a PAUSE period.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPC_A::_0
    }
    ///PAUSE frame is not transmitted during a PAUSE period.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPC_A::_1
    }
}
///Field `TPC` writer - PAUSE Frame Transmit
pub type TPC_W<'a, REG> = crate::BitWriter<'a, REG, TPC_A>;
impl<'a, REG> TPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PAUSE frame is transmitted even during a PAUSE period.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TPC_A::_0)
    }
    ///PAUSE frame is not transmitted during a PAUSE period.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TPC_A::_1)
    }
}
impl R {
    ///Bit 0 - Promiscuous Mode
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Duplex Mode
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bit Rate
    #[inline(always)]
    pub fn rtm(&self) -> RTM_R {
        RTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Internal Loopback Mode
    #[inline(always)]
    pub fn ilb(&self) -> ILB_R {
        ILB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Transmission Enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Reception Enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Magic Packet Detection Enable
    #[inline(always)]
    pub fn mpde(&self) -> MPDE_R {
        MPDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC Error Frame Receive Mode
    #[inline(always)]
    pub fn prcef(&self) -> PRCEF_R {
        PRCEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Transmit Flow Control Operating Mode
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Flow Control Operating Mode
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PAUSE Frame Receive Mode
    #[inline(always)]
    pub fn pfr(&self) -> PFR_R {
        PFR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - 0 Time PAUSE Frame Enable
    #[inline(always)]
    pub fn zpf(&self) -> ZPF_R {
        ZPF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PAUSE Frame Transmit
    #[inline(always)]
    pub fn tpc(&self) -> TPC_R {
        TPC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Promiscuous Mode
    #[inline(always)]
    pub fn prm(&mut self) -> PRM_W<ECMR_SPEC> {
        PRM_W::new(self, 0)
    }
    ///Bit 1 - Duplex Mode
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<ECMR_SPEC> {
        DM_W::new(self, 1)
    }
    ///Bit 2 - Bit Rate
    #[inline(always)]
    pub fn rtm(&mut self) -> RTM_W<ECMR_SPEC> {
        RTM_W::new(self, 2)
    }
    ///Bit 3 - Internal Loopback Mode
    #[inline(always)]
    pub fn ilb(&mut self) -> ILB_W<ECMR_SPEC> {
        ILB_W::new(self, 3)
    }
    ///Bit 5 - Transmission Enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<ECMR_SPEC> {
        TE_W::new(self, 5)
    }
    ///Bit 6 - Reception Enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<ECMR_SPEC> {
        RE_W::new(self, 6)
    }
    ///Bit 9 - Magic Packet Detection Enable
    #[inline(always)]
    pub fn mpde(&mut self) -> MPDE_W<ECMR_SPEC> {
        MPDE_W::new(self, 9)
    }
    ///Bit 12 - CRC Error Frame Receive Mode
    #[inline(always)]
    pub fn prcef(&mut self) -> PRCEF_W<ECMR_SPEC> {
        PRCEF_W::new(self, 12)
    }
    ///Bit 16 - Transmit Flow Control Operating Mode
    #[inline(always)]
    pub fn txf(&mut self) -> TXF_W<ECMR_SPEC> {
        TXF_W::new(self, 16)
    }
    ///Bit 17 - Receive Flow Control Operating Mode
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W<ECMR_SPEC> {
        RXF_W::new(self, 17)
    }
    ///Bit 18 - PAUSE Frame Receive Mode
    #[inline(always)]
    pub fn pfr(&mut self) -> PFR_W<ECMR_SPEC> {
        PFR_W::new(self, 18)
    }
    ///Bit 19 - 0 Time PAUSE Frame Enable
    #[inline(always)]
    pub fn zpf(&mut self) -> ZPF_W<ECMR_SPEC> {
        ZPF_W::new(self, 19)
    }
    ///Bit 20 - PAUSE Frame Transmit
    #[inline(always)]
    pub fn tpc(&mut self) -> TPC_W<ECMR_SPEC> {
        TPC_W::new(self, 20)
    }
}
/**ETHERC Mode Register

You can [`read`](crate::Reg::read) this register and get [`ecmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECMR_SPEC;
impl crate::RegisterSpec for ECMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ecmr::R`](R) reader structure
impl crate::Readable for ECMR_SPEC {}
///`write(|w| ..)` method takes [`ecmr::W`](W) writer structure
impl crate::Writable for ECMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECMR to value 0
impl crate::Resettable for ECMR_SPEC {}
