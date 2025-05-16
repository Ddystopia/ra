///Register `LVD%sCR0` reader
pub type R = crate::R<LVDCR0_SPEC>;
///Register `LVD%sCR0` writer
pub type W = crate::W<LVDCR0_SPEC>;
/**Voltage Monitor Interrupt/Reset Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Voltage Monitor Interrupt/Reset Enable
pub type RIE_R = crate::BitReader<RIE_A>;
impl RIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
///Field `RIE` writer - Voltage Monitor Interrupt/Reset Enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG, RIE_A>;
impl<'a, REG> RIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
/**Voltage Monitor Digital Filter Disable Mode Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFDIS_A {
    ///0: Enable digital filter
    _0 = 0,
    ///1: Disable digital filter
    _1 = 1,
}
impl From<DFDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DFDIS` reader - Voltage Monitor Digital Filter Disable Mode Select
pub type DFDIS_R = crate::BitReader<DFDIS_A>;
impl DFDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFDIS_A {
        match self.bits {
            false => DFDIS_A::_0,
            true => DFDIS_A::_1,
        }
    }
    ///Enable digital filter
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFDIS_A::_0
    }
    ///Disable digital filter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFDIS_A::_1
    }
}
///Field `DFDIS` writer - Voltage Monitor Digital Filter Disable Mode Select
pub type DFDIS_W<'a, REG> = crate::BitWriter<'a, REG, DFDIS_A>;
impl<'a, REG> DFDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable digital filter
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DFDIS_A::_0)
    }
    ///Disable digital filter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DFDIS_A::_1)
    }
}
/**Voltage Monitor Circuit Comparison Result Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPE_A {
    ///0: Disable voltage monitor 1 circuit comparison result output
    _0 = 0,
    ///1: Enable voltage monitor 1 circuit comparison result output.
    _1 = 1,
}
impl From<CMPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPE` reader - Voltage Monitor Circuit Comparison Result Output Enable
pub type CMPE_R = crate::BitReader<CMPE_A>;
impl CMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPE_A {
        match self.bits {
            false => CMPE_A::_0,
            true => CMPE_A::_1,
        }
    }
    ///Disable voltage monitor 1 circuit comparison result output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPE_A::_0
    }
    ///Enable voltage monitor 1 circuit comparison result output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPE_A::_1
    }
}
///Field `CMPE` writer - Voltage Monitor Circuit Comparison Result Output Enable
pub type CMPE_W<'a, REG> = crate::BitWriter<'a, REG, CMPE_A>;
impl<'a, REG> CMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable voltage monitor 1 circuit comparison result output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPE_A::_0)
    }
    ///Enable voltage monitor 1 circuit comparison result output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPE_A::_1)
    }
}
/**Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSAMP_A {
    ///0: 1/2 LOCO frequency
    _00 = 0,
    ///1: 1/4 LOCO frequency
    _01 = 1,
    ///2: 1/8 LOCO frequency
    _10 = 2,
    ///3: 1/16 LOCO frequency
    _11 = 3,
}
impl From<FSAMP_A> for u8 {
    #[inline(always)]
    fn from(variant: FSAMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSAMP_A {
    type Ux = u8;
}
impl crate::IsEnum for FSAMP_A {}
///Field `FSAMP` reader - Sampling Clock Select
pub type FSAMP_R = crate::FieldReader<FSAMP_A>;
impl FSAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSAMP_A {
        match self.bits {
            0 => FSAMP_A::_00,
            1 => FSAMP_A::_01,
            2 => FSAMP_A::_10,
            3 => FSAMP_A::_11,
            _ => unreachable!(),
        }
    }
    ///1/2 LOCO frequency
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FSAMP_A::_00
    }
    ///1/4 LOCO frequency
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FSAMP_A::_01
    }
    ///1/8 LOCO frequency
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FSAMP_A::_10
    }
    ///1/16 LOCO frequency
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FSAMP_A::_11
    }
}
///Field `FSAMP` writer - Sampling Clock Select
pub type FSAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FSAMP_A, crate::Safe>;
impl<'a, REG> FSAMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/2 LOCO frequency
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(FSAMP_A::_00)
    }
    ///1/4 LOCO frequency
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(FSAMP_A::_01)
    }
    ///1/8 LOCO frequency
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(FSAMP_A::_10)
    }
    ///1/16 LOCO frequency
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(FSAMP_A::_11)
    }
}
/**Voltage Monitor Circuit Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI_A {
    ///0: Voltage Monitor interrupt during Vdet1 passage
    _0 = 0,
    ///1: Voltage Monitor reset enabled when the voltage falls to and below Vdet1
    _1 = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RI` reader - Voltage Monitor Circuit Mode Select
pub type RI_R = crate::BitReader<RI_A>;
impl RI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::_0,
            true => RI_A::_1,
        }
    }
    ///Voltage Monitor interrupt during Vdet1 passage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI_A::_0
    }
    ///Voltage Monitor reset enabled when the voltage falls to and below Vdet1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI_A::_1
    }
}
///Field `RI` writer - Voltage Monitor Circuit Mode Select
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG, RI_A>;
impl<'a, REG> RI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage Monitor interrupt during Vdet1 passage
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RI_A::_0)
    }
    ///Voltage Monitor reset enabled when the voltage falls to and below Vdet1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RI_A::_1)
    }
}
/**Voltage Monitor Reset Negate Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RN_A {
    ///0: Negation follows a stabilization time (tLVD) after VCC > Vdet is detected.
    _0 = 0,
    ///1: Negation follows a stabilization time (tLVD) after assertion of the LVD reset.
    _1 = 1,
}
impl From<RN_A> for bool {
    #[inline(always)]
    fn from(variant: RN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RN` reader - Voltage Monitor Reset Negate Select
pub type RN_R = crate::BitReader<RN_A>;
impl RN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RN_A {
        match self.bits {
            false => RN_A::_0,
            true => RN_A::_1,
        }
    }
    ///Negation follows a stabilization time (tLVD) after VCC > Vdet is detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RN_A::_0
    }
    ///Negation follows a stabilization time (tLVD) after assertion of the LVD reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RN_A::_1
    }
}
///Field `RN` writer - Voltage Monitor Reset Negate Select
pub type RN_W<'a, REG> = crate::BitWriter<'a, REG, RN_A>;
impl<'a, REG> RN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Negation follows a stabilization time (tLVD) after VCC > Vdet is detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RN_A::_0)
    }
    ///Negation follows a stabilization time (tLVD) after assertion of the LVD reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RN_A::_1)
    }
}
impl R {
    ///Bit 0 - Voltage Monitor Interrupt/Reset Enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage Monitor Digital Filter Disable Mode Select
    #[inline(always)]
    pub fn dfdis(&self) -> DFDIS_R {
        DFDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage Monitor Circuit Comparison Result Output Enable
    #[inline(always)]
    pub fn cmpe(&self) -> CMPE_R {
        CMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Sampling Clock Select
    #[inline(always)]
    pub fn fsamp(&self) -> FSAMP_R {
        FSAMP_R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - Voltage Monitor Circuit Mode Select
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Voltage Monitor Reset Negate Select
    #[inline(always)]
    pub fn rn(&self) -> RN_R {
        RN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Voltage Monitor Interrupt/Reset Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<LVDCR0_SPEC> {
        RIE_W::new(self, 0)
    }
    ///Bit 1 - Voltage Monitor Digital Filter Disable Mode Select
    #[inline(always)]
    pub fn dfdis(&mut self) -> DFDIS_W<LVDCR0_SPEC> {
        DFDIS_W::new(self, 1)
    }
    ///Bit 2 - Voltage Monitor Circuit Comparison Result Output Enable
    #[inline(always)]
    pub fn cmpe(&mut self) -> CMPE_W<LVDCR0_SPEC> {
        CMPE_W::new(self, 2)
    }
    ///Bits 4:5 - Sampling Clock Select
    #[inline(always)]
    pub fn fsamp(&mut self) -> FSAMP_W<LVDCR0_SPEC> {
        FSAMP_W::new(self, 4)
    }
    ///Bit 6 - Voltage Monitor Circuit Mode Select
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W<LVDCR0_SPEC> {
        RI_W::new(self, 6)
    }
    ///Bit 7 - Voltage Monitor Reset Negate Select
    #[inline(always)]
    pub fn rn(&mut self) -> RN_W<LVDCR0_SPEC> {
        RN_W::new(self, 7)
    }
}
/**Voltage Monitor %s Circuit Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lvdcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVDCR0_SPEC;
impl crate::RegisterSpec for LVDCR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lvdcr0::R`](R) reader structure
impl crate::Readable for LVDCR0_SPEC {}
///`write(|w| ..)` method takes [`lvdcr0::W`](W) writer structure
impl crate::Writable for LVDCR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD%sCR0 to value 0x8a
impl crate::Resettable for LVDCR0_SPEC {
    const RESET_VALUE: u8 = 0x8a;
}
