///Register `SPCR` reader
pub type R = crate::R<SPCR_SPEC>;
///Register `SPCR` writer
pub type W = crate::W<SPCR_SPEC>;
/**SPI Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPMS_A {
    ///0: SPI operation (4-wire method)
    _0 = 0,
    ///1: Clock synchronous operation (3-wire method)
    _1 = 1,
}
impl From<SPMS_A> for bool {
    #[inline(always)]
    fn from(variant: SPMS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPMS` reader - SPI Mode Select
pub type SPMS_R = crate::BitReader<SPMS_A>;
impl SPMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPMS_A {
        match self.bits {
            false => SPMS_A::_0,
            true => SPMS_A::_1,
        }
    }
    ///SPI operation (4-wire method)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPMS_A::_0
    }
    ///Clock synchronous operation (3-wire method)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPMS_A::_1
    }
}
///Field `SPMS` writer - SPI Mode Select
pub type SPMS_W<'a, REG> = crate::BitWriter<'a, REG, SPMS_A>;
impl<'a, REG> SPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI operation (4-wire method)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPMS_A::_0)
    }
    ///Clock synchronous operation (3-wire method)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPMS_A::_1)
    }
}
/**Communications Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMD_A {
    ///0: Full-duplex synchronous serial communications
    _0 = 0,
    ///1: Serial communications consisting of only transmit operations
    _1 = 1,
}
impl From<TXMD_A> for bool {
    #[inline(always)]
    fn from(variant: TXMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMD` reader - Communications Operating Mode Select
pub type TXMD_R = crate::BitReader<TXMD_A>;
impl TXMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXMD_A {
        match self.bits {
            false => TXMD_A::_0,
            true => TXMD_A::_1,
        }
    }
    ///Full-duplex synchronous serial communications
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXMD_A::_0
    }
    ///Serial communications consisting of only transmit operations
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXMD_A::_1
    }
}
///Field `TXMD` writer - Communications Operating Mode Select
pub type TXMD_W<'a, REG> = crate::BitWriter<'a, REG, TXMD_A>;
impl<'a, REG> TXMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full-duplex synchronous serial communications
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TXMD_A::_0)
    }
    ///Serial communications consisting of only transmit operations
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TXMD_A::_1)
    }
}
/**Mode Fault Error Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFEN_A {
    ///0: Disables the detection of mode fault error
    _0 = 0,
    ///1: Enables the detection of mode fault error
    _1 = 1,
}
impl From<MODFEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MODFEN` reader - Mode Fault Error Detection Enable
pub type MODFEN_R = crate::BitReader<MODFEN_A>;
impl MODFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODFEN_A {
        match self.bits {
            false => MODFEN_A::_0,
            true => MODFEN_A::_1,
        }
    }
    ///Disables the detection of mode fault error
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFEN_A::_0
    }
    ///Enables the detection of mode fault error
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFEN_A::_1
    }
}
///Field `MODFEN` writer - Mode Fault Error Detection Enable
pub type MODFEN_W<'a, REG> = crate::BitWriter<'a, REG, MODFEN_A>;
impl<'a, REG> MODFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the detection of mode fault error
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MODFEN_A::_0)
    }
    ///Enables the detection of mode fault error
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MODFEN_A::_1)
    }
}
/**SPI Master/Slave Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    ///0: Slave mode
    _0 = 0,
    ///1: Master mode
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTR` reader - SPI Master/Slave Mode Select
pub type MSTR_R = crate::BitReader<MSTR_A>;
impl MSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    ///Slave mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_A::_0
    }
    ///Master mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_A::_1
    }
}
///Field `MSTR` writer - SPI Master/Slave Mode Select
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG, MSTR_A>;
impl<'a, REG> MSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::_0)
    }
    ///Master mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::_1)
    }
}
/**SPI Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEIE_A {
    ///0: Disables the generation of SPI error interrupt requests
    _0 = 0,
    ///1: Enables the generation of SPI error interrupt requests
    _1 = 1,
}
impl From<SPEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPEIE` reader - SPI Error Interrupt Enable
pub type SPEIE_R = crate::BitReader<SPEIE_A>;
impl SPEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPEIE_A {
        match self.bits {
            false => SPEIE_A::_0,
            true => SPEIE_A::_1,
        }
    }
    ///Disables the generation of SPI error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEIE_A::_0
    }
    ///Enables the generation of SPI error interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEIE_A::_1
    }
}
///Field `SPEIE` writer - SPI Error Interrupt Enable
pub type SPEIE_W<'a, REG> = crate::BitWriter<'a, REG, SPEIE_A>;
impl<'a, REG> SPEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the generation of SPI error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPEIE_A::_0)
    }
    ///Enables the generation of SPI error interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPEIE_A::_1)
    }
}
/**Transmit Buffer Empty Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPTIE_A {
    ///0: Disables the generation of transmit buffer empty interrupt requests
    _0 = 0,
    ///1: Enables the generation of transmit buffer empty interrupt requests
    _1 = 1,
}
impl From<SPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPTIE` reader - Transmit Buffer Empty Interrupt Enable
pub type SPTIE_R = crate::BitReader<SPTIE_A>;
impl SPTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPTIE_A {
        match self.bits {
            false => SPTIE_A::_0,
            true => SPTIE_A::_1,
        }
    }
    ///Disables the generation of transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTIE_A::_0
    }
    ///Enables the generation of transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTIE_A::_1
    }
}
///Field `SPTIE` writer - Transmit Buffer Empty Interrupt Enable
pub type SPTIE_W<'a, REG> = crate::BitWriter<'a, REG, SPTIE_A>;
impl<'a, REG> SPTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the generation of transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPTIE_A::_0)
    }
    ///Enables the generation of transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPTIE_A::_1)
    }
}
/**SPI Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    ///0: Disables the SPI function
    _0 = 0,
    ///1: Enables the SPI function
    _1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPE` reader - SPI Function Enable
pub type SPE_R = crate::BitReader<SPE_A>;
impl SPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::_0,
            true => SPE_A::_1,
        }
    }
    ///Disables the SPI function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPE_A::_0
    }
    ///Enables the SPI function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPE_A::_1
    }
}
///Field `SPE` writer - SPI Function Enable
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE_A>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the SPI function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::_0)
    }
    ///Enables the SPI function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::_1)
    }
}
/**SPI Receive Buffer Full Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRIE_A {
    ///0: Disables the generation of SPI receive buffer full interrupt requests
    _0 = 0,
    ///1: Enables the generation of SPI receive buffer full interrupt requests
    _1 = 1,
}
impl From<SPRIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPRIE` reader - SPI Receive Buffer Full Interrupt Enable
pub type SPRIE_R = crate::BitReader<SPRIE_A>;
impl SPRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPRIE_A {
        match self.bits {
            false => SPRIE_A::_0,
            true => SPRIE_A::_1,
        }
    }
    ///Disables the generation of SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRIE_A::_0
    }
    ///Enables the generation of SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRIE_A::_1
    }
}
///Field `SPRIE` writer - SPI Receive Buffer Full Interrupt Enable
pub type SPRIE_W<'a, REG> = crate::BitWriter<'a, REG, SPRIE_A>;
impl<'a, REG> SPRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the generation of SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPRIE_A::_0)
    }
    ///Enables the generation of SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPRIE_A::_1)
    }
}
impl R {
    ///Bit 0 - SPI Mode Select
    #[inline(always)]
    pub fn spms(&self) -> SPMS_R {
        SPMS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Communications Operating Mode Select
    #[inline(always)]
    pub fn txmd(&self) -> TXMD_R {
        TXMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mode Fault Error Detection Enable
    #[inline(always)]
    pub fn modfen(&self) -> MODFEN_R {
        MODFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SPI Master/Slave Mode Select
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPI Error Interrupt Enable
    #[inline(always)]
    pub fn speie(&self) -> SPEIE_R {
        SPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit Buffer Empty Interrupt Enable
    #[inline(always)]
    pub fn sptie(&self) -> SPTIE_R {
        SPTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPI Function Enable
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPI Receive Buffer Full Interrupt Enable
    #[inline(always)]
    pub fn sprie(&self) -> SPRIE_R {
        SPRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SPI Mode Select
    #[inline(always)]
    pub fn spms(&mut self) -> SPMS_W<SPCR_SPEC> {
        SPMS_W::new(self, 0)
    }
    ///Bit 1 - Communications Operating Mode Select
    #[inline(always)]
    pub fn txmd(&mut self) -> TXMD_W<SPCR_SPEC> {
        TXMD_W::new(self, 1)
    }
    ///Bit 2 - Mode Fault Error Detection Enable
    #[inline(always)]
    pub fn modfen(&mut self) -> MODFEN_W<SPCR_SPEC> {
        MODFEN_W::new(self, 2)
    }
    ///Bit 3 - SPI Master/Slave Mode Select
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<SPCR_SPEC> {
        MSTR_W::new(self, 3)
    }
    ///Bit 4 - SPI Error Interrupt Enable
    #[inline(always)]
    pub fn speie(&mut self) -> SPEIE_W<SPCR_SPEC> {
        SPEIE_W::new(self, 4)
    }
    ///Bit 5 - Transmit Buffer Empty Interrupt Enable
    #[inline(always)]
    pub fn sptie(&mut self) -> SPTIE_W<SPCR_SPEC> {
        SPTIE_W::new(self, 5)
    }
    ///Bit 6 - SPI Function Enable
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<SPCR_SPEC> {
        SPE_W::new(self, 6)
    }
    ///Bit 7 - SPI Receive Buffer Full Interrupt Enable
    #[inline(always)]
    pub fn sprie(&mut self) -> SPRIE_W<SPCR_SPEC> {
        SPRIE_W::new(self, 7)
    }
}
/**SPI Control Register

You can [`read`](crate::Reg::read) this register and get [`spcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPCR_SPEC;
impl crate::RegisterSpec for SPCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spcr::R`](R) reader structure
impl crate::Readable for SPCR_SPEC {}
///`write(|w| ..)` method takes [`spcr::W`](W) writer structure
impl crate::Writable for SPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCR to value 0
impl crate::Resettable for SPCR_SPEC {}
