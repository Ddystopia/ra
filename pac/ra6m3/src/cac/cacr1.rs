///Register `CACR1` reader
pub type R = crate::R<CACR1_SPEC>;
///Register `CACR1` writer
pub type W = crate::W<CACR1_SPEC>;
/**CACREF Pin Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACREFE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<CACREFE_A> for bool {
    #[inline(always)]
    fn from(variant: CACREFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CACREFE` reader - CACREF Pin Input Enable
pub type CACREFE_R = crate::BitReader<CACREFE_A>;
impl CACREFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CACREFE_A {
        match self.bits {
            false => CACREFE_A::_0,
            true => CACREFE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CACREFE_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CACREFE_A::_1
    }
}
///Field `CACREFE` writer - CACREF Pin Input Enable
pub type CACREFE_W<'a, REG> = crate::BitWriter<'a, REG, CACREFE_A>;
impl<'a, REG> CACREFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CACREFE_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CACREFE_A::_1)
    }
}
/**Measurement Target Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMCS_A {
    ///0: Main clock
    _000 = 0,
    ///1: Sub-clock
    _001 = 1,
    ///2: HOCO clock
    _010 = 2,
    ///3: MOCO clock
    _011 = 3,
    ///4: LOCO clock
    _100 = 4,
    ///5: Peripheral module clock(PCLKB)
    _101 = 5,
    ///6: IWDTCLK clock
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<FMCS_A> for u8 {
    #[inline(always)]
    fn from(variant: FMCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMCS_A {
    type Ux = u8;
}
impl crate::IsEnum for FMCS_A {}
///Field `FMCS` reader - Measurement Target Clock Select
pub type FMCS_R = crate::FieldReader<FMCS_A>;
impl FMCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMCS_A {
        match self.bits {
            0 => FMCS_A::_000,
            1 => FMCS_A::_001,
            2 => FMCS_A::_010,
            3 => FMCS_A::_011,
            4 => FMCS_A::_100,
            5 => FMCS_A::_101,
            6 => FMCS_A::_110,
            7 => FMCS_A::_111,
            _ => unreachable!(),
        }
    }
    ///Main clock
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FMCS_A::_000
    }
    ///Sub-clock
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FMCS_A::_001
    }
    ///HOCO clock
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FMCS_A::_010
    }
    ///MOCO clock
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FMCS_A::_011
    }
    ///LOCO clock
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FMCS_A::_100
    }
    ///Peripheral module clock(PCLKB)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FMCS_A::_101
    }
    ///IWDTCLK clock
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FMCS_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FMCS_A::_111
    }
}
///Field `FMCS` writer - Measurement Target Clock Select
pub type FMCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FMCS_A, crate::Safe>;
impl<'a, REG> FMCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main clock
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_000)
    }
    ///Sub-clock
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_001)
    }
    ///HOCO clock
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_010)
    }
    ///MOCO clock
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_011)
    }
    ///LOCO clock
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_100)
    }
    ///Peripheral module clock(PCLKB)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_101)
    }
    ///IWDTCLK clock
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(FMCS_A::_111)
    }
}
/**Measurement Target Clock Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCSS_A {
    ///0: No division
    _00 = 0,
    ///1: x 1/4 clock
    _01 = 1,
    ///2: x 1/8 clock
    _10 = 2,
    ///3: x 1/32 clock
    _11 = 3,
}
impl From<TCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCSS_A {
    type Ux = u8;
}
impl crate::IsEnum for TCSS_A {}
///Field `TCSS` reader - Measurement Target Clock Frequency Division Ratio Select
pub type TCSS_R = crate::FieldReader<TCSS_A>;
impl TCSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCSS_A {
        match self.bits {
            0 => TCSS_A::_00,
            1 => TCSS_A::_01,
            2 => TCSS_A::_10,
            3 => TCSS_A::_11,
            _ => unreachable!(),
        }
    }
    ///No division
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCSS_A::_00
    }
    ///x 1/4 clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCSS_A::_01
    }
    ///x 1/8 clock
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCSS_A::_10
    }
    ///x 1/32 clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCSS_A::_11
    }
}
///Field `TCSS` writer - Measurement Target Clock Frequency Division Ratio Select
pub type TCSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCSS_A, crate::Safe>;
impl<'a, REG> TCSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No division
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_00)
    }
    ///x 1/4 clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_01)
    }
    ///x 1/8 clock
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_10)
    }
    ///x 1/32 clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TCSS_A::_11)
    }
}
/**Valid Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGES_A {
    ///0: Rising edge
    _00 = 0,
    ///1: Falling edge
    _01 = 1,
    ///2: Both rising and falling edges
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<EDGES_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDGES_A {
    type Ux = u8;
}
impl crate::IsEnum for EDGES_A {}
///Field `EDGES` reader - Valid Edge Select
pub type EDGES_R = crate::FieldReader<EDGES_A>;
impl EDGES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDGES_A {
        match self.bits {
            0 => EDGES_A::_00,
            1 => EDGES_A::_01,
            2 => EDGES_A::_10,
            3 => EDGES_A::_11,
            _ => unreachable!(),
        }
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EDGES_A::_00
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EDGES_A::_01
    }
    ///Both rising and falling edges
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EDGES_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EDGES_A::_11
    }
}
///Field `EDGES` writer - Valid Edge Select
pub type EDGES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EDGES_A, crate::Safe>;
impl<'a, REG> EDGES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Rising edge
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_00)
    }
    ///Falling edge
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_01)
    }
    ///Both rising and falling edges
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(EDGES_A::_11)
    }
}
impl R {
    ///Bit 0 - CACREF Pin Input Enable
    #[inline(always)]
    pub fn cacrefe(&self) -> CACREFE_R {
        CACREFE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Measurement Target Clock Select
    #[inline(always)]
    pub fn fmcs(&self) -> FMCS_R {
        FMCS_R::new((self.bits >> 1) & 7)
    }
    ///Bits 4:5 - Measurement Target Clock Frequency Division Ratio Select
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - Valid Edge Select
    #[inline(always)]
    pub fn edges(&self) -> EDGES_R {
        EDGES_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bit 0 - CACREF Pin Input Enable
    #[inline(always)]
    pub fn cacrefe(&mut self) -> CACREFE_W<CACR1_SPEC> {
        CACREFE_W::new(self, 0)
    }
    ///Bits 1:3 - Measurement Target Clock Select
    #[inline(always)]
    pub fn fmcs(&mut self) -> FMCS_W<CACR1_SPEC> {
        FMCS_W::new(self, 1)
    }
    ///Bits 4:5 - Measurement Target Clock Frequency Division Ratio Select
    #[inline(always)]
    pub fn tcss(&mut self) -> TCSS_W<CACR1_SPEC> {
        TCSS_W::new(self, 4)
    }
    ///Bits 6:7 - Valid Edge Select
    #[inline(always)]
    pub fn edges(&mut self) -> EDGES_W<CACR1_SPEC> {
        EDGES_W::new(self, 6)
    }
}
/**CAC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cacr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACR1_SPEC;
impl crate::RegisterSpec for CACR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cacr1::R`](R) reader structure
impl crate::Readable for CACR1_SPEC {}
///`write(|w| ..)` method takes [`cacr1::W`](W) writer structure
impl crate::Writable for CACR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CACR1 to value 0
impl crate::Resettable for CACR1_SPEC {}
