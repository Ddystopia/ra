///Register `SD_INFO2` reader
pub type R = crate::R<SD_INFO2_SPEC>;
///Register `SD_INFO2` writer
pub type W = crate::W<SD_INFO2_SPEC>;
/**Command Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDE_A {
    ///0: Command error not detected
    _0 = 0,
    ///1: Command error detected
    _1 = 1,
}
impl From<CMDE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMDE` reader - Command Error

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMDE_R = crate::BitReader<CMDE_A>;
impl CMDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDE_A {
        match self.bits {
            false => CMDE_A::_0,
            true => CMDE_A::_1,
        }
    }
    ///Command error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDE_A::_0
    }
    ///Command error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDE_A::_1
    }
}
///Field `CMDE` writer - Command Error
pub type CMDE_W<'a, REG> = crate::BitWriter0C<'a, REG, CMDE_A>;
impl<'a, REG> CMDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMDE_A::_0)
    }
    ///Command error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDE_A::_1)
    }
}
/**CRC Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE_A {
    ///0: CRC error not detected
    _0 = 0,
    ///1: CRC error detected
    _1 = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CRCE` reader - CRC Error

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CRCE_R = crate::BitReader<CRCE_A>;
impl CRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::_0,
            true => CRCE_A::_1,
        }
    }
    ///CRC error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCE_A::_0
    }
    ///CRC error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCE_A::_1
    }
}
///Field `CRCE` writer - CRC Error
pub type CRCE_W<'a, REG> = crate::BitWriter0C<'a, REG, CRCE_A>;
impl<'a, REG> CRCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCE_A::_0)
    }
    ///CRC error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCE_A::_1)
    }
}
/**END Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDE_A {
    ///0: End bit error not detected
    _0 = 0,
    ///1: End bit error detected
    _1 = 1,
}
impl From<ENDE_A> for bool {
    #[inline(always)]
    fn from(variant: ENDE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ENDE` reader - END Error

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ENDE_R = crate::BitReader<ENDE_A>;
impl ENDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENDE_A {
        match self.bits {
            false => ENDE_A::_0,
            true => ENDE_A::_1,
        }
    }
    ///End bit error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDE_A::_0
    }
    ///End bit error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDE_A::_1
    }
}
///Field `ENDE` writer - END Error
pub type ENDE_W<'a, REG> = crate::BitWriter0C<'a, REG, ENDE_A>;
impl<'a, REG> ENDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End bit error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENDE_A::_0)
    }
    ///End bit error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDE_A::_1)
    }
}
/**Data Timeout

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTO_A {
    ///0: Data timeout not detected
    _0 = 0,
    ///1: Data timeout detected
    _1 = 1,
}
impl From<DTO_A> for bool {
    #[inline(always)]
    fn from(variant: DTO_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DTO` reader - Data Timeout

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DTO_R = crate::BitReader<DTO_A>;
impl DTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTO_A {
        match self.bits {
            false => DTO_A::_0,
            true => DTO_A::_1,
        }
    }
    ///Data timeout not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTO_A::_0
    }
    ///Data timeout detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTO_A::_1
    }
}
///Field `DTO` writer - Data Timeout
pub type DTO_W<'a, REG> = crate::BitWriter0C<'a, REG, DTO_A>;
impl<'a, REG> DTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data timeout not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTO_A::_0)
    }
    ///Data timeout detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTO_A::_1)
    }
}
/**SD_BUF Illegal Write Access

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILW_A {
    ///0: Illegal write access to the SD_BUF register not detected
    _0 = 0,
    ///1: Illegal write access to the SD_BUF register detected
    _1 = 1,
}
impl From<ILW_A> for bool {
    #[inline(always)]
    fn from(variant: ILW_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ILW` reader - SD_BUF Illegal Write Access

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ILW_R = crate::BitReader<ILW_A>;
impl ILW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILW_A {
        match self.bits {
            false => ILW_A::_0,
            true => ILW_A::_1,
        }
    }
    ///Illegal write access to the SD_BUF register not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILW_A::_0
    }
    ///Illegal write access to the SD_BUF register detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILW_A::_1
    }
}
///Field `ILW` writer - SD_BUF Illegal Write Access
pub type ILW_W<'a, REG> = crate::BitWriter0C<'a, REG, ILW_A>;
impl<'a, REG> ILW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal write access to the SD_BUF register not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILW_A::_0)
    }
    ///Illegal write access to the SD_BUF register detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILW_A::_1)
    }
}
/**SD_BUF Illegal Read Access

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILR_A {
    ///0: Illegal read access to the SD_BUF register not detected
    _0 = 0,
    ///1: Illegal read access to the SD_BUF register detected
    _1 = 1,
}
impl From<ILR_A> for bool {
    #[inline(always)]
    fn from(variant: ILR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ILR` reader - SD_BUF Illegal Read Access

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ILR_R = crate::BitReader<ILR_A>;
impl ILR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILR_A {
        match self.bits {
            false => ILR_A::_0,
            true => ILR_A::_1,
        }
    }
    ///Illegal read access to the SD_BUF register not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILR_A::_0
    }
    ///Illegal read access to the SD_BUF register detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILR_A::_1
    }
}
///Field `ILR` writer - SD_BUF Illegal Read Access
pub type ILR_W<'a, REG> = crate::BitWriter0C<'a, REG, ILR_A>;
impl<'a, REG> ILR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal read access to the SD_BUF register not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILR_A::_0)
    }
    ///Illegal read access to the SD_BUF register detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILR_A::_1)
    }
}
/**Response Timeout

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO_A {
    ///0: Response timeout not detected
    _0 = 0,
    ///1: Response timeout detected
    _1 = 1,
}
impl From<RSPTO_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RSPTO` reader - Response Timeout

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RSPTO_R = crate::BitReader<RSPTO_A>;
impl RSPTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPTO_A {
        match self.bits {
            false => RSPTO_A::_0,
            true => RSPTO_A::_1,
        }
    }
    ///Response timeout not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO_A::_0
    }
    ///Response timeout detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO_A::_1
    }
}
///Field `RSPTO` writer - Response Timeout
pub type RSPTO_W<'a, REG> = crate::BitWriter0C<'a, REG, RSPTO_A>;
impl<'a, REG> RSPTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Response timeout not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTO_A::_0)
    }
    ///Response timeout detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTO_A::_1)
    }
}
/**SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD0MON_A {
    ///1: SDDAT0 is set to 1.
    _1 = 1,
    ///0: SDDAT0 is set to 0.
    _0 = 0,
}
impl From<SDD0MON_A> for bool {
    #[inline(always)]
    fn from(variant: SDD0MON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD0MON` reader - SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL.
pub type SDD0MON_R = crate::BitReader<SDD0MON_A>;
impl SDD0MON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDD0MON_A {
        match self.bits {
            true => SDD0MON_A::_1,
            false => SDD0MON_A::_0,
        }
    }
    ///SDDAT0 is set to 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD0MON_A::_1
    }
    ///SDDAT0 is set to 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD0MON_A::_0
    }
}
/**SD_BUF Read Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRE_A {
    ///1: Data can be read from SD_BUF0.
    _1 = 1,
    ///0: Data cannot be read from SD_BUF0.
    _0 = 0,
}
impl From<BRE_A> for bool {
    #[inline(always)]
    fn from(variant: BRE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `BRE` reader - SD_BUF Read Enable

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type BRE_R = crate::BitReader<BRE_A>;
impl BRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRE_A {
        match self.bits {
            true => BRE_A::_1,
            false => BRE_A::_0,
        }
    }
    ///Data can be read from SD_BUF0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRE_A::_1
    }
    ///Data cannot be read from SD_BUF0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRE_A::_0
    }
}
///Field `BRE` writer - SD_BUF Read Enable
pub type BRE_W<'a, REG> = crate::BitWriter0C<'a, REG, BRE_A>;
impl<'a, REG> BRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data can be read from SD_BUF0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRE_A::_1)
    }
    ///Data cannot be read from SD_BUF0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRE_A::_0)
    }
}
/**SD_BUF Write Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWE_A {
    ///1: Data can be written in SD_BUF0.
    _1 = 1,
    ///0: Data cannot be written in SD_BUF0.
    _0 = 0,
}
impl From<BWE_A> for bool {
    #[inline(always)]
    fn from(variant: BWE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `BWE` reader - SD_BUF Write Enable

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type BWE_R = crate::BitReader<BWE_A>;
impl BWE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BWE_A {
        match self.bits {
            true => BWE_A::_1,
            false => BWE_A::_0,
        }
    }
    ///Data can be written in SD_BUF0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWE_A::_1
    }
    ///Data cannot be written in SD_BUF0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWE_A::_0
    }
}
///Field `BWE` writer - SD_BUF Write Enable
pub type BWE_W<'a, REG> = crate::BitWriter0C<'a, REG, BWE_A>;
impl<'a, REG> BWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data can be written in SD_BUF0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BWE_A::_1)
    }
    ///Data cannot be written in SD_BUF0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BWE_A::_0)
    }
}
/**When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SD_CLK_CTRLEN_A {
    ///0: The SD/MMC bus (CMD, DAT) is busy. Writing to the SCLKEN and DIV bits in SD_CLK_CTRL is not possible.
    _0 = 0,
    ///1: The SD/MMC bus (CMD, DAT) is not busy.
    _1 = 1,
}
impl From<SD_CLK_CTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: SD_CLK_CTRLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SD_CLK_CTRLEN` reader - When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence.
pub type SD_CLK_CTRLEN_R = crate::BitReader<SD_CLK_CTRLEN_A>;
impl SD_CLK_CTRLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SD_CLK_CTRLEN_A {
        match self.bits {
            false => SD_CLK_CTRLEN_A::_0,
            true => SD_CLK_CTRLEN_A::_1,
        }
    }
    ///The SD/MMC bus (CMD, DAT) is busy. Writing to the SCLKEN and DIV bits in SD_CLK_CTRL is not possible.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SD_CLK_CTRLEN_A::_0
    }
    ///The SD/MMC bus (CMD, DAT) is not busy.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SD_CLK_CTRLEN_A::_1
    }
}
/**Command Type Register Busy

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBSY_A {
    ///0: A command sequence is being executed.
    _0 = 0,
    ///1: A command sequence has been completed.
    _1 = 1,
}
impl From<CBSY_A> for bool {
    #[inline(always)]
    fn from(variant: CBSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CBSY` reader - Command Type Register Busy
pub type CBSY_R = crate::BitReader<CBSY_A>;
impl CBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CBSY_A {
        match self.bits {
            false => CBSY_A::_0,
            true => CBSY_A::_1,
        }
    }
    ///A command sequence is being executed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CBSY_A::_0
    }
    ///A command sequence has been completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CBSY_A::_1
    }
}
/**Illegal Access Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILA_A {
    ///0: Illegal access error not detected
    _0 = 0,
    ///1: Illegal access error detected
    _1 = 1,
}
impl From<ILA_A> for bool {
    #[inline(always)]
    fn from(variant: ILA_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ILA` reader - Illegal Access Error

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ILA_R = crate::BitReader<ILA_A>;
impl ILA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILA_A {
        match self.bits {
            false => ILA_A::_0,
            true => ILA_A::_1,
        }
    }
    ///Illegal access error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILA_A::_0
    }
    ///Illegal access error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILA_A::_1
    }
}
///Field `ILA` writer - Illegal Access Error
pub type ILA_W<'a, REG> = crate::BitWriter0C<'a, REG, ILA_A>;
impl<'a, REG> ILA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal access error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILA_A::_0)
    }
    ///Illegal access error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILA_A::_1)
    }
}
impl R {
    ///Bit 0 - Command Error
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRC Error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - END Error
    #[inline(always)]
    pub fn ende(&self) -> ENDE_R {
        ENDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data Timeout
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SD_BUF Illegal Write Access
    #[inline(always)]
    pub fn ilw(&self) -> ILW_R {
        ILW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SD_BUF Illegal Read Access
    #[inline(always)]
    pub fn ilr(&self) -> ILR_R {
        ILR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Response Timeout
    #[inline(always)]
    pub fn rspto(&self) -> RSPTO_R {
        RSPTO_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL.
    #[inline(always)]
    pub fn sdd0mon(&self) -> SDD0MON_R {
        SDD0MON_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SD_BUF Read Enable
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SD_BUF Write Enable
    #[inline(always)]
    pub fn bwe(&self) -> BWE_R {
        BWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence.
    #[inline(always)]
    pub fn sd_clk_ctrlen(&self) -> SD_CLK_CTRLEN_R {
        SD_CLK_CTRLEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Command Type Register Busy
    #[inline(always)]
    pub fn cbsy(&self) -> CBSY_R {
        CBSY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Illegal Access Error
    #[inline(always)]
    pub fn ila(&self) -> ILA_R {
        ILA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Command Error
    #[inline(always)]
    pub fn cmde(&mut self) -> CMDE_W<SD_INFO2_SPEC> {
        CMDE_W::new(self, 0)
    }
    ///Bit 1 - CRC Error
    #[inline(always)]
    pub fn crce(&mut self) -> CRCE_W<SD_INFO2_SPEC> {
        CRCE_W::new(self, 1)
    }
    ///Bit 2 - END Error
    #[inline(always)]
    pub fn ende(&mut self) -> ENDE_W<SD_INFO2_SPEC> {
        ENDE_W::new(self, 2)
    }
    ///Bit 3 - Data Timeout
    #[inline(always)]
    pub fn dto(&mut self) -> DTO_W<SD_INFO2_SPEC> {
        DTO_W::new(self, 3)
    }
    ///Bit 4 - SD_BUF Illegal Write Access
    #[inline(always)]
    pub fn ilw(&mut self) -> ILW_W<SD_INFO2_SPEC> {
        ILW_W::new(self, 4)
    }
    ///Bit 5 - SD_BUF Illegal Read Access
    #[inline(always)]
    pub fn ilr(&mut self) -> ILR_W<SD_INFO2_SPEC> {
        ILR_W::new(self, 5)
    }
    ///Bit 6 - Response Timeout
    #[inline(always)]
    pub fn rspto(&mut self) -> RSPTO_W<SD_INFO2_SPEC> {
        RSPTO_W::new(self, 6)
    }
    ///Bit 8 - SD_BUF Read Enable
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<SD_INFO2_SPEC> {
        BRE_W::new(self, 8)
    }
    ///Bit 9 - SD_BUF Write Enable
    #[inline(always)]
    pub fn bwe(&mut self) -> BWE_W<SD_INFO2_SPEC> {
        BWE_W::new(self, 9)
    }
    ///Bit 15 - Illegal Access Error
    #[inline(always)]
    pub fn ila(&mut self) -> ILA_W<SD_INFO2_SPEC> {
        ILA_W::new(self, 15)
    }
}
/**SD Card Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_info2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_INFO2_SPEC;
impl crate::RegisterSpec for SD_INFO2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_info2::R`](R) reader structure
impl crate::Readable for SD_INFO2_SPEC {}
///`write(|w| ..)` method takes [`sd_info2::W`](W) writer structure
impl crate::Writable for SD_INFO2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x837f;
}
///`reset()` method sets SD_INFO2 to value 0x2000
impl crate::Resettable for SD_INFO2_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
