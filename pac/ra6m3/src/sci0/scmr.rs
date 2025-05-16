///Register `SCMR` reader
pub type R = crate::R<SCMR_SPEC>;
///Register `SCMR` writer
pub type W = crate::W<SCMR_SPEC>;
/**Smart Card Interface Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMIF_A {
    ///0: Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)
    _0 = 0,
    ///1: Smart card interface mode
    _1 = 1,
}
impl From<SMIF_A> for bool {
    #[inline(always)]
    fn from(variant: SMIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SMIF` reader - Smart Card Interface Mode Select
pub type SMIF_R = crate::BitReader<SMIF_A>;
impl SMIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMIF_A {
        match self.bits {
            false => SMIF_A::_0,
            true => SMIF_A::_1,
        }
    }
    ///Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMIF_A::_0
    }
    ///Smart card interface mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMIF_A::_1
    }
}
///Field `SMIF` writer - Smart Card Interface Mode Select
pub type SMIF_W<'a, REG> = crate::BitWriter<'a, REG, SMIF_A>;
impl<'a, REG> SMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SMIF_A::_0)
    }
    ///Smart card interface mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SMIF_A::_1)
    }
}
/**Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINV_A {
    ///0: TDR contents are transmitted as they are. Receive data is stored as it is in RDR.
    _0 = 0,
    ///1: TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR.
    _1 = 1,
}
impl From<SINV_A> for bool {
    #[inline(always)]
    fn from(variant: SINV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SINV` reader - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode.
pub type SINV_R = crate::BitReader<SINV_A>;
impl SINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SINV_A {
        match self.bits {
            false => SINV_A::_0,
            true => SINV_A::_1,
        }
    }
    ///TDR contents are transmitted as they are. Receive data is stored as it is in RDR.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SINV_A::_0
    }
    ///TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SINV_A::_1
    }
}
///Field `SINV` writer - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode.
pub type SINV_W<'a, REG> = crate::BitWriter<'a, REG, SINV_A>;
impl<'a, REG> SINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TDR contents are transmitted as they are. Receive data is stored as it is in RDR.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SINV_A::_0)
    }
    ///TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SINV_A::_1)
    }
}
/**Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIR_A {
    ///0: Transfer with LSB first
    _0 = 0,
    ///1: Transfer with MSB first
    _1 = 1,
}
impl From<SDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SDIR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDIR` reader - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode.
pub type SDIR_R = crate::BitReader<SDIR_A>;
impl SDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDIR_A {
        match self.bits {
            false => SDIR_A::_0,
            true => SDIR_A::_1,
        }
    }
    ///Transfer with LSB first
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDIR_A::_0
    }
    ///Transfer with MSB first
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDIR_A::_1
    }
}
///Field `SDIR` writer - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode.
pub type SDIR_W<'a, REG> = crate::BitWriter<'a, REG, SDIR_A>;
impl<'a, REG> SDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer with LSB first
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDIR_A::_0)
    }
    ///Transfer with MSB first
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDIR_A::_1)
    }
}
/**Character Length 1(Only valid in asynchronous mode)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHR1_A {
    ///0: Transmit/receive in 9-bit data length
    _0 = 0,
    ///1: Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)
    _1 = 1,
}
impl From<CHR1_A> for bool {
    #[inline(always)]
    fn from(variant: CHR1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHR1` reader - Character Length 1(Only valid in asynchronous mode)
pub type CHR1_R = crate::BitReader<CHR1_A>;
impl CHR1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHR1_A {
        match self.bits {
            false => CHR1_A::_0,
            true => CHR1_A::_1,
        }
    }
    ///Transmit/receive in 9-bit data length
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHR1_A::_0
    }
    ///Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHR1_A::_1
    }
}
///Field `CHR1` writer - Character Length 1(Only valid in asynchronous mode)
pub type CHR1_W<'a, REG> = crate::BitWriter<'a, REG, CHR1_A>;
impl<'a, REG> CHR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit/receive in 9-bit data length
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CHR1_A::_0)
    }
    ///Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CHR1_A::_1)
    }
}
/**Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\[1:0\] bits

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCP2_A {
    ///0: S=93(SMR.BCP\[1:0\]=00), 128(SMR.BCP\[1:0\]=01), 186(SMR.BCP\[1:0\]=10), 512(SMR.BCP\[1:0\]=11)
    _0 = 0,
    ///1: S=32(SMR.BCP\[1:0\]=00), 64(SMR.BCP\[1:0\]=01), 372(SMR.BCP\[1:0\]=10), 256(SMR.BCP\[1:0\]=11)
    _1 = 1,
}
impl From<BCP2_A> for bool {
    #[inline(always)]
    fn from(variant: BCP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCP2` reader - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\[1:0\] bits
pub type BCP2_R = crate::BitReader<BCP2_A>;
impl BCP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCP2_A {
        match self.bits {
            false => BCP2_A::_0,
            true => BCP2_A::_1,
        }
    }
    ///S=93(SMR.BCP\[1:0\]=00), 128(SMR.BCP\[1:0\]=01), 186(SMR.BCP\[1:0\]=10), 512(SMR.BCP\[1:0\]=11)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCP2_A::_0
    }
    ///S=32(SMR.BCP\[1:0\]=00), 64(SMR.BCP\[1:0\]=01), 372(SMR.BCP\[1:0\]=10), 256(SMR.BCP\[1:0\]=11)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCP2_A::_1
    }
}
///Field `BCP2` writer - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\[1:0\] bits
pub type BCP2_W<'a, REG> = crate::BitWriter<'a, REG, BCP2_A>;
impl<'a, REG> BCP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S=93(SMR.BCP\[1:0\]=00), 128(SMR.BCP\[1:0\]=01), 186(SMR.BCP\[1:0\]=10), 512(SMR.BCP\[1:0\]=11)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCP2_A::_0)
    }
    ///S=32(SMR.BCP\[1:0\]=00), 64(SMR.BCP\[1:0\]=01), 372(SMR.BCP\[1:0\]=10), 256(SMR.BCP\[1:0\]=11)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCP2_A::_1)
    }
}
impl R {
    ///Bit 0 - Smart Card Interface Mode Select
    #[inline(always)]
    pub fn smif(&self) -> SMIF_R {
        SMIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode.
    #[inline(always)]
    pub fn sinv(&self) -> SINV_R {
        SINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode.
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Character Length 1(Only valid in asynchronous mode)
    #[inline(always)]
    pub fn chr1(&self) -> CHR1_R {
        CHR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\[1:0\] bits
    #[inline(always)]
    pub fn bcp2(&self) -> BCP2_R {
        BCP2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Smart Card Interface Mode Select
    #[inline(always)]
    pub fn smif(&mut self) -> SMIF_W<SCMR_SPEC> {
        SMIF_W::new(self, 0)
    }
    ///Bit 2 - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode.
    #[inline(always)]
    pub fn sinv(&mut self) -> SINV_W<SCMR_SPEC> {
        SINV_W::new(self, 2)
    }
    ///Bit 3 - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode.
    #[inline(always)]
    pub fn sdir(&mut self) -> SDIR_W<SCMR_SPEC> {
        SDIR_W::new(self, 3)
    }
    ///Bit 4 - Character Length 1(Only valid in asynchronous mode)
    #[inline(always)]
    pub fn chr1(&mut self) -> CHR1_W<SCMR_SPEC> {
        CHR1_W::new(self, 4)
    }
    ///Bit 7 - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\[1:0\] bits
    #[inline(always)]
    pub fn bcp2(&mut self) -> BCP2_W<SCMR_SPEC> {
        BCP2_W::new(self, 7)
    }
}
/**Smart Card Mode Register

You can [`read`](crate::Reg::read) this register and get [`scmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCMR_SPEC;
impl crate::RegisterSpec for SCMR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`scmr::R`](R) reader structure
impl crate::Readable for SCMR_SPEC {}
///`write(|w| ..)` method takes [`scmr::W`](W) writer structure
impl crate::Writable for SCMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCMR to value 0xf2
impl crate::Resettable for SCMR_SPEC {
    const RESET_VALUE: u8 = 0xf2;
}
