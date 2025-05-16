///Register `GTDTCR` reader
pub type R = crate::R<GTDTCR_SPEC>;
///Register `GTDTCR` writer
pub type W = crate::W<GTDTCR_SPEC>;
/**Negative-Phase Waveform Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    ///0: Set GTCCRB without using GTDVU and GTDVD.
    _0 = 0,
    ///1: Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB.
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDE` reader - Negative-Phase Waveform Setting
pub type TDE_R = crate::BitReader<TDE_A>;
impl TDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    ///Set GTCCRB without using GTDVU and GTDVD.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    ///Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
///Field `TDE` writer - Negative-Phase Waveform Setting
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG, TDE_A>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set GTCCRB without using GTDVU and GTDVD.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    ///Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
/**GTDVU Buffer Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBUE_A {
    ///0: Disable GTDVU buffer operation
    _0 = 0,
    ///1: Enable GTDVU buffer operation
    _1 = 1,
}
impl From<TDBUE_A> for bool {
    #[inline(always)]
    fn from(variant: TDBUE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDBUE` reader - GTDVU Buffer Operation Enable
pub type TDBUE_R = crate::BitReader<TDBUE_A>;
impl TDBUE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDBUE_A {
        match self.bits {
            false => TDBUE_A::_0,
            true => TDBUE_A::_1,
        }
    }
    ///Disable GTDVU buffer operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBUE_A::_0
    }
    ///Enable GTDVU buffer operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBUE_A::_1
    }
}
///Field `TDBUE` writer - GTDVU Buffer Operation Enable
pub type TDBUE_W<'a, REG> = crate::BitWriter<'a, REG, TDBUE_A>;
impl<'a, REG> TDBUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTDVU buffer operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDBUE_A::_0)
    }
    ///Enable GTDVU buffer operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDBUE_A::_1)
    }
}
/**GTDVD Buffer Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBDE_A {
    ///0: Disable GTDVD buffer operation
    _0 = 0,
    ///1: Enable GTDVD buffer operation
    _1 = 1,
}
impl From<TDBDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDBDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDBDE` reader - GTDVD Buffer Operation Enable
pub type TDBDE_R = crate::BitReader<TDBDE_A>;
impl TDBDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDBDE_A {
        match self.bits {
            false => TDBDE_A::_0,
            true => TDBDE_A::_1,
        }
    }
    ///Disable GTDVD buffer operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBDE_A::_0
    }
    ///Enable GTDVD buffer operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBDE_A::_1
    }
}
///Field `TDBDE` writer - GTDVD Buffer Operation Enable
pub type TDBDE_W<'a, REG> = crate::BitWriter<'a, REG, TDBDE_A>;
impl<'a, REG> TDBDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTDVD buffer operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDBDE_A::_0)
    }
    ///Enable GTDVD buffer operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDBDE_A::_1)
    }
}
/**GTDVD Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDFER_A {
    ///0: Set GTDVU and GTDVD separately
    _0 = 0,
    ///1: Automatically set the value written to GTDVU to GTDVD
    _1 = 1,
}
impl From<TDFER_A> for bool {
    #[inline(always)]
    fn from(variant: TDFER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDFER` reader - GTDVD Setting
pub type TDFER_R = crate::BitReader<TDFER_A>;
impl TDFER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDFER_A {
        match self.bits {
            false => TDFER_A::_0,
            true => TDFER_A::_1,
        }
    }
    ///Set GTDVU and GTDVD separately
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDFER_A::_0
    }
    ///Automatically set the value written to GTDVU to GTDVD
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDFER_A::_1
    }
}
///Field `TDFER` writer - GTDVD Setting
pub type TDFER_W<'a, REG> = crate::BitWriter<'a, REG, TDFER_A>;
impl<'a, REG> TDFER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set GTDVU and GTDVD separately
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDFER_A::_0)
    }
    ///Automatically set the value written to GTDVU to GTDVD
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDFER_A::_1)
    }
}
impl R {
    ///Bit 0 - Negative-Phase Waveform Setting
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - GTDVU Buffer Operation Enable
    #[inline(always)]
    pub fn tdbue(&self) -> TDBUE_R {
        TDBUE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTDVD Buffer Operation Enable
    #[inline(always)]
    pub fn tdbde(&self) -> TDBDE_R {
        TDBDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - GTDVD Setting
    #[inline(always)]
    pub fn tdfer(&self) -> TDFER_R {
        TDFER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Negative-Phase Waveform Setting
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<GTDTCR_SPEC> {
        TDE_W::new(self, 0)
    }
    ///Bit 4 - GTDVU Buffer Operation Enable
    #[inline(always)]
    pub fn tdbue(&mut self) -> TDBUE_W<GTDTCR_SPEC> {
        TDBUE_W::new(self, 4)
    }
    ///Bit 5 - GTDVD Buffer Operation Enable
    #[inline(always)]
    pub fn tdbde(&mut self) -> TDBDE_W<GTDTCR_SPEC> {
        TDBDE_W::new(self, 5)
    }
    ///Bit 8 - GTDVD Setting
    #[inline(always)]
    pub fn tdfer(&mut self) -> TDFER_W<GTDTCR_SPEC> {
        TDFER_W::new(self, 8)
    }
}
/**General PWM Timer Dead Time Control Register

You can [`read`](crate::Reg::read) this register and get [`gtdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDTCR_SPEC;
impl crate::RegisterSpec for GTDTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdtcr::R`](R) reader structure
impl crate::Readable for GTDTCR_SPEC {}
///`write(|w| ..)` method takes [`gtdtcr::W`](W) writer structure
impl crate::Writable for GTDTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDTCR to value 0
impl crate::Resettable for GTDTCR_SPEC {}
