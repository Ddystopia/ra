///Register `SD_ERR_STS1` reader
pub type R = crate::R<SD_ERR_STS1_SPEC>;
/**Command Error 0NOTE: other than a response to a command issued within a command sequence

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDE0_A {
    ///0: An error has not occured in the command index of a response.
    _0 = 0,
    ///1: An error has occured in the command index of a response.
    _1 = 1,
}
impl From<CMDE0_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDE0` reader - Command Error 0NOTE: other than a response to a command issued within a command sequence
pub type CMDE0_R = crate::BitReader<CMDE0_A>;
impl CMDE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDE0_A {
        match self.bits {
            false => CMDE0_A::_0,
            true => CMDE0_A::_1,
        }
    }
    ///An error has not occured in the command index of a response.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDE0_A::_0
    }
    ///An error has occured in the command index of a response.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDE0_A::_1
    }
}
/**Command Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is Indicated in CMDE0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDE1_A {
    ///0: An error has not occurs in the command index of the response to a command issued within a command sequence.
    _0 = 0,
    ///1: An error has occured in the command index of the response to a command issued within a command sequence.
    _1 = 1,
}
impl From<CMDE1_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDE1` reader - Command Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is Indicated in CMDE0.
pub type CMDE1_R = crate::BitReader<CMDE1_A>;
impl CMDE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDE1_A {
        match self.bits {
            false => CMDE1_A::_0,
            true => CMDE1_A::_1,
        }
    }
    ///An error has not occurs in the command index of the response to a command issued within a command sequence.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDE1_A::_0
    }
    ///An error has occured in the command index of the response to a command issued within a command sequence.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDE1_A::_1
    }
}
/**Response Length Error 0NOTE: other than a response to a command issued within a command sequence

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPLENE0_A {
    ///0: An error has not occured in the response length
    _0 = 0,
    ///1: An error has occured in the response length
    _1 = 1,
}
impl From<RSPLENE0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPLENE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPLENE0` reader - Response Length Error 0NOTE: other than a response to a command issued within a command sequence
pub type RSPLENE0_R = crate::BitReader<RSPLENE0_A>;
impl RSPLENE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPLENE0_A {
        match self.bits {
            false => RSPLENE0_A::_0,
            true => RSPLENE0_A::_1,
        }
    }
    ///An error has not occured in the response length
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPLENE0_A::_0
    }
    ///An error has occured in the response length
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPLENE0_A::_1
    }
}
/**Response Length Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPLENE0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPLENE1_A {
    ///0: An error has not occurred in the response length to a command issued within a command sequence.
    _0 = 0,
    ///1: An error has occured in the response length to a command issued within a command sequence.
    _1 = 1,
}
impl From<RSPLENE1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPLENE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPLENE1` reader - Response Length Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPLENE0.
pub type RSPLENE1_R = crate::BitReader<RSPLENE1_A>;
impl RSPLENE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPLENE1_A {
        match self.bits {
            false => RSPLENE1_A::_0,
            true => RSPLENE1_A::_1,
        }
    }
    ///An error has not occurred in the response length to a command issued within a command sequence.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPLENE1_A::_0
    }
    ///An error has occured in the response length to a command issued within a command sequence.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPLENE1_A::_1
    }
}
/**Read Data Length Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDLENE_A {
    ///0: An error has occurred not in the read data length.
    _0 = 0,
    ///1: An error has occured in the read data length (and the end bit has not been detected among the valid bits).
    _1 = 1,
}
impl From<RDLENE_A> for bool {
    #[inline(always)]
    fn from(variant: RDLENE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDLENE` reader - Read Data Length Error
pub type RDLENE_R = crate::BitReader<RDLENE_A>;
impl RDLENE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDLENE_A {
        match self.bits {
            false => RDLENE_A::_0,
            true => RDLENE_A::_1,
        }
    }
    ///An error has occurred not in the read data length.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDLENE_A::_0
    }
    ///An error has occured in the read data length (and the end bit has not been detected among the valid bits).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDLENE_A::_1
    }
}
/**CRC Status Token Length Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCLENE_A {
    ///0: An error has not occured in the CRC status length.
    _0 = 0,
    ///1: An error has occured in the CRC status length (and the end bit has not been detected)
    _1 = 1,
}
impl From<CRCLENE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCLENE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCLENE` reader - CRC Status Token Length Error
pub type CRCLENE_R = crate::BitReader<CRCLENE_A>;
impl CRCLENE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCLENE_A {
        match self.bits {
            false => CRCLENE_A::_0,
            true => CRCLENE_A::_1,
        }
    }
    ///An error has not occured in the CRC status length.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCLENE_A::_0
    }
    ///An error has occured in the CRC status length (and the end bit has not been detected)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCLENE_A::_1
    }
}
/**Response CRC Error 0NOTE: other than a response to a command issued within a command sequence

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPCRCE0_A {
    ///0: A CRC error has not occur in a response
    _0 = 0,
    ///1: A CRC error has occured in a response
    _1 = 1,
}
impl From<RSPCRCE0_A> for bool {
    #[inline(always)]
    fn from(variant: RSPCRCE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPCRCE0` reader - Response CRC Error 0NOTE: other than a response to a command issued within a command sequence
pub type RSPCRCE0_R = crate::BitReader<RSPCRCE0_A>;
impl RSPCRCE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPCRCE0_A {
        match self.bits {
            false => RSPCRCE0_A::_0,
            true => RSPCRCE0_A::_1,
        }
    }
    ///A CRC error has not occur in a response
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPCRCE0_A::_0
    }
    ///A CRC error has occured in a response
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPCRCE0_A::_1
    }
}
/**Response CRC Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPCRCE0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPCRCE1_A {
    ///0: CRC error has not occured.
    _0 = 0,
    ///1: CRC error has occured in the response to a command issued within a command sequence.
    _1 = 1,
}
impl From<RSPCRCE1_A> for bool {
    #[inline(always)]
    fn from(variant: RSPCRCE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPCRCE1` reader - Response CRC Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPCRCE0.
pub type RSPCRCE1_R = crate::BitReader<RSPCRCE1_A>;
impl RSPCRCE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPCRCE1_A {
        match self.bits {
            false => RSPCRCE1_A::_0,
            true => RSPCRCE1_A::_1,
        }
    }
    ///CRC error has not occured.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPCRCE1_A::_0
    }
    ///CRC error has occured in the response to a command issued within a command sequence.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPCRCE1_A::_1
    }
}
/**Read Data CRC Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDCRCE_A {
    ///0: CRC error has detected in read data
    _0 = 0,
    ///1: CRC error has not detected in read data
    _1 = 1,
}
impl From<RDCRCE_A> for bool {
    #[inline(always)]
    fn from(variant: RDCRCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDCRCE` reader - Read Data CRC Error
pub type RDCRCE_R = crate::BitReader<RDCRCE_A>;
impl RDCRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDCRCE_A {
        match self.bits {
            false => RDCRCE_A::_0,
            true => RDCRCE_A::_1,
        }
    }
    ///CRC error has detected in read data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDCRCE_A::_0
    }
    ///CRC error has not detected in read data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDCRCE_A::_1
    }
}
/**CRC Status Token Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCTKE_A {
    ///0: An error has not occured in the CRC status.
    _0 = 0,
    ///1: An error has occured in the CRC status.
    _1 = 1,
}
impl From<CRCTKE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCTKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCTKE` reader - CRC Status Token Error
pub type CRCTKE_R = crate::BitReader<CRCTKE_A>;
impl CRCTKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCTKE_A {
        match self.bits {
            false => CRCTKE_A::_0,
            true => CRCTKE_A::_1,
        }
    }
    ///An error has not occured in the CRC status.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCTKE_A::_0
    }
    ///An error has occured in the CRC status.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCTKE_A::_1
    }
}
///Field `CRCTK` reader - CRC Status TokenStore the CRC status token value (normal value is 010b)
pub type CRCTK_R = crate::FieldReader;
impl R {
    ///Bit 0 - Command Error 0NOTE: other than a response to a command issued within a command sequence
    #[inline(always)]
    pub fn cmde0(&self) -> CMDE0_R {
        CMDE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is Indicated in CMDE0.
    #[inline(always)]
    pub fn cmde1(&self) -> CMDE1_R {
        CMDE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Response Length Error 0NOTE: other than a response to a command issued within a command sequence
    #[inline(always)]
    pub fn rsplene0(&self) -> RSPLENE0_R {
        RSPLENE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Response Length Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPLENE0.
    #[inline(always)]
    pub fn rsplene1(&self) -> RSPLENE1_R {
        RSPLENE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Read Data Length Error
    #[inline(always)]
    pub fn rdlene(&self) -> RDLENE_R {
        RDLENE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CRC Status Token Length Error
    #[inline(always)]
    pub fn crclene(&self) -> CRCLENE_R {
        CRCLENE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Response CRC Error 0NOTE: other than a response to a command issued within a command sequence
    #[inline(always)]
    pub fn rspcrce0(&self) -> RSPCRCE0_R {
        RSPCRCE0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Response CRC Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPCRCE0.
    #[inline(always)]
    pub fn rspcrce1(&self) -> RSPCRCE1_R {
        RSPCRCE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read Data CRC Error
    #[inline(always)]
    pub fn rdcrce(&self) -> RDCRCE_R {
        RDCRCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC Status Token Error
    #[inline(always)]
    pub fn crctke(&self) -> CRCTKE_R {
        CRCTKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - CRC Status TokenStore the CRC status token value (normal value is 010b)
    #[inline(always)]
    pub fn crctk(&self) -> CRCTK_R {
        CRCTK_R::new(((self.bits >> 12) & 7) as u8)
    }
}
/**SD Error Status Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_ERR_STS1_SPEC;
impl crate::RegisterSpec for SD_ERR_STS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_err_sts1::R`](R) reader structure
impl crate::Readable for SD_ERR_STS1_SPEC {}
///`reset()` method sets SD_ERR_STS1 to value 0x2000
impl crate::Resettable for SD_ERR_STS1_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
