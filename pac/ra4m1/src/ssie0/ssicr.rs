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
/**Mute Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUEN_A {
    ///0: Disables muting on the next frame boundary
    _0 = 0,
    ///1: Enables muting on the next frame boundary.
    _1 = 1,
}
impl From<MUEN_A> for bool {
    #[inline(always)]
    fn from(variant: MUEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MUEN` reader - Mute Enable
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
    ///Disables muting on the next frame boundary
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MUEN_A::_0
    }
    ///Enables muting on the next frame boundary.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MUEN_A::_1
    }
}
///Field `MUEN` writer - Mute Enable
pub type MUEN_W<'a, REG> = crate::BitWriter<'a, REG, MUEN_A>;
impl<'a, REG> MUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables muting on the next frame boundary
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MUEN_A::_0)
    }
    ///Enables muting on the next frame boundary.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MUEN_A::_1)
    }
}
/**Selects Bit Clock Division Ratio

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKDV_A {
    ///0: AUDIO_MCK
    _0000 = 0,
    ///1: AUDIO_MCK/2
    _0001 = 1,
    ///2: AUDIO_MCK/4
    _0010 = 2,
    ///3: AUDIO_MCK/8
    _0011 = 3,
    ///4: AUDIO_MCK/16
    _0100 = 4,
    ///5: AUDIO_MCK/32
    _0101 = 5,
    ///6: AUDIO_MCK/64
    _0110 = 6,
    ///7: AUDIO_MCK/128
    _0111 = 7,
    ///8: AUDIO_MCK/6
    _1000 = 8,
    ///9: AUDIO_MCK/12
    _1001 = 9,
    ///10: AUDIO_MCK/24
    _1010 = 10,
    ///11: AUDIO_MCK/48
    _1011 = 11,
    ///12: AUDIO_MCK/96
    _1100 = 12,
    ///13: Setting prohibited
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
///Field `CKDV` reader - Selects Bit Clock Division Ratio
pub type CKDV_R = crate::FieldReader<CKDV_A>;
impl CKDV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKDV_A {
        match self.bits {
            0 => CKDV_A::_0000,
            1 => CKDV_A::_0001,
            2 => CKDV_A::_0010,
            3 => CKDV_A::_0011,
            4 => CKDV_A::_0100,
            5 => CKDV_A::_0101,
            6 => CKDV_A::_0110,
            7 => CKDV_A::_0111,
            8 => CKDV_A::_1000,
            9 => CKDV_A::_1001,
            10 => CKDV_A::_1010,
            11 => CKDV_A::_1011,
            12 => CKDV_A::_1100,
            _ => CKDV_A::OTHERS,
        }
    }
    ///AUDIO_MCK
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CKDV_A::_0000
    }
    ///AUDIO_MCK/2
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CKDV_A::_0001
    }
    ///AUDIO_MCK/4
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CKDV_A::_0010
    }
    ///AUDIO_MCK/8
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CKDV_A::_0011
    }
    ///AUDIO_MCK/16
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CKDV_A::_0100
    }
    ///AUDIO_MCK/32
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CKDV_A::_0101
    }
    ///AUDIO_MCK/64
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CKDV_A::_0110
    }
    ///AUDIO_MCK/128
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CKDV_A::_0111
    }
    ///AUDIO_MCK/6
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CKDV_A::_1000
    }
    ///AUDIO_MCK/12
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CKDV_A::_1001
    }
    ///AUDIO_MCK/24
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == CKDV_A::_1010
    }
    ///AUDIO_MCK/48
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == CKDV_A::_1011
    }
    ///AUDIO_MCK/96
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == CKDV_A::_1100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CKDV_A::OTHERS)
    }
}
///Field `CKDV` writer - Selects Bit Clock Division Ratio
pub type CKDV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CKDV_A, crate::Safe>;
impl<'a, REG> CKDV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AUDIO_MCK
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0000)
    }
    ///AUDIO_MCK/2
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0001)
    }
    ///AUDIO_MCK/4
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0010)
    }
    ///AUDIO_MCK/8
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0011)
    }
    ///AUDIO_MCK/16
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0100)
    }
    ///AUDIO_MCK/32
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0101)
    }
    ///AUDIO_MCK/64
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0110)
    }
    ///AUDIO_MCK/128
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_0111)
    }
    ///AUDIO_MCK/6
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_1000)
    }
    ///AUDIO_MCK/12
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_1001)
    }
    ///AUDIO_MCK/24
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_1010)
    }
    ///AUDIO_MCK/48
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_1011)
    }
    ///AUDIO_MCK/96
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::_1100)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CKDV_A::OTHERS)
    }
}
/**Selects Serial Data Delay

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEL_A {
    ///0: Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0
    _0 = 0,
    ///1: No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS.
    _1 = 1,
}
impl From<DEL_A> for bool {
    #[inline(always)]
    fn from(variant: DEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DEL` reader - Selects Serial Data Delay
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
    ///Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEL_A::_0
    }
    ///No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEL_A::_1
    }
}
///Field `DEL` writer - Selects Serial Data Delay
pub type DEL_W<'a, REG> = crate::BitWriter<'a, REG, DEL_A>;
impl<'a, REG> DEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DEL_A::_0)
    }
    ///No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DEL_A::_1)
    }
}
/**Selects Placement Data Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDTA_A {
    ///0: Left-justifies placement data (SSIFTDR, SSIFRDR)
    _0 = 0,
    ///1: Right-justifies placement data (SSIFTDR, SSIFRDR).
    _1 = 1,
}
impl From<PDTA_A> for bool {
    #[inline(always)]
    fn from(variant: PDTA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDTA` reader - Selects Placement Data Alignment
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
    ///Left-justifies placement data (SSIFTDR, SSIFRDR)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDTA_A::_0
    }
    ///Right-justifies placement data (SSIFTDR, SSIFRDR).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDTA_A::_1
    }
}
///Field `PDTA` writer - Selects Placement Data Alignment
pub type PDTA_W<'a, REG> = crate::BitWriter<'a, REG, PDTA_A>;
impl<'a, REG> PDTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Left-justifies placement data (SSIFTDR, SSIFRDR)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDTA_A::_0)
    }
    ///Right-justifies placement data (SSIFTDR, SSIFRDR).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDTA_A::_1)
    }
}
/**Selects Serial Data Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTA_A {
    ///0: Transmits and receives serial data first and then padding bits
    _0 = 0,
    ///1: Transmit and receives padding bits first and then serial data.
    _1 = 1,
}
impl From<SDTA_A> for bool {
    #[inline(always)]
    fn from(variant: SDTA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTA` reader - Selects Serial Data Alignment
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
    ///Transmits and receives serial data first and then padding bits
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDTA_A::_0
    }
    ///Transmit and receives padding bits first and then serial data.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDTA_A::_1
    }
}
///Field `SDTA` writer - Selects Serial Data Alignment
pub type SDTA_W<'a, REG> = crate::BitWriter<'a, REG, SDTA_A>;
impl<'a, REG> SDTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmits and receives serial data first and then padding bits
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDTA_A::_0)
    }
    ///Transmit and receives padding bits first and then serial data.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDTA_A::_1)
    }
}
/**Selects Serial Padding Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDP_A {
    ///0: Padding data is at a low level
    _0 = 0,
    ///1: Padding data is at a high level.
    _1 = 1,
}
impl From<SPDP_A> for bool {
    #[inline(always)]
    fn from(variant: SPDP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPDP` reader - Selects Serial Padding Polarity
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
    ///Padding data is at a low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDP_A::_0
    }
    ///Padding data is at a high level.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDP_A::_1
    }
}
///Field `SPDP` writer - Selects Serial Padding Polarity
pub type SPDP_W<'a, REG> = crate::BitWriter<'a, REG, SPDP_A>;
impl<'a, REG> SPDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Padding data is at a low level
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPDP_A::_0)
    }
    ///Padding data is at a high level.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPDP_A::_1)
    }
}
/**Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCKP_A {
    ///0: The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS
    _0 = 0,
    ///1: The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS.
    _1 = 1,
}
impl From<LRCKP_A> for bool {
    #[inline(always)]
    fn from(variant: LRCKP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LRCKP` reader - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
pub type LRCKP_R = crate::BitReader<LRCKP_A>;
impl LRCKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LRCKP_A {
        match self.bits {
            false => LRCKP_A::_0,
            true => LRCKP_A::_1,
        }
    }
    ///The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRCKP_A::_0
    }
    ///The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRCKP_A::_1
    }
}
///Field `LRCKP` writer - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
pub type LRCKP_W<'a, REG> = crate::BitWriter<'a, REG, LRCKP_A>;
impl<'a, REG> LRCKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LRCKP_A::_0)
    }
    ///The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LRCKP_A::_1)
    }
}
/**Selects Bit Clock Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCKP_A {
    ///0: SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)
    _0 = 0,
    ///1: SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK).
    _1 = 1,
}
impl From<BCKP_A> for bool {
    #[inline(always)]
    fn from(variant: BCKP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCKP` reader - Selects Bit Clock Polarity
pub type BCKP_R = crate::BitReader<BCKP_A>;
impl BCKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCKP_A {
        match self.bits {
            false => BCKP_A::_0,
            true => BCKP_A::_1,
        }
    }
    ///SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCKP_A::_0
    }
    ///SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCKP_A::_1
    }
}
///Field `BCKP` writer - Selects Bit Clock Polarity
pub type BCKP_W<'a, REG> = crate::BitWriter<'a, REG, BCKP_A>;
impl<'a, REG> BCKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCKP_A::_0)
    }
    ///SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCKP_A::_1)
    }
}
/**Master Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_A {
    ///0: Slave-mode communication
    _0 = 0,
    ///1: Master-mode communication.
    _1 = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MST` reader - Master Enable
pub type MST_R = crate::BitReader<MST_A>;
impl MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::_0,
            true => MST_A::_1,
        }
    }
    ///Slave-mode communication
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MST_A::_0
    }
    ///Master-mode communication.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MST_A::_1
    }
}
///Field `MST` writer - Master Enable
pub type MST_W<'a, REG> = crate::BitWriter<'a, REG, MST_A>;
impl<'a, REG> MST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave-mode communication
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MST_A::_0)
    }
    ///Master-mode communication.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MST_A::_1)
    }
}
/**Selects System Word Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWL_A {
    ///0: 8 bits
    _000 = 0,
    ///1: 16 bits
    _001 = 1,
    ///2: 24 bits
    _010 = 2,
    ///3: 32 bits
    _011 = 3,
    ///4: 48 bits
    _100 = 4,
    ///5: 64 bits
    _101 = 5,
    ///6: 128 bits
    _110 = 6,
    ///7: 256 bits.
    _111 = 7,
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
///Field `SWL` reader - Selects System Word Length
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
            4 => SWL_A::_100,
            5 => SWL_A::_101,
            6 => SWL_A::_110,
            7 => SWL_A::_111,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SWL_A::_000
    }
    ///16 bits
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SWL_A::_001
    }
    ///24 bits
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SWL_A::_010
    }
    ///32 bits
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SWL_A::_011
    }
    ///48 bits
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SWL_A::_100
    }
    ///64 bits
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SWL_A::_101
    }
    ///128 bits
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SWL_A::_110
    }
    ///256 bits.
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SWL_A::_111
    }
}
///Field `SWL` writer - Selects System Word Length
pub type SWL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SWL_A, crate::Safe>;
impl<'a, REG> SWL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_000)
    }
    ///16 bits
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_001)
    }
    ///24 bits
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_010)
    }
    ///32 bits
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_011)
    }
    ///48 bits
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_100)
    }
    ///64 bits
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_101)
    }
    ///128 bits
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_110)
    }
    ///256 bits.
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SWL_A::_111)
    }
}
/**Selects Data Word Length

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
    ///6: 32 bits
    _110 = 6,
    ///7: Settings other than above are prohibited.
    OTHERS = 7,
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
///Field `DWL` reader - Selects Data Word Length
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
            6 => DWL_A::_110,
            7 => DWL_A::OTHERS,
            _ => unreachable!(),
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
    ///32 bits
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DWL_A::_110
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == DWL_A::OTHERS
    }
}
///Field `DWL` writer - Selects Data Word Length
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
    ///32 bits
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::_110)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DWL_A::OTHERS)
    }
}
/**Idle Mode Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIEN_A {
    ///0: Disables idle mode interrupt output
    _0 = 0,
    ///1: Enables idle mode interrupt output.
    _1 = 1,
}
impl From<IIEN_A> for bool {
    #[inline(always)]
    fn from(variant: IIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IIEN` reader - Idle Mode Interrupt Output Enable
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
    ///Disables idle mode interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIEN_A::_0
    }
    ///Enables idle mode interrupt output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIEN_A::_1
    }
}
///Field `IIEN` writer - Idle Mode Interrupt Output Enable
pub type IIEN_W<'a, REG> = crate::BitWriter<'a, REG, IIEN_A>;
impl<'a, REG> IIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables idle mode interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IIEN_A::_0)
    }
    ///Enables idle mode interrupt output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IIEN_A::_1)
    }
}
/**Receive Overflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIEN_A {
    ///0: Disables receive overflow interrupt output
    _0 = 0,
    ///1: Enables receive overflow interrupt output.
    _1 = 1,
}
impl From<ROIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ROIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ROIEN` reader - Receive Overflow Interrupt Output Enable
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
    ///Disables receive overflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROIEN_A::_0
    }
    ///Enables receive overflow interrupt output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROIEN_A::_1
    }
}
///Field `ROIEN` writer - Receive Overflow Interrupt Output Enable
pub type ROIEN_W<'a, REG> = crate::BitWriter<'a, REG, ROIEN_A>;
impl<'a, REG> ROIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables receive overflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ROIEN_A::_0)
    }
    ///Enables receive overflow interrupt output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ROIEN_A::_1)
    }
}
/**Receive Underflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIEN_A {
    ///0: Disables receive underflow interrupt output
    _0 = 0,
    ///1: Enables receive underflow interrupt output.
    _1 = 1,
}
impl From<RUIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RUIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RUIEN` reader - Receive Underflow Interrupt Output Enable
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
    ///Disables receive underflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RUIEN_A::_0
    }
    ///Enables receive underflow interrupt output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RUIEN_A::_1
    }
}
///Field `RUIEN` writer - Receive Underflow Interrupt Output Enable
pub type RUIEN_W<'a, REG> = crate::BitWriter<'a, REG, RUIEN_A>;
impl<'a, REG> RUIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables receive underflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RUIEN_A::_0)
    }
    ///Enables receive underflow interrupt output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RUIEN_A::_1)
    }
}
/**Transmit Overflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIEN_A {
    ///0: Disables transmit overflow interrupt output
    _0 = 0,
    ///1: Enables transmit overflow interrupt output.
    _1 = 1,
}
impl From<TOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOIEN` reader - Transmit Overflow Interrupt Output Enable
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
    ///Disables transmit overflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIEN_A::_0
    }
    ///Enables transmit overflow interrupt output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIEN_A::_1
    }
}
///Field `TOIEN` writer - Transmit Overflow Interrupt Output Enable
pub type TOIEN_W<'a, REG> = crate::BitWriter<'a, REG, TOIEN_A>;
impl<'a, REG> TOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmit overflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOIEN_A::_0)
    }
    ///Enables transmit overflow interrupt output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOIEN_A::_1)
    }
}
/**Transmit Underflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUIEN_A {
    ///0: Disables transmit underflow interrupt output
    _0 = 0,
    ///1: Enables transmit underflow interrupt output.
    _1 = 1,
}
impl From<TUIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TUIEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TUIEN` reader - Transmit Underflow Interrupt Output Enable
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
    ///Disables transmit underflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUIEN_A::_0
    }
    ///Enables transmit underflow interrupt output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUIEN_A::_1
    }
}
///Field `TUIEN` writer - Transmit Underflow Interrupt Output Enable
pub type TUIEN_W<'a, REG> = crate::BitWriter<'a, REG, TUIEN_A>;
impl<'a, REG> TUIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmit underflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TUIEN_A::_0)
    }
    ///Enables transmit underflow interrupt output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TUIEN_A::_1)
    }
}
/**Selects an Audio Clock for Master-mode Communication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKS_A {
    ///0: Selects the AUDIO_CLK input
    _0 = 0,
    ///1: Selects the GTIOC1A (GPT output).
    _1 = 1,
}
impl From<CKS_A> for bool {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKS` reader - Selects an Audio Clock for Master-mode Communication
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
    ///Selects the AUDIO_CLK input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKS_A::_0
    }
    ///Selects the GTIOC1A (GPT output).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKS_A::_1
    }
}
///Field `CKS` writer - Selects an Audio Clock for Master-mode Communication
pub type CKS_W<'a, REG> = crate::BitWriter<'a, REG, CKS_A>;
impl<'a, REG> CKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selects the AUDIO_CLK input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_0)
    }
    ///Selects the GTIOC1A (GPT output).
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
    ///Bit 3 - Mute Enable
    #[inline(always)]
    pub fn muen(&self) -> MUEN_R {
        MUEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Selects Bit Clock Division Ratio
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Selects Serial Data Delay
    #[inline(always)]
    pub fn del(&self) -> DEL_R {
        DEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Selects Placement Data Alignment
    #[inline(always)]
    pub fn pdta(&self) -> PDTA_R {
        PDTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Selects Serial Data Alignment
    #[inline(always)]
    pub fn sdta(&self) -> SDTA_R {
        SDTA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Selects Serial Padding Polarity
    #[inline(always)]
    pub fn spdp(&self) -> SPDP_R {
        SPDP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
    #[inline(always)]
    pub fn lrckp(&self) -> LRCKP_R {
        LRCKP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Selects Bit Clock Polarity
    #[inline(always)]
    pub fn bckp(&self) -> BCKP_R {
        BCKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Master Enable
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:18 - Selects System Word Length
    #[inline(always)]
    pub fn swl(&self) -> SWL_R {
        SWL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - Selects Data Word Length
    #[inline(always)]
    pub fn dwl(&self) -> DWL_R {
        DWL_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 25 - Idle Mode Interrupt Output Enable
    #[inline(always)]
    pub fn iien(&self) -> IIEN_R {
        IIEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn roien(&self) -> ROIEN_R {
        ROIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Receive Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn ruien(&self) -> RUIEN_R {
        RUIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Transmit Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn toien(&self) -> TOIEN_R {
        TOIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transmit Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn tuien(&self) -> TUIEN_R {
        TUIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Selects an Audio Clock for Master-mode Communication
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
    ///Bit 3 - Mute Enable
    #[inline(always)]
    pub fn muen(&mut self) -> MUEN_W<SSICR_SPEC> {
        MUEN_W::new(self, 3)
    }
    ///Bits 4:7 - Selects Bit Clock Division Ratio
    #[inline(always)]
    pub fn ckdv(&mut self) -> CKDV_W<SSICR_SPEC> {
        CKDV_W::new(self, 4)
    }
    ///Bit 8 - Selects Serial Data Delay
    #[inline(always)]
    pub fn del(&mut self) -> DEL_W<SSICR_SPEC> {
        DEL_W::new(self, 8)
    }
    ///Bit 9 - Selects Placement Data Alignment
    #[inline(always)]
    pub fn pdta(&mut self) -> PDTA_W<SSICR_SPEC> {
        PDTA_W::new(self, 9)
    }
    ///Bit 10 - Selects Serial Data Alignment
    #[inline(always)]
    pub fn sdta(&mut self) -> SDTA_W<SSICR_SPEC> {
        SDTA_W::new(self, 10)
    }
    ///Bit 11 - Selects Serial Padding Polarity
    #[inline(always)]
    pub fn spdp(&mut self) -> SPDP_W<SSICR_SPEC> {
        SPDP_W::new(self, 11)
    }
    ///Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
    #[inline(always)]
    pub fn lrckp(&mut self) -> LRCKP_W<SSICR_SPEC> {
        LRCKP_W::new(self, 12)
    }
    ///Bit 13 - Selects Bit Clock Polarity
    #[inline(always)]
    pub fn bckp(&mut self) -> BCKP_W<SSICR_SPEC> {
        BCKP_W::new(self, 13)
    }
    ///Bit 14 - Master Enable
    #[inline(always)]
    pub fn mst(&mut self) -> MST_W<SSICR_SPEC> {
        MST_W::new(self, 14)
    }
    ///Bits 16:18 - Selects System Word Length
    #[inline(always)]
    pub fn swl(&mut self) -> SWL_W<SSICR_SPEC> {
        SWL_W::new(self, 16)
    }
    ///Bits 19:21 - Selects Data Word Length
    #[inline(always)]
    pub fn dwl(&mut self) -> DWL_W<SSICR_SPEC> {
        DWL_W::new(self, 19)
    }
    ///Bit 25 - Idle Mode Interrupt Output Enable
    #[inline(always)]
    pub fn iien(&mut self) -> IIEN_W<SSICR_SPEC> {
        IIEN_W::new(self, 25)
    }
    ///Bit 26 - Receive Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn roien(&mut self) -> ROIEN_W<SSICR_SPEC> {
        ROIEN_W::new(self, 26)
    }
    ///Bit 27 - Receive Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn ruien(&mut self) -> RUIEN_W<SSICR_SPEC> {
        RUIEN_W::new(self, 27)
    }
    ///Bit 28 - Transmit Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn toien(&mut self) -> TOIEN_W<SSICR_SPEC> {
        TOIEN_W::new(self, 28)
    }
    ///Bit 29 - Transmit Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn tuien(&mut self) -> TUIEN_W<SSICR_SPEC> {
        TUIEN_W::new(self, 29)
    }
    ///Bit 30 - Selects an Audio Clock for Master-mode Communication
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
