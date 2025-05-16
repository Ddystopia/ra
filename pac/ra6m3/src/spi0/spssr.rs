///Register `SPSSR` reader
pub type R = crate::R<SPSSR_SPEC>;
/**RSPI Command Pointer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPCP_A {
    ///0: SPCMD0
    _000 = 0,
    ///1: SPCMD1
    _001 = 1,
    ///2: SPCMD2
    _010 = 2,
    ///3: SPCMD3
    _011 = 3,
    ///4: SPCMD4
    _100 = 4,
    ///5: SPCMD5
    _101 = 5,
    ///6: SPCMD6
    _110 = 6,
    ///7: SPCMD7
    _111 = 7,
}
impl From<SPCP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPCP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPCP_A {
    type Ux = u8;
}
impl crate::IsEnum for SPCP_A {}
///Field `SPCP` reader - RSPI Command Pointer
pub type SPCP_R = crate::FieldReader<SPCP_A>;
impl SPCP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPCP_A {
        match self.bits {
            0 => SPCP_A::_000,
            1 => SPCP_A::_001,
            2 => SPCP_A::_010,
            3 => SPCP_A::_011,
            4 => SPCP_A::_100,
            5 => SPCP_A::_101,
            6 => SPCP_A::_110,
            7 => SPCP_A::_111,
            _ => unreachable!(),
        }
    }
    ///SPCMD0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPCP_A::_000
    }
    ///SPCMD1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPCP_A::_001
    }
    ///SPCMD2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPCP_A::_010
    }
    ///SPCMD3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPCP_A::_011
    }
    ///SPCMD4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPCP_A::_100
    }
    ///SPCMD5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPCP_A::_101
    }
    ///SPCMD6
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPCP_A::_110
    }
    ///SPCMD7
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPCP_A::_111
    }
}
/**RSPI Error Command

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPECM_A {
    ///0: SPCMD0
    _000 = 0,
    ///1: SPCMD1
    _001 = 1,
    ///2: SPCMD2
    _010 = 2,
    ///3: SPCMD3
    _011 = 3,
    ///4: SPCMD4
    _100 = 4,
    ///5: SPCMD5
    _101 = 5,
    ///6: SPCMD6
    _110 = 6,
    ///7: SPCMD7
    _111 = 7,
}
impl From<SPECM_A> for u8 {
    #[inline(always)]
    fn from(variant: SPECM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPECM_A {
    type Ux = u8;
}
impl crate::IsEnum for SPECM_A {}
///Field `SPECM` reader - RSPI Error Command
pub type SPECM_R = crate::FieldReader<SPECM_A>;
impl SPECM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPECM_A {
        match self.bits {
            0 => SPECM_A::_000,
            1 => SPECM_A::_001,
            2 => SPECM_A::_010,
            3 => SPECM_A::_011,
            4 => SPECM_A::_100,
            5 => SPECM_A::_101,
            6 => SPECM_A::_110,
            7 => SPECM_A::_111,
            _ => unreachable!(),
        }
    }
    ///SPCMD0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPECM_A::_000
    }
    ///SPCMD1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPECM_A::_001
    }
    ///SPCMD2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPECM_A::_010
    }
    ///SPCMD3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPECM_A::_011
    }
    ///SPCMD4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPECM_A::_100
    }
    ///SPCMD5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPECM_A::_101
    }
    ///SPCMD6
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPECM_A::_110
    }
    ///SPCMD7
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPECM_A::_111
    }
}
impl R {
    ///Bits 0:2 - RSPI Command Pointer
    #[inline(always)]
    pub fn spcp(&self) -> SPCP_R {
        SPCP_R::new(self.bits & 7)
    }
    ///Bits 4:6 - RSPI Error Command
    #[inline(always)]
    pub fn specm(&self) -> SPECM_R {
        SPECM_R::new((self.bits >> 4) & 7)
    }
}
/**SPI Sequence Status Register

You can [`read`](crate::Reg::read) this register and get [`spssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPSSR_SPEC;
impl crate::RegisterSpec for SPSSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spssr::R`](R) reader structure
impl crate::Readable for SPSSR_SPEC {}
///`reset()` method sets SPSSR to value 0
impl crate::Resettable for SPSSR_SPEC {}
