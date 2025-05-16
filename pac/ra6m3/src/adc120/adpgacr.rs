///Register `ADPGACR` reader
pub type R = crate::R<ADPGACR_SPEC>;
///Register `ADPGACR` writer
pub type W = crate::W<ADPGACR_SPEC>;
/**A through amplifier is enable for PGA P000

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000SEL0_A {
    ///0: Not through the PGA in amplifier
    _0 = 0,
    ///1: I will through in the PGA amplifier.
    _1 = 1,
}
impl From<P000SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: P000SEL0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P000SEL0` reader - A through amplifier is enable for PGA P000
pub type P000SEL0_R = crate::BitReader<P000SEL0_A>;
impl P000SEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000SEL0_A {
        match self.bits {
            false => P000SEL0_A::_0,
            true => P000SEL0_A::_1,
        }
    }
    ///Not through the PGA in amplifier
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000SEL0_A::_0
    }
    ///I will through in the PGA amplifier.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000SEL0_A::_1
    }
}
///Field `P000SEL0` writer - A through amplifier is enable for PGA P000
pub type P000SEL0_W<'a, REG> = crate::BitWriter<'a, REG, P000SEL0_A>;
impl<'a, REG> P000SEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not through the PGA in amplifier
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P000SEL0_A::_0)
    }
    ///I will through in the PGA amplifier.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P000SEL0_A::_1)
    }
}
/**The amplifier passing is enable for PGA P000

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000SEL1_A {
    ///0: By way of the amplifier in PGA.
    _0 = 0,
    ///1: Note 1 that by way of amplifier in PGA
    _1 = 1,
}
impl From<P000SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: P000SEL1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P000SEL1` reader - The amplifier passing is enable for PGA P000
pub type P000SEL1_R = crate::BitReader<P000SEL1_A>;
impl P000SEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000SEL1_A {
        match self.bits {
            false => P000SEL1_A::_0,
            true => P000SEL1_A::_1,
        }
    }
    ///By way of the amplifier in PGA.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000SEL1_A::_0
    }
    ///Note 1 that by way of amplifier in PGA
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000SEL1_A::_1
    }
}
///Field `P000SEL1` writer - The amplifier passing is enable for PGA P000
pub type P000SEL1_W<'a, REG> = crate::BitWriter<'a, REG, P000SEL1_A>;
impl<'a, REG> P000SEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///By way of the amplifier in PGA.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P000SEL1_A::_0)
    }
    ///Note 1 that by way of amplifier in PGA
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P000SEL1_A::_1)
    }
}
/**Amplifier enable bit for PGA P000

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000ENAMP_A {
    ///0: The amplifier in PGA is not used.
    _0 = 0,
    ///1: The amplifier in PGA is used.
    _1 = 1,
}
impl From<P000ENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: P000ENAMP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P000ENAMP` reader - Amplifier enable bit for PGA P000
pub type P000ENAMP_R = crate::BitReader<P000ENAMP_A>;
impl P000ENAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000ENAMP_A {
        match self.bits {
            false => P000ENAMP_A::_0,
            true => P000ENAMP_A::_1,
        }
    }
    ///The amplifier in PGA is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000ENAMP_A::_0
    }
    ///The amplifier in PGA is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000ENAMP_A::_1
    }
}
///Field `P000ENAMP` writer - Amplifier enable bit for PGA P000
pub type P000ENAMP_W<'a, REG> = crate::BitWriter<'a, REG, P000ENAMP_A>;
impl<'a, REG> P000ENAMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The amplifier in PGA is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P000ENAMP_A::_0)
    }
    ///The amplifier in PGA is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P000ENAMP_A::_1)
    }
}
/**PGA P000 gain setting and enable bit

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000GEN_A {
    ///0: The gain setting is invalidated (AIN is not input in PGA).
    _0 = 0,
    ///1: The gain setting is effectively done (AIN is input in PGA).
    _1 = 1,
}
impl From<P000GEN_A> for bool {
    #[inline(always)]
    fn from(variant: P000GEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P000GEN` reader - PGA P000 gain setting and enable bit
pub type P000GEN_R = crate::BitReader<P000GEN_A>;
impl P000GEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000GEN_A {
        match self.bits {
            false => P000GEN_A::_0,
            true => P000GEN_A::_1,
        }
    }
    ///The gain setting is invalidated (AIN is not input in PGA).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000GEN_A::_0
    }
    ///The gain setting is effectively done (AIN is input in PGA).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000GEN_A::_1
    }
}
///Field `P000GEN` writer - PGA P000 gain setting and enable bit
pub type P000GEN_W<'a, REG> = crate::BitWriter<'a, REG, P000GEN_A>;
impl<'a, REG> P000GEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gain setting is invalidated (AIN is not input in PGA).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P000GEN_A::_0)
    }
    ///The gain setting is effectively done (AIN is input in PGA).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P000GEN_A::_1)
    }
}
/**A through amplifier is enable for PGA P001

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001SEL0_A {
    ///0: Not through the PGA in amplifier
    _0 = 0,
    ///1: I will through in the PGA amplifier.
    _1 = 1,
}
impl From<P001SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: P001SEL0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P001SEL0` reader - A through amplifier is enable for PGA P001
pub type P001SEL0_R = crate::BitReader<P001SEL0_A>;
impl P001SEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001SEL0_A {
        match self.bits {
            false => P001SEL0_A::_0,
            true => P001SEL0_A::_1,
        }
    }
    ///Not through the PGA in amplifier
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001SEL0_A::_0
    }
    ///I will through in the PGA amplifier.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001SEL0_A::_1
    }
}
///Field `P001SEL0` writer - A through amplifier is enable for PGA P001
pub type P001SEL0_W<'a, REG> = crate::BitWriter<'a, REG, P001SEL0_A>;
impl<'a, REG> P001SEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not through the PGA in amplifier
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P001SEL0_A::_0)
    }
    ///I will through in the PGA amplifier.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P001SEL0_A::_1)
    }
}
/**The amplifier passing is enable for PGA P001

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001SEL1_A {
    ///0: By way of the amplifier in PGA.
    _0 = 0,
    ///1: Note 1 that by way of amplifier in PGA
    _1 = 1,
}
impl From<P001SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: P001SEL1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P001SEL1` reader - The amplifier passing is enable for PGA P001
pub type P001SEL1_R = crate::BitReader<P001SEL1_A>;
impl P001SEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001SEL1_A {
        match self.bits {
            false => P001SEL1_A::_0,
            true => P001SEL1_A::_1,
        }
    }
    ///By way of the amplifier in PGA.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001SEL1_A::_0
    }
    ///Note 1 that by way of amplifier in PGA
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001SEL1_A::_1
    }
}
///Field `P001SEL1` writer - The amplifier passing is enable for PGA P001
pub type P001SEL1_W<'a, REG> = crate::BitWriter<'a, REG, P001SEL1_A>;
impl<'a, REG> P001SEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///By way of the amplifier in PGA.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P001SEL1_A::_0)
    }
    ///Note 1 that by way of amplifier in PGA
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P001SEL1_A::_1)
    }
}
/**Amplifier enable bit for PGA P001

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001ENAMP_A {
    ///0: The amplifier in PGA is not used.
    _0 = 0,
    ///1: The amplifier in PGA is used.
    _1 = 1,
}
impl From<P001ENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: P001ENAMP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P001ENAMP` reader - Amplifier enable bit for PGA P001
pub type P001ENAMP_R = crate::BitReader<P001ENAMP_A>;
impl P001ENAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001ENAMP_A {
        match self.bits {
            false => P001ENAMP_A::_0,
            true => P001ENAMP_A::_1,
        }
    }
    ///The amplifier in PGA is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001ENAMP_A::_0
    }
    ///The amplifier in PGA is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001ENAMP_A::_1
    }
}
///Field `P001ENAMP` writer - Amplifier enable bit for PGA P001
pub type P001ENAMP_W<'a, REG> = crate::BitWriter<'a, REG, P001ENAMP_A>;
impl<'a, REG> P001ENAMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The amplifier in PGA is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P001ENAMP_A::_0)
    }
    ///The amplifier in PGA is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P001ENAMP_A::_1)
    }
}
/**PGA P001 gain setting and enable bit

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001GEN_A {
    ///0: The gain setting is invalidated (AIN is not input in PGA).
    _0 = 0,
    ///1: The gain setting is effectively done (AIN is input in PGA).
    _1 = 1,
}
impl From<P001GEN_A> for bool {
    #[inline(always)]
    fn from(variant: P001GEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P001GEN` reader - PGA P001 gain setting and enable bit
pub type P001GEN_R = crate::BitReader<P001GEN_A>;
impl P001GEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001GEN_A {
        match self.bits {
            false => P001GEN_A::_0,
            true => P001GEN_A::_1,
        }
    }
    ///The gain setting is invalidated (AIN is not input in PGA).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001GEN_A::_0
    }
    ///The gain setting is effectively done (AIN is input in PGA).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001GEN_A::_1
    }
}
///Field `P001GEN` writer - PGA P001 gain setting and enable bit
pub type P001GEN_W<'a, REG> = crate::BitWriter<'a, REG, P001GEN_A>;
impl<'a, REG> P001GEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gain setting is invalidated (AIN is not input in PGA).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P001GEN_A::_0)
    }
    ///The gain setting is effectively done (AIN is input in PGA).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P001GEN_A::_1)
    }
}
/**A through amplifier is enable for PGA P002

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002SEL0_A {
    ///0: Not through the PGA in amplifier
    _0 = 0,
    ///1: I will through in the PGA amplifier.
    _1 = 1,
}
impl From<P002SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: P002SEL0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P002SEL0` reader - A through amplifier is enable for PGA P002
pub type P002SEL0_R = crate::BitReader<P002SEL0_A>;
impl P002SEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002SEL0_A {
        match self.bits {
            false => P002SEL0_A::_0,
            true => P002SEL0_A::_1,
        }
    }
    ///Not through the PGA in amplifier
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002SEL0_A::_0
    }
    ///I will through in the PGA amplifier.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002SEL0_A::_1
    }
}
///Field `P002SEL0` writer - A through amplifier is enable for PGA P002
pub type P002SEL0_W<'a, REG> = crate::BitWriter<'a, REG, P002SEL0_A>;
impl<'a, REG> P002SEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not through the PGA in amplifier
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P002SEL0_A::_0)
    }
    ///I will through in the PGA amplifier.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P002SEL0_A::_1)
    }
}
/**The amplifier passing is enable for PGA P002

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002SEL1_A {
    ///0: By way of the amplifier in PGA.
    _0 = 0,
    ///1: Note 1 that by way of amplifier in PGA
    _1 = 1,
}
impl From<P002SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: P002SEL1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P002SEL1` reader - The amplifier passing is enable for PGA P002
pub type P002SEL1_R = crate::BitReader<P002SEL1_A>;
impl P002SEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002SEL1_A {
        match self.bits {
            false => P002SEL1_A::_0,
            true => P002SEL1_A::_1,
        }
    }
    ///By way of the amplifier in PGA.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002SEL1_A::_0
    }
    ///Note 1 that by way of amplifier in PGA
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002SEL1_A::_1
    }
}
///Field `P002SEL1` writer - The amplifier passing is enable for PGA P002
pub type P002SEL1_W<'a, REG> = crate::BitWriter<'a, REG, P002SEL1_A>;
impl<'a, REG> P002SEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///By way of the amplifier in PGA.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P002SEL1_A::_0)
    }
    ///Note 1 that by way of amplifier in PGA
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P002SEL1_A::_1)
    }
}
/**Amplifier enable bit for PGA P002

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002ENAMP_A {
    ///0: The amplifier in PGA is not used.
    _0 = 0,
    ///1: The amplifier in PGA is used.
    _1 = 1,
}
impl From<P002ENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: P002ENAMP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P002ENAMP` reader - Amplifier enable bit for PGA P002
pub type P002ENAMP_R = crate::BitReader<P002ENAMP_A>;
impl P002ENAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002ENAMP_A {
        match self.bits {
            false => P002ENAMP_A::_0,
            true => P002ENAMP_A::_1,
        }
    }
    ///The amplifier in PGA is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002ENAMP_A::_0
    }
    ///The amplifier in PGA is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002ENAMP_A::_1
    }
}
///Field `P002ENAMP` writer - Amplifier enable bit for PGA P002
pub type P002ENAMP_W<'a, REG> = crate::BitWriter<'a, REG, P002ENAMP_A>;
impl<'a, REG> P002ENAMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The amplifier in PGA is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P002ENAMP_A::_0)
    }
    ///The amplifier in PGA is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P002ENAMP_A::_1)
    }
}
/**PGA P002 gain setting and enable bit

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002GEN_A {
    ///0: The gain setting is invalidated (AIN is not input in PGA).
    _0 = 0,
    ///1: The gain setting is effectively done (AIN is input in PGA).
    _1 = 1,
}
impl From<P002GEN_A> for bool {
    #[inline(always)]
    fn from(variant: P002GEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P002GEN` reader - PGA P002 gain setting and enable bit
pub type P002GEN_R = crate::BitReader<P002GEN_A>;
impl P002GEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002GEN_A {
        match self.bits {
            false => P002GEN_A::_0,
            true => P002GEN_A::_1,
        }
    }
    ///The gain setting is invalidated (AIN is not input in PGA).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002GEN_A::_0
    }
    ///The gain setting is effectively done (AIN is input in PGA).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002GEN_A::_1
    }
}
///Field `P002GEN` writer - PGA P002 gain setting and enable bit
pub type P002GEN_W<'a, REG> = crate::BitWriter<'a, REG, P002GEN_A>;
impl<'a, REG> P002GEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The gain setting is invalidated (AIN is not input in PGA).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P002GEN_A::_0)
    }
    ///The gain setting is effectively done (AIN is input in PGA).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P002GEN_A::_1)
    }
}
impl R {
    ///Bit 0 - A through amplifier is enable for PGA P000
    #[inline(always)]
    pub fn p000sel0(&self) -> P000SEL0_R {
        P000SEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The amplifier passing is enable for PGA P000
    #[inline(always)]
    pub fn p000sel1(&self) -> P000SEL1_R {
        P000SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Amplifier enable bit for PGA P000
    #[inline(always)]
    pub fn p000enamp(&self) -> P000ENAMP_R {
        P000ENAMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PGA P000 gain setting and enable bit
    #[inline(always)]
    pub fn p000gen(&self) -> P000GEN_R {
        P000GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A through amplifier is enable for PGA P001
    #[inline(always)]
    pub fn p001sel0(&self) -> P001SEL0_R {
        P001SEL0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The amplifier passing is enable for PGA P001
    #[inline(always)]
    pub fn p001sel1(&self) -> P001SEL1_R {
        P001SEL1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Amplifier enable bit for PGA P001
    #[inline(always)]
    pub fn p001enamp(&self) -> P001ENAMP_R {
        P001ENAMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PGA P001 gain setting and enable bit
    #[inline(always)]
    pub fn p001gen(&self) -> P001GEN_R {
        P001GEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A through amplifier is enable for PGA P002
    #[inline(always)]
    pub fn p002sel0(&self) -> P002SEL0_R {
        P002SEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The amplifier passing is enable for PGA P002
    #[inline(always)]
    pub fn p002sel1(&self) -> P002SEL1_R {
        P002SEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Amplifier enable bit for PGA P002
    #[inline(always)]
    pub fn p002enamp(&self) -> P002ENAMP_R {
        P002ENAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PGA P002 gain setting and enable bit
    #[inline(always)]
    pub fn p002gen(&self) -> P002GEN_R {
        P002GEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A through amplifier is enable for PGA P000
    #[inline(always)]
    pub fn p000sel0(&mut self) -> P000SEL0_W<ADPGACR_SPEC> {
        P000SEL0_W::new(self, 0)
    }
    ///Bit 1 - The amplifier passing is enable for PGA P000
    #[inline(always)]
    pub fn p000sel1(&mut self) -> P000SEL1_W<ADPGACR_SPEC> {
        P000SEL1_W::new(self, 1)
    }
    ///Bit 2 - Amplifier enable bit for PGA P000
    #[inline(always)]
    pub fn p000enamp(&mut self) -> P000ENAMP_W<ADPGACR_SPEC> {
        P000ENAMP_W::new(self, 2)
    }
    ///Bit 3 - PGA P000 gain setting and enable bit
    #[inline(always)]
    pub fn p000gen(&mut self) -> P000GEN_W<ADPGACR_SPEC> {
        P000GEN_W::new(self, 3)
    }
    ///Bit 4 - A through amplifier is enable for PGA P001
    #[inline(always)]
    pub fn p001sel0(&mut self) -> P001SEL0_W<ADPGACR_SPEC> {
        P001SEL0_W::new(self, 4)
    }
    ///Bit 5 - The amplifier passing is enable for PGA P001
    #[inline(always)]
    pub fn p001sel1(&mut self) -> P001SEL1_W<ADPGACR_SPEC> {
        P001SEL1_W::new(self, 5)
    }
    ///Bit 6 - Amplifier enable bit for PGA P001
    #[inline(always)]
    pub fn p001enamp(&mut self) -> P001ENAMP_W<ADPGACR_SPEC> {
        P001ENAMP_W::new(self, 6)
    }
    ///Bit 7 - PGA P001 gain setting and enable bit
    #[inline(always)]
    pub fn p001gen(&mut self) -> P001GEN_W<ADPGACR_SPEC> {
        P001GEN_W::new(self, 7)
    }
    ///Bit 8 - A through amplifier is enable for PGA P002
    #[inline(always)]
    pub fn p002sel0(&mut self) -> P002SEL0_W<ADPGACR_SPEC> {
        P002SEL0_W::new(self, 8)
    }
    ///Bit 9 - The amplifier passing is enable for PGA P002
    #[inline(always)]
    pub fn p002sel1(&mut self) -> P002SEL1_W<ADPGACR_SPEC> {
        P002SEL1_W::new(self, 9)
    }
    ///Bit 10 - Amplifier enable bit for PGA P002
    #[inline(always)]
    pub fn p002enamp(&mut self) -> P002ENAMP_W<ADPGACR_SPEC> {
        P002ENAMP_W::new(self, 10)
    }
    ///Bit 11 - PGA P002 gain setting and enable bit
    #[inline(always)]
    pub fn p002gen(&mut self) -> P002GEN_W<ADPGACR_SPEC> {
        P002GEN_W::new(self, 11)
    }
}
/**A/D Programmable Gain Amplifier Control Register

You can [`read`](crate::Reg::read) this register and get [`adpgacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adpgacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADPGACR_SPEC;
impl crate::RegisterSpec for ADPGACR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adpgacr::R`](R) reader structure
impl crate::Readable for ADPGACR_SPEC {}
///`write(|w| ..)` method takes [`adpgacr::W`](W) writer structure
impl crate::Writable for ADPGACR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADPGACR to value 0x9999
impl crate::Resettable for ADPGACR_SPEC {
    const RESET_VALUE: u16 = 0x9999;
}
