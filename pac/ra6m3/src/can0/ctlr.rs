///Register `CTLR` reader
pub type R = crate::R<CTLR_SPEC>;
///Register `CTLR` writer
pub type W = crate::W<CTLR_SPEC>;
/**CAN Mailbox Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBM_A {
    ///0: Normal mailbox mode
    _0 = 0,
    ///1: FIFO mailbox mode
    _1 = 1,
}
impl From<MBM_A> for bool {
    #[inline(always)]
    fn from(variant: MBM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MBM` reader - CAN Mailbox Mode Select
pub type MBM_R = crate::BitReader<MBM_A>;
impl MBM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MBM_A {
        match self.bits {
            false => MBM_A::_0,
            true => MBM_A::_1,
        }
    }
    ///Normal mailbox mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MBM_A::_0
    }
    ///FIFO mailbox mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MBM_A::_1
    }
}
///Field `MBM` writer - CAN Mailbox Mode Select
pub type MBM_W<'a, REG> = crate::BitWriter<'a, REG, MBM_A>;
impl<'a, REG> MBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mailbox mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MBM_A::_0)
    }
    ///FIFO mailbox mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MBM_A::_1)
    }
}
/**ID Format Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDFM_A {
    ///0: Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids.
    _00 = 0,
    ///1: Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs.
    _01 = 1,
    ///2: Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \[0\] to \[23\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \[24\] is used for the transmit FIFO.
    _10 = 2,
    ///3: Do not use this combination
    _11 = 3,
}
impl From<IDFM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDFM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDFM_A {
    type Ux = u8;
}
impl crate::IsEnum for IDFM_A {}
///Field `IDFM` reader - ID Format Mode Select
pub type IDFM_R = crate::FieldReader<IDFM_A>;
impl IDFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDFM_A {
        match self.bits {
            0 => IDFM_A::_00,
            1 => IDFM_A::_01,
            2 => IDFM_A::_10,
            3 => IDFM_A::_11,
            _ => unreachable!(),
        }
    }
    ///Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDFM_A::_00
    }
    ///Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDFM_A::_01
    }
    ///Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \[0\] to \[23\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \[24\] is used for the transmit FIFO.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDFM_A::_10
    }
    ///Do not use this combination
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDFM_A::_11
    }
}
///Field `IDFM` writer - ID Format Mode Select
pub type IDFM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IDFM_A, crate::Safe>;
impl<'a, REG> IDFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_00)
    }
    ///Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_01)
    }
    ///Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \[0\] to \[23\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \[24\] is used for the transmit FIFO.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_10)
    }
    ///Do not use this combination
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_11)
    }
}
/**Message Lost Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MLM_A {
    ///0: Overwrite mode
    _0 = 0,
    ///1: Overrun mode
    _1 = 1,
}
impl From<MLM_A> for bool {
    #[inline(always)]
    fn from(variant: MLM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MLM` reader - Message Lost Mode Select
pub type MLM_R = crate::BitReader<MLM_A>;
impl MLM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MLM_A {
        match self.bits {
            false => MLM_A::_0,
            true => MLM_A::_1,
        }
    }
    ///Overwrite mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MLM_A::_0
    }
    ///Overrun mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MLM_A::_1
    }
}
///Field `MLM` writer - Message Lost Mode Select
pub type MLM_W<'a, REG> = crate::BitWriter<'a, REG, MLM_A>;
impl<'a, REG> MLM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overwrite mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MLM_A::_0)
    }
    ///Overrun mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MLM_A::_1)
    }
}
/**Transmission Priority Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPM_A {
    ///0: ID priority transmit mode
    _0 = 0,
    ///1: Mailbox number priority transmit mode
    _1 = 1,
}
impl From<TPM_A> for bool {
    #[inline(always)]
    fn from(variant: TPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TPM` reader - Transmission Priority Mode Select
pub type TPM_R = crate::BitReader<TPM_A>;
impl TPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPM_A {
        match self.bits {
            false => TPM_A::_0,
            true => TPM_A::_1,
        }
    }
    ///ID priority transmit mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM_A::_0
    }
    ///Mailbox number priority transmit mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM_A::_1
    }
}
///Field `TPM` writer - Transmission Priority Mode Select
pub type TPM_W<'a, REG> = crate::BitWriter<'a, REG, TPM_A>;
impl<'a, REG> TPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ID priority transmit mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TPM_A::_0)
    }
    ///Mailbox number priority transmit mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TPM_A::_1)
    }
}
/**Time Stamp Counter Reset Command

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSRC_A {
    ///0: Nothing occurred
    _0 = 0,
    ///1: Reset
    _1 = 1,
}
impl From<TSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TSRC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSRC` reader - Time Stamp Counter Reset Command
pub type TSRC_R = crate::BitReader<TSRC_A>;
impl TSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSRC_A {
        match self.bits {
            false => TSRC_A::_0,
            true => TSRC_A::_1,
        }
    }
    ///Nothing occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSRC_A::_0
    }
    ///Reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSRC_A::_1
    }
}
///Field `TSRC` writer - Time Stamp Counter Reset Command
pub type TSRC_W<'a, REG> = crate::BitWriter<'a, REG, TSRC_A>;
impl<'a, REG> TSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Nothing occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSRC_A::_0)
    }
    ///Reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSRC_A::_1)
    }
}
/**Time Stamp Prescaler Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSPS_A {
    ///0: Every bit time
    _00 = 0,
    ///1: Every 2-bit time
    _01 = 1,
    ///2: Every 4-bit time
    _10 = 2,
    ///3: Every 8-bit time
    _11 = 3,
}
impl From<TSPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSPS_A {
    type Ux = u8;
}
impl crate::IsEnum for TSPS_A {}
///Field `TSPS` reader - Time Stamp Prescaler Select
pub type TSPS_R = crate::FieldReader<TSPS_A>;
impl TSPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSPS_A {
        match self.bits {
            0 => TSPS_A::_00,
            1 => TSPS_A::_01,
            2 => TSPS_A::_10,
            3 => TSPS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Every bit time
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSPS_A::_00
    }
    ///Every 2-bit time
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSPS_A::_01
    }
    ///Every 4-bit time
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSPS_A::_10
    }
    ///Every 8-bit time
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSPS_A::_11
    }
}
///Field `TSPS` writer - Time Stamp Prescaler Select
pub type TSPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TSPS_A, crate::Safe>;
impl<'a, REG> TSPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Every bit time
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_00)
    }
    ///Every 2-bit time
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_01)
    }
    ///Every 4-bit time
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_10)
    }
    ///Every 8-bit time
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_11)
    }
}
/**CAN Operating Mode Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CANM_A {
    ///0: CAN operation mode
    _00 = 0,
    ///1: CAN reset mode
    _01 = 1,
    ///2: CAN halt mode
    _10 = 2,
    ///3: CAN reset mode (forcible transition)
    _11 = 3,
}
impl From<CANM_A> for u8 {
    #[inline(always)]
    fn from(variant: CANM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CANM_A {
    type Ux = u8;
}
impl crate::IsEnum for CANM_A {}
///Field `CANM` reader - CAN Operating Mode Select
pub type CANM_R = crate::FieldReader<CANM_A>;
impl CANM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CANM_A {
        match self.bits {
            0 => CANM_A::_00,
            1 => CANM_A::_01,
            2 => CANM_A::_10,
            3 => CANM_A::_11,
            _ => unreachable!(),
        }
    }
    ///CAN operation mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CANM_A::_00
    }
    ///CAN reset mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CANM_A::_01
    }
    ///CAN halt mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CANM_A::_10
    }
    ///CAN reset mode (forcible transition)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CANM_A::_11
    }
}
///Field `CANM` writer - CAN Operating Mode Select
pub type CANM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CANM_A, crate::Safe>;
impl<'a, REG> CANM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CAN operation mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_00)
    }
    ///CAN reset mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_01)
    }
    ///CAN halt mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_10)
    }
    ///CAN reset mode (forcible transition)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_11)
    }
}
/**CAN Sleep Mode

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLPM_A {
    ///0: Other than CAN sleep mode
    _0 = 0,
    ///1: CAN sleep mode
    _1 = 1,
}
impl From<SLPM_A> for bool {
    #[inline(always)]
    fn from(variant: SLPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLPM` reader - CAN Sleep Mode
pub type SLPM_R = crate::BitReader<SLPM_A>;
impl SLPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLPM_A {
        match self.bits {
            false => SLPM_A::_0,
            true => SLPM_A::_1,
        }
    }
    ///Other than CAN sleep mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLPM_A::_0
    }
    ///CAN sleep mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLPM_A::_1
    }
}
///Field `SLPM` writer - CAN Sleep Mode
pub type SLPM_W<'a, REG> = crate::BitWriter<'a, REG, SLPM_A>;
impl<'a, REG> SLPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Other than CAN sleep mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLPM_A::_0)
    }
    ///CAN sleep mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLPM_A::_1)
    }
}
/**Bus-Off Recovery Mode by a program request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOM_A {
    ///0: Normal mode (ISO11898-1 compliant)
    _00 = 0,
    ///1: Entry to CAN halt mode automatically at bus-off entry
    _01 = 1,
    ///2: Entry to CAN halt mode automatically at bus-off end
    _10 = 2,
    ///3: Entry to CAN halt mode (during bus-off recovery period)
    _11 = 3,
}
impl From<BOM_A> for u8 {
    #[inline(always)]
    fn from(variant: BOM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOM_A {
    type Ux = u8;
}
impl crate::IsEnum for BOM_A {}
///Field `BOM` reader - Bus-Off Recovery Mode by a program request
pub type BOM_R = crate::FieldReader<BOM_A>;
impl BOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOM_A {
        match self.bits {
            0 => BOM_A::_00,
            1 => BOM_A::_01,
            2 => BOM_A::_10,
            3 => BOM_A::_11,
            _ => unreachable!(),
        }
    }
    ///Normal mode (ISO11898-1 compliant)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BOM_A::_00
    }
    ///Entry to CAN halt mode automatically at bus-off entry
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BOM_A::_01
    }
    ///Entry to CAN halt mode automatically at bus-off end
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BOM_A::_10
    }
    ///Entry to CAN halt mode (during bus-off recovery period)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BOM_A::_11
    }
}
///Field `BOM` writer - Bus-Off Recovery Mode by a program request
pub type BOM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BOM_A, crate::Safe>;
impl<'a, REG> BOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode (ISO11898-1 compliant)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_00)
    }
    ///Entry to CAN halt mode automatically at bus-off entry
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_01)
    }
    ///Entry to CAN halt mode automatically at bus-off end
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_10)
    }
    ///Entry to CAN halt mode (during bus-off recovery period)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_11)
    }
}
/**Forcible Return From Bus-Off

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBOC_A {
    ///0: Nothing occurred
    _0 = 0,
    ///1: Forcible return from bus-off
    _1 = 1,
}
impl From<RBOC_A> for bool {
    #[inline(always)]
    fn from(variant: RBOC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RBOC` reader - Forcible Return From Bus-Off
pub type RBOC_R = crate::BitReader<RBOC_A>;
impl RBOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RBOC_A {
        match self.bits {
            false => RBOC_A::_0,
            true => RBOC_A::_1,
        }
    }
    ///Nothing occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBOC_A::_0
    }
    ///Forcible return from bus-off
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBOC_A::_1
    }
}
///Field `RBOC` writer - Forcible Return From Bus-Off
pub type RBOC_W<'a, REG> = crate::BitWriter<'a, REG, RBOC_A>;
impl<'a, REG> RBOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Nothing occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RBOC_A::_0)
    }
    ///Forcible return from bus-off
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RBOC_A::_1)
    }
}
impl R {
    ///Bit 0 - CAN Mailbox Mode Select
    #[inline(always)]
    pub fn mbm(&self) -> MBM_R {
        MBM_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - ID Format Mode Select
    #[inline(always)]
    pub fn idfm(&self) -> IDFM_R {
        IDFM_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Message Lost Mode Select
    #[inline(always)]
    pub fn mlm(&self) -> MLM_R {
        MLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmission Priority Mode Select
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Time Stamp Counter Reset Command
    #[inline(always)]
    pub fn tsrc(&self) -> TSRC_R {
        TSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Time Stamp Prescaler Select
    #[inline(always)]
    pub fn tsps(&self) -> TSPS_R {
        TSPS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - CAN Operating Mode Select
    #[inline(always)]
    pub fn canm(&self) -> CANM_R {
        CANM_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - CAN Sleep Mode
    #[inline(always)]
    pub fn slpm(&self) -> SLPM_R {
        SLPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Bus-Off Recovery Mode by a program request
    #[inline(always)]
    pub fn bom(&self) -> BOM_R {
        BOM_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - Forcible Return From Bus-Off
    #[inline(always)]
    pub fn rboc(&self) -> RBOC_R {
        RBOC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CAN Mailbox Mode Select
    #[inline(always)]
    pub fn mbm(&mut self) -> MBM_W<CTLR_SPEC> {
        MBM_W::new(self, 0)
    }
    ///Bits 1:2 - ID Format Mode Select
    #[inline(always)]
    pub fn idfm(&mut self) -> IDFM_W<CTLR_SPEC> {
        IDFM_W::new(self, 1)
    }
    ///Bit 3 - Message Lost Mode Select
    #[inline(always)]
    pub fn mlm(&mut self) -> MLM_W<CTLR_SPEC> {
        MLM_W::new(self, 3)
    }
    ///Bit 4 - Transmission Priority Mode Select
    #[inline(always)]
    pub fn tpm(&mut self) -> TPM_W<CTLR_SPEC> {
        TPM_W::new(self, 4)
    }
    ///Bit 5 - Time Stamp Counter Reset Command
    #[inline(always)]
    pub fn tsrc(&mut self) -> TSRC_W<CTLR_SPEC> {
        TSRC_W::new(self, 5)
    }
    ///Bits 6:7 - Time Stamp Prescaler Select
    #[inline(always)]
    pub fn tsps(&mut self) -> TSPS_W<CTLR_SPEC> {
        TSPS_W::new(self, 6)
    }
    ///Bits 8:9 - CAN Operating Mode Select
    #[inline(always)]
    pub fn canm(&mut self) -> CANM_W<CTLR_SPEC> {
        CANM_W::new(self, 8)
    }
    ///Bit 10 - CAN Sleep Mode
    #[inline(always)]
    pub fn slpm(&mut self) -> SLPM_W<CTLR_SPEC> {
        SLPM_W::new(self, 10)
    }
    ///Bits 11:12 - Bus-Off Recovery Mode by a program request
    #[inline(always)]
    pub fn bom(&mut self) -> BOM_W<CTLR_SPEC> {
        BOM_W::new(self, 11)
    }
    ///Bit 13 - Forcible Return From Bus-Off
    #[inline(always)]
    pub fn rboc(&mut self) -> RBOC_W<CTLR_SPEC> {
        RBOC_W::new(self, 13)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctlr::R`](R) reader structure
impl crate::Readable for CTLR_SPEC {}
///`write(|w| ..)` method takes [`ctlr::W`](W) writer structure
impl crate::Writable for CTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTLR to value 0x0500
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: u16 = 0x0500;
}
