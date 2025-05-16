///Register `SIMR3` reader
pub type R = crate::R<SIMR3_SPEC>;
///Register `SIMR3` writer
pub type W = crate::W<SIMR3_SPEC>;
/**Start Condition Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTAREQ_A {
    ///0: A start condition is not generated.
    _0 = 0,
    ///1: A start condition is generated.
    _1 = 1,
}
impl From<IICSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICSTAREQ` reader - Start Condition Generation
pub type IICSTAREQ_R = crate::BitReader<IICSTAREQ_A>;
impl IICSTAREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICSTAREQ_A {
        match self.bits {
            false => IICSTAREQ_A::_0,
            true => IICSTAREQ_A::_1,
        }
    }
    ///A start condition is not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTAREQ_A::_0
    }
    ///A start condition is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTAREQ_A::_1
    }
}
///Field `IICSTAREQ` writer - Start Condition Generation
pub type IICSTAREQ_W<'a, REG> = crate::BitWriter<'a, REG, IICSTAREQ_A>;
impl<'a, REG> IICSTAREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A start condition is not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTAREQ_A::_0)
    }
    ///A start condition is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTAREQ_A::_1)
    }
}
/**Restart Condition Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICRSTAREQ_A {
    ///0: A restart condition is not generated.
    _0 = 0,
    ///1: A restart condition is generated.
    _1 = 1,
}
impl From<IICRSTAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICRSTAREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICRSTAREQ` reader - Restart Condition Generation
pub type IICRSTAREQ_R = crate::BitReader<IICRSTAREQ_A>;
impl IICRSTAREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICRSTAREQ_A {
        match self.bits {
            false => IICRSTAREQ_A::_0,
            true => IICRSTAREQ_A::_1,
        }
    }
    ///A restart condition is not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICRSTAREQ_A::_0
    }
    ///A restart condition is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICRSTAREQ_A::_1
    }
}
///Field `IICRSTAREQ` writer - Restart Condition Generation
pub type IICRSTAREQ_W<'a, REG> = crate::BitWriter<'a, REG, IICRSTAREQ_A>;
impl<'a, REG> IICRSTAREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A restart condition is not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICRSTAREQ_A::_0)
    }
    ///A restart condition is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICRSTAREQ_A::_1)
    }
}
/**Stop Condition Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTPREQ_A {
    ///0: A stop condition is not generated.
    _0 = 0,
    ///1: A stop condition is generated.
    _1 = 1,
}
impl From<IICSTPREQ_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTPREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICSTPREQ` reader - Stop Condition Generation
pub type IICSTPREQ_R = crate::BitReader<IICSTPREQ_A>;
impl IICSTPREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICSTPREQ_A {
        match self.bits {
            false => IICSTPREQ_A::_0,
            true => IICSTPREQ_A::_1,
        }
    }
    ///A stop condition is not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTPREQ_A::_0
    }
    ///A stop condition is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTPREQ_A::_1
    }
}
///Field `IICSTPREQ` writer - Stop Condition Generation
pub type IICSTPREQ_W<'a, REG> = crate::BitWriter<'a, REG, IICSTPREQ_A>;
impl<'a, REG> IICSTPREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A stop condition is not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTPREQ_A::_0)
    }
    ///A stop condition is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTPREQ_A::_1)
    }
}
/**Issuing of Start, Restart, or Stop Condition Completed Flag(When 0 is written to IICSTIF, it is cleared to 0.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTIF_A {
    ///0: There are no requests for generating conditions or a condition is being generated.
    _0 = 0,
    ///1: A start, restart, or stop condition is completely generated.
    _1 = 1,
}
impl From<IICSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `IICSTIF` reader - Issuing of Start, Restart, or Stop Condition Completed Flag(When 0 is written to IICSTIF, it is cleared to 0.)

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type IICSTIF_R = crate::BitReader<IICSTIF_A>;
impl IICSTIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICSTIF_A {
        match self.bits {
            false => IICSTIF_A::_0,
            true => IICSTIF_A::_1,
        }
    }
    ///There are no requests for generating conditions or a condition is being generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTIF_A::_0
    }
    ///A start, restart, or stop condition is completely generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTIF_A::_1
    }
}
///Field `IICSTIF` writer - Issuing of Start, Restart, or Stop Condition Completed Flag(When 0 is written to IICSTIF, it is cleared to 0.)
pub type IICSTIF_W<'a, REG> = crate::BitWriter0C<'a, REG, IICSTIF_A>;
impl<'a, REG> IICSTIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///There are no requests for generating conditions or a condition is being generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTIF_A::_0)
    }
    ///A start, restart, or stop condition is completely generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICSTIF_A::_1)
    }
}
/**SDA Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSDAS_A {
    ///0: Serial data output
    _00 = 0,
    ///1: Generate a start, restart, or stop condition.
    _01 = 1,
    ///2: Output the low level on the SSDAn pin.
    _10 = 2,
    ///3: Place the SSDAn pin in the high-impedance state.
    _11 = 3,
}
impl From<IICSDAS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSDAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IICSDAS_A {
    type Ux = u8;
}
impl crate::IsEnum for IICSDAS_A {}
///Field `IICSDAS` reader - SDA Output Select
pub type IICSDAS_R = crate::FieldReader<IICSDAS_A>;
impl IICSDAS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICSDAS_A {
        match self.bits {
            0 => IICSDAS_A::_00,
            1 => IICSDAS_A::_01,
            2 => IICSDAS_A::_10,
            3 => IICSDAS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Serial data output
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSDAS_A::_00
    }
    ///Generate a start, restart, or stop condition.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSDAS_A::_01
    }
    ///Output the low level on the SSDAn pin.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSDAS_A::_10
    }
    ///Place the SSDAn pin in the high-impedance state.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSDAS_A::_11
    }
}
///Field `IICSDAS` writer - SDA Output Select
pub type IICSDAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IICSDAS_A, crate::Safe>;
impl<'a, REG> IICSDAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Serial data output
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_00)
    }
    ///Generate a start, restart, or stop condition.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_01)
    }
    ///Output the low level on the SSDAn pin.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_10)
    }
    ///Place the SSDAn pin in the high-impedance state.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IICSDAS_A::_11)
    }
}
/**SCL Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICSCLS_A {
    ///0: Serial clock output
    _00 = 0,
    ///1: Generate a start, restart, or stop condition.
    _01 = 1,
    ///2: Output the low level on the SSCLn pin.
    _10 = 2,
    ///3: Place the SSCLn pin in the high-impedance state.
    _11 = 3,
}
impl From<IICSCLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IICSCLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IICSCLS_A {
    type Ux = u8;
}
impl crate::IsEnum for IICSCLS_A {}
///Field `IICSCLS` reader - SCL Output Select
pub type IICSCLS_R = crate::FieldReader<IICSCLS_A>;
impl IICSCLS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICSCLS_A {
        match self.bits {
            0 => IICSCLS_A::_00,
            1 => IICSCLS_A::_01,
            2 => IICSCLS_A::_10,
            3 => IICSCLS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Serial clock output
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IICSCLS_A::_00
    }
    ///Generate a start, restart, or stop condition.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IICSCLS_A::_01
    }
    ///Output the low level on the SSCLn pin.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IICSCLS_A::_10
    }
    ///Place the SSCLn pin in the high-impedance state.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IICSCLS_A::_11
    }
}
///Field `IICSCLS` writer - SCL Output Select
pub type IICSCLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IICSCLS_A, crate::Safe>;
impl<'a, REG> IICSCLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Serial clock output
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_00)
    }
    ///Generate a start, restart, or stop condition.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_01)
    }
    ///Output the low level on the SSCLn pin.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_10)
    }
    ///Place the SSCLn pin in the high-impedance state.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IICSCLS_A::_11)
    }
}
impl R {
    ///Bit 0 - Start Condition Generation
    #[inline(always)]
    pub fn iicstareq(&self) -> IICSTAREQ_R {
        IICSTAREQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Restart Condition Generation
    #[inline(always)]
    pub fn iicrstareq(&self) -> IICRSTAREQ_R {
        IICRSTAREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Stop Condition Generation
    #[inline(always)]
    pub fn iicstpreq(&self) -> IICSTPREQ_R {
        IICSTPREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag(When 0 is written to IICSTIF, it is cleared to 0.)
    #[inline(always)]
    pub fn iicstif(&self) -> IICSTIF_R {
        IICSTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SDA Output Select
    #[inline(always)]
    pub fn iicsdas(&self) -> IICSDAS_R {
        IICSDAS_R::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - SCL Output Select
    #[inline(always)]
    pub fn iicscls(&self) -> IICSCLS_R {
        IICSCLS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bit 0 - Start Condition Generation
    #[inline(always)]
    pub fn iicstareq(&mut self) -> IICSTAREQ_W<SIMR3_SPEC> {
        IICSTAREQ_W::new(self, 0)
    }
    ///Bit 1 - Restart Condition Generation
    #[inline(always)]
    pub fn iicrstareq(&mut self) -> IICRSTAREQ_W<SIMR3_SPEC> {
        IICRSTAREQ_W::new(self, 1)
    }
    ///Bit 2 - Stop Condition Generation
    #[inline(always)]
    pub fn iicstpreq(&mut self) -> IICSTPREQ_W<SIMR3_SPEC> {
        IICSTPREQ_W::new(self, 2)
    }
    ///Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag(When 0 is written to IICSTIF, it is cleared to 0.)
    #[inline(always)]
    pub fn iicstif(&mut self) -> IICSTIF_W<SIMR3_SPEC> {
        IICSTIF_W::new(self, 3)
    }
    ///Bits 4:5 - SDA Output Select
    #[inline(always)]
    pub fn iicsdas(&mut self) -> IICSDAS_W<SIMR3_SPEC> {
        IICSDAS_W::new(self, 4)
    }
    ///Bits 6:7 - SCL Output Select
    #[inline(always)]
    pub fn iicscls(&mut self) -> IICSCLS_W<SIMR3_SPEC> {
        IICSCLS_W::new(self, 6)
    }
}
/**I2C Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`simr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SIMR3_SPEC;
impl crate::RegisterSpec for SIMR3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`simr3::R`](R) reader structure
impl crate::Readable for SIMR3_SPEC {}
///`write(|w| ..)` method takes [`simr3::W`](W) writer structure
impl crate::Writable for SIMR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x08;
}
///`reset()` method sets SIMR3 to value 0
impl crate::Resettable for SIMR3_SPEC {}
