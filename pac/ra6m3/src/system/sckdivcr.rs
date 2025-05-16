///Register `SCKDIVCR` reader
pub type R = crate::R<SCKDIVCR_SPEC>;
///Register `SCKDIVCR` writer
pub type W = crate::W<SCKDIVCR_SPEC>;
/**Peripheral Module Clock D (PCLKD) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKD_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<PCKD_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCKD_A {
    type Ux = u8;
}
impl crate::IsEnum for PCKD_A {}
///Field `PCKD` reader - Peripheral Module Clock D (PCLKD) Select
pub type PCKD_R = crate::FieldReader<PCKD_A>;
impl PCKD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKD_A {
        match self.bits {
            0 => PCKD_A::_000,
            1 => PCKD_A::_001,
            2 => PCKD_A::_010,
            3 => PCKD_A::_011,
            4 => PCKD_A::_100,
            5 => PCKD_A::_101,
            6 => PCKD_A::_110,
            7 => PCKD_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKD_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKD_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKD_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKD_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKD_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKD_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKD_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == PCKD_A::OTHERS
    }
}
///Field `PCKD` writer - Peripheral Module Clock D (PCLKD) Select
pub type PCKD_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PCKD_A, crate::Safe>;
impl<'a, REG> PCKD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PCKD_A::OTHERS)
    }
}
/**Peripheral Module Clock C (PCLKC) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKC_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<PCKC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCKC_A {
    type Ux = u8;
}
impl crate::IsEnum for PCKC_A {}
///Field `PCKC` reader - Peripheral Module Clock C (PCLKC) Select
pub type PCKC_R = crate::FieldReader<PCKC_A>;
impl PCKC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKC_A {
        match self.bits {
            0 => PCKC_A::_000,
            1 => PCKC_A::_001,
            2 => PCKC_A::_010,
            3 => PCKC_A::_011,
            4 => PCKC_A::_100,
            5 => PCKC_A::_101,
            6 => PCKC_A::_110,
            7 => PCKC_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKC_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKC_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKC_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKC_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKC_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKC_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKC_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == PCKC_A::OTHERS
    }
}
///Field `PCKC` writer - Peripheral Module Clock C (PCLKC) Select
pub type PCKC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PCKC_A, crate::Safe>;
impl<'a, REG> PCKC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PCKC_A::OTHERS)
    }
}
/**Peripheral Module Clock B (PCLKB) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKB_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<PCKB_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCKB_A {
    type Ux = u8;
}
impl crate::IsEnum for PCKB_A {}
///Field `PCKB` reader - Peripheral Module Clock B (PCLKB) Select
pub type PCKB_R = crate::FieldReader<PCKB_A>;
impl PCKB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKB_A {
        match self.bits {
            0 => PCKB_A::_000,
            1 => PCKB_A::_001,
            2 => PCKB_A::_010,
            3 => PCKB_A::_011,
            4 => PCKB_A::_100,
            5 => PCKB_A::_101,
            6 => PCKB_A::_110,
            7 => PCKB_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKB_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKB_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKB_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKB_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKB_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKB_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKB_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == PCKB_A::OTHERS
    }
}
///Field `PCKB` writer - Peripheral Module Clock B (PCLKB) Select
pub type PCKB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PCKB_A, crate::Safe>;
impl<'a, REG> PCKB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PCKB_A::OTHERS)
    }
}
/**Peripheral Module Clock A (PCLKA) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKA_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<PCKA_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCKA_A {
    type Ux = u8;
}
impl crate::IsEnum for PCKA_A {}
///Field `PCKA` reader - Peripheral Module Clock A (PCLKA) Select
pub type PCKA_R = crate::FieldReader<PCKA_A>;
impl PCKA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKA_A {
        match self.bits {
            0 => PCKA_A::_000,
            1 => PCKA_A::_001,
            2 => PCKA_A::_010,
            3 => PCKA_A::_011,
            4 => PCKA_A::_100,
            5 => PCKA_A::_101,
            6 => PCKA_A::_110,
            7 => PCKA_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKA_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKA_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKA_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKA_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKA_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKA_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKA_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == PCKA_A::OTHERS
    }
}
///Field `PCKA` writer - Peripheral Module Clock A (PCLKA) Select
pub type PCKA_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PCKA_A, crate::Safe>;
impl<'a, REG> PCKA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PCKA_A::OTHERS)
    }
}
/**External Bus Clock (BCLK) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCK_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<BCK_A> for u8 {
    #[inline(always)]
    fn from(variant: BCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCK_A {
    type Ux = u8;
}
impl crate::IsEnum for BCK_A {}
///Field `BCK` reader - External Bus Clock (BCLK) Select
pub type BCK_R = crate::FieldReader<BCK_A>;
impl BCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCK_A {
        match self.bits {
            0 => BCK_A::_000,
            1 => BCK_A::_001,
            2 => BCK_A::_010,
            3 => BCK_A::_011,
            4 => BCK_A::_100,
            5 => BCK_A::_101,
            6 => BCK_A::_110,
            7 => BCK_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BCK_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BCK_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BCK_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BCK_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BCK_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BCK_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BCK_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == BCK_A::OTHERS
    }
}
///Field `BCK` writer - External Bus Clock (BCLK) Select
pub type BCK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BCK_A, crate::Safe>;
impl<'a, REG> BCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(BCK_A::OTHERS)
    }
}
/**System Clock (ICLK) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICK_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<ICK_A> for u8 {
    #[inline(always)]
    fn from(variant: ICK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICK_A {
    type Ux = u8;
}
impl crate::IsEnum for ICK_A {}
///Field `ICK` reader - System Clock (ICLK) Select
pub type ICK_R = crate::FieldReader<ICK_A>;
impl ICK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICK_A {
        match self.bits {
            0 => ICK_A::_000,
            1 => ICK_A::_001,
            2 => ICK_A::_010,
            3 => ICK_A::_011,
            4 => ICK_A::_100,
            5 => ICK_A::_101,
            6 => ICK_A::_110,
            7 => ICK_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICK_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICK_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICK_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICK_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICK_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICK_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICK_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == ICK_A::OTHERS
    }
}
///Field `ICK` writer - System Clock (ICLK) Select
pub type ICK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ICK_A, crate::Safe>;
impl<'a, REG> ICK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ICK_A::OTHERS)
    }
}
/**Flash IF Clock (FCLK) Select

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCK_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: Setting prohibited
    OTHERS = 7,
}
impl From<FCK_A> for u8 {
    #[inline(always)]
    fn from(variant: FCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCK_A {
    type Ux = u8;
}
impl crate::IsEnum for FCK_A {}
///Field `FCK` reader - Flash IF Clock (FCLK) Select
pub type FCK_R = crate::FieldReader<FCK_A>;
impl FCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FCK_A {
        match self.bits {
            0 => FCK_A::_000,
            1 => FCK_A::_001,
            2 => FCK_A::_010,
            3 => FCK_A::_011,
            4 => FCK_A::_100,
            5 => FCK_A::_101,
            6 => FCK_A::_110,
            7 => FCK_A::OTHERS,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FCK_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FCK_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FCK_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FCK_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FCK_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FCK_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FCK_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == FCK_A::OTHERS
    }
}
///Field `FCK` writer - Flash IF Clock (FCLK) Select
pub type FCK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FCK_A, crate::Safe>;
impl<'a, REG> FCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(FCK_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Peripheral Module Clock D (PCLKD) Select
    #[inline(always)]
    pub fn pckd(&self) -> PCKD_R {
        PCKD_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Peripheral Module Clock C (PCLKC) Select
    #[inline(always)]
    pub fn pckc(&self) -> PCKC_R {
        PCKC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Peripheral Module Clock B (PCLKB) Select
    #[inline(always)]
    pub fn pckb(&self) -> PCKB_R {
        PCKB_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Peripheral Module Clock A (PCLKA) Select
    #[inline(always)]
    pub fn pcka(&self) -> PCKA_R {
        PCKA_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - External Bus Clock (BCLK) Select
    #[inline(always)]
    pub fn bck(&self) -> BCK_R {
        BCK_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - System Clock (ICLK) Select
    #[inline(always)]
    pub fn ick(&self) -> ICK_R {
        ICK_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Flash IF Clock (FCLK) Select
    #[inline(always)]
    pub fn fck(&self) -> FCK_R {
        FCK_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Peripheral Module Clock D (PCLKD) Select
    #[inline(always)]
    pub fn pckd(&mut self) -> PCKD_W<SCKDIVCR_SPEC> {
        PCKD_W::new(self, 0)
    }
    ///Bits 4:6 - Peripheral Module Clock C (PCLKC) Select
    #[inline(always)]
    pub fn pckc(&mut self) -> PCKC_W<SCKDIVCR_SPEC> {
        PCKC_W::new(self, 4)
    }
    ///Bits 8:10 - Peripheral Module Clock B (PCLKB) Select
    #[inline(always)]
    pub fn pckb(&mut self) -> PCKB_W<SCKDIVCR_SPEC> {
        PCKB_W::new(self, 8)
    }
    ///Bits 12:14 - Peripheral Module Clock A (PCLKA) Select
    #[inline(always)]
    pub fn pcka(&mut self) -> PCKA_W<SCKDIVCR_SPEC> {
        PCKA_W::new(self, 12)
    }
    ///Bits 16:18 - External Bus Clock (BCLK) Select
    #[inline(always)]
    pub fn bck(&mut self) -> BCK_W<SCKDIVCR_SPEC> {
        BCK_W::new(self, 16)
    }
    ///Bits 24:26 - System Clock (ICLK) Select
    #[inline(always)]
    pub fn ick(&mut self) -> ICK_W<SCKDIVCR_SPEC> {
        ICK_W::new(self, 24)
    }
    ///Bits 28:30 - Flash IF Clock (FCLK) Select
    #[inline(always)]
    pub fn fck(&mut self) -> FCK_W<SCKDIVCR_SPEC> {
        FCK_W::new(self, 28)
    }
}
/**System Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCKDIVCR_SPEC;
impl crate::RegisterSpec for SCKDIVCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sckdivcr::R`](R) reader structure
impl crate::Readable for SCKDIVCR_SPEC {}
///`write(|w| ..)` method takes [`sckdivcr::W`](W) writer structure
impl crate::Writable for SCKDIVCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCKDIVCR to value 0x2202_2222
impl crate::Resettable for SCKDIVCR_SPEC {
    const RESET_VALUE: u32 = 0x2202_2222;
}
