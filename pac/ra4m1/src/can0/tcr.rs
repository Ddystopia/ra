///Register `TCR` reader
pub type R = crate::R<TCR_SPEC>;
///Register `TCR` writer
pub type W = crate::W<TCR_SPEC>;
/**CAN Test Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTE_A {
    ///0: CAN test mode disabled
    _0 = 0,
    ///1: CAN test mode enabled
    _1 = 1,
}
impl From<TSTE_A> for bool {
    #[inline(always)]
    fn from(variant: TSTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTE` reader - CAN Test Mode Enable
pub type TSTE_R = crate::BitReader<TSTE_A>;
impl TSTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSTE_A {
        match self.bits {
            false => TSTE_A::_0,
            true => TSTE_A::_1,
        }
    }
    ///CAN test mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSTE_A::_0
    }
    ///CAN test mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSTE_A::_1
    }
}
///Field `TSTE` writer - CAN Test Mode Enable
pub type TSTE_W<'a, REG> = crate::BitWriter<'a, REG, TSTE_A>;
impl<'a, REG> TSTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN test mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTE_A::_0)
    }
    ///CAN test mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTE_A::_1)
    }
}
/**CAN Test Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTM_A {
    ///0: Other than CAN test mode
    _00 = 0,
    ///1: Listen-only mode
    _01 = 1,
    ///2: Self-test mode 0 (external loopback)
    _10 = 2,
    ///3: Self-test mode 1 (internal loopback)
    _11 = 3,
}
impl From<TSTM_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTM_A {
    type Ux = u8;
}
impl crate::IsEnum for TSTM_A {}
///Field `TSTM` reader - CAN Test Mode Select
pub type TSTM_R = crate::FieldReader<TSTM_A>;
impl TSTM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSTM_A {
        match self.bits {
            0 => TSTM_A::_00,
            1 => TSTM_A::_01,
            2 => TSTM_A::_10,
            3 => TSTM_A::_11,
            _ => unreachable!(),
        }
    }
    ///Other than CAN test mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSTM_A::_00
    }
    ///Listen-only mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSTM_A::_01
    }
    ///Self-test mode 0 (external loopback)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSTM_A::_10
    }
    ///Self-test mode 1 (internal loopback)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSTM_A::_11
    }
}
///Field `TSTM` writer - CAN Test Mode Select
pub type TSTM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TSTM_A, crate::Safe>;
impl<'a, REG> TSTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Other than CAN test mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TSTM_A::_00)
    }
    ///Listen-only mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TSTM_A::_01)
    }
    ///Self-test mode 0 (external loopback)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TSTM_A::_10)
    }
    ///Self-test mode 1 (internal loopback)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TSTM_A::_11)
    }
}
impl R {
    ///Bit 0 - CAN Test Mode Enable
    #[inline(always)]
    pub fn tste(&self) -> TSTE_R {
        TSTE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - CAN Test Mode Select
    #[inline(always)]
    pub fn tstm(&self) -> TSTM_R {
        TSTM_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    ///Bit 0 - CAN Test Mode Enable
    #[inline(always)]
    pub fn tste(&mut self) -> TSTE_W<TCR_SPEC> {
        TSTE_W::new(self, 0)
    }
    ///Bits 1:2 - CAN Test Mode Select
    #[inline(always)]
    pub fn tstm(&mut self) -> TSTM_W<TCR_SPEC> {
        TSTM_W::new(self, 1)
    }
}
/**Test Control Register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCR_SPEC {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCR_SPEC {}
