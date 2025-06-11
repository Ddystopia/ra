///Register `VBTSR` reader
pub type R = crate::R<VBTSR_SPEC>;
///Register `VBTSR` writer
pub type W = crate::W<VBTSR_SPEC>;
/**VBAT_R Reset Detect Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTRDF_A {
    ///0: VBATT_R voltage power-on reset not detected
    _0 = 0,
    ///1: VBATT_R selected voltage power-on reset detected.
    _1 = 1,
}
impl From<VBTRDF_A> for bool {
    #[inline(always)]
    fn from(variant: VBTRDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VBTRDF` reader - VBAT_R Reset Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VBTRDF_R = crate::BitReader<VBTRDF_A>;
impl VBTRDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTRDF_A {
        match self.bits {
            false => VBTRDF_A::_0,
            true => VBTRDF_A::_1,
        }
    }
    ///VBATT_R voltage power-on reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTRDF_A::_0
    }
    ///VBATT_R selected voltage power-on reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTRDF_A::_1
    }
}
///Field `VBTRDF` writer - VBAT_R Reset Detect Flag
pub type VBTRDF_W<'a, REG> = crate::BitWriter0C<'a, REG, VBTRDF_A>;
impl<'a, REG> VBTRDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT_R voltage power-on reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTRDF_A::_0)
    }
    ///VBATT_R selected voltage power-on reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTRDF_A::_1)
    }
}
/**VBATT Battery Low voltage Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTBLDF_A {
    ///0: VBATT pin low voltage not detected
    _0 = 0,
    ///1: VBATT pin low voltage detected.
    _1 = 1,
}
impl From<VBTBLDF_A> for bool {
    #[inline(always)]
    fn from(variant: VBTBLDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VBTBLDF` reader - VBATT Battery Low voltage Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VBTBLDF_R = crate::BitReader<VBTBLDF_A>;
impl VBTBLDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTBLDF_A {
        match self.bits {
            false => VBTBLDF_A::_0,
            true => VBTBLDF_A::_1,
        }
    }
    ///VBATT pin low voltage not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTBLDF_A::_0
    }
    ///VBATT pin low voltage detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTBLDF_A::_1
    }
}
///Field `VBTBLDF` writer - VBATT Battery Low voltage Detect Flag
pub type VBTBLDF_W<'a, REG> = crate::BitWriter0C<'a, REG, VBTBLDF_A>;
impl<'a, REG> VBTBLDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT pin low voltage not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTBLDF_A::_0)
    }
    ///VBATT pin low voltage detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTBLDF_A::_1)
    }
}
/**VBATT_R Valid

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTRVLD_A {
    ///0: VBATT_R area not valid
    _0 = 0,
    ///1: VBATT_R area valid
    _1 = 1,
}
impl From<VBTRVLD_A> for bool {
    #[inline(always)]
    fn from(variant: VBTRVLD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBTRVLD` reader - VBATT_R Valid
pub type VBTRVLD_R = crate::BitReader<VBTRVLD_A>;
impl VBTRVLD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTRVLD_A {
        match self.bits {
            false => VBTRVLD_A::_0,
            true => VBTRVLD_A::_1,
        }
    }
    ///VBATT_R area not valid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTRVLD_A::_0
    }
    ///VBATT_R area valid
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTRVLD_A::_1
    }
}
impl R {
    ///Bit 0 - VBAT_R Reset Detect Flag
    #[inline(always)]
    pub fn vbtrdf(&self) -> VBTRDF_R {
        VBTRDF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBATT Battery Low voltage Detect Flag
    #[inline(always)]
    pub fn vbtbldf(&self) -> VBTBLDF_R {
        VBTBLDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - VBATT_R Valid
    #[inline(always)]
    pub fn vbtrvld(&self) -> VBTRVLD_R {
        VBTRVLD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBAT_R Reset Detect Flag
    #[inline(always)]
    pub fn vbtrdf(&mut self) -> VBTRDF_W<VBTSR_SPEC> {
        VBTRDF_W::new(self, 0)
    }
    ///Bit 1 - VBATT Battery Low voltage Detect Flag
    #[inline(always)]
    pub fn vbtbldf(&mut self) -> VBTBLDF_W<VBTSR_SPEC> {
        VBTBLDF_W::new(self, 1)
    }
}
/**VBATT Status Register

You can [`read`](crate::Reg::read) this register and get [`vbtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTSR_SPEC;
impl crate::RegisterSpec for VBTSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtsr::R`](R) reader structure
impl crate::Readable for VBTSR_SPEC {}
///`write(|w| ..)` method takes [`vbtsr::W`](W) writer structure
impl crate::Writable for VBTSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
///`reset()` method sets VBTSR to value 0x01
impl crate::Resettable for VBTSR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
