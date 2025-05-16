///Register `SSIOFR` reader
pub type R = crate::R<SSIOFR_SPEC>;
///Register `SSIOFR` writer
pub type W = crate::W<SSIOFR_SPEC>;
/**Audio Format Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OMOD_A {
    ///0: I2S format
    _00 = 0,
    ///1: TDM format
    _01 = 1,
    ///2: Monaural format
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<OMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for OMOD_A {}
///Field `OMOD` reader - Audio Format Select
pub type OMOD_R = crate::FieldReader<OMOD_A>;
impl OMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OMOD_A {
        match self.bits {
            0 => OMOD_A::_00,
            1 => OMOD_A::_01,
            2 => OMOD_A::_10,
            3 => OMOD_A::_11,
            _ => unreachable!(),
        }
    }
    ///I2S format
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OMOD_A::_00
    }
    ///TDM format
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OMOD_A::_01
    }
    ///Monaural format
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OMOD_A::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OMOD_A::_11
    }
}
///Field `OMOD` writer - Audio Format Select
pub type OMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OMOD_A, crate::Safe>;
impl<'a, REG> OMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2S format
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OMOD_A::_00)
    }
    ///TDM format
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OMOD_A::_01)
    }
    ///Monaural format
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OMOD_A::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OMOD_A::_11)
    }
}
/**Whether to Enable LRCK/FS Continuation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCONT_A {
    ///0: Disables LRCK/FS continuation.
    _0 = 0,
    ///1: Enables LRCK/FS continuation.
    _1 = 1,
}
impl From<LRCONT_A> for bool {
    #[inline(always)]
    fn from(variant: LRCONT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LRCONT` reader - Whether to Enable LRCK/FS Continuation
pub type LRCONT_R = crate::BitReader<LRCONT_A>;
impl LRCONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LRCONT_A {
        match self.bits {
            false => LRCONT_A::_0,
            true => LRCONT_A::_1,
        }
    }
    ///Disables LRCK/FS continuation.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRCONT_A::_0
    }
    ///Enables LRCK/FS continuation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRCONT_A::_1
    }
}
///Field `LRCONT` writer - Whether to Enable LRCK/FS Continuation
pub type LRCONT_W<'a, REG> = crate::BitWriter<'a, REG, LRCONT_A>;
impl<'a, REG> LRCONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables LRCK/FS continuation.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LRCONT_A::_0)
    }
    ///Enables LRCK/FS continuation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LRCONT_A::_1)
    }
}
/**Whether to Enable Stopping BCK Output When SSIE is in Idle Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCKASTP_A {
    ///0: Always outputs BCK to the SSIBCK pin.
    _0 = 0,
    ///1: Automatically controls output of BCK to the SSIBCK pin.
    _1 = 1,
}
impl From<BCKASTP_A> for bool {
    #[inline(always)]
    fn from(variant: BCKASTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCKASTP` reader - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
pub type BCKASTP_R = crate::BitReader<BCKASTP_A>;
impl BCKASTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCKASTP_A {
        match self.bits {
            false => BCKASTP_A::_0,
            true => BCKASTP_A::_1,
        }
    }
    ///Always outputs BCK to the SSIBCK pin.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCKASTP_A::_0
    }
    ///Automatically controls output of BCK to the SSIBCK pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCKASTP_A::_1
    }
}
///Field `BCKASTP` writer - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
pub type BCKASTP_W<'a, REG> = crate::BitWriter<'a, REG, BCKASTP_A>;
impl<'a, REG> BCKASTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Always outputs BCK to the SSIBCK pin.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCKASTP_A::_0)
    }
    ///Automatically controls output of BCK to the SSIBCK pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCKASTP_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Audio Format Select
    #[inline(always)]
    pub fn omod(&self) -> OMOD_R {
        OMOD_R::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Whether to Enable LRCK/FS Continuation
    #[inline(always)]
    pub fn lrcont(&self) -> LRCONT_R {
        LRCONT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
    #[inline(always)]
    pub fn bckastp(&self) -> BCKASTP_R {
        BCKASTP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Audio Format Select
    #[inline(always)]
    pub fn omod(&mut self) -> OMOD_W<SSIOFR_SPEC> {
        OMOD_W::new(self, 0)
    }
    ///Bit 8 - Whether to Enable LRCK/FS Continuation
    #[inline(always)]
    pub fn lrcont(&mut self) -> LRCONT_W<SSIOFR_SPEC> {
        LRCONT_W::new(self, 8)
    }
    ///Bit 9 - Whether to Enable Stopping BCK Output When SSIE is in Idle Status
    #[inline(always)]
    pub fn bckastp(&mut self) -> BCKASTP_W<SSIOFR_SPEC> {
        BCKASTP_W::new(self, 9)
    }
}
/**Audio Format Register

You can [`read`](crate::Reg::read) this register and get [`ssiofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSIOFR_SPEC;
impl crate::RegisterSpec for SSIOFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssiofr::R`](R) reader structure
impl crate::Readable for SSIOFR_SPEC {}
///`write(|w| ..)` method takes [`ssiofr::W`](W) writer structure
impl crate::Writable for SSIOFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIOFR to value 0
impl crate::Resettable for SSIOFR_SPEC {}
