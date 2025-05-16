///Register `SPCMD%s` reader
pub type R = crate::R<SPCMD_SPEC>;
///Register `SPCMD%s` writer
pub type W = crate::W<SPCMD_SPEC>;
/**RSPCK Phase Setting

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    ///0: Data sampling on odd edge, data variation on even edge
    _0 = 0,
    ///1: Data variation on odd edge, data sampling on even edge
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CPHA` reader - RSPCK Phase Setting
pub type CPHA_R = crate::BitReader<CPHA_A>;
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    ///Data sampling on odd edge, data variation on even edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    ///Data variation on odd edge, data sampling on even edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
///Field `CPHA` writer - RSPCK Phase Setting
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA_A>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data sampling on odd edge, data variation on even edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::_0)
    }
    ///Data variation on odd edge, data sampling on even edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::_1)
    }
}
/**RSPCK Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    ///0: RSPCK is low when idle
    _0 = 0,
    ///1: RSPCK is high when idle
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOL` reader - RSPCK Polarity Setting
pub type CPOL_R = crate::BitReader<CPOL_A>;
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    ///RSPCK is low when idle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    ///RSPCK is high when idle
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
///Field `CPOL` writer - RSPCK Polarity Setting
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL_A>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RSPCK is low when idle
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::_0)
    }
    ///RSPCK is high when idle
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::_1)
    }
}
/**Bit Rate Division Setting

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRDV_A {
    ///0: These bits select the base bit rate
    _00 = 0,
    ///1: These bits select the base bit rate divided by 2
    _01 = 1,
    ///2: These bits select the base bit rate divided by 4
    _10 = 2,
    ///3: These bits select the base bit rate divided by 8
    _11 = 3,
}
impl From<BRDV_A> for u8 {
    #[inline(always)]
    fn from(variant: BRDV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BRDV_A {
    type Ux = u8;
}
impl crate::IsEnum for BRDV_A {}
///Field `BRDV` reader - Bit Rate Division Setting
pub type BRDV_R = crate::FieldReader<BRDV_A>;
impl BRDV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRDV_A {
        match self.bits {
            0 => BRDV_A::_00,
            1 => BRDV_A::_01,
            2 => BRDV_A::_10,
            3 => BRDV_A::_11,
            _ => unreachable!(),
        }
    }
    ///These bits select the base bit rate
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BRDV_A::_00
    }
    ///These bits select the base bit rate divided by 2
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BRDV_A::_01
    }
    ///These bits select the base bit rate divided by 4
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BRDV_A::_10
    }
    ///These bits select the base bit rate divided by 8
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BRDV_A::_11
    }
}
///Field `BRDV` writer - Bit Rate Division Setting
pub type BRDV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BRDV_A, crate::Safe>;
impl<'a, REG> BRDV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///These bits select the base bit rate
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_00)
    }
    ///These bits select the base bit rate divided by 2
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_01)
    }
    ///These bits select the base bit rate divided by 4
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_10)
    }
    ///These bits select the base bit rate divided by 8
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_11)
    }
}
/**SSL Signal Assertion Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSLA_A {
    ///0: SSL0
    _000 = 0,
    ///1: SSL1
    _001 = 1,
    ///2: SSL2
    _010 = 2,
    ///3: SSL3
    _011 = 3,
    ///4: Setting prohibited
    OTHERS = 4,
}
impl From<SSLA_A> for u8 {
    #[inline(always)]
    fn from(variant: SSLA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSLA_A {
    type Ux = u8;
}
impl crate::IsEnum for SSLA_A {}
///Field `SSLA` reader - SSL Signal Assertion Setting
pub type SSLA_R = crate::FieldReader<SSLA_A>;
impl SSLA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSLA_A {
        match self.bits {
            0 => SSLA_A::_000,
            1 => SSLA_A::_001,
            2 => SSLA_A::_010,
            3 => SSLA_A::_011,
            _ => SSLA_A::OTHERS,
        }
    }
    ///SSL0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SSLA_A::_000
    }
    ///SSL1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SSLA_A::_001
    }
    ///SSL2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SSLA_A::_010
    }
    ///SSL3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SSLA_A::_011
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SSLA_A::OTHERS)
    }
}
///Field `SSLA` writer - SSL Signal Assertion Setting
pub type SSLA_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SSLA_A, crate::Safe>;
impl<'a, REG> SSLA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SSL0
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_000)
    }
    ///SSL1
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_001)
    }
    ///SSL2
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_010)
    }
    ///SSL3
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_011)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::OTHERS)
    }
}
/**SSL Signal Level Keeping

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSLKP_A {
    ///0: Negate all SSL signals on completion of transfer
    _0 = 0,
    ///1: Keep SSL signal level from the end of transfer until the beginning
    _1 = 1,
}
impl From<SSLKP_A> for bool {
    #[inline(always)]
    fn from(variant: SSLKP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSLKP` reader - SSL Signal Level Keeping
pub type SSLKP_R = crate::BitReader<SSLKP_A>;
impl SSLKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSLKP_A {
        match self.bits {
            false => SSLKP_A::_0,
            true => SSLKP_A::_1,
        }
    }
    ///Negate all SSL signals on completion of transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSLKP_A::_0
    }
    ///Keep SSL signal level from the end of transfer until the beginning
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSLKP_A::_1
    }
}
///Field `SSLKP` writer - SSL Signal Level Keeping
pub type SSLKP_W<'a, REG> = crate::BitWriter<'a, REG, SSLKP_A>;
impl<'a, REG> SSLKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Negate all SSL signals on completion of transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSLKP_A::_0)
    }
    ///Keep SSL signal level from the end of transfer until the beginning
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSLKP_A::_1)
    }
}
/**RSPI Data Length Setting

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPB_A {
    ///0: 20 bits
    _0000 = 0,
    ///1: 24 bits
    _0001 = 1,
    ///2: 32 bits
    _0010 = 2,
    ///3: 32 bits
    _0011 = 3,
    ///8: 9 bits
    _1000 = 8,
    ///9: 10 bits
    _1001 = 9,
    ///10: 11 bits
    _1010 = 10,
    ///11: 12 bits
    _1011 = 11,
    ///12: 13 bits
    _1100 = 12,
    ///13: 14 bits
    _1101 = 13,
    ///14: 15 bits
    _1110 = 14,
    ///15: 16 bits
    _1111 = 15,
    ///4: 8bits
    OTHERS = 4,
}
impl From<SPB_A> for u8 {
    #[inline(always)]
    fn from(variant: SPB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPB_A {
    type Ux = u8;
}
impl crate::IsEnum for SPB_A {}
///Field `SPB` reader - RSPI Data Length Setting
pub type SPB_R = crate::FieldReader<SPB_A>;
impl SPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPB_A {
        match self.bits {
            0 => SPB_A::_0000,
            1 => SPB_A::_0001,
            2 => SPB_A::_0010,
            3 => SPB_A::_0011,
            8 => SPB_A::_1000,
            9 => SPB_A::_1001,
            10 => SPB_A::_1010,
            11 => SPB_A::_1011,
            12 => SPB_A::_1100,
            13 => SPB_A::_1101,
            14 => SPB_A::_1110,
            15 => SPB_A::_1111,
            _ => SPB_A::OTHERS,
        }
    }
    ///20 bits
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SPB_A::_0000
    }
    ///24 bits
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SPB_A::_0001
    }
    ///32 bits
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SPB_A::_0010
    }
    ///32 bits
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SPB_A::_0011
    }
    ///9 bits
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SPB_A::_1000
    }
    ///10 bits
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == SPB_A::_1001
    }
    ///11 bits
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == SPB_A::_1010
    }
    ///12 bits
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == SPB_A::_1011
    }
    ///13 bits
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == SPB_A::_1100
    }
    ///14 bits
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == SPB_A::_1101
    }
    ///15 bits
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == SPB_A::_1110
    }
    ///16 bits
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == SPB_A::_1111
    }
    ///8bits
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SPB_A::OTHERS)
    }
}
///Field `SPB` writer - RSPI Data Length Setting
pub type SPB_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SPB_A, crate::Safe>;
impl<'a, REG> SPB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///20 bits
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0000)
    }
    ///24 bits
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0001)
    }
    ///32 bits
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0010)
    }
    ///32 bits
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0011)
    }
    ///9 bits
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1000)
    }
    ///10 bits
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1001)
    }
    ///11 bits
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1010)
    }
    ///12 bits
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1011)
    }
    ///13 bits
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1100)
    }
    ///14 bits
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1101)
    }
    ///15 bits
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1110)
    }
    ///16 bits
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1111)
    }
    ///8bits
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::OTHERS)
    }
}
/**RSPI LSB First

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBF_A {
    ///0: MSB first
    _0 = 0,
    ///1: LSB first
    _1 = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSBF` reader - RSPI LSB First
pub type LSBF_R = crate::BitReader<LSBF_A>;
impl LSBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::_0,
            true => LSBF_A::_1,
        }
    }
    ///MSB first
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSBF_A::_0
    }
    ///LSB first
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSBF_A::_1
    }
}
///Field `LSBF` writer - RSPI LSB First
pub type LSBF_W<'a, REG> = crate::BitWriter<'a, REG, LSBF_A>;
impl<'a, REG> LSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSB first
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LSBF_A::_0)
    }
    ///LSB first
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LSBF_A::_1)
    }
}
/**RSPI Next-Access Delay Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPNDEN_A {
    ///0: A next-access delay of 1 RSPCK + 2 PCLK
    _0 = 0,
    ///1: A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)
    _1 = 1,
}
impl From<SPNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPNDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPNDEN` reader - RSPI Next-Access Delay Enable
pub type SPNDEN_R = crate::BitReader<SPNDEN_A>;
impl SPNDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPNDEN_A {
        match self.bits {
            false => SPNDEN_A::_0,
            true => SPNDEN_A::_1,
        }
    }
    ///A next-access delay of 1 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPNDEN_A::_0
    }
    ///A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPNDEN_A::_1
    }
}
///Field `SPNDEN` writer - RSPI Next-Access Delay Enable
pub type SPNDEN_W<'a, REG> = crate::BitWriter<'a, REG, SPNDEN_A>;
impl<'a, REG> SPNDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A next-access delay of 1 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDEN_A::_0)
    }
    ///A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDEN_A::_1)
    }
}
/**SSL Negation Delay Setting Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLNDEN_A {
    ///0: An SSL negation delay of 1 RSPCK
    _0 = 0,
    ///1: An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)
    _1 = 1,
}
impl From<SLNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLNDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLNDEN` reader - SSL Negation Delay Setting Enable
pub type SLNDEN_R = crate::BitReader<SLNDEN_A>;
impl SLNDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLNDEN_A {
        match self.bits {
            false => SLNDEN_A::_0,
            true => SLNDEN_A::_1,
        }
    }
    ///An SSL negation delay of 1 RSPCK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLNDEN_A::_0
    }
    ///An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLNDEN_A::_1
    }
}
///Field `SLNDEN` writer - SSL Negation Delay Setting Enable
pub type SLNDEN_W<'a, REG> = crate::BitWriter<'a, REG, SLNDEN_A>;
impl<'a, REG> SLNDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///An SSL negation delay of 1 RSPCK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDEN_A::_0)
    }
    ///An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDEN_A::_1)
    }
}
/**RSPCK Delay Setting Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKDEN_A {
    ///0: An RSPCK delay of 1 RSPCK
    _0 = 0,
    ///1: An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)
    _1 = 1,
}
impl From<SCKDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCKDEN` reader - RSPCK Delay Setting Enable
pub type SCKDEN_R = crate::BitReader<SCKDEN_A>;
impl SCKDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCKDEN_A {
        match self.bits {
            false => SCKDEN_A::_0,
            true => SCKDEN_A::_1,
        }
    }
    ///An RSPCK delay of 1 RSPCK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKDEN_A::_0
    }
    ///An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKDEN_A::_1
    }
}
///Field `SCKDEN` writer - RSPCK Delay Setting Enable
pub type SCKDEN_W<'a, REG> = crate::BitWriter<'a, REG, SCKDEN_A>;
impl<'a, REG> SCKDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///An RSPCK delay of 1 RSPCK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDEN_A::_0)
    }
    ///An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDEN_A::_1)
    }
}
impl R {
    ///Bit 0 - RSPCK Phase Setting
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RSPCK Polarity Setting
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Bit Rate Division Setting
    #[inline(always)]
    pub fn brdv(&self) -> BRDV_R {
        BRDV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - SSL Signal Assertion Setting
    #[inline(always)]
    pub fn ssla(&self) -> SSLA_R {
        SSLA_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - SSL Signal Level Keeping
    #[inline(always)]
    pub fn sslkp(&self) -> SSLKP_R {
        SSLKP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - RSPI Data Length Setting
    #[inline(always)]
    pub fn spb(&self) -> SPB_R {
        SPB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - RSPI LSB First
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RSPI Next-Access Delay Enable
    #[inline(always)]
    pub fn spnden(&self) -> SPNDEN_R {
        SPNDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SSL Negation Delay Setting Enable
    #[inline(always)]
    pub fn slnden(&self) -> SLNDEN_R {
        SLNDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RSPCK Delay Setting Enable
    #[inline(always)]
    pub fn sckden(&self) -> SCKDEN_R {
        SCKDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RSPCK Phase Setting
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<SPCMD_SPEC> {
        CPHA_W::new(self, 0)
    }
    ///Bit 1 - RSPCK Polarity Setting
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<SPCMD_SPEC> {
        CPOL_W::new(self, 1)
    }
    ///Bits 2:3 - Bit Rate Division Setting
    #[inline(always)]
    pub fn brdv(&mut self) -> BRDV_W<SPCMD_SPEC> {
        BRDV_W::new(self, 2)
    }
    ///Bits 4:6 - SSL Signal Assertion Setting
    #[inline(always)]
    pub fn ssla(&mut self) -> SSLA_W<SPCMD_SPEC> {
        SSLA_W::new(self, 4)
    }
    ///Bit 7 - SSL Signal Level Keeping
    #[inline(always)]
    pub fn sslkp(&mut self) -> SSLKP_W<SPCMD_SPEC> {
        SSLKP_W::new(self, 7)
    }
    ///Bits 8:11 - RSPI Data Length Setting
    #[inline(always)]
    pub fn spb(&mut self) -> SPB_W<SPCMD_SPEC> {
        SPB_W::new(self, 8)
    }
    ///Bit 12 - RSPI LSB First
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W<SPCMD_SPEC> {
        LSBF_W::new(self, 12)
    }
    ///Bit 13 - RSPI Next-Access Delay Enable
    #[inline(always)]
    pub fn spnden(&mut self) -> SPNDEN_W<SPCMD_SPEC> {
        SPNDEN_W::new(self, 13)
    }
    ///Bit 14 - SSL Negation Delay Setting Enable
    #[inline(always)]
    pub fn slnden(&mut self) -> SLNDEN_W<SPCMD_SPEC> {
        SLNDEN_W::new(self, 14)
    }
    ///Bit 15 - RSPCK Delay Setting Enable
    #[inline(always)]
    pub fn sckden(&mut self) -> SCKDEN_W<SPCMD_SPEC> {
        SCKDEN_W::new(self, 15)
    }
}
/**SPI Command Register %s

You can [`read`](crate::Reg::read) this register and get [`spcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPCMD_SPEC;
impl crate::RegisterSpec for SPCMD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`spcmd::R`](R) reader structure
impl crate::Readable for SPCMD_SPEC {}
///`write(|w| ..)` method takes [`spcmd::W`](W) writer structure
impl crate::Writable for SPCMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCMD%s to value 0x070d
impl crate::Resettable for SPCMD_SPEC {
    const RESET_VALUE: u16 = 0x070d;
}
