///Register `SRCCTRL` reader
pub type R = crate::R<SRCCTRL_SPEC>;
///Register `SRCCTRL` writer
pub type W = crate::W<SRCCTRL_SPEC>;
/**Output Sampling Rate

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OFS_A {
    ///0: 44.1 kHz
    _000 = 0,
    ///1: 48.0 kHz
    _001 = 1,
    ///2: 32.0 kHz
    _010 = 2,
    ///3: Setting prohibited
    _011 = 3,
    ///4: 8.0 kHz ( Valid only when IFS\[3:0\] =1001b )
    _100 = 4,
    ///5: 16.0 kHz ( Valid only when IFS\[3:0\] =1001b )
    _101 = 5,
    ///6: Settings other than above are prohibited.
    OTHERS = 6,
}
impl From<OFS_A> for u8 {
    #[inline(always)]
    fn from(variant: OFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OFS_A {
    type Ux = u8;
}
impl crate::IsEnum for OFS_A {}
///Field `OFS` reader - Output Sampling Rate
pub type OFS_R = crate::FieldReader<OFS_A>;
impl OFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFS_A {
        match self.bits {
            0 => OFS_A::_000,
            1 => OFS_A::_001,
            2 => OFS_A::_010,
            3 => OFS_A::_011,
            4 => OFS_A::_100,
            5 => OFS_A::_101,
            _ => OFS_A::OTHERS,
        }
    }
    ///44.1 kHz
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == OFS_A::_000
    }
    ///48.0 kHz
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == OFS_A::_001
    }
    ///32.0 kHz
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == OFS_A::_010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == OFS_A::_011
    }
    ///8.0 kHz ( Valid only when IFS\[3:0\] =1001b )
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == OFS_A::_100
    }
    ///16.0 kHz ( Valid only when IFS\[3:0\] =1001b )
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == OFS_A::_101
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), OFS_A::OTHERS)
    }
}
///Field `OFS` writer - Output Sampling Rate
pub type OFS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OFS_A, crate::Safe>;
impl<'a, REG> OFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///44.1 kHz
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::_000)
    }
    ///48.0 kHz
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::_001)
    }
    ///32.0 kHz
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::_010)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::_011)
    }
    ///8.0 kHz ( Valid only when IFS\[3:0\] =1001b )
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::_100)
    }
    ///16.0 kHz ( Valid only when IFS\[3:0\] =1001b )
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::_101)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(OFS_A::OTHERS)
    }
}
/**Input Sampling Rate

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFS_A {
    ///0: 8.0 kHz
    _0000 = 0,
    ///1: 11.025 kHz
    _0001 = 1,
    ///2: 12.0 kHz
    _0010 = 2,
    ///3: Setting prohibited
    _0011 = 3,
    ///4: 16.0 kHz
    _0100 = 4,
    ///5: 22.05 kHz
    _0101 = 5,
    ///6: 24.0 kHz
    _0110 = 6,
    ///7: Setting prohibited
    _0111 = 7,
    ///8: 32.0 kHz
    _1000 = 8,
    ///9: 44.1 kHz
    _1001 = 9,
    ///10: 48.0 kHz
    _1010 = 10,
    ///11: Settings prohibited.
    OTHERS = 11,
}
impl From<IFS_A> for u8 {
    #[inline(always)]
    fn from(variant: IFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFS_A {
    type Ux = u8;
}
impl crate::IsEnum for IFS_A {}
///Field `IFS` reader - Input Sampling Rate
pub type IFS_R = crate::FieldReader<IFS_A>;
impl IFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IFS_A {
        match self.bits {
            0 => IFS_A::_0000,
            1 => IFS_A::_0001,
            2 => IFS_A::_0010,
            3 => IFS_A::_0011,
            4 => IFS_A::_0100,
            5 => IFS_A::_0101,
            6 => IFS_A::_0110,
            7 => IFS_A::_0111,
            8 => IFS_A::_1000,
            9 => IFS_A::_1001,
            10 => IFS_A::_1010,
            _ => IFS_A::OTHERS,
        }
    }
    ///8.0 kHz
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == IFS_A::_0000
    }
    ///11.025 kHz
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == IFS_A::_0001
    }
    ///12.0 kHz
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == IFS_A::_0010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == IFS_A::_0011
    }
    ///16.0 kHz
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == IFS_A::_0100
    }
    ///22.05 kHz
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == IFS_A::_0101
    }
    ///24.0 kHz
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == IFS_A::_0110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == IFS_A::_0111
    }
    ///32.0 kHz
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == IFS_A::_1000
    }
    ///44.1 kHz
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == IFS_A::_1001
    }
    ///48.0 kHz
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == IFS_A::_1010
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), IFS_A::OTHERS)
    }
}
///Field `IFS` writer - Input Sampling Rate
pub type IFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IFS_A, crate::Safe>;
impl<'a, REG> IFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8.0 kHz
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0000)
    }
    ///11.025 kHz
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0001)
    }
    ///12.0 kHz
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0010)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0011)
    }
    ///16.0 kHz
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0100)
    }
    ///22.05 kHz
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0101)
    }
    ///24.0 kHz
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_0111)
    }
    ///32.0 kHz
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_1000)
    }
    ///44.1 kHz
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_1001)
    }
    ///48.0 kHz
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::_1010)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(IFS_A::OTHERS)
    }
}
/**Internal Work Memory Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CL_A {
    ///0: no effect
    _0 = 0,
    ///1: Clears the input FIFO, output FIFO, input buffer memory, intermediate memory and accumulator.
    _1 = 1,
}
impl From<CL_A> for bool {
    #[inline(always)]
    fn from(variant: CL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CL` writer - Internal Work Memory Clear
pub type CL_W<'a, REG> = crate::BitWriter<'a, REG, CL_A>;
impl<'a, REG> CL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CL_A::_0)
    }
    ///Clears the input FIFO, output FIFO, input buffer memory, intermediate memory and accumulator.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CL_A::_1)
    }
}
/**Internal Work Memory Flush

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FL_A {
    ///0: no effect
    _0 = 0,
    ///1: starts converting the sampling rate of all the data in the input FIFO, input buffer memory, and intermediate memory(i.e., flush processing).
    _1 = 1,
}
impl From<FL_A> for bool {
    #[inline(always)]
    fn from(variant: FL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FL` writer - Internal Work Memory Flush
pub type FL_W<'a, REG> = crate::BitWriter<'a, REG, FL_A>;
impl<'a, REG> FL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FL_A::_0)
    }
    ///starts converting the sampling rate of all the data in the input FIFO, input buffer memory, and intermediate memory(i.e., flush processing).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FL_A::_1)
    }
}
/**Output Data FIFO Overwrite Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVEN_A {
    ///0: Output data FIFO overwrite interrupt is disabled.
    _0 = 0,
    ///1: Output data FIFO overwrite interrupt is enabled.
    _1 = 1,
}
impl From<OVEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVEN` reader - Output Data FIFO Overwrite Interrupt Enable
pub type OVEN_R = crate::BitReader<OVEN_A>;
impl OVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVEN_A {
        match self.bits {
            false => OVEN_A::_0,
            true => OVEN_A::_1,
        }
    }
    ///Output data FIFO overwrite interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVEN_A::_0
    }
    ///Output data FIFO overwrite interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVEN_A::_1
    }
}
///Field `OVEN` writer - Output Data FIFO Overwrite Interrupt Enable
pub type OVEN_W<'a, REG> = crate::BitWriter<'a, REG, OVEN_A>;
impl<'a, REG> OVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output data FIFO overwrite interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVEN_A::_0)
    }
    ///Output data FIFO overwrite interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVEN_A::_1)
    }
}
/**Output Data FIFO Underflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDEN_A {
    ///0: Disables output data FIFO underflow interrupt requests.
    _0 = 0,
    ///1: Enables output data FIFO underflow interrupt requests.
    _1 = 1,
}
impl From<UDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UDEN` reader - Output Data FIFO Underflow Interrupt Enable
pub type UDEN_R = crate::BitReader<UDEN_A>;
impl UDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDEN_A {
        match self.bits {
            false => UDEN_A::_0,
            true => UDEN_A::_1,
        }
    }
    ///Disables output data FIFO underflow interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDEN_A::_0
    }
    ///Enables output data FIFO underflow interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDEN_A::_1
    }
}
///Field `UDEN` writer - Output Data FIFO Underflow Interrupt Enable
pub type UDEN_W<'a, REG> = crate::BitWriter<'a, REG, UDEN_A>;
impl<'a, REG> UDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables output data FIFO underflow interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UDEN_A::_0)
    }
    ///Enables output data FIFO underflow interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UDEN_A::_1)
    }
}
/**Module Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCEN_A {
    ///0: Disables this module operation.
    _0 = 0,
    ///1: Enables this module operation.
    _1 = 1,
}
impl From<SRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRCEN` reader - Module Enable
pub type SRCEN_R = crate::BitReader<SRCEN_A>;
impl SRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRCEN_A {
        match self.bits {
            false => SRCEN_A::_0,
            true => SRCEN_A::_1,
        }
    }
    ///Disables this module operation.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRCEN_A::_0
    }
    ///Enables this module operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRCEN_A::_1
    }
}
///Field `SRCEN` writer - Module Enable
pub type SRCEN_W<'a, REG> = crate::BitWriter<'a, REG, SRCEN_A>;
impl<'a, REG> SRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables this module operation.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SRCEN_A::_0)
    }
    ///Enables this module operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SRCEN_A::_1)
    }
}
/**Conversion End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEEN_A {
    ///0: Disables conversion end interrupt requests.
    _0 = 0,
    ///1: Enables conversion end interrupt requests.
    _1 = 1,
}
impl From<CEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CEEN` reader - Conversion End Interrupt Enable
pub type CEEN_R = crate::BitReader<CEEN_A>;
impl CEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEEN_A {
        match self.bits {
            false => CEEN_A::_0,
            true => CEEN_A::_1,
        }
    }
    ///Disables conversion end interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEEN_A::_0
    }
    ///Enables conversion end interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEEN_A::_1
    }
}
///Field `CEEN` writer - Conversion End Interrupt Enable
pub type CEEN_W<'a, REG> = crate::BitWriter<'a, REG, CEEN_A>;
impl<'a, REG> CEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables conversion end interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CEEN_A::_0)
    }
    ///Enables conversion end interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CEEN_A::_1)
    }
}
/**Filter Coefficient Table Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FICRAE_A {
    ///0: Reading/writing to filter coefficient table RAM is disabled.
    _0 = 0,
    ///1: Reading/writing to filter coefficient table RAM is enabled.
    _1 = 1,
}
impl From<FICRAE_A> for bool {
    #[inline(always)]
    fn from(variant: FICRAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FICRAE` reader - Filter Coefficient Table Access Enable
pub type FICRAE_R = crate::BitReader<FICRAE_A>;
impl FICRAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FICRAE_A {
        match self.bits {
            false => FICRAE_A::_0,
            true => FICRAE_A::_1,
        }
    }
    ///Reading/writing to filter coefficient table RAM is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FICRAE_A::_0
    }
    ///Reading/writing to filter coefficient table RAM is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FICRAE_A::_1
    }
}
///Field `FICRAE` writer - Filter Coefficient Table Access Enable
pub type FICRAE_W<'a, REG> = crate::BitWriter<'a, REG, FICRAE_A>;
impl<'a, REG> FICRAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reading/writing to filter coefficient table RAM is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FICRAE_A::_0)
    }
    ///Reading/writing to filter coefficient table RAM is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FICRAE_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Output Sampling Rate
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:7 - Input Sampling Rate
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 10 - Output Data FIFO Overwrite Interrupt Enable
    #[inline(always)]
    pub fn oven(&self) -> OVEN_R {
        OVEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Data FIFO Underflow Interrupt Enable
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Module Enable
    #[inline(always)]
    pub fn srcen(&self) -> SRCEN_R {
        SRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Conversion End Interrupt Enable
    #[inline(always)]
    pub fn ceen(&self) -> CEEN_R {
        CEEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Filter Coefficient Table Access Enable
    #[inline(always)]
    pub fn ficrae(&self) -> FICRAE_R {
        FICRAE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Output Sampling Rate
    #[inline(always)]
    pub fn ofs(&mut self) -> OFS_W<SRCCTRL_SPEC> {
        OFS_W::new(self, 0)
    }
    ///Bits 4:7 - Input Sampling Rate
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W<SRCCTRL_SPEC> {
        IFS_W::new(self, 4)
    }
    ///Bit 8 - Internal Work Memory Clear
    #[inline(always)]
    pub fn cl(&mut self) -> CL_W<SRCCTRL_SPEC> {
        CL_W::new(self, 8)
    }
    ///Bit 9 - Internal Work Memory Flush
    #[inline(always)]
    pub fn fl(&mut self) -> FL_W<SRCCTRL_SPEC> {
        FL_W::new(self, 9)
    }
    ///Bit 10 - Output Data FIFO Overwrite Interrupt Enable
    #[inline(always)]
    pub fn oven(&mut self) -> OVEN_W<SRCCTRL_SPEC> {
        OVEN_W::new(self, 10)
    }
    ///Bit 11 - Output Data FIFO Underflow Interrupt Enable
    #[inline(always)]
    pub fn uden(&mut self) -> UDEN_W<SRCCTRL_SPEC> {
        UDEN_W::new(self, 11)
    }
    ///Bit 12 - Module Enable
    #[inline(always)]
    pub fn srcen(&mut self) -> SRCEN_W<SRCCTRL_SPEC> {
        SRCEN_W::new(self, 12)
    }
    ///Bit 13 - Conversion End Interrupt Enable
    #[inline(always)]
    pub fn ceen(&mut self) -> CEEN_W<SRCCTRL_SPEC> {
        CEEN_W::new(self, 13)
    }
    ///Bit 15 - Filter Coefficient Table Access Enable
    #[inline(always)]
    pub fn ficrae(&mut self) -> FICRAE_W<SRCCTRL_SPEC> {
        FICRAE_W::new(self, 15)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`srcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCCTRL_SPEC;
impl crate::RegisterSpec for SRCCTRL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`srcctrl::R`](R) reader structure
impl crate::Readable for SRCCTRL_SPEC {}
///`write(|w| ..)` method takes [`srcctrl::W`](W) writer structure
impl crate::Writable for SRCCTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCCTRL to value 0
impl crate::Resettable for SRCCTRL_SPEC {}
