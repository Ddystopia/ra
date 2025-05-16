///Register `SSICR` reader
pub type R = crate::R<SSICR_SPEC>;
///Register `SSICR` writer
pub type W = crate::W<SSICR_SPEC>;
/**Receive Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REN_A {
    ///0: Disables the receive operation.
    _0 = 0,
    ///1: Enables the receive operation.
    _1 = 1,
}
impl From<REN_A> for bool {
    #[inline(always)]
    fn from(variant: REN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REN` reader - Receive Enable
pub type REN_R = crate::BitReader<REN_A>;
impl REN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REN_A {
        match self.bits {
            false => REN_A::_0,
            true => REN_A::_1,
        }
    }
    ///Disables the receive operation.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REN_A::_0
    }
    ///Enables the receive operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REN_A::_1
    }
}
///Field `REN` writer - Receive Enable
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG, REN_A>;
impl<'a, REG> REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the receive operation.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(REN_A::_0)
    }
    ///Enables the receive operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(REN_A::_1)
    }
}
/**Transmit Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    ///0: Disables the transmit operation.
    _0 = 0,
    ///1: Enables the transmit operation.
    _1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEN` reader - Transmit Enable
pub type TEN_R = crate::BitReader<TEN_A>;
impl TEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::_0,
            true => TEN_A::_1,
        }
    }
    ///Disables the transmit operation.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEN_A::_0
    }
    ///Enables the transmit operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEN_A::_1
    }
}
///Field `TEN` writer - Transmit Enable
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG, TEN_A>;
impl<'a, REG> TEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the transmit operation.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::_0)
    }
    ///Enables the transmit operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::_1)
    }
}
/**Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUEN_A {
    ///0: This module is not muted.
    _0 = 0,
    ///1: This module is muted.
    _1 = 1,
}
impl From<MUEN_A> for bool {
    #[inline(always)]
    fn from(variant: MUEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MUEN` reader - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing.
pub type MUEN_R = crate::BitReader<MUEN_A>;
impl MUEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MUEN_A {
        match self.bits {
            false => MUEN_A::_0,
            true => MUEN_A::_1,
        }
    }
    ///This module is not muted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MUEN_A::_0
    }
    ///This module is muted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MUEN_A::_1
    }
}
///Field `MUEN` writer - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing.
pub type MUEN_W<'a, REG> = crate::BitWriter<'a, REG, MUEN_A>;
impl<'a, REG> MUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This module is not muted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MUEN_A::_0)
    }
    ///This module is muted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MUEN_A::_1)
    }
}
/**Serial Oversampling Clock Division Ratio

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKDV_A {
    ///0: CLK
    _0X0 = 0,
    ///1: CLK/2
    _0X1 = 1,
    ///2: CLK/4
    _0X2 = 2,
    ///3: CLK/8
    _0X3 = 3,
    ///4: CLK/16
    _0X4 = 4,
    ///5: CLK/32
    _0X5 = 5,
    ///6: CLK/64
    _0X6 = 6,
    ///7: CLK/128
    _0X7 = 7,
    ///8: CLK/6
    _0X8 = 8,
    ///9: CLK/12 (These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    _0X9 = 9,
    ///10: CLK/24
    _0X_A = 10,
    ///11: CLK/48(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    _0X_B = 11,
    ///12: CLK/96(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    _0X_C = 12,
    ///13: Settings other than above are prohibited.
    OTHERS = 13,
}
impl From<CKDV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKDV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKDV_A {
    type Ux = u8;
}
impl crate::IsEnum for CKDV_A {}
///Field `CKDV` reader - Serial Oversampling Clock Division Ratio
pub type CKDV_R = crate::FieldReader<CKDV_A>;
impl CKDV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKDV_A {
        match self.bits {
            0 => CKDV_A::_0X0,
            1 => CKDV_A::_0X1,
            2 => CKDV_A::_0X2,
            3 => CKDV_A::_0X3,
            4 => CKDV_A::_0X4,
            5 => CKDV_A::_0X5,
            6 => CKDV_A::_0X6,
            7 => CKDV_A::_0X7,
            8 => CKDV_A::_0X8,
            9 => CKDV_A::_0X9,
            10 => CKDV_A::_0X_A,
            11 => CKDV_A::_0X_B,
            12 => CKDV_A::_0X_C,
            _ => CKDV_A::OTHERS,
        }
    }
    ///CLK
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CKDV_A::_0X0
    }
    ///CLK/2
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CKDV_A::_0X1
    }
    ///CLK/4
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == CKDV_A::_0X2
    }
    ///CLK/8
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == CKDV_A::_0X3
    }
    ///CLK/16
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CKDV_A::_0X4
    }
    ///CLK/32
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == CKDV_A::_0X5
    }
    ///CLK/64
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == CKDV_A::_0X6
    }
    ///CLK/128
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == CKDV_A::_0X7
    }
    ///CLK/6
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == CKDV_A::_0X8
    }
    ///CLK/12 (These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == CKDV_A::_0X9
    }
    ///CLK/24
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == CKDV_A::_0X_A
    }
    ///CLK/48(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == CKDV_A::_0X_B
    }
    ///CLK/96(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == CKDV_A::_0X_C
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CKDV_A::OTHERS)
    }
}
///Field `CKDV` writer - Serial Oversampling Clock Division Ratio
pub type CKDV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CKDV_A, crate::Safe>;
impl<'a, REG> CKDV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CLK
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X0)
    }
    ///CLK/2
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X1)
    }
    ///CLK/4
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X2)
    }
    ///CLK/8
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X3)
    }
    ///CLK/16
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X4)
    }
    ///CLK/32
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X5)
    }
    ///CLK/64
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X6)
    }
    ///CLK/128
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X7)
    }
    ///CLK/6
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X8)
    }
    ///CLK/12 (These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X9)
    }
    ///CLK/24
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X_A)
    }
    ///CLK/48(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X_B)
    }
    ///CLK/96(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0X_C)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::OTHERS)
    }
}
/**Serial Data Delay

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEL_A {
    ///0: 1 clock cycle delay between SSIWS and SSIDATA
    _0 = 0,
    ///1: No delay between SSIWS and SSIDATA
    _1 = 1,
}
impl From<DEL_A> for bool {
    #[inline(always)]
    fn from(variant: DEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DEL` reader - Serial Data Delay
pub type DEL_R = crate::BitReader<DEL_A>;
impl DEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEL_A {
        match self.bits {
            false => DEL_A::_0,
            true => DEL_A::_1,
        }
    }
    ///1 clock cycle delay between SSIWS and SSIDATA
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEL_A::_0
    }
    ///No delay between SSIWS and SSIDATA
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEL_A::_1
    }
}
///Field `DEL` writer - Serial Data Delay
pub type DEL_W<'a, REG> = crate::BitWriter<'a, REG, DEL_A>;
impl<'a, REG> DEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///1 clock cycle delay between SSIWS and SSIDATA
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DEL_A::_0)
    }
    ///No delay between SSIWS and SSIDATA
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DEL_A::_1)
    }
}
/**Parallel Data Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDTA_A {
    ///0: The lower bits of parallel data (SSITDR, SSIRDR) are transferred prior to the upper bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is left-aligned.(When data word length is 18, 20, 22, or 24 bits)
    _0 = 0,
    ///1: The upper bits of parallel data (SSITDR, SSIRDR) are transferred prior to the lower bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is right-aligned.(When data word length is 18, 20, 22, or 24 bits)
    _1 = 1,
}
impl From<PDTA_A> for bool {
    #[inline(always)]
    fn from(variant: PDTA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDTA` reader - Parallel Data Alignment
pub type PDTA_R = crate::BitReader<PDTA_A>;
impl PDTA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDTA_A {
        match self.bits {
            false => PDTA_A::_0,
            true => PDTA_A::_1,
        }
    }
    ///The lower bits of parallel data (SSITDR, SSIRDR) are transferred prior to the upper bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is left-aligned.(When data word length is 18, 20, 22, or 24 bits)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDTA_A::_0
    }
    ///The upper bits of parallel data (SSITDR, SSIRDR) are transferred prior to the lower bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is right-aligned.(When data word length is 18, 20, 22, or 24 bits)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDTA_A::_1
    }
}
///Field `PDTA` writer - Parallel Data Alignment
pub type PDTA_W<'a, REG> = crate::BitWriter<'a, REG, PDTA_A>;
impl<'a, REG> PDTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The lower bits of parallel data (SSITDR, SSIRDR) are transferred prior to the upper bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is left-aligned.(When data word length is 18, 20, 22, or 24 bits)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDTA_A::_0)
    }
    ///The upper bits of parallel data (SSITDR, SSIRDR) are transferred prior to the lower bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is right-aligned.(When data word length is 18, 20, 22, or 24 bits)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDTA_A::_1)
    }
}
/**Serial Data Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTA_A {
    ///0: Transmitting and receiving in the order of serial data and padding bits
    _0 = 0,
    ///1: Transmitting and receiving in the order of padding bits and serial data
    _1 = 1,
}
impl From<SDTA_A> for bool {
    #[inline(always)]
    fn from(variant: SDTA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTA` reader - Serial Data Alignment
pub type SDTA_R = crate::BitReader<SDTA_A>;
impl SDTA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDTA_A {
        match self.bits {
            false => SDTA_A::_0,
            true => SDTA_A::_1,
        }
    }
    ///Transmitting and receiving in the order of serial data and padding bits
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDTA_A::_0
    }
    ///Transmitting and receiving in the order of padding bits and serial data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDTA_A::_1
    }
}
///Field `SDTA` writer - Serial Data Alignment
pub type SDTA_W<'a, REG> = crate::BitWriter<'a, REG, SDTA_A>;
impl<'a, REG> SDTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitting and receiving in the order of serial data and padding bits
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDTA_A::_0)
    }
    ///Transmitting and receiving in the order of padding bits and serial data
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDTA_A::_1)
    }
}
/**Serial Padding Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDP_A {
    ///0: Padding bits are low.
    _0 = 0,
    ///1: Padding bits are high.
    _1 = 1,
}
impl From<SPDP_A> for bool {
    #[inline(always)]
    fn from(variant: SPDP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPDP` reader - Serial Padding Polarity
pub type SPDP_R = crate::BitReader<SPDP_A>;
impl SPDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPDP_A {
        match self.bits {
            false => SPDP_A::_0,
            true => SPDP_A::_1,
        }
    }
    ///Padding bits are low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDP_A::_0
    }
    ///Padding bits are high.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDP_A::_1
    }
}
///Field `SPDP` writer - Serial Padding Polarity
pub type SPDP_W<'a, REG> = crate::BitWriter<'a, REG, SPDP_A>;
impl<'a, REG> SPDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Padding bits are low.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPDP_A::_0)
    }
    ///Padding bits are high.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPDP_A::_1)
    }
}
/**Serial WS Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSP_A {
    ///0: SSIWS is low for 1st channel, high for 2nd channel.
    _0 = 0,
    ///1: SSIWS is high for 1st channel, low for 2nd channel.
    _1 = 1,
}
impl From<SWSP_A> for bool {
    #[inline(always)]
    fn from(variant: SWSP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSP` reader - Serial WS Polarity
pub type SWSP_R = crate::BitReader<SWSP_A>;
impl SWSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWSP_A {
        match self.bits {
            false => SWSP_A::_0,
            true => SWSP_A::_1,
        }
    }
    ///SSIWS is low for 1st channel, high for 2nd channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSP_A::_0
    }
    ///SSIWS is high for 1st channel, low for 2nd channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSP_A::_1
    }
}
///Field `SWSP` writer - Serial WS Polarity
pub type SWSP_W<'a, REG> = crate::BitWriter<'a, REG, SWSP_A>;
impl<'a, REG> SWSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSIWS is low for 1st channel, high for 2nd channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWSP_A::_0)
    }
    ///SSIWS is high for 1st channel, low for 2nd channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWSP_A::_1)
    }
}
/**Serial Bit Clock Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKP_A {
    ///0: SSIWS and SSIDATA change at the SSISCK falling edge (sampled at the SCK rising edge).
    _0 = 0,
    ///1: SSIWS and SSIDATA change at the SSISCK rising edge (sampled at the SCK falling edge).
    _1 = 1,
}
impl From<SCKP_A> for bool {
    #[inline(always)]
    fn from(variant: SCKP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCKP` reader - Serial Bit Clock Polarity
pub type SCKP_R = crate::BitReader<SCKP_A>;
impl SCKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCKP_A {
        match self.bits {
            false => SCKP_A::_0,
            true => SCKP_A::_1,
        }
    }
    ///SSIWS and SSIDATA change at the SSISCK falling edge (sampled at the SCK rising edge).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKP_A::_0
    }
    ///SSIWS and SSIDATA change at the SSISCK rising edge (sampled at the SCK falling edge).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKP_A::_1
    }
}
///Field `SCKP` writer - Serial Bit Clock Polarity
pub type SCKP_W<'a, REG> = crate::BitWriter<'a, REG, SCKP_A>;
impl<'a, REG> SCKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSIWS and SSIDATA change at the SSISCK falling edge (sampled at the SCK rising edge).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCKP_A::_0)
    }
    ///SSIWS and SSIDATA change at the SSISCK rising edge (sampled at the SCK falling edge).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCKP_A::_1)
    }
}
/**Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSD_A {
    ///0: Serial word select is input, slave mode.
    _0 = 0,
    ///1: Serial word select is output, master mode.
    _1 = 1,
}
impl From<SWSD_A> for bool {
    #[inline(always)]
    fn from(variant: SWSD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSD` reader - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited.
pub type SWSD_R = crate::BitReader<SWSD_A>;
impl SWSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWSD_A {
        match self.bits {
            false => SWSD_A::_0,
            true => SWSD_A::_1,
        }
    }
    ///Serial word select is input, slave mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSD_A::_0
    }
    ///Serial word select is output, master mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSD_A::_1
    }
}
///Field `SWSD` writer - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited.
pub type SWSD_W<'a, REG> = crate::BitWriter<'a, REG, SWSD_A>;
impl<'a, REG> SWSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Serial word select is input, slave mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWSD_A::_0)
    }
    ///Serial word select is output, master mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWSD_A::_1)
    }
}
/**System Word LengthSet the system word length to the bit clock frequency/2 fs.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWL_A {
    ///0: 8 bits (serial bit clock frequency = 16fs )
    _000 = 0,
    ///1: 16 bits (serial bit clock frequency = 32fs )
    _001 = 1,
    ///2: 24 bits (serial bit clock frequency = 48fs )
    _010 = 2,
    ///3: 32 bits (serial bit clock frequency = 64fs )
    _011 = 3,
    ///4: Settings other than above are prohibited.
    OTHERS = 4,
}
impl From<SWL_A> for u8 {
    #[inline(always)]
    fn from(variant: SWL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWL_A {
    type Ux = u8;
}
impl crate::IsEnum for SWL_A {}
///Field `SWL` reader - System Word LengthSet the system word length to the bit clock frequency/2 fs.
pub type SWL_R = crate::FieldReader<SWL_A>;
impl SWL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWL_A {
        match self.bits {
            0 => SWL_A::_000,
            1 => SWL_A::_001,
            2 => SWL_A::_010,
            3 => SWL_A::_011,
            _ => SWL_A::OTHERS,
        }
    }
    ///8 bits (serial bit clock frequency = 16fs )
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SWL_A::_000
    }
    ///16 bits (serial bit clock frequency = 32fs )
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SWL_A::_001
    }
    ///24 bits (serial bit clock frequency = 48fs )
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SWL_A::_010
    }
    ///32 bits (serial bit clock frequency = 64fs )
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SWL_A::_011
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SWL_A::OTHERS)
    }
}
///Field `SWL` writer - System Word LengthSet the system word length to the bit clock frequency/2 fs.
pub type SWL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SWL_A, crate::Safe>;
impl<'a, REG> SWL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits (serial bit clock frequency = 16fs )
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_000)
    }
    ///16 bits (serial bit clock frequency = 32fs )
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_001)
    }
    ///24 bits (serial bit clock frequency = 48fs )
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_010)
    }
    ///32 bits (serial bit clock frequency = 64fs )
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_011)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::OTHERS)
    }
}
/**Data Word Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DWL_A {
    ///0: 8 bits
    _000 = 0,
    ///1: 16 bits
    _001 = 1,
    ///2: 18 bits
    _010 = 2,
    ///3: 20 bits
    _011 = 3,
    ///4: 22 bits
    _100 = 4,
    ///5: 24 bits
    _101 = 5,
    ///6: Settings other than above are prohibited.
    OTHERS = 6,
}
impl From<DWL_A> for u8 {
    #[inline(always)]
    fn from(variant: DWL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DWL_A {
    type Ux = u8;
}
impl crate::IsEnum for DWL_A {}
///Field `DWL` reader - Data Word Length
pub type DWL_R = crate::FieldReader<DWL_A>;
impl DWL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DWL_A {
        match self.bits {
            0 => DWL_A::_000,
            1 => DWL_A::_001,
            2 => DWL_A::_010,
            3 => DWL_A::_011,
            4 => DWL_A::_100,
            5 => DWL_A::_101,
            _ => DWL_A::OTHERS,
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DWL_A::_000
    }
    ///16 bits
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DWL_A::_001
    }
    ///18 bits
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DWL_A::_010
    }
    ///20 bits
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DWL_A::_011
    }
    ///22 bits
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DWL_A::_100
    }
    ///24 bits
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DWL_A::_101
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DWL_A::OTHERS)
    }
}
///Field `DWL` writer - Data Word Length
pub type DWL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DWL_A, crate::Safe>;
impl<'a, REG> DWL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_000)
    }
    ///16 bits
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_001)
    }
    ///18 bits
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_010)
    }
    ///20 bits
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_011)
    }
    ///22 bits
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_100)
    }
    ///24 bits
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_101)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::OTHERS)
    }
}
/**Channels

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHNL_A {
    ///0: One channel
    _00 = 0,
    ///1: Settings other than above are prohibited.
    OTHERS = 1,
}
impl From<CHNL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHNL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHNL_A {
    type Ux = u8;
}
impl crate::IsEnum for CHNL_A {}
///Field `CHNL` reader - Channels
pub type CHNL_R = crate::FieldReader<CHNL_A>;
impl CHNL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHNL_A {
        match self.bits {
            0 => CHNL_A::_00,
            _ => CHNL_A::OTHERS,
        }
    }
    ///One channel
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CHNL_A::_00
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CHNL_A::OTHERS)
    }
}
///Field `CHNL` writer - Channels
pub type CHNL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CHNL_A, crate::Safe>;
impl<'a, REG> CHNL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///One channel
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CHNL_A::_00)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CHNL_A::OTHERS)
    }
}
/**Idle Mode Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIEN_A {
    ///0: Disables an idle mode interrupt.
    _0 = 0,
    ///1: Enables an idle mode interrupt.
    _1 = 1,
}
impl From<IIEN_A> for bool {
    #[inline(always)]
    fn from(variant: IIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IIEN` reader - Idle Mode Interrupt Enable
pub type IIEN_R = crate::BitReader<IIEN_A>;
impl IIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IIEN_A {
        match self.bits {
            false => IIEN_A::_0,
            true => IIEN_A::_1,
        }
    }
    ///Disables an idle mode interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIEN_A::_0
    }
    ///Enables an idle mode interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIEN_A::_1
    }
}
///Field `IIEN` writer - Idle Mode Interrupt Enable
pub type IIEN_W<'a, REG> = crate::BitWriter<'a, REG, IIEN_A>;
impl<'a, REG> IIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an idle mode interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IIEN_A::_0)
    }
    ///Enables an idle mode interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IIEN_A::_1)
    }
}
/**Receive Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIEN_A {
    ///0: Disables an overflow interrupt.
    _0 = 0,
    ///1: Enables an overflow interrupt.
    _1 = 1,
}
impl From<ROIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ROIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ROIEN` reader - Receive Overflow Interrupt Enable
pub type ROIEN_R = crate::BitReader<ROIEN_A>;
impl ROIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROIEN_A {
        match self.bits {
            false => ROIEN_A::_0,
            true => ROIEN_A::_1,
        }
    }
    ///Disables an overflow interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROIEN_A::_0
    }
    ///Enables an overflow interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROIEN_A::_1
    }
}
///Field `ROIEN` writer - Receive Overflow Interrupt Enable
pub type ROIEN_W<'a, REG> = crate::BitWriter<'a, REG, ROIEN_A>;
impl<'a, REG> ROIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an overflow interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ROIEN_A::_0)
    }
    ///Enables an overflow interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ROIEN_A::_1)
    }
}
/**Receive Underflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIEN_A {
    ///0: Disables an underflow interrupt.
    _0 = 0,
    ///1: Enables an underflow interrupt.
    _1 = 1,
}
impl From<RUIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RUIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RUIEN` reader - Receive Underflow Interrupt Enable
pub type RUIEN_R = crate::BitReader<RUIEN_A>;
impl RUIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RUIEN_A {
        match self.bits {
            false => RUIEN_A::_0,
            true => RUIEN_A::_1,
        }
    }
    ///Disables an underflow interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RUIEN_A::_0
    }
    ///Enables an underflow interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RUIEN_A::_1
    }
}
///Field `RUIEN` writer - Receive Underflow Interrupt Enable
pub type RUIEN_W<'a, REG> = crate::BitWriter<'a, REG, RUIEN_A>;
impl<'a, REG> RUIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an underflow interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RUIEN_A::_0)
    }
    ///Enables an underflow interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RUIEN_A::_1)
    }
}
/**Transmit Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIEN_A {
    ///0: Disables an overflow interrupt.
    _0 = 0,
    ///1: Enables an overflow interrupt.
    _1 = 1,
}
impl From<TOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOIEN` reader - Transmit Overflow Interrupt Enable
pub type TOIEN_R = crate::BitReader<TOIEN_A>;
impl TOIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOIEN_A {
        match self.bits {
            false => TOIEN_A::_0,
            true => TOIEN_A::_1,
        }
    }
    ///Disables an overflow interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIEN_A::_0
    }
    ///Enables an overflow interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIEN_A::_1
    }
}
///Field `TOIEN` writer - Transmit Overflow Interrupt Enable
pub type TOIEN_W<'a, REG> = crate::BitWriter<'a, REG, TOIEN_A>;
impl<'a, REG> TOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an overflow interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOIEN_A::_0)
    }
    ///Enables an overflow interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOIEN_A::_1)
    }
}
/**Transmit Underflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUIEN_A {
    ///0: Disables an underflow interrupt.
    _0 = 0,
    ///1: Enables an underflow interrupt.
    _1 = 1,
}
impl From<TUIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TUIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TUIEN` reader - Transmit Underflow Interrupt Enable
pub type TUIEN_R = crate::BitReader<TUIEN_A>;
impl TUIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TUIEN_A {
        match self.bits {
            false => TUIEN_A::_0,
            true => TUIEN_A::_1,
        }
    }
    ///Disables an underflow interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUIEN_A::_0
    }
    ///Enables an underflow interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUIEN_A::_1
    }
}
///Field `TUIEN` writer - Transmit Underflow Interrupt Enable
pub type TUIEN_W<'a, REG> = crate::BitWriter<'a, REG, TUIEN_A>;
impl<'a, REG> TUIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an underflow interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TUIEN_A::_0)
    }
    ///Enables an underflow interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TUIEN_A::_1)
    }
}
/**Oversampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKS_A {
    ///0: AUDIO_CLK input
    _0 = 0,
    ///1: Setting prohibited
    _1 = 1,
}
impl From<CKS_A> for bool {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKS` reader - Oversampling Clock Select
pub type CKS_R = crate::BitReader<CKS_A>;
impl CKS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            false => CKS_A::_0,
            true => CKS_A::_1,
        }
    }
    ///AUDIO_CLK input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKS_A::_0
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKS_A::_1
    }
}
///Field `CKS` writer - Oversampling Clock Select
pub type CKS_W<'a, REG> = crate::BitWriter<'a, REG, CKS_A>;
impl<'a, REG> CKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AUDIO_CLK input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_0)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive Enable
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Enable
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing.
    #[inline(always)]
    pub fn muen(&self) -> MUEN_R {
        MUEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Serial Oversampling Clock Division Ratio
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Serial Data Delay
    #[inline(always)]
    pub fn del(&self) -> DEL_R {
        DEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parallel Data Alignment
    #[inline(always)]
    pub fn pdta(&self) -> PDTA_R {
        PDTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Serial Data Alignment
    #[inline(always)]
    pub fn sdta(&self) -> SDTA_R {
        SDTA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Serial Padding Polarity
    #[inline(always)]
    pub fn spdp(&self) -> SPDP_R {
        SPDP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Serial WS Polarity
    #[inline(always)]
    pub fn swsp(&self) -> SWSP_R {
        SWSP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Serial Bit Clock Polarity
    #[inline(always)]
    pub fn sckp(&self) -> SCKP_R {
        SCKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited.
    #[inline(always)]
    pub fn swsd(&self) -> SWSD_R {
        SWSD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:18 - System Word LengthSet the system word length to the bit clock frequency/2 fs.
    #[inline(always)]
    pub fn swl(&self) -> SWL_R {
        SWL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - Data Word Length
    #[inline(always)]
    pub fn dwl(&self) -> DWL_R {
        DWL_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:23 - Channels
    #[inline(always)]
    pub fn chnl(&self) -> CHNL_R {
        CHNL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 25 - Idle Mode Interrupt Enable
    #[inline(always)]
    pub fn iien(&self) -> IIEN_R {
        IIEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive Overflow Interrupt Enable
    #[inline(always)]
    pub fn roien(&self) -> ROIEN_R {
        ROIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Receive Underflow Interrupt Enable
    #[inline(always)]
    pub fn ruien(&self) -> RUIEN_R {
        RUIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Transmit Overflow Interrupt Enable
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transmit Underflow Interrupt Enable
    #[inline(always)]
    pub fn tuien(&self) -> TUIEN_R {
        TUIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Oversampling Clock Select
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive Enable
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<SSICR_SPEC> {
        REN_W::new(self, 0)
    }
    ///Bit 1 - Transmit Enable
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W<SSICR_SPEC> {
        TEN_W::new(self, 1)
    }
    ///Bit 3 - Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing.
    #[inline(always)]
    pub fn muen(&mut self) -> MUEN_W<SSICR_SPEC> {
        MUEN_W::new(self, 3)
    }
    ///Bits 4:7 - Serial Oversampling Clock Division Ratio
    #[inline(always)]
    pub fn ckdv(&mut self) -> CKDV_W<SSICR_SPEC> {
        CKDV_W::new(self, 4)
    }
    ///Bit 8 - Serial Data Delay
    #[inline(always)]
    pub fn del(&mut self) -> DEL_W<SSICR_SPEC> {
        DEL_W::new(self, 8)
    }
    ///Bit 9 - Parallel Data Alignment
    #[inline(always)]
    pub fn pdta(&mut self) -> PDTA_W<SSICR_SPEC> {
        PDTA_W::new(self, 9)
    }
    ///Bit 10 - Serial Data Alignment
    #[inline(always)]
    pub fn sdta(&mut self) -> SDTA_W<SSICR_SPEC> {
        SDTA_W::new(self, 10)
    }
    ///Bit 11 - Serial Padding Polarity
    #[inline(always)]
    pub fn spdp(&mut self) -> SPDP_W<SSICR_SPEC> {
        SPDP_W::new(self, 11)
    }
    ///Bit 12 - Serial WS Polarity
    #[inline(always)]
    pub fn swsp(&mut self) -> SWSP_W<SSICR_SPEC> {
        SWSP_W::new(self, 12)
    }
    ///Bit 13 - Serial Bit Clock Polarity
    #[inline(always)]
    pub fn sckp(&mut self) -> SCKP_W<SSICR_SPEC> {
        SCKP_W::new(self, 13)
    }
    ///Bit 14 - Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited.
    #[inline(always)]
    pub fn swsd(&mut self) -> SWSD_W<SSICR_SPEC> {
        SWSD_W::new(self, 14)
    }
    ///Bits 16:18 - System Word LengthSet the system word length to the bit clock frequency/2 fs.
    #[inline(always)]
    pub fn swl(&mut self) -> SWL_W<SSICR_SPEC> {
        SWL_W::new(self, 16)
    }
    ///Bits 19:21 - Data Word Length
    #[inline(always)]
    pub fn dwl(&mut self) -> DWL_W<SSICR_SPEC> {
        DWL_W::new(self, 19)
    }
    ///Bits 22:23 - Channels
    #[inline(always)]
    pub fn chnl(&mut self) -> CHNL_W<SSICR_SPEC> {
        CHNL_W::new(self, 22)
    }
    ///Bit 25 - Idle Mode Interrupt Enable
    #[inline(always)]
    pub fn iien(&mut self) -> IIEN_W<SSICR_SPEC> {
        IIEN_W::new(self, 25)
    }
    ///Bit 26 - Receive Overflow Interrupt Enable
    #[inline(always)]
    pub fn roien(&mut self) -> ROIEN_W<SSICR_SPEC> {
        ROIEN_W::new(self, 26)
    }
    ///Bit 27 - Receive Underflow Interrupt Enable
    #[inline(always)]
    pub fn ruien(&mut self) -> RUIEN_W<SSICR_SPEC> {
        RUIEN_W::new(self, 27)
    }
    ///Bit 28 - Transmit Overflow Interrupt Enable
    #[inline(always)]
    pub fn toien(&mut self) -> TOIEN_W<SSICR_SPEC> {
        TOIEN_W::new(self, 28)
    }
    ///Bit 29 - Transmit Underflow Interrupt Enable
    #[inline(always)]
    pub fn tuien(&mut self) -> TUIEN_W<SSICR_SPEC> {
        TUIEN_W::new(self, 29)
    }
    ///Bit 30 - Oversampling Clock Select
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<SSICR_SPEC> {
        CKS_W::new(self, 30)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`ssicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSICR_SPEC;
impl crate::RegisterSpec for SSICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssicr::R`](R) reader structure
impl crate::Readable for SSICR_SPEC {}
///`write(|w| ..)` method takes [`ssicr::W`](W) writer structure
impl crate::Writable for SSICR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSICR to value 0
impl crate::Resettable for SSICR_SPEC {}
