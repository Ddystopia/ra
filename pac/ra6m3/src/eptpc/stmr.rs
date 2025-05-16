///Register `STMR` reader
pub type R = crate::R<STMR_SPEC>;
///Register `STMR` writer
pub type W = crate::W<STMR_SPEC>;
/**Worst 10 Acquisition Time

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINT_A {
    ///0: The worst 10 values are not acquired.
    _0X00 = 0,
    ///1: Sync message reception: (WINT) time
    OTHERS = 1,
}
impl From<WINT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WINT_A {
    type Ux = u8;
}
impl crate::IsEnum for WINT_A {}
///Field `WINT` reader - Worst 10 Acquisition Time
pub type WINT_R = crate::FieldReader<WINT_A>;
impl WINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WINT_A {
        match self.bits {
            0 => WINT_A::_0X00,
            _ => WINT_A::OTHERS,
        }
    }
    ///The worst 10 values are not acquired.
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == WINT_A::_0X00
    }
    ///Sync message reception: (WINT) time
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), WINT_A::OTHERS)
    }
}
///Field `WINT` writer - Worst 10 Acquisition Time
pub type WINT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, WINT_A, crate::Safe>;
impl<'a, REG> WINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The worst 10 values are not acquired.
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(WINT_A::_0X00)
    }
    ///Sync message reception: (WINT) time
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(WINT_A::OTHERS)
    }
}
/**Time Synchronization Correction Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMOD_A {
    ///0: Mode 1
    _0 = 0,
    ///1: Mode 2
    _1 = 1,
}
impl From<CMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMOD` reader - Time Synchronization Correction Mode
pub type CMOD_R = crate::BitReader<CMOD_A>;
impl CMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMOD_A {
        match self.bits {
            false => CMOD_A::_0,
            true => CMOD_A::_1,
        }
    }
    ///Mode 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMOD_A::_0
    }
    ///Mode 2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMOD_A::_1
    }
}
///Field `CMOD` writer - Time Synchronization Correction Mode
pub type CMOD_W<'a, REG> = crate::BitWriter<'a, REG, CMOD_A>;
impl<'a, REG> CMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mode 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMOD_A::_0)
    }
    ///Mode 2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMOD_A::_1)
    }
}
/**Worst 10 Acquisition Control Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W10S_A {
    ///0: Measurement is started by hardware and the value acquired in the PW10VR or MW10R register is used as the limit for filtering.
    _0 = 0,
    ///1: Measurement is started by the GETW10R.GW10 bit. Also, the value set in the PLIMITR or MLIMITR register is used as the limit for filtering.
    _1 = 1,
}
impl From<W10S_A> for bool {
    #[inline(always)]
    fn from(variant: W10S_A) -> Self {
        variant as u8 != 0
    }
}
///Field `W10S` reader - Worst 10 Acquisition Control Select
pub type W10S_R = crate::BitReader<W10S_A>;
impl W10S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> W10S_A {
        match self.bits {
            false => W10S_A::_0,
            true => W10S_A::_1,
        }
    }
    ///Measurement is started by hardware and the value acquired in the PW10VR or MW10R register is used as the limit for filtering.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == W10S_A::_0
    }
    ///Measurement is started by the GETW10R.GW10 bit. Also, the value set in the PLIMITR or MLIMITR register is used as the limit for filtering.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == W10S_A::_1
    }
}
///Field `W10S` writer - Worst 10 Acquisition Control Select
pub type W10S_W<'a, REG> = crate::BitWriter<'a, REG, W10S_A>;
impl<'a, REG> W10S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Measurement is started by hardware and the value acquired in the PW10VR or MW10R register is used as the limit for filtering.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(W10S_A::_0)
    }
    ///Measurement is started by the GETW10R.GW10 bit. Also, the value set in the PLIMITR or MLIMITR register is used as the limit for filtering.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(W10S_A::_1)
    }
}
/**Synchronized State Detection Threshold Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYTH_A {
    ///0: None
    _0X0 = 0,
    ///1: (SYTH) time
    OTHERS = 1,
}
impl From<SYTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SYTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYTH_A {
    type Ux = u8;
}
impl crate::IsEnum for SYTH_A {}
///Field `SYTH` reader - Synchronized State Detection Threshold Setting
pub type SYTH_R = crate::FieldReader<SYTH_A>;
impl SYTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYTH_A {
        match self.bits {
            0 => SYTH_A::_0X0,
            _ => SYTH_A::OTHERS,
        }
    }
    ///None
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == SYTH_A::_0X0
    }
    ///(SYTH) time
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SYTH_A::OTHERS)
    }
}
///Field `SYTH` writer - Synchronized State Detection Threshold Setting
pub type SYTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SYTH_A, crate::Safe>;
impl<'a, REG> SYTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///None
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYTH_A::_0X0)
    }
    ///(SYTH) time
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SYTH_A::OTHERS)
    }
}
/**Synchronization Loss Detection Threshold Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVTH_A {
    ///0: None
    _0X0 = 0,
    ///1: (DVTH) time
    OTHERS = 1,
}
impl From<DVTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DVTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DVTH_A {
    type Ux = u8;
}
impl crate::IsEnum for DVTH_A {}
///Field `DVTH` reader - Synchronization Loss Detection Threshold Setting
pub type DVTH_R = crate::FieldReader<DVTH_A>;
impl DVTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVTH_A {
        match self.bits {
            0 => DVTH_A::_0X0,
            _ => DVTH_A::OTHERS,
        }
    }
    ///None
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == DVTH_A::_0X0
    }
    ///(DVTH) time
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DVTH_A::OTHERS)
    }
}
///Field `DVTH` writer - Synchronization Loss Detection Threshold Setting
pub type DVTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DVTH_A, crate::Safe>;
impl<'a, REG> DVTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///None
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DVTH_A::_0X0)
    }
    ///(DVTH) time
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DVTH_A::OTHERS)
    }
}
/**Alarm Detection Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEN0_A {
    ///0: The STSR.SYNC or SYNCOUT flag is not set to 1 on detection of synchronization or loss of synchronization.
    _0 = 0,
    ///1: The STSR.SYNC or SYNCOUT flag is set to 1 on detection of synchronization or loss of synchronization.
    _1 = 1,
}
impl From<ALEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ALEN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALEN0` reader - Alarm Detection Enable 0
pub type ALEN0_R = crate::BitReader<ALEN0_A>;
impl ALEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALEN0_A {
        match self.bits {
            false => ALEN0_A::_0,
            true => ALEN0_A::_1,
        }
    }
    ///The STSR.SYNC or SYNCOUT flag is not set to 1 on detection of synchronization or loss of synchronization.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALEN0_A::_0
    }
    ///The STSR.SYNC or SYNCOUT flag is set to 1 on detection of synchronization or loss of synchronization.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALEN0_A::_1
    }
}
///Field `ALEN0` writer - Alarm Detection Enable 0
pub type ALEN0_W<'a, REG> = crate::BitWriter<'a, REG, ALEN0_A>;
impl<'a, REG> ALEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The STSR.SYNC or SYNCOUT flag is not set to 1 on detection of synchronization or loss of synchronization.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ALEN0_A::_0)
    }
    ///The STSR.SYNC or SYNCOUT flag is set to 1 on detection of synchronization or loss of synchronization.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEN0_A::_1)
    }
}
/**Alarm Detection Enable 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALEN1_A {
    ///0: The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt.
    _0 = 0,
    ///1: The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt.
    _1 = 1,
}
impl From<ALEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ALEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALEN1` reader - Alarm Detection Enable 1
pub type ALEN1_R = crate::BitReader<ALEN1_A>;
impl ALEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALEN1_A {
        match self.bits {
            false => ALEN1_A::_0,
            true => ALEN1_A::_1,
        }
    }
    ///The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALEN1_A::_0
    }
    ///The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALEN1_A::_1
    }
}
///Field `ALEN1` writer - Alarm Detection Enable 1
pub type ALEN1_W<'a, REG> = crate::BitWriter<'a, REG, ALEN1_A>;
impl<'a, REG> ALEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ALEN1_A::_0)
    }
    ///The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEN1_A::_1)
    }
}
impl R {
    ///Bits 0:7 - Worst 10 Acquisition Time
    #[inline(always)]
    pub fn wint(&self) -> WINT_R {
        WINT_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 13 - Time Synchronization Correction Mode
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Worst 10 Acquisition Control Select
    #[inline(always)]
    pub fn w10s(&self) -> W10S_R {
        W10S_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Synchronized State Detection Threshold Setting
    #[inline(always)]
    pub fn syth(&self) -> SYTH_R {
        SYTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Synchronization Loss Detection Threshold Setting
    #[inline(always)]
    pub fn dvth(&self) -> DVTH_R {
        DVTH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 28 - Alarm Detection Enable 0
    #[inline(always)]
    pub fn alen0(&self) -> ALEN0_R {
        ALEN0_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Alarm Detection Enable 1
    #[inline(always)]
    pub fn alen1(&self) -> ALEN1_R {
        ALEN1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Worst 10 Acquisition Time
    #[inline(always)]
    pub fn wint(&mut self) -> WINT_W<STMR_SPEC> {
        WINT_W::new(self, 0)
    }
    ///Bit 13 - Time Synchronization Correction Mode
    #[inline(always)]
    pub fn cmod(&mut self) -> CMOD_W<STMR_SPEC> {
        CMOD_W::new(self, 13)
    }
    ///Bit 15 - Worst 10 Acquisition Control Select
    #[inline(always)]
    pub fn w10s(&mut self) -> W10S_W<STMR_SPEC> {
        W10S_W::new(self, 15)
    }
    ///Bits 16:19 - Synchronized State Detection Threshold Setting
    #[inline(always)]
    pub fn syth(&mut self) -> SYTH_W<STMR_SPEC> {
        SYTH_W::new(self, 16)
    }
    ///Bits 20:23 - Synchronization Loss Detection Threshold Setting
    #[inline(always)]
    pub fn dvth(&mut self) -> DVTH_W<STMR_SPEC> {
        DVTH_W::new(self, 20)
    }
    ///Bit 28 - Alarm Detection Enable 0
    #[inline(always)]
    pub fn alen0(&mut self) -> ALEN0_W<STMR_SPEC> {
        ALEN0_W::new(self, 28)
    }
    ///Bit 29 - Alarm Detection Enable 1
    #[inline(always)]
    pub fn alen1(&mut self) -> ALEN1_W<STMR_SPEC> {
        ALEN1_W::new(self, 29)
    }
}
/**STCA Operating Mode Register

You can [`read`](crate::Reg::read) this register and get [`stmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STMR_SPEC;
impl crate::RegisterSpec for STMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`stmr::R`](R) reader structure
impl crate::Readable for STMR_SPEC {}
///`write(|w| ..)` method takes [`stmr::W`](W) writer structure
impl crate::Writable for STMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STMR to value 0
impl crate::Resettable for STMR_SPEC {}
