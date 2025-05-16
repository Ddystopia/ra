///Register `SD_ERR_STS2` reader
pub type R = crate::R<SD_ERR_STS2_SPEC>;
/**Response Timeout 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO0_A {
    ///0: Not timeout.
    _0 = 0,
    ///1: The response (other than a response to a command issued within a command sequence) is not received though a longer time than 640 cycles of SD/MMC clock has elapsed.
    _1 = 1,
}
impl From<RSPTO0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTO0` reader - Response Timeout 0
pub type RSPTO0_R = crate::BitReader<RSPTO0_A>;
impl RSPTO0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPTO0_A {
        match self.bits {
            false => RSPTO0_A::_0,
            true => RSPTO0_A::_1,
        }
    }
    ///Not timeout.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO0_A::_0
    }
    ///The response (other than a response to a command issued within a command sequence) is not received though a longer time than 640 cycles of SD/MMC clock has elapsed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO0_A::_1
    }
}
/**Response Timeout 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO1_A {
    ///0: Not timeout.
    _0 = 0,
    ///1: The response to a command issued within a command sequence*2 is not received though a longer time than 640 cycles of SD/MMC clock has elapsed. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPTO0.
    _1 = 1,
}
impl From<RSPTO1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTO1` reader - Response Timeout 1
pub type RSPTO1_R = crate::BitReader<RSPTO1_A>;
impl RSPTO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPTO1_A {
        match self.bits {
            false => RSPTO1_A::_0,
            true => RSPTO1_A::_1,
        }
    }
    ///Not timeout.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO1_A::_0
    }
    ///The response to a command issued within a command sequence*2 is not received though a longer time than 640 cycles of SD/MMC clock has elapsed. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPTO0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO1_A::_1
    }
}
/**Busy Timeout 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYTO0_A {
    ///0: Not timeout.
    _0 = 0,
    ///1: The busy state for longer than N-cycle continues after R1b response.
    _1 = 1,
}
impl From<BSYTO0_A> for bool {
    #[inline(always)]
    fn from(variant: BSYTO0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSYTO0` reader - Busy Timeout 0
pub type BSYTO0_R = crate::BitReader<BSYTO0_A>;
impl BSYTO0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSYTO0_A {
        match self.bits {
            false => BSYTO0_A::_0,
            true => BSYTO0_A::_1,
        }
    }
    ///Not timeout.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYTO0_A::_0
    }
    ///The busy state for longer than N-cycle continues after R1b response.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYTO0_A::_1
    }
}
/**Busy Timeout 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYTO1_A {
    ///0: Not timeout.
    _0 = 0,
    ///1: The busy state for longer than N-cycle continues after CMD12 has been issued within a command sequence. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in BSYTO0.
    _1 = 1,
}
impl From<BSYTO1_A> for bool {
    #[inline(always)]
    fn from(variant: BSYTO1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSYTO1` reader - Busy Timeout 1
pub type BSYTO1_R = crate::BitReader<BSYTO1_A>;
impl BSYTO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSYTO1_A {
        match self.bits {
            false => BSYTO1_A::_0,
            true => BSYTO1_A::_1,
        }
    }
    ///Not timeout.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSYTO1_A::_0
    }
    ///The busy state for longer than N-cycle continues after CMD12 has been issued within a command sequence. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in BSYTO0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSYTO1_A::_1
    }
}
/**Read Data Timeout

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDTO_A {
    ///0: Not timeout
    _0 = 0,
    ///1: The read data is not received though a longer time than N-cycle has elapsed after read command. / The read data for the next block are not received though a longer time than N-cycle has elapsed after the reception of read data. / The read data for the next block are not received though a longer time than N-cycle has elapsed after release of the read wait state.
    _1 = 1,
}
impl From<RDTO_A> for bool {
    #[inline(always)]
    fn from(variant: RDTO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDTO` reader - Read Data Timeout
pub type RDTO_R = crate::BitReader<RDTO_A>;
impl RDTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDTO_A {
        match self.bits {
            false => RDTO_A::_0,
            true => RDTO_A::_1,
        }
    }
    ///Not timeout
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDTO_A::_0
    }
    ///The read data is not received though a longer time than N-cycle has elapsed after read command. / The read data for the next block are not received though a longer time than N-cycle has elapsed after the reception of read data. / The read data for the next block are not received though a longer time than N-cycle has elapsed after release of the read wait state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDTO_A::_1
    }
}
/**CRC Status Token Timeout

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCTO_A {
    ///0: Not timeout
    _0 = 0,
    ///1: The CRC status is not received though a longer time than N-cycle has elapsed after data writing.
    _1 = 1,
}
impl From<CRCTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRCTO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCTO` reader - CRC Status Token Timeout
pub type CRCTO_R = crate::BitReader<CRCTO_A>;
impl CRCTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCTO_A {
        match self.bits {
            false => CRCTO_A::_0,
            true => CRCTO_A::_1,
        }
    }
    ///Not timeout
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCTO_A::_0
    }
    ///The CRC status is not received though a longer time than N-cycle has elapsed after data writing.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCTO_A::_1
    }
}
/**CRC Status Token Busy Timeout

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCBSYTO_A {
    ///0: Not timeout
    _0 = 0,
    ///1: The busy state continues for longer than N-cycle after the CRC status
    _1 = 1,
}
impl From<CRCBSYTO_A> for bool {
    #[inline(always)]
    fn from(variant: CRCBSYTO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCBSYTO` reader - CRC Status Token Busy Timeout
pub type CRCBSYTO_R = crate::BitReader<CRCBSYTO_A>;
impl CRCBSYTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCBSYTO_A {
        match self.bits {
            false => CRCBSYTO_A::_0,
            true => CRCBSYTO_A::_1,
        }
    }
    ///Not timeout
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCBSYTO_A::_0
    }
    ///The busy state continues for longer than N-cycle after the CRC status
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCBSYTO_A::_1
    }
}
impl R {
    ///Bit 0 - Response Timeout 0
    #[inline(always)]
    pub fn rspto0(&self) -> RSPTO0_R {
        RSPTO0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Response Timeout 1
    #[inline(always)]
    pub fn rspto1(&self) -> RSPTO1_R {
        RSPTO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Busy Timeout 0
    #[inline(always)]
    pub fn bsyto0(&self) -> BSYTO0_R {
        BSYTO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Busy Timeout 1
    #[inline(always)]
    pub fn bsyto1(&self) -> BSYTO1_R {
        BSYTO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Read Data Timeout
    #[inline(always)]
    pub fn rdto(&self) -> RDTO_R {
        RDTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CRC Status Token Timeout
    #[inline(always)]
    pub fn crcto(&self) -> CRCTO_R {
        CRCTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CRC Status Token Busy Timeout
    #[inline(always)]
    pub fn crcbsyto(&self) -> CRCBSYTO_R {
        CRCBSYTO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
/**SD Error Status Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_ERR_STS2_SPEC;
impl crate::RegisterSpec for SD_ERR_STS2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_err_sts2::R`](R) reader structure
impl crate::Readable for SD_ERR_STS2_SPEC {}
///`reset()` method sets SD_ERR_STS2 to value 0
impl crate::Resettable for SD_ERR_STS2_SPEC {}
