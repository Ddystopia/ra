///Register `SFMSMD` reader
pub type R = crate::R<SFMSMD_SPEC>;
///Register `SFMSMD` writer
pub type W = crate::W<SFMSMD_SPEC>;
/**Serial interface read mode selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMRM_A {
    ///0: Standard Read
    _000 = 0,
    ///1: Fast Read
    _001 = 1,
    ///2: Fast Read Dual Output
    _010 = 2,
    ///3: Fast Read Dual I/O
    _011 = 3,
    ///4: Fast Read Quad Output
    _100 = 4,
    ///5: Fast Read Quad I/O
    _101 = 5,
    ///6: Setting prohibited
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<SFMRM_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMRM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMRM_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMRM_A {}
///Field `SFMRM` reader - Serial interface read mode selection
pub type SFMRM_R = crate::FieldReader<SFMRM_A>;
impl SFMRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMRM_A {
        match self.bits {
            0 => SFMRM_A::_000,
            1 => SFMRM_A::_001,
            2 => SFMRM_A::_010,
            3 => SFMRM_A::_011,
            4 => SFMRM_A::_100,
            5 => SFMRM_A::_101,
            6 => SFMRM_A::_110,
            7 => SFMRM_A::_111,
            _ => unreachable!(),
        }
    }
    ///Standard Read
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SFMRM_A::_000
    }
    ///Fast Read
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SFMRM_A::_001
    }
    ///Fast Read Dual Output
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SFMRM_A::_010
    }
    ///Fast Read Dual I/O
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SFMRM_A::_011
    }
    ///Fast Read Quad Output
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SFMRM_A::_100
    }
    ///Fast Read Quad I/O
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SFMRM_A::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SFMRM_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SFMRM_A::_111
    }
}
///Field `SFMRM` writer - Serial interface read mode selection
pub type SFMRM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SFMRM_A, crate::Safe>;
impl<'a, REG> SFMRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Standard Read
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_000)
    }
    ///Fast Read
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_001)
    }
    ///Fast Read Dual Output
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_010)
    }
    ///Fast Read Dual I/O
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_011)
    }
    ///Fast Read Quad Output
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_100)
    }
    ///Fast Read Quad I/O
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SFMRM_A::_111)
    }
}
/**Selection of the prefetch function

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSE_A {
    ///0: Does not extend QSSL
    _00 = 0,
    ///1: Extends QSSL by 33*QSPCLK
    _01 = 1,
    ///2: Extends QSSL by 129*QSPCLK
    _10 = 2,
    ///3: Extends QSSL infinitely
    _11 = 3,
}
impl From<SFMSE_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMSE_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMSE_A {}
///Field `SFMSE` reader - Selection of the prefetch function
pub type SFMSE_R = crate::FieldReader<SFMSE_A>;
impl SFMSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMSE_A {
        match self.bits {
            0 => SFMSE_A::_00,
            1 => SFMSE_A::_01,
            2 => SFMSE_A::_10,
            3 => SFMSE_A::_11,
            _ => unreachable!(),
        }
    }
    ///Does not extend QSSL
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SFMSE_A::_00
    }
    ///Extends QSSL by 33*QSPCLK
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SFMSE_A::_01
    }
    ///Extends QSSL by 129*QSPCLK
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SFMSE_A::_10
    }
    ///Extends QSSL infinitely
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SFMSE_A::_11
    }
}
///Field `SFMSE` writer - Selection of the prefetch function
pub type SFMSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SFMSE_A, crate::Safe>;
impl<'a, REG> SFMSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Does not extend QSSL
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSE_A::_00)
    }
    ///Extends QSSL by 33*QSPCLK
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSE_A::_01)
    }
    ///Extends QSSL by 129*QSPCLK
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSE_A::_10)
    }
    ///Extends QSSL infinitely
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSE_A::_11)
    }
}
/**Selection of the prefetch function

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMPFE_A {
    ///0: Disables prefetch
    _0 = 0,
    ///1: Enables prefetch
    _1 = 1,
}
impl From<SFMPFE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMPFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMPFE` reader - Selection of the prefetch function
pub type SFMPFE_R = crate::BitReader<SFMPFE_A>;
impl SFMPFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMPFE_A {
        match self.bits {
            false => SFMPFE_A::_0,
            true => SFMPFE_A::_1,
        }
    }
    ///Disables prefetch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMPFE_A::_0
    }
    ///Enables prefetch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMPFE_A::_1
    }
}
///Field `SFMPFE` writer - Selection of the prefetch function
pub type SFMPFE_W<'a, REG> = crate::BitWriter<'a, REG, SFMPFE_A>;
impl<'a, REG> SFMPFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables prefetch
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMPFE_A::_0)
    }
    ///Enables prefetch
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMPFE_A::_1)
    }
}
/**Selection of the function for stopping prefetch at locations other than on byte boundaries

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMPAE_A {
    ///0: Disables prefetch stopping at locations other than on byte boundaries
    _0 = 0,
    ///1: Enables prefetch stopping at locations other than on byte boundaries
    _1 = 1,
}
impl From<SFMPAE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMPAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMPAE` reader - Selection of the function for stopping prefetch at locations other than on byte boundaries
pub type SFMPAE_R = crate::BitReader<SFMPAE_A>;
impl SFMPAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMPAE_A {
        match self.bits {
            false => SFMPAE_A::_0,
            true => SFMPAE_A::_1,
        }
    }
    ///Disables prefetch stopping at locations other than on byte boundaries
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMPAE_A::_0
    }
    ///Enables prefetch stopping at locations other than on byte boundaries
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMPAE_A::_1
    }
}
///Field `SFMPAE` writer - Selection of the function for stopping prefetch at locations other than on byte boundaries
pub type SFMPAE_W<'a, REG> = crate::BitWriter<'a, REG, SFMPAE_A>;
impl<'a, REG> SFMPAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables prefetch stopping at locations other than on byte boundaries
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMPAE_A::_0)
    }
    ///Enables prefetch stopping at locations other than on byte boundaries
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMPAE_A::_1)
    }
}
/**SPI mode selection. An initial value is determined by input to CFGMD3.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMMD3_A {
    ///0: SPI mode 0
    _0 = 0,
    ///1: SPI mode 3
    _1 = 1,
}
impl From<SFMMD3_A> for bool {
    #[inline(always)]
    fn from(variant: SFMMD3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMMD3` reader - SPI mode selection. An initial value is determined by input to CFGMD3.
pub type SFMMD3_R = crate::BitReader<SFMMD3_A>;
impl SFMMD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMMD3_A {
        match self.bits {
            false => SFMMD3_A::_0,
            true => SFMMD3_A::_1,
        }
    }
    ///SPI mode 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMMD3_A::_0
    }
    ///SPI mode 3
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMMD3_A::_1
    }
}
///Field `SFMMD3` writer - SPI mode selection. An initial value is determined by input to CFGMD3.
pub type SFMMD3_W<'a, REG> = crate::BitWriter<'a, REG, SFMMD3_A>;
impl<'a, REG> SFMMD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI mode 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMMD3_A::_0)
    }
    ///SPI mode 3
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMMD3_A::_1)
    }
}
/**Extension of the I/O buffer output enable signal for the serial interface

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMOEX_A {
    ///0: Does not extend the output enable signal
    _0 = 0,
    ///1: Extends the output enable signal by 1*QSPCLK
    _1 = 1,
}
impl From<SFMOEX_A> for bool {
    #[inline(always)]
    fn from(variant: SFMOEX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMOEX` reader - Extension of the I/O buffer output enable signal for the serial interface
pub type SFMOEX_R = crate::BitReader<SFMOEX_A>;
impl SFMOEX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMOEX_A {
        match self.bits {
            false => SFMOEX_A::_0,
            true => SFMOEX_A::_1,
        }
    }
    ///Does not extend the output enable signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMOEX_A::_0
    }
    ///Extends the output enable signal by 1*QSPCLK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMOEX_A::_1
    }
}
///Field `SFMOEX` writer - Extension of the I/O buffer output enable signal for the serial interface
pub type SFMOEX_W<'a, REG> = crate::BitWriter<'a, REG, SFMOEX_A>;
impl<'a, REG> SFMOEX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not extend the output enable signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMOEX_A::_0)
    }
    ///Extends the output enable signal by 1*QSPCLK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMOEX_A::_1)
    }
}
/**Hold time adjustment for serial transmission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMOHW_A {
    ///0: Does not extend the high-level width of SCK at transmission time
    _0 = 0,
    ///1: Extends the high-level width of SCK by 1*PCLKA at transmission time
    _1 = 1,
}
impl From<SFMOHW_A> for bool {
    #[inline(always)]
    fn from(variant: SFMOHW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMOHW` reader - Hold time adjustment for serial transmission
pub type SFMOHW_R = crate::BitReader<SFMOHW_A>;
impl SFMOHW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMOHW_A {
        match self.bits {
            false => SFMOHW_A::_0,
            true => SFMOHW_A::_1,
        }
    }
    ///Does not extend the high-level width of SCK at transmission time
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMOHW_A::_0
    }
    ///Extends the high-level width of SCK by 1*PCLKA at transmission time
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMOHW_A::_1
    }
}
///Field `SFMOHW` writer - Hold time adjustment for serial transmission
pub type SFMOHW_W<'a, REG> = crate::BitWriter<'a, REG, SFMOHW_A>;
impl<'a, REG> SFMOHW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not extend the high-level width of SCK at transmission time
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMOHW_A::_0)
    }
    ///Extends the high-level width of SCK by 1*PCLKA at transmission time
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMOHW_A::_1)
    }
}
/**Setup time adjustment for serial transmission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMOSW_A {
    ///0: Does not extend the low-level width of SCK at transmission time
    _0 = 0,
    ///1: Extends the low-level width of SCK by 1*PCLKA at transmission time
    _1 = 1,
}
impl From<SFMOSW_A> for bool {
    #[inline(always)]
    fn from(variant: SFMOSW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMOSW` reader - Setup time adjustment for serial transmission
pub type SFMOSW_R = crate::BitReader<SFMOSW_A>;
impl SFMOSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMOSW_A {
        match self.bits {
            false => SFMOSW_A::_0,
            true => SFMOSW_A::_1,
        }
    }
    ///Does not extend the low-level width of SCK at transmission time
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMOSW_A::_0
    }
    ///Extends the low-level width of SCK by 1*PCLKA at transmission time
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMOSW_A::_1
    }
}
///Field `SFMOSW` writer - Setup time adjustment for serial transmission
pub type SFMOSW_W<'a, REG> = crate::BitWriter<'a, REG, SFMOSW_A>;
impl<'a, REG> SFMOSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not extend the low-level width of SCK at transmission time
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMOSW_A::_0)
    }
    ///Extends the low-level width of SCK by 1*PCLKA at transmission time
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMOSW_A::_1)
    }
}
/**Read instruction code selection.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMCCE_A {
    ///0: Default instruction code set for each instruction
    _0 = 0,
    ///1: Instruction code written in the SFMSIC register
    _1 = 1,
}
impl From<SFMCCE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMCCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMCCE` reader - Read instruction code selection.
pub type SFMCCE_R = crate::BitReader<SFMCCE_A>;
impl SFMCCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMCCE_A {
        match self.bits {
            false => SFMCCE_A::_0,
            true => SFMCCE_A::_1,
        }
    }
    ///Default instruction code set for each instruction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMCCE_A::_0
    }
    ///Instruction code written in the SFMSIC register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMCCE_A::_1
    }
}
///Field `SFMCCE` writer - Read instruction code selection.
pub type SFMCCE_W<'a, REG> = crate::BitWriter<'a, REG, SFMCCE_A>;
impl<'a, REG> SFMCCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default instruction code set for each instruction
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMCCE_A::_0)
    }
    ///Instruction code written in the SFMSIC register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMCCE_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Serial interface read mode selection
    #[inline(always)]
    pub fn sfmrm(&self) -> SFMRM_R {
        SFMRM_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:5 - Selection of the prefetch function
    #[inline(always)]
    pub fn sfmse(&self) -> SFMSE_R {
        SFMSE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Selection of the prefetch function
    #[inline(always)]
    pub fn sfmpfe(&self) -> SFMPFE_R {
        SFMPFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Selection of the function for stopping prefetch at locations other than on byte boundaries
    #[inline(always)]
    pub fn sfmpae(&self) -> SFMPAE_R {
        SFMPAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPI mode selection. An initial value is determined by input to CFGMD3.
    #[inline(always)]
    pub fn sfmmd3(&self) -> SFMMD3_R {
        SFMMD3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Extension of the I/O buffer output enable signal for the serial interface
    #[inline(always)]
    pub fn sfmoex(&self) -> SFMOEX_R {
        SFMOEX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hold time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmohw(&self) -> SFMOHW_R {
        SFMOHW_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Setup time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmosw(&self) -> SFMOSW_R {
        SFMOSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Read instruction code selection.
    #[inline(always)]
    pub fn sfmcce(&self) -> SFMCCE_R {
        SFMCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Serial interface read mode selection
    #[inline(always)]
    pub fn sfmrm(&mut self) -> SFMRM_W<SFMSMD_SPEC> {
        SFMRM_W::new(self, 0)
    }
    ///Bits 4:5 - Selection of the prefetch function
    #[inline(always)]
    pub fn sfmse(&mut self) -> SFMSE_W<SFMSMD_SPEC> {
        SFMSE_W::new(self, 4)
    }
    ///Bit 6 - Selection of the prefetch function
    #[inline(always)]
    pub fn sfmpfe(&mut self) -> SFMPFE_W<SFMSMD_SPEC> {
        SFMPFE_W::new(self, 6)
    }
    ///Bit 7 - Selection of the function for stopping prefetch at locations other than on byte boundaries
    #[inline(always)]
    pub fn sfmpae(&mut self) -> SFMPAE_W<SFMSMD_SPEC> {
        SFMPAE_W::new(self, 7)
    }
    ///Bit 8 - SPI mode selection. An initial value is determined by input to CFGMD3.
    #[inline(always)]
    pub fn sfmmd3(&mut self) -> SFMMD3_W<SFMSMD_SPEC> {
        SFMMD3_W::new(self, 8)
    }
    ///Bit 9 - Extension of the I/O buffer output enable signal for the serial interface
    #[inline(always)]
    pub fn sfmoex(&mut self) -> SFMOEX_W<SFMSMD_SPEC> {
        SFMOEX_W::new(self, 9)
    }
    ///Bit 10 - Hold time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmohw(&mut self) -> SFMOHW_W<SFMSMD_SPEC> {
        SFMOHW_W::new(self, 10)
    }
    ///Bit 11 - Setup time adjustment for serial transmission
    #[inline(always)]
    pub fn sfmosw(&mut self) -> SFMOSW_W<SFMSMD_SPEC> {
        SFMOSW_W::new(self, 11)
    }
    ///Bit 15 - Read instruction code selection.
    #[inline(always)]
    pub fn sfmcce(&mut self) -> SFMCCE_W<SFMSMD_SPEC> {
        SFMCCE_W::new(self, 15)
    }
}
/**Transfer Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSMD_SPEC;
impl crate::RegisterSpec for SFMSMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmsmd::R`](R) reader structure
impl crate::Readable for SFMSMD_SPEC {}
///`write(|w| ..)` method takes [`sfmsmd::W`](W) writer structure
impl crate::Writable for SFMSMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSMD to value 0
impl crate::Resettable for SFMSMD_SPEC {}
