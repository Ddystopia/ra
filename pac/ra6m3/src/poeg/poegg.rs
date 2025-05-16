///Register `POEGG%s` reader
pub type R = crate::R<POEGG_SPEC>;
///Register `POEGG%s` writer
pub type W = crate::W<POEGG_SPEC>;
/**Port Input Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDF_A {
    ///0: A output-disable request from the GTETRG pin has not been generated.
    _0 = 0,
    ///1: A output-disable request from the GTETRG pin has been generated.
    _1 = 1,
}
impl From<PIDF_A> for bool {
    #[inline(always)]
    fn from(variant: PIDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIDF` reader - Port Input Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIDF_R = crate::BitReader<PIDF_A>;
impl PIDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIDF_A {
        match self.bits {
            false => PIDF_A::_0,
            true => PIDF_A::_1,
        }
    }
    ///A output-disable request from the GTETRG pin has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDF_A::_0
    }
    ///A output-disable request from the GTETRG pin has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDF_A::_1
    }
}
///Field `PIDF` writer - Port Input Detection Flag
pub type PIDF_W<'a, REG> = crate::BitWriter0C<'a, REG, PIDF_A>;
impl<'a, REG> PIDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A output-disable request from the GTETRG pin has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIDF_A::_0)
    }
    ///A output-disable request from the GTETRG pin has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIDF_A::_1)
    }
}
/**Real Time Overcurrent Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCF_A {
    ///0: A output-disable request from GPT disable request or comparator interrupt has not been generated.
    _0 = 0,
    ///1: A output-disable request from GPT disable request or comparator interrupt has been generated.
    _1 = 1,
}
impl From<IOCF_A> for bool {
    #[inline(always)]
    fn from(variant: IOCF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `IOCF` reader - Real Time Overcurrent Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type IOCF_R = crate::BitReader<IOCF_A>;
impl IOCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOCF_A {
        match self.bits {
            false => IOCF_A::_0,
            true => IOCF_A::_1,
        }
    }
    ///A output-disable request from GPT disable request or comparator interrupt has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCF_A::_0
    }
    ///A output-disable request from GPT disable request or comparator interrupt has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCF_A::_1
    }
}
///Field `IOCF` writer - Real Time Overcurrent Detection Flag
pub type IOCF_W<'a, REG> = crate::BitWriter0C<'a, REG, IOCF_A>;
impl<'a, REG> IOCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A output-disable request from GPT disable request or comparator interrupt has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOCF_A::_0)
    }
    ///A output-disable request from GPT disable request or comparator interrupt has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOCF_A::_1)
    }
}
/**Oscillation Stop Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTPF_A {
    ///0: A output-disable request from the oscillation stop detection has not been generated.
    _0 = 0,
    ///1: A output-disable request from the oscillation stop detection has been generated.
    _1 = 1,
}
impl From<OSTPF_A> for bool {
    #[inline(always)]
    fn from(variant: OSTPF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OSTPF` reader - Oscillation Stop Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OSTPF_R = crate::BitReader<OSTPF_A>;
impl OSTPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTPF_A {
        match self.bits {
            false => OSTPF_A::_0,
            true => OSTPF_A::_1,
        }
    }
    ///A output-disable request from the oscillation stop detection has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTPF_A::_0
    }
    ///A output-disable request from the oscillation stop detection has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTPF_A::_1
    }
}
///Field `OSTPF` writer - Oscillation Stop Detection Flag
pub type OSTPF_W<'a, REG> = crate::BitWriter0C<'a, REG, OSTPF_A>;
impl<'a, REG> OSTPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A output-disable request from the oscillation stop detection has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPF_A::_0)
    }
    ///A output-disable request from the oscillation stop detection has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPF_A::_1)
    }
}
/**Software Stop Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSF_A {
    ///0: A output-disable request from software has not been generated.
    _0 = 0,
    ///1: A output-disable request from software has been generated.
    _1 = 1,
}
impl From<SSF_A> for bool {
    #[inline(always)]
    fn from(variant: SSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSF` reader - Software Stop Flag
pub type SSF_R = crate::BitReader<SSF_A>;
impl SSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSF_A {
        match self.bits {
            false => SSF_A::_0,
            true => SSF_A::_1,
        }
    }
    ///A output-disable request from software has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSF_A::_0
    }
    ///A output-disable request from software has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSF_A::_1
    }
}
///Field `SSF` writer - Software Stop Flag
pub type SSF_W<'a, REG> = crate::BitWriter<'a, REG, SSF_A>;
impl<'a, REG> SSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A output-disable request from software has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSF_A::_0)
    }
    ///A output-disable request from software has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSF_A::_1)
    }
}
/**Port Input Detection EnableNote: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDE_A {
    ///0: A output-disable request from the GTETRG pins disabled.
    _0 = 0,
    ///1: A output-disable request from the GTETRG pins enabled.
    _1 = 1,
}
impl From<PIDE_A> for bool {
    #[inline(always)]
    fn from(variant: PIDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDE` reader - Port Input Detection EnableNote: Can be modified only once after a reset.
pub type PIDE_R = crate::BitReader<PIDE_A>;
impl PIDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIDE_A {
        match self.bits {
            false => PIDE_A::_0,
            true => PIDE_A::_1,
        }
    }
    ///A output-disable request from the GTETRG pins disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDE_A::_0
    }
    ///A output-disable request from the GTETRG pins enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDE_A::_1
    }
}
///Field `PIDE` writer - Port Input Detection EnableNote: Can be modified only once after a reset.
pub type PIDE_W<'a, REG> = crate::BitWriter<'a, REG, PIDE_A>;
impl<'a, REG> PIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A output-disable request from the GTETRG pins disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIDE_A::_0)
    }
    ///A output-disable request from the GTETRG pins enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIDE_A::_1)
    }
}
/**Enable for GPT Output-Disable RequestNote: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCE_A {
    ///0: Disable output-disable requests from GPT disable request
    _0 = 0,
    ///1: Enable output-disable requests from GPT disable request
    _1 = 1,
}
impl From<IOCE_A> for bool {
    #[inline(always)]
    fn from(variant: IOCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IOCE` reader - Enable for GPT Output-Disable RequestNote: Can be modified only once after a reset.
pub type IOCE_R = crate::BitReader<IOCE_A>;
impl IOCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOCE_A {
        match self.bits {
            false => IOCE_A::_0,
            true => IOCE_A::_1,
        }
    }
    ///Disable output-disable requests from GPT disable request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCE_A::_0
    }
    ///Enable output-disable requests from GPT disable request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCE_A::_1
    }
}
///Field `IOCE` writer - Enable for GPT Output-Disable RequestNote: Can be modified only once after a reset.
pub type IOCE_W<'a, REG> = crate::BitWriter<'a, REG, IOCE_A>;
impl<'a, REG> IOCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output-disable requests from GPT disable request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOCE_A::_0)
    }
    ///Enable output-disable requests from GPT disable request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOCE_A::_1)
    }
}
/**Oscillation Stop Detection EnableNote: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTPE_A {
    ///0: A output-disable request from the oscillation stop detection disabled.
    _0 = 0,
    ///1: A output-disable request from the oscillation stop detection enabled.
    _1 = 1,
}
impl From<OSTPE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTPE` reader - Oscillation Stop Detection EnableNote: Can be modified only once after a reset.
pub type OSTPE_R = crate::BitReader<OSTPE_A>;
impl OSTPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTPE_A {
        match self.bits {
            false => OSTPE_A::_0,
            true => OSTPE_A::_1,
        }
    }
    ///A output-disable request from the oscillation stop detection disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTPE_A::_0
    }
    ///A output-disable request from the oscillation stop detection enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTPE_A::_1
    }
}
///Field `OSTPE` writer - Oscillation Stop Detection EnableNote: Can be modified only once after a reset.
pub type OSTPE_W<'a, REG> = crate::BitWriter<'a, REG, OSTPE_A>;
impl<'a, REG> OSTPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A output-disable request from the oscillation stop detection disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPE_A::_0)
    }
    ///A output-disable request from the oscillation stop detection enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTPE_A::_1)
    }
}
/**Comparator Disable Request Enable 0Note: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE0_A {
    ///0: A disable request of comparator 0 disabled.
    _0 = 0,
    ///1: A disable request of comparator 0 enabled.
    _1 = 1,
}
impl From<CDRE0_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRE0` reader - Comparator Disable Request Enable 0Note: Can be modified only once after a reset.
pub type CDRE0_R = crate::BitReader<CDRE0_A>;
impl CDRE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRE0_A {
        match self.bits {
            false => CDRE0_A::_0,
            true => CDRE0_A::_1,
        }
    }
    ///A disable request of comparator 0 disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE0_A::_0
    }
    ///A disable request of comparator 0 enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE0_A::_1
    }
}
///Field `CDRE0` writer - Comparator Disable Request Enable 0Note: Can be modified only once after a reset.
pub type CDRE0_W<'a, REG> = crate::BitWriter<'a, REG, CDRE0_A>;
impl<'a, REG> CDRE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A disable request of comparator 0 disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE0_A::_0)
    }
    ///A disable request of comparator 0 enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE0_A::_1)
    }
}
/**Comparator Disable Request Enable 1Note: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE1_A {
    ///0: A disable request of comparator 1 disabled.
    _0 = 0,
    ///1: A disable request of comparator 1 enabled.
    _1 = 1,
}
impl From<CDRE1_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRE1` reader - Comparator Disable Request Enable 1Note: Can be modified only once after a reset.
pub type CDRE1_R = crate::BitReader<CDRE1_A>;
impl CDRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRE1_A {
        match self.bits {
            false => CDRE1_A::_0,
            true => CDRE1_A::_1,
        }
    }
    ///A disable request of comparator 1 disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE1_A::_0
    }
    ///A disable request of comparator 1 enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE1_A::_1
    }
}
///Field `CDRE1` writer - Comparator Disable Request Enable 1Note: Can be modified only once after a reset.
pub type CDRE1_W<'a, REG> = crate::BitWriter<'a, REG, CDRE1_A>;
impl<'a, REG> CDRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A disable request of comparator 1 disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE1_A::_0)
    }
    ///A disable request of comparator 1 enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE1_A::_1)
    }
}
/**Comparator Disable Request Enable 2Note: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE2_A {
    ///0: A disable request of comparator 2 disabled.
    _0 = 0,
    ///1: A disable request of comparator 2 enabled.
    _1 = 1,
}
impl From<CDRE2_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRE2` reader - Comparator Disable Request Enable 2Note: Can be modified only once after a reset.
pub type CDRE2_R = crate::BitReader<CDRE2_A>;
impl CDRE2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRE2_A {
        match self.bits {
            false => CDRE2_A::_0,
            true => CDRE2_A::_1,
        }
    }
    ///A disable request of comparator 2 disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE2_A::_0
    }
    ///A disable request of comparator 2 enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE2_A::_1
    }
}
///Field `CDRE2` writer - Comparator Disable Request Enable 2Note: Can be modified only once after a reset.
pub type CDRE2_W<'a, REG> = crate::BitWriter<'a, REG, CDRE2_A>;
impl<'a, REG> CDRE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A disable request of comparator 2 disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE2_A::_0)
    }
    ///A disable request of comparator 2 enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE2_A::_1)
    }
}
/**Comparator Disable Request Enable 3Note: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE3_A {
    ///0: A disable request of comparator 3 disabled.
    _0 = 0,
    ///1: A disable request of comparator 3 enabled.
    _1 = 1,
}
impl From<CDRE3_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRE3` reader - Comparator Disable Request Enable 3Note: Can be modified only once after a reset.
pub type CDRE3_R = crate::BitReader<CDRE3_A>;
impl CDRE3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRE3_A {
        match self.bits {
            false => CDRE3_A::_0,
            true => CDRE3_A::_1,
        }
    }
    ///A disable request of comparator 3 disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE3_A::_0
    }
    ///A disable request of comparator 3 enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE3_A::_1
    }
}
///Field `CDRE3` writer - Comparator Disable Request Enable 3Note: Can be modified only once after a reset.
pub type CDRE3_W<'a, REG> = crate::BitWriter<'a, REG, CDRE3_A>;
impl<'a, REG> CDRE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A disable request of comparator 3 disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE3_A::_0)
    }
    ///A disable request of comparator 3 enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE3_A::_1)
    }
}
/**Comparator Disable Request Enable 4Note: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE4_A {
    ///0: A disable request of comparator 4 disabled.
    _0 = 0,
    ///1: A disable request of comparator 4 enabled.
    _1 = 1,
}
impl From<CDRE4_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRE4` reader - Comparator Disable Request Enable 4Note: Can be modified only once after a reset.
pub type CDRE4_R = crate::BitReader<CDRE4_A>;
impl CDRE4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRE4_A {
        match self.bits {
            false => CDRE4_A::_0,
            true => CDRE4_A::_1,
        }
    }
    ///A disable request of comparator 4 disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE4_A::_0
    }
    ///A disable request of comparator 4 enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE4_A::_1
    }
}
///Field `CDRE4` writer - Comparator Disable Request Enable 4Note: Can be modified only once after a reset.
pub type CDRE4_W<'a, REG> = crate::BitWriter<'a, REG, CDRE4_A>;
impl<'a, REG> CDRE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A disable request of comparator 4 disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE4_A::_0)
    }
    ///A disable request of comparator 4 enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE4_A::_1)
    }
}
/**Comparator Disable Request Enable 5Note: Can be modified only once after a reset.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE5_A {
    ///0: A disable request of comparator 5 disabled.
    _0 = 0,
    ///1: A disable request of comparator 5 enabled.
    _1 = 1,
}
impl From<CDRE5_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRE5` reader - Comparator Disable Request Enable 5Note: Can be modified only once after a reset.
pub type CDRE5_R = crate::BitReader<CDRE5_A>;
impl CDRE5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRE5_A {
        match self.bits {
            false => CDRE5_A::_0,
            true => CDRE5_A::_1,
        }
    }
    ///A disable request of comparator 5 disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE5_A::_0
    }
    ///A disable request of comparator 5 enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE5_A::_1
    }
}
///Field `CDRE5` writer - Comparator Disable Request Enable 5Note: Can be modified only once after a reset.
pub type CDRE5_W<'a, REG> = crate::BitWriter<'a, REG, CDRE5_A>;
impl<'a, REG> CDRE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A disable request of comparator 5 disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE5_A::_0)
    }
    ///A disable request of comparator 5 enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CDRE5_A::_1)
    }
}
/**GTETRG Input Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    ///0: GTETRG input after filtering is 0.
    _0 = 0,
    ///1: GTETRG input after filtering is 1.
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ST` reader - GTETRG Input Status Flag
pub type ST_R = crate::BitReader<ST_A>;
impl ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    ///GTETRG input after filtering is 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    ///GTETRG input after filtering is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
/**GTETRG Input Reverse

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    ///0: GTETRG Input
    _0 = 0,
    ///1: GTETRG Input Reversed.
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INV` reader - GTETRG Input Reverse
pub type INV_R = crate::BitReader<INV_A>;
impl INV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    ///GTETRG Input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    ///GTETRG Input Reversed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
///Field `INV` writer - GTETRG Input Reverse
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG, INV_A>;
impl<'a, REG> INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTETRG Input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_0)
    }
    ///GTETRG Input Reversed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_1)
    }
}
/**Noise Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    ///0: Filtering noise disabled
    _0 = 0,
    ///1: Filtering noise enabled
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFEN` reader - Noise Filter Enable
pub type NFEN_R = crate::BitReader<NFEN_A>;
impl NFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    ///Filtering noise disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    ///Filtering noise enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
///Field `NFEN` writer - Noise Filter Enable
pub type NFEN_W<'a, REG> = crate::BitWriter<'a, REG, NFEN_A>;
impl<'a, REG> NFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Filtering noise disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_0)
    }
    ///Filtering noise enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_1)
    }
}
/**Noise Filter Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    ///0: Sampling GTETRG pin input level for three times in every PCLKB.
    _00 = 0,
    ///1: Sampling GTETRG pin input level for three times in every PCLKB /8.
    _01 = 1,
    ///2: Sampling GTETRG pin input level for three times in every PCLKB /32.
    _10 = 2,
    ///3: Sampling GTETRG pin input level for three times in every PCLKB /128.
    _11 = 3,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCS_A {
    type Ux = u8;
}
impl crate::IsEnum for NFCS_A {}
///Field `NFCS` reader - Noise Filter Clock Select
pub type NFCS_R = crate::FieldReader<NFCS_A>;
impl NFCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_00,
            1 => NFCS_A::_01,
            2 => NFCS_A::_10,
            3 => NFCS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCS_A::_00
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB /8.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCS_A::_01
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB /32.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCS_A::_10
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB /128.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCS_A::_11
    }
}
///Field `NFCS` writer - Noise Filter Clock Select
pub type NFCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NFCS_A, crate::Safe>;
impl<'a, REG> NFCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sampling GTETRG pin input level for three times in every PCLKB.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_00)
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB /8.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_01)
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB /32.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_10)
    }
    ///Sampling GTETRG pin input level for three times in every PCLKB /128.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_11)
    }
}
impl R {
    ///Bit 0 - Port Input Detection Flag
    #[inline(always)]
    pub fn pidf(&self) -> PIDF_R {
        PIDF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Real Time Overcurrent Detection Flag
    #[inline(always)]
    pub fn iocf(&self) -> IOCF_R {
        IOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostpf(&self) -> OSTPF_R {
        OSTPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software Stop Flag
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port Input Detection EnableNote: Can be modified only once after a reset.
    #[inline(always)]
    pub fn pide(&self) -> PIDE_R {
        PIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable for GPT Output-Disable RequestNote: Can be modified only once after a reset.
    #[inline(always)]
    pub fn ioce(&self) -> IOCE_R {
        IOCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Oscillation Stop Detection EnableNote: Can be modified only once after a reset.
    #[inline(always)]
    pub fn ostpe(&self) -> OSTPE_R {
        OSTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Comparator Disable Request Enable 0Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre0(&self) -> CDRE0_R {
        CDRE0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Comparator Disable Request Enable 1Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre1(&self) -> CDRE1_R {
        CDRE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Comparator Disable Request Enable 2Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre2(&self) -> CDRE2_R {
        CDRE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Comparator Disable Request Enable 3Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre3(&self) -> CDRE3_R {
        CDRE3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Comparator Disable Request Enable 4Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre4(&self) -> CDRE4_R {
        CDRE4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Comparator Disable Request Enable 5Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre5(&self) -> CDRE5_R {
        CDRE5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - GTETRG Input Status Flag
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - GTETRG Input Reverse
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Noise Filter Clock Select
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Port Input Detection Flag
    #[inline(always)]
    pub fn pidf(&mut self) -> PIDF_W<POEGG_SPEC> {
        PIDF_W::new(self, 0)
    }
    ///Bit 1 - Real Time Overcurrent Detection Flag
    #[inline(always)]
    pub fn iocf(&mut self) -> IOCF_W<POEGG_SPEC> {
        IOCF_W::new(self, 1)
    }
    ///Bit 2 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostpf(&mut self) -> OSTPF_W<POEGG_SPEC> {
        OSTPF_W::new(self, 2)
    }
    ///Bit 3 - Software Stop Flag
    #[inline(always)]
    pub fn ssf(&mut self) -> SSF_W<POEGG_SPEC> {
        SSF_W::new(self, 3)
    }
    ///Bit 4 - Port Input Detection EnableNote: Can be modified only once after a reset.
    #[inline(always)]
    pub fn pide(&mut self) -> PIDE_W<POEGG_SPEC> {
        PIDE_W::new(self, 4)
    }
    ///Bit 5 - Enable for GPT Output-Disable RequestNote: Can be modified only once after a reset.
    #[inline(always)]
    pub fn ioce(&mut self) -> IOCE_W<POEGG_SPEC> {
        IOCE_W::new(self, 5)
    }
    ///Bit 6 - Oscillation Stop Detection EnableNote: Can be modified only once after a reset.
    #[inline(always)]
    pub fn ostpe(&mut self) -> OSTPE_W<POEGG_SPEC> {
        OSTPE_W::new(self, 6)
    }
    ///Bit 8 - Comparator Disable Request Enable 0Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre0(&mut self) -> CDRE0_W<POEGG_SPEC> {
        CDRE0_W::new(self, 8)
    }
    ///Bit 9 - Comparator Disable Request Enable 1Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre1(&mut self) -> CDRE1_W<POEGG_SPEC> {
        CDRE1_W::new(self, 9)
    }
    ///Bit 10 - Comparator Disable Request Enable 2Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre2(&mut self) -> CDRE2_W<POEGG_SPEC> {
        CDRE2_W::new(self, 10)
    }
    ///Bit 11 - Comparator Disable Request Enable 3Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre3(&mut self) -> CDRE3_W<POEGG_SPEC> {
        CDRE3_W::new(self, 11)
    }
    ///Bit 12 - Comparator Disable Request Enable 4Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre4(&mut self) -> CDRE4_W<POEGG_SPEC> {
        CDRE4_W::new(self, 12)
    }
    ///Bit 13 - Comparator Disable Request Enable 5Note: Can be modified only once after a reset.
    #[inline(always)]
    pub fn cdre5(&mut self) -> CDRE5_W<POEGG_SPEC> {
        CDRE5_W::new(self, 13)
    }
    ///Bit 28 - GTETRG Input Reverse
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<POEGG_SPEC> {
        INV_W::new(self, 28)
    }
    ///Bit 29 - Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&mut self) -> NFEN_W<POEGG_SPEC> {
        NFEN_W::new(self, 29)
    }
    ///Bits 30:31 - Noise Filter Clock Select
    #[inline(always)]
    pub fn nfcs(&mut self) -> NFCS_W<POEGG_SPEC> {
        NFCS_W::new(self, 30)
    }
}
/**POEG Group %s Setting Register

You can [`read`](crate::Reg::read) this register and get [`poegg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poegg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POEGG_SPEC;
impl crate::RegisterSpec for POEGG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`poegg::R`](R) reader structure
impl crate::Readable for POEGG_SPEC {}
///`write(|w| ..)` method takes [`poegg::W`](W) writer structure
impl crate::Writable for POEGG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
///`reset()` method sets POEGG%s to value 0
impl crate::Resettable for POEGG_SPEC {}
