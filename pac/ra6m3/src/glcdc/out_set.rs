///Register `OUT_SET` reader
pub type R = crate::R<OUT_SET_SPEC>;
///Register `OUT_SET` writer
pub type W = crate::W<OUT_SET_SPEC>;
/**Data delay in serial RGB format (based on OUTCLK)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PHASE_A {
    ///3: 3 cycles
    _11 = 3,
    ///2: 2 cycles
    _10 = 2,
    ///1: 1 cycle
    _01 = 1,
    ///0: 0 cycle
    _00 = 0,
}
impl From<PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: PHASE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PHASE_A {
    type Ux = u8;
}
impl crate::IsEnum for PHASE_A {}
///Field `PHASE` reader - Data delay in serial RGB format (based on OUTCLK)
pub type PHASE_R = crate::FieldReader<PHASE_A>;
impl PHASE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PHASE_A {
        match self.bits {
            3 => PHASE_A::_11,
            2 => PHASE_A::_10,
            1 => PHASE_A::_01,
            0 => PHASE_A::_00,
            _ => unreachable!(),
        }
    }
    ///3 cycles
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PHASE_A::_11
    }
    ///2 cycles
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PHASE_A::_10
    }
    ///1 cycle
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PHASE_A::_01
    }
    ///0 cycle
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PHASE_A::_00
    }
}
///Field `PHASE` writer - Data delay in serial RGB format (based on OUTCLK)
pub type PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PHASE_A, crate::Safe>;
impl<'a, REG> PHASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///3 cycles
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PHASE_A::_11)
    }
    ///2 cycles
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PHASE_A::_10)
    }
    ///1 cycle
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PHASE_A::_01)
    }
    ///0 cycle
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PHASE_A::_00)
    }
}
/**Invalid data position control in serial RGB format

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSEL_A {
    ///1: Invalid data is output prior to valid (RGB) data.
    _1 = 1,
    ///0: Invalid data is output following valid (RGB) data.
    _0 = 0,
}
impl From<DIRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRSEL` reader - Invalid data position control in serial RGB format
pub type DIRSEL_R = crate::BitReader<DIRSEL_A>;
impl DIRSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRSEL_A {
        match self.bits {
            true => DIRSEL_A::_1,
            false => DIRSEL_A::_0,
        }
    }
    ///Invalid data is output prior to valid (RGB) data.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRSEL_A::_1
    }
    ///Invalid data is output following valid (RGB) data.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRSEL_A::_0
    }
}
///Field `DIRSEL` writer - Invalid data position control in serial RGB format
pub type DIRSEL_W<'a, REG> = crate::BitWriter<'a, REG, DIRSEL_A>;
impl<'a, REG> DIRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid data is output prior to valid (RGB) data.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRSEL_A::_1)
    }
    ///Invalid data is output following valid (RGB) data.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRSEL_A::_0)
    }
}
/**Clock frequency division control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQSEL_A {
    ///3: Setting prohibited
    _11 = 3,
    ///2: Quarter frequency (serial RGB)
    _10 = 2,
    ///1: Setting prohibited
    _01 = 1,
    ///0: No frequency division, parallel RGB
    _00 = 0,
}
impl From<FRQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRQSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for FRQSEL_A {}
///Field `FRQSEL` reader - Clock frequency division control
pub type FRQSEL_R = crate::FieldReader<FRQSEL_A>;
impl FRQSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRQSEL_A {
        match self.bits {
            3 => FRQSEL_A::_11,
            2 => FRQSEL_A::_10,
            1 => FRQSEL_A::_01,
            0 => FRQSEL_A::_00,
            _ => unreachable!(),
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FRQSEL_A::_11
    }
    ///Quarter frequency (serial RGB)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FRQSEL_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FRQSEL_A::_01
    }
    ///No frequency division, parallel RGB
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FRQSEL_A::_00
    }
}
///Field `FRQSEL` writer - Clock frequency division control
pub type FRQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FRQSEL_A, crate::Safe>;
impl<'a, REG> FRQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_11)
    }
    ///Quarter frequency (serial RGB)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_01)
    }
    ///No frequency division, parallel RGB
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_00)
    }
}
/**Output format select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    ///3: Serial RGB; select RGB888 as dither output format.
    _11 = 3,
    ///2: RGB565; select RGB565 as dither output format.
    _10 = 2,
    ///1: RGB666; select RGB666 as dither output format.
    _01 = 1,
    ///0: RGB888; select RGB888 as dither output format.
    _00 = 0,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORMAT_A {
    type Ux = u8;
}
impl crate::IsEnum for FORMAT_A {}
///Field `FORMAT` reader - Output format select
pub type FORMAT_R = crate::FieldReader<FORMAT_A>;
impl FORMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORMAT_A {
        match self.bits {
            3 => FORMAT_A::_11,
            2 => FORMAT_A::_10,
            1 => FORMAT_A::_01,
            0 => FORMAT_A::_00,
            _ => unreachable!(),
        }
    }
    ///Serial RGB; select RGB888 as dither output format.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FORMAT_A::_11
    }
    ///RGB565; select RGB565 as dither output format.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FORMAT_A::_10
    }
    ///RGB666; select RGB666 as dither output format.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FORMAT_A::_01
    }
    ///RGB888; select RGB888 as dither output format.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FORMAT_A::_00
    }
}
///Field `FORMAT` writer - Output format select
pub type FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FORMAT_A, crate::Safe>;
impl<'a, REG> FORMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Serial RGB; select RGB888 as dither output format.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_11)
    }
    ///RGB565; select RGB565 as dither output format.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_10)
    }
    ///RGB666; select RGB666 as dither output format.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_01)
    }
    ///RGB888; select RGB888 as dither output format.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::_00)
    }
}
/**Pixel order control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAPON_A {
    ///1: In the order of BGR
    _1 = 1,
    ///0: In the order of RGB
    _0 = 0,
}
impl From<SWAPON_A> for bool {
    #[inline(always)]
    fn from(variant: SWAPON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWAPON` reader - Pixel order control
pub type SWAPON_R = crate::BitReader<SWAPON_A>;
impl SWAPON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWAPON_A {
        match self.bits {
            true => SWAPON_A::_1,
            false => SWAPON_A::_0,
        }
    }
    ///In the order of BGR
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAPON_A::_1
    }
    ///In the order of RGB
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAPON_A::_0
    }
}
///Field `SWAPON` writer - Pixel order control
pub type SWAPON_W<'a, REG> = crate::BitWriter<'a, REG, SWAPON_A>;
impl<'a, REG> SWAPON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In the order of BGR
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWAPON_A::_1)
    }
    ///In the order of RGB
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWAPON_A::_0)
    }
}
/**Bit endian change control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIANON_A {
    ///1: Ascending order (big endian)
    _1 = 1,
    ///0: Descending order (little endian)
    _0 = 0,
}
impl From<ENDIANON_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENDIANON` reader - Bit endian change control
pub type ENDIANON_R = crate::BitReader<ENDIANON_A>;
impl ENDIANON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENDIANON_A {
        match self.bits {
            true => ENDIANON_A::_1,
            false => ENDIANON_A::_0,
        }
    }
    ///Ascending order (big endian)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDIANON_A::_1
    }
    ///Descending order (little endian)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDIANON_A::_0
    }
}
///Field `ENDIANON` writer - Bit endian change control
pub type ENDIANON_W<'a, REG> = crate::BitWriter<'a, REG, ENDIANON_A>;
impl<'a, REG> ENDIANON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Ascending order (big endian)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIANON_A::_1)
    }
    ///Descending order (little endian)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIANON_A::_0)
    }
}
impl R {
    ///Bits 0:1 - Data delay in serial RGB format (based on OUTCLK)
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Invalid data position control in serial RGB format
    #[inline(always)]
    pub fn dirsel(&self) -> DIRSEL_R {
        DIRSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:9 - Clock frequency division control
    #[inline(always)]
    pub fn frqsel(&self) -> FRQSEL_R {
        FRQSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Output format select
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 24 - Pixel order control
    #[inline(always)]
    pub fn swapon(&self) -> SWAPON_R {
        SWAPON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Bit endian change control
    #[inline(always)]
    pub fn endianon(&self) -> ENDIANON_R {
        ENDIANON_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Data delay in serial RGB format (based on OUTCLK)
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W<OUT_SET_SPEC> {
        PHASE_W::new(self, 0)
    }
    ///Bit 4 - Invalid data position control in serial RGB format
    #[inline(always)]
    pub fn dirsel(&mut self) -> DIRSEL_W<OUT_SET_SPEC> {
        DIRSEL_W::new(self, 4)
    }
    ///Bits 8:9 - Clock frequency division control
    #[inline(always)]
    pub fn frqsel(&mut self) -> FRQSEL_W<OUT_SET_SPEC> {
        FRQSEL_W::new(self, 8)
    }
    ///Bits 12:13 - Output format select
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W<OUT_SET_SPEC> {
        FORMAT_W::new(self, 12)
    }
    ///Bit 24 - Pixel order control
    #[inline(always)]
    pub fn swapon(&mut self) -> SWAPON_W<OUT_SET_SPEC> {
        SWAPON_W::new(self, 24)
    }
    ///Bit 28 - Bit endian change control
    #[inline(always)]
    pub fn endianon(&mut self) -> ENDIANON_W<OUT_SET_SPEC> {
        ENDIANON_W::new(self, 28)
    }
}
/**Output Control Block Output Interface Register

You can [`read`](crate::Reg::read) this register and get [`out_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_SET_SPEC;
impl crate::RegisterSpec for OUT_SET_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_set::R`](R) reader structure
impl crate::Readable for OUT_SET_SPEC {}
///`write(|w| ..)` method takes [`out_set::W`](W) writer structure
impl crate::Writable for OUT_SET_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUT_SET to value 0
impl crate::Resettable for OUT_SET_SPEC {}
