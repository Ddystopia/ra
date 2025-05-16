///Register `P110PFS_HA` reader
pub type R = crate::R<P110PFS_HA_SPEC>;
///Register `P110PFS_HA` writer
pub type W = crate::W<P110PFS_HA_SPEC>;
/**Port Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PODR_A {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<PODR_A> for bool {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR` reader - Port Output Data
pub type PODR_R = crate::BitReader<PODR_A>;
impl PODR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PODR_A {
        match self.bits {
            false => PODR_A::_0,
            true => PODR_A::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
///Field `PODR` writer - Port Output Data
pub type PODR_W<'a, REG> = crate::BitWriter<'a, REG, PODR_A>;
impl<'a, REG> PODR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_1)
    }
}
/**Port Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR_A {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<PIDR_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR` reader - Port Input Data
pub type PIDR_R = crate::BitReader<PIDR_A>;
impl PIDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIDR_A {
        match self.bits {
            false => PIDR_A::_0,
            true => PIDR_A::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR_A::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR_A::_1
    }
}
/**Port Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR_A {
    ///0: Input (Functions as an input pin.)
    _0 = 0,
    ///1: Output (Functions as an output pin.)
    _1 = 1,
}
impl From<PDR_A> for bool {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR` reader - Port Direction
pub type PDR_R = crate::BitReader<PDR_A>;
impl PDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDR_A {
        match self.bits {
            false => PDR_A::_0,
            true => PDR_A::_1,
        }
    }
    ///Input (Functions as an input pin.)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR_A::_0
    }
    ///Output (Functions as an output pin.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR_A::_1
    }
}
///Field `PDR` writer - Port Direction
pub type PDR_W<'a, REG> = crate::BitWriter<'a, REG, PDR_A>;
impl<'a, REG> PDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (Functions as an input pin.)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDR_A::_0)
    }
    ///Output (Functions as an output pin.)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDR_A::_1)
    }
}
/**Pull-up Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCR_A {
    ///0: Disables an input pull-up.
    _0 = 0,
    ///1: Enables an input pull-up.
    _1 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCR` reader - Pull-up Control
pub type PCR_R = crate::BitReader<PCR_A>;
impl PCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::_0,
            true => PCR_A::_1,
        }
    }
    ///Disables an input pull-up.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCR_A::_0
    }
    ///Enables an input pull-up.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCR_A::_1
    }
}
///Field `PCR` writer - Pull-up Control
pub type PCR_W<'a, REG> = crate::BitWriter<'a, REG, PCR_A>;
impl<'a, REG> PCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an input pull-up.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PCR_A::_0)
    }
    ///Enables an input pull-up.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PCR_A::_1)
    }
}
/**N-Channel Open Drain Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCODR_A {
    ///0: CMOS output
    _0 = 0,
    ///1: NMOS open-drain output
    _1 = 1,
}
impl From<NCODR_A> for bool {
    #[inline(always)]
    fn from(variant: NCODR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NCODR` reader - N-Channel Open Drain Control
pub type NCODR_R = crate::BitReader<NCODR_A>;
impl NCODR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NCODR_A {
        match self.bits {
            false => NCODR_A::_0,
            true => NCODR_A::_1,
        }
    }
    ///CMOS output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCODR_A::_0
    }
    ///NMOS open-drain output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCODR_A::_1
    }
}
///Field `NCODR` writer - N-Channel Open Drain Control
pub type NCODR_W<'a, REG> = crate::BitWriter<'a, REG, NCODR_A>;
impl<'a, REG> NCODR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CMOS output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NCODR_A::_0)
    }
    ///NMOS open-drain output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NCODR_A::_1)
    }
}
/**Drive Strength Control Register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSCR_A {
    ///0: Normal drive output
    _00 = 0,
    ///1: Middle drive output
    _01 = 1,
    ///2: Setting prohibited
    _10 = 2,
    ///3: High-drive output
    _11 = 3,
}
impl From<DSCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DSCR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSCR_A {
    type Ux = u8;
}
impl crate::IsEnum for DSCR_A {}
///Field `DSCR` reader - Drive Strength Control Register
pub type DSCR_R = crate::FieldReader<DSCR_A>;
impl DSCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCR_A {
        match self.bits {
            0 => DSCR_A::_00,
            1 => DSCR_A::_01,
            2 => DSCR_A::_10,
            3 => DSCR_A::_11,
            _ => unreachable!(),
        }
    }
    ///Normal drive output
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DSCR_A::_00
    }
    ///Middle drive output
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DSCR_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DSCR_A::_10
    }
    ///High-drive output
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DSCR_A::_11
    }
}
///Field `DSCR` writer - Drive Strength Control Register
pub type DSCR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DSCR_A, crate::Safe>;
impl<'a, REG> DSCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal drive output
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DSCR_A::_00)
    }
    ///Middle drive output
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DSCR_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DSCR_A::_10)
    }
    ///High-drive output
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DSCR_A::_11)
    }
}
/**Event on Rising

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOR_A {
    ///0: Do not care
    _0 = 0,
    ///1: Detect rising edge
    _1 = 1,
}
impl From<EOR_A> for bool {
    #[inline(always)]
    fn from(variant: EOR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOR` reader - Event on Rising
pub type EOR_R = crate::BitReader<EOR_A>;
impl EOR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOR_A {
        match self.bits {
            false => EOR_A::_0,
            true => EOR_A::_1,
        }
    }
    ///Do not care
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOR_A::_0
    }
    ///Detect rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOR_A::_1
    }
}
///Field `EOR` writer - Event on Rising
pub type EOR_W<'a, REG> = crate::BitWriter<'a, REG, EOR_A>;
impl<'a, REG> EOR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not care
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOR_A::_0)
    }
    ///Detect rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOR_A::_1)
    }
}
/**Event on Falling

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOF_A {
    ///0: Do not care
    _0 = 0,
    ///1: Detect falling edge
    _1 = 1,
}
impl From<EOF_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOF` reader - Event on Falling
pub type EOF_R = crate::BitReader<EOF_A>;
impl EOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOF_A {
        match self.bits {
            false => EOF_A::_0,
            true => EOF_A::_1,
        }
    }
    ///Do not care
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOF_A::_0
    }
    ///Detect falling edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOF_A::_1
    }
}
///Field `EOF` writer - Event on Falling
pub type EOF_W<'a, REG> = crate::BitWriter<'a, REG, EOF_A>;
impl<'a, REG> EOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not care
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOF_A::_0)
    }
    ///Detect falling edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOF_A::_1)
    }
}
/**IRQ input enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    ///0: Not used as IRQn input pin
    _0 = 0,
    ///1: Used as IRQn input pin
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ISEL` reader - IRQ input enable
pub type ISEL_R = crate::BitReader<ISEL_A>;
impl ISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    ///Not used as IRQn input pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    ///Used as IRQn input pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
///Field `ISEL` writer - IRQ input enable
pub type ISEL_W<'a, REG> = crate::BitWriter<'a, REG, ISEL_A>;
impl<'a, REG> ISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not used as IRQn input pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_0)
    }
    ///Used as IRQn input pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_1)
    }
}
/**Analog Input enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEL_A {
    ///0: Used other than as analog pin
    _0 = 0,
    ///1: Used as analog pin
    _1 = 1,
}
impl From<ASEL_A> for bool {
    #[inline(always)]
    fn from(variant: ASEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASEL` reader - Analog Input enable
pub type ASEL_R = crate::BitReader<ASEL_A>;
impl ASEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASEL_A {
        match self.bits {
            false => ASEL_A::_0,
            true => ASEL_A::_1,
        }
    }
    ///Used other than as analog pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASEL_A::_0
    }
    ///Used as analog pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASEL_A::_1
    }
}
///Field `ASEL` writer - Analog Input enable
pub type ASEL_W<'a, REG> = crate::BitWriter<'a, REG, ASEL_A>;
impl<'a, REG> ASEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Used other than as analog pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASEL_A::_0)
    }
    ///Used as analog pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASEL_A::_1)
    }
}
impl R {
    ///Bit 0 - Port Output Data
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port Input Data
    #[inline(always)]
    pub fn pidr(&self) -> PIDR_R {
        PIDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port Direction
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Pull-up Control
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - N-Channel Open Drain Control
    #[inline(always)]
    pub fn ncodr(&self) -> NCODR_R {
        NCODR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 10:11 - Drive Strength Control Register
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Event on Rising
    #[inline(always)]
    pub fn eor(&self) -> EOR_R {
        EOR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Event on Falling
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - IRQ input enable
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog Input enable
    #[inline(always)]
    pub fn asel(&self) -> ASEL_R {
        ASEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port Output Data
    #[inline(always)]
    pub fn podr(&mut self) -> PODR_W<P110PFS_HA_SPEC> {
        PODR_W::new(self, 0)
    }
    ///Bit 2 - Port Direction
    #[inline(always)]
    pub fn pdr(&mut self) -> PDR_W<P110PFS_HA_SPEC> {
        PDR_W::new(self, 2)
    }
    ///Bit 4 - Pull-up Control
    #[inline(always)]
    pub fn pcr(&mut self) -> PCR_W<P110PFS_HA_SPEC> {
        PCR_W::new(self, 4)
    }
    ///Bit 6 - N-Channel Open Drain Control
    #[inline(always)]
    pub fn ncodr(&mut self) -> NCODR_W<P110PFS_HA_SPEC> {
        NCODR_W::new(self, 6)
    }
    ///Bits 10:11 - Drive Strength Control Register
    #[inline(always)]
    pub fn dscr(&mut self) -> DSCR_W<P110PFS_HA_SPEC> {
        DSCR_W::new(self, 10)
    }
    ///Bit 12 - Event on Rising
    #[inline(always)]
    pub fn eor(&mut self) -> EOR_W<P110PFS_HA_SPEC> {
        EOR_W::new(self, 12)
    }
    ///Bit 13 - Event on Falling
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W<P110PFS_HA_SPEC> {
        EOF_W::new(self, 13)
    }
    ///Bit 14 - IRQ input enable
    #[inline(always)]
    pub fn isel(&mut self) -> ISEL_W<P110PFS_HA_SPEC> {
        ISEL_W::new(self, 14)
    }
    ///Bit 15 - Analog Input enable
    #[inline(always)]
    pub fn asel(&mut self) -> ASEL_W<P110PFS_HA_SPEC> {
        ASEL_W::new(self, 15)
    }
}
/**P110 Pin Function Control Register

You can [`read`](crate::Reg::read) this register and get [`p110pfs_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p110pfs_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct P110PFS_HA_SPEC;
impl crate::RegisterSpec for P110PFS_HA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`p110pfs_ha::R`](R) reader structure
impl crate::Readable for P110PFS_HA_SPEC {}
///`write(|w| ..)` method takes [`p110pfs_ha::W`](W) writer structure
impl crate::Writable for P110PFS_HA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P110PFS_HA to value 0x10
impl crate::Resettable for P110PFS_HA_SPEC {
    const RESET_VALUE: u16 = 0x10;
}
