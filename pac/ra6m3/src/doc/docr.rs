///Register `DOCR` reader
pub type R = crate::R<DOCR_SPEC>;
///Register `DOCR` writer
pub type W = crate::W<DOCR_SPEC>;
/**Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OMS_A {
    ///0: Data comparison mode
    _00 = 0,
    ///1: Data addition mode
    _01 = 1,
    ///2: Data subtraction mode
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<OMS_A> for u8 {
    #[inline(always)]
    fn from(variant: OMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OMS_A {
    type Ux = u8;
}
impl crate::IsEnum for OMS_A {}
///Field `OMS` reader - Operating Mode Select
pub type OMS_R = crate::FieldReader<OMS_A>;
impl OMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OMS_A {
        match self.bits {
            0 => OMS_A::_00,
            1 => OMS_A::_01,
            2 => OMS_A::_10,
            3 => OMS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Data comparison mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OMS_A::_00
    }
    ///Data addition mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OMS_A::_01
    }
    ///Data subtraction mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OMS_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OMS_A::_11
    }
}
///Field `OMS` writer - Operating Mode Select
pub type OMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OMS_A, crate::Safe>;
impl<'a, REG> OMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Data comparison mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_00)
    }
    ///Data addition mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_01)
    }
    ///Data subtraction mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OMS_A::_11)
    }
}
/**Detection Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCSEL_A {
    ///0: DOPCF is set when data mismatch is detected.
    _0 = 0,
    ///1: DOPCF is set when data match is detected.
    _1 = 1,
}
impl From<DCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DCSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DCSEL` reader - Detection Condition Select
pub type DCSEL_R = crate::BitReader<DCSEL_A>;
impl DCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCSEL_A {
        match self.bits {
            false => DCSEL_A::_0,
            true => DCSEL_A::_1,
        }
    }
    ///DOPCF is set when data mismatch is detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCSEL_A::_0
    }
    ///DOPCF is set when data match is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCSEL_A::_1
    }
}
///Field `DCSEL` writer - Detection Condition Select
pub type DCSEL_W<'a, REG> = crate::BitWriter<'a, REG, DCSEL_A>;
impl<'a, REG> DCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DOPCF is set when data mismatch is detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCSEL_A::_0)
    }
    ///DOPCF is set when data match is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCSEL_A::_1)
    }
}
///Field `DOPCF` reader - Data Operation Circuit FlagIndicates the result of an operation.
pub type DOPCF_R = crate::BitReader;
/**DOPCF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOPCFCL_A {
    ///0: Maintains the DOPCF flag state.
    _0 = 0,
    ///1: Clears the DOPCF flag.
    _1 = 1,
}
impl From<DOPCFCL_A> for bool {
    #[inline(always)]
    fn from(variant: DOPCFCL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOPCFCL` reader - DOPCF Clear
pub type DOPCFCL_R = crate::BitReader<DOPCFCL_A>;
impl DOPCFCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOPCFCL_A {
        match self.bits {
            false => DOPCFCL_A::_0,
            true => DOPCFCL_A::_1,
        }
    }
    ///Maintains the DOPCF flag state.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOPCFCL_A::_0
    }
    ///Clears the DOPCF flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOPCFCL_A::_1
    }
}
///Field `DOPCFCL` writer - DOPCF Clear
pub type DOPCFCL_W<'a, REG> = crate::BitWriter<'a, REG, DOPCFCL_A>;
impl<'a, REG> DOPCFCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Maintains the DOPCF flag state.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOPCFCL_A::_0)
    }
    ///Clears the DOPCF flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOPCFCL_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Operating Mode Select
    #[inline(always)]
    pub fn oms(&self) -> OMS_R {
        OMS_R::new(self.bits & 3)
    }
    ///Bit 2 - Detection Condition Select
    #[inline(always)]
    pub fn dcsel(&self) -> DCSEL_R {
        DCSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Data Operation Circuit FlagIndicates the result of an operation.
    #[inline(always)]
    pub fn dopcf(&self) -> DOPCF_R {
        DOPCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DOPCF Clear
    #[inline(always)]
    pub fn dopcfcl(&self) -> DOPCFCL_R {
        DOPCFCL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Operating Mode Select
    #[inline(always)]
    pub fn oms(&mut self) -> OMS_W<DOCR_SPEC> {
        OMS_W::new(self, 0)
    }
    ///Bit 2 - Detection Condition Select
    #[inline(always)]
    pub fn dcsel(&mut self) -> DCSEL_W<DOCR_SPEC> {
        DCSEL_W::new(self, 2)
    }
    ///Bit 6 - DOPCF Clear
    #[inline(always)]
    pub fn dopcfcl(&mut self) -> DOPCFCL_W<DOCR_SPEC> {
        DOPCFCL_W::new(self, 6)
    }
}
/**DOC Control Register

You can [`read`](crate::Reg::read) this register and get [`docr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DOCR_SPEC;
impl crate::RegisterSpec for DOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`docr::R`](R) reader structure
impl crate::Readable for DOCR_SPEC {}
///`write(|w| ..)` method takes [`docr::W`](W) writer structure
impl crate::Writable for DOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOCR to value 0
impl crate::Resettable for DOCR_SPEC {}
