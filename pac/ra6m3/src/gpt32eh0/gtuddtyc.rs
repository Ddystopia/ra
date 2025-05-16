///Register `GTUDDTYC` reader
pub type R = crate::R<GTUDDTYC_SPEC>;
///Register `GTUDDTYC` writer
pub type W = crate::W<GTUDDTYC_SPEC>;
/**Count Direction Setting

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UD_A {
    ///0: Count down on GTCNT
    _0 = 0,
    ///1: Counts up on GTCNT
    _1 = 1,
}
impl From<UD_A> for bool {
    #[inline(always)]
    fn from(variant: UD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UD` reader - Count Direction Setting
pub type UD_R = crate::BitReader<UD_A>;
impl UD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UD_A {
        match self.bits {
            false => UD_A::_0,
            true => UD_A::_1,
        }
    }
    ///Count down on GTCNT
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UD_A::_0
    }
    ///Counts up on GTCNT
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UD_A::_1
    }
}
///Field `UD` writer - Count Direction Setting
pub type UD_W<'a, REG> = crate::BitWriter<'a, REG, UD_A>;
impl<'a, REG> UD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count down on GTCNT
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UD_A::_0)
    }
    ///Counts up on GTCNT
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UD_A::_1)
    }
}
/**Forcible Count Direction Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDF_A {
    ///0: Do not force setting
    _0 = 0,
    ///1: Force setting
    _1 = 1,
}
impl From<UDF_A> for bool {
    #[inline(always)]
    fn from(variant: UDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UDF` reader - Forcible Count Direction Setting
pub type UDF_R = crate::BitReader<UDF_A>;
impl UDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDF_A {
        match self.bits {
            false => UDF_A::_0,
            true => UDF_A::_1,
        }
    }
    ///Do not force setting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDF_A::_0
    }
    ///Force setting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDF_A::_1
    }
}
///Field `UDF` writer - Forcible Count Direction Setting
pub type UDF_W<'a, REG> = crate::BitWriter<'a, REG, UDF_A>;
impl<'a, REG> UDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not force setting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UDF_A::_0)
    }
    ///Force setting
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UDF_A::_1)
    }
}
/**GTIOCA Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OADTY_A {
    ///0: GTIOCA pin duty is depend on compare match
    _00 = 0,
    ///1: GTIOCA pin duty is depend on compare match
    _01 = 1,
    ///2: GTIOCA pin duty 0 percent
    _10 = 2,
    ///3: GTIOCA pin duty 100 percent
    _11 = 3,
}
impl From<OADTY_A> for u8 {
    #[inline(always)]
    fn from(variant: OADTY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OADTY_A {
    type Ux = u8;
}
impl crate::IsEnum for OADTY_A {}
///Field `OADTY` reader - GTIOCA Output Duty Setting
pub type OADTY_R = crate::FieldReader<OADTY_A>;
impl OADTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OADTY_A {
        match self.bits {
            0 => OADTY_A::_00,
            1 => OADTY_A::_01,
            2 => OADTY_A::_10,
            3 => OADTY_A::_11,
            _ => unreachable!(),
        }
    }
    ///GTIOCA pin duty is depend on compare match
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OADTY_A::_00
    }
    ///GTIOCA pin duty is depend on compare match
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OADTY_A::_01
    }
    ///GTIOCA pin duty 0 percent
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OADTY_A::_10
    }
    ///GTIOCA pin duty 100 percent
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OADTY_A::_11
    }
}
///Field `OADTY` writer - GTIOCA Output Duty Setting
pub type OADTY_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OADTY_A, crate::Safe>;
impl<'a, REG> OADTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GTIOCA pin duty is depend on compare match
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OADTY_A::_00)
    }
    ///GTIOCA pin duty is depend on compare match
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OADTY_A::_01)
    }
    ///GTIOCA pin duty 0 percent
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OADTY_A::_10)
    }
    ///GTIOCA pin duty 100 percent
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OADTY_A::_11)
    }
}
/**Forcible GTIOCA Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADTYF_A {
    ///0: Do not force setting
    _0 = 0,
    ///1: Force setting
    _1 = 1,
}
impl From<OADTYF_A> for bool {
    #[inline(always)]
    fn from(variant: OADTYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OADTYF` reader - Forcible GTIOCA Output Duty Setting
pub type OADTYF_R = crate::BitReader<OADTYF_A>;
impl OADTYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OADTYF_A {
        match self.bits {
            false => OADTYF_A::_0,
            true => OADTYF_A::_1,
        }
    }
    ///Do not force setting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADTYF_A::_0
    }
    ///Force setting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADTYF_A::_1
    }
}
///Field `OADTYF` writer - Forcible GTIOCA Output Duty Setting
pub type OADTYF_W<'a, REG> = crate::BitWriter<'a, REG, OADTYF_A>;
impl<'a, REG> OADTYF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not force setting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OADTYF_A::_0)
    }
    ///Force setting
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OADTYF_A::_1)
    }
}
/**GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OADTYR_A {
    ///0: Apply output value set in 0 percent/100 percent duty to GTIOA\[3:2\] function after releasing 0 percent/100 percent duty setting.
    _0 = 0,
    ///1: Apply masked compare match output value to GTIOA\[3:2\] function after releasing 0 percent/100 percent duty setting.
    _1 = 1,
}
impl From<OADTYR_A> for bool {
    #[inline(always)]
    fn from(variant: OADTYR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OADTYR` reader - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
pub type OADTYR_R = crate::BitReader<OADTYR_A>;
impl OADTYR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OADTYR_A {
        match self.bits {
            false => OADTYR_A::_0,
            true => OADTYR_A::_1,
        }
    }
    ///Apply output value set in 0 percent/100 percent duty to GTIOA\[3:2\] function after releasing 0 percent/100 percent duty setting.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OADTYR_A::_0
    }
    ///Apply masked compare match output value to GTIOA\[3:2\] function after releasing 0 percent/100 percent duty setting.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OADTYR_A::_1
    }
}
///Field `OADTYR` writer - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
pub type OADTYR_W<'a, REG> = crate::BitWriter<'a, REG, OADTYR_A>;
impl<'a, REG> OADTYR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Apply output value set in 0 percent/100 percent duty to GTIOA\[3:2\] function after releasing 0 percent/100 percent duty setting.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OADTYR_A::_0)
    }
    ///Apply masked compare match output value to GTIOA\[3:2\] function after releasing 0 percent/100 percent duty setting.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OADTYR_A::_1)
    }
}
/**GTIOCB Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OBDTY_A {
    ///0: GTIOCB pin duty is depend on compare match
    _00 = 0,
    ///1: GTIOCB pin duty is depend on compare match
    _01 = 1,
    ///2: GTIOCB pin duty 0percent
    _10 = 2,
    ///3: GTIOCB pin duty 100percent
    _11 = 3,
}
impl From<OBDTY_A> for u8 {
    #[inline(always)]
    fn from(variant: OBDTY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OBDTY_A {
    type Ux = u8;
}
impl crate::IsEnum for OBDTY_A {}
///Field `OBDTY` reader - GTIOCB Output Duty Setting
pub type OBDTY_R = crate::FieldReader<OBDTY_A>;
impl OBDTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBDTY_A {
        match self.bits {
            0 => OBDTY_A::_00,
            1 => OBDTY_A::_01,
            2 => OBDTY_A::_10,
            3 => OBDTY_A::_11,
            _ => unreachable!(),
        }
    }
    ///GTIOCB pin duty is depend on compare match
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OBDTY_A::_00
    }
    ///GTIOCB pin duty is depend on compare match
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OBDTY_A::_01
    }
    ///GTIOCB pin duty 0percent
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OBDTY_A::_10
    }
    ///GTIOCB pin duty 100percent
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OBDTY_A::_11
    }
}
///Field `OBDTY` writer - GTIOCB Output Duty Setting
pub type OBDTY_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OBDTY_A, crate::Safe>;
impl<'a, REG> OBDTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///GTIOCB pin duty is depend on compare match
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTY_A::_00)
    }
    ///GTIOCB pin duty is depend on compare match
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTY_A::_01)
    }
    ///GTIOCB pin duty 0percent
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTY_A::_10)
    }
    ///GTIOCB pin duty 100percent
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTY_A::_11)
    }
}
/**Forcible GTIOCB Output Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDTYF_A {
    ///0: Do not force setting
    _0 = 0,
    ///1: Force setting
    _1 = 1,
}
impl From<OBDTYF_A> for bool {
    #[inline(always)]
    fn from(variant: OBDTYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OBDTYF` reader - Forcible GTIOCB Output Duty Setting
pub type OBDTYF_R = crate::BitReader<OBDTYF_A>;
impl OBDTYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBDTYF_A {
        match self.bits {
            false => OBDTYF_A::_0,
            true => OBDTYF_A::_1,
        }
    }
    ///Do not force setting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDTYF_A::_0
    }
    ///Force setting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDTYF_A::_1
    }
}
///Field `OBDTYF` writer - Forcible GTIOCB Output Duty Setting
pub type OBDTYF_W<'a, REG> = crate::BitWriter<'a, REG, OBDTYF_A>;
impl<'a, REG> OBDTYF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not force setting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTYF_A::_0)
    }
    ///Force setting
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTYF_A::_1)
    }
}
/**GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBDTYR_A {
    ///0: Apply output value set in 0 percent/100 percent duty to GTIOB\[3:2\] function after releasing 0percent/100percent duty setting.
    _0 = 0,
    ///1: Apply masked compare match output value to GTIOB\[3:2\] function after releasing 0percent/100percent duty setting.
    _1 = 1,
}
impl From<OBDTYR_A> for bool {
    #[inline(always)]
    fn from(variant: OBDTYR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OBDTYR` reader - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
pub type OBDTYR_R = crate::BitReader<OBDTYR_A>;
impl OBDTYR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBDTYR_A {
        match self.bits {
            false => OBDTYR_A::_0,
            true => OBDTYR_A::_1,
        }
    }
    ///Apply output value set in 0 percent/100 percent duty to GTIOB\[3:2\] function after releasing 0percent/100percent duty setting.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OBDTYR_A::_0
    }
    ///Apply masked compare match output value to GTIOB\[3:2\] function after releasing 0percent/100percent duty setting.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OBDTYR_A::_1
    }
}
///Field `OBDTYR` writer - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
pub type OBDTYR_W<'a, REG> = crate::BitWriter<'a, REG, OBDTYR_A>;
impl<'a, REG> OBDTYR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Apply output value set in 0 percent/100 percent duty to GTIOB\[3:2\] function after releasing 0percent/100percent duty setting.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTYR_A::_0)
    }
    ///Apply masked compare match output value to GTIOB\[3:2\] function after releasing 0percent/100percent duty setting.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OBDTYR_A::_1)
    }
}
impl R {
    ///Bit 0 - Count Direction Setting
    #[inline(always)]
    pub fn ud(&self) -> UD_R {
        UD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Forcible Count Direction Setting
    #[inline(always)]
    pub fn udf(&self) -> UDF_R {
        UDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:17 - GTIOCA Output Duty Setting
    #[inline(always)]
    pub fn oadty(&self) -> OADTY_R {
        OADTY_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Forcible GTIOCA Output Duty Setting
    #[inline(always)]
    pub fn oadtyf(&self) -> OADTYF_R {
        OADTYF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
    #[inline(always)]
    pub fn oadtyr(&self) -> OADTYR_R {
        OADTYR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:25 - GTIOCB Output Duty Setting
    #[inline(always)]
    pub fn obdty(&self) -> OBDTY_R {
        OBDTY_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Forcible GTIOCB Output Duty Setting
    #[inline(always)]
    pub fn obdtyf(&self) -> OBDTYF_R {
        OBDTYF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
    #[inline(always)]
    pub fn obdtyr(&self) -> OBDTYR_R {
        OBDTYR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Count Direction Setting
    #[inline(always)]
    pub fn ud(&mut self) -> UD_W<GTUDDTYC_SPEC> {
        UD_W::new(self, 0)
    }
    ///Bit 1 - Forcible Count Direction Setting
    #[inline(always)]
    pub fn udf(&mut self) -> UDF_W<GTUDDTYC_SPEC> {
        UDF_W::new(self, 1)
    }
    ///Bits 16:17 - GTIOCA Output Duty Setting
    #[inline(always)]
    pub fn oadty(&mut self) -> OADTY_W<GTUDDTYC_SPEC> {
        OADTY_W::new(self, 16)
    }
    ///Bit 18 - Forcible GTIOCA Output Duty Setting
    #[inline(always)]
    pub fn oadtyf(&mut self) -> OADTYF_W<GTUDDTYC_SPEC> {
        OADTYF_W::new(self, 18)
    }
    ///Bit 19 - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
    #[inline(always)]
    pub fn oadtyr(&mut self) -> OADTYR_W<GTUDDTYC_SPEC> {
        OADTYR_W::new(self, 19)
    }
    ///Bits 24:25 - GTIOCB Output Duty Setting
    #[inline(always)]
    pub fn obdty(&mut self) -> OBDTY_W<GTUDDTYC_SPEC> {
        OBDTY_W::new(self, 24)
    }
    ///Bit 26 - Forcible GTIOCB Output Duty Setting
    #[inline(always)]
    pub fn obdtyf(&mut self) -> OBDTYF_W<GTUDDTYC_SPEC> {
        OBDTYF_W::new(self, 26)
    }
    ///Bit 27 - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting
    #[inline(always)]
    pub fn obdtyr(&mut self) -> OBDTYR_W<GTUDDTYC_SPEC> {
        OBDTYR_W::new(self, 27)
    }
}
/**General PWM Timer Count Direction and Duty Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtuddtyc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtuddtyc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTUDDTYC_SPEC;
impl crate::RegisterSpec for GTUDDTYC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtuddtyc::R`](R) reader structure
impl crate::Readable for GTUDDTYC_SPEC {}
///`write(|w| ..)` method takes [`gtuddtyc::W`](W) writer structure
impl crate::Writable for GTUDDTYC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTUDDTYC to value 0x01
impl crate::Resettable for GTUDDTYC_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
