///Register `ADGSPCR` reader
pub type R = crate::R<ADGSPCR_SPEC>;
///Register `ADGSPCR` writer
pub type W = crate::W<ADGSPCR_SPEC>;
/**Group A priority control setting bit.Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\[1:0\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGS_A {
    ///0: Operation is without group A priority control
    _0 = 0,
    ///1: Operation is with group A priority control
    _1 = 1,
}
impl From<PGS_A> for bool {
    #[inline(always)]
    fn from(variant: PGS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PGS` reader - Group A priority control setting bit.Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\[1:0\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.
pub type PGS_R = crate::BitReader<PGS_A>;
impl PGS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGS_A {
        match self.bits {
            false => PGS_A::_0,
            true => PGS_A::_1,
        }
    }
    ///Operation is without group A priority control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGS_A::_0
    }
    ///Operation is with group A priority control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGS_A::_1
    }
}
///Field `PGS` writer - Group A priority control setting bit.Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\[1:0\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.
pub type PGS_W<'a, REG> = crate::BitWriter<'a, REG, PGS_A>;
impl<'a, REG> PGS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation is without group A priority control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PGS_A::_0)
    }
    ///Operation is with group A priority control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PGS_A::_1)
    }
}
/**Group B Restart Setting(Enabled only when PGS = 1. Reserved when PGS = 0.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GBRSCN_A {
    ///0: Scanning for group B is not restarted after having been discontinued due to group A priority control.
    _0 = 0,
    ///1: Scanning for group B is restarted after having been discontinued due to group A priority control.
    _1 = 1,
}
impl From<GBRSCN_A> for bool {
    #[inline(always)]
    fn from(variant: GBRSCN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GBRSCN` reader - Group B Restart Setting(Enabled only when PGS = 1. Reserved when PGS = 0.)
pub type GBRSCN_R = crate::BitReader<GBRSCN_A>;
impl GBRSCN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GBRSCN_A {
        match self.bits {
            false => GBRSCN_A::_0,
            true => GBRSCN_A::_1,
        }
    }
    ///Scanning for group B is not restarted after having been discontinued due to group A priority control.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBRSCN_A::_0
    }
    ///Scanning for group B is restarted after having been discontinued due to group A priority control.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBRSCN_A::_1
    }
}
///Field `GBRSCN` writer - Group B Restart Setting(Enabled only when PGS = 1. Reserved when PGS = 0.)
pub type GBRSCN_W<'a, REG> = crate::BitWriter<'a, REG, GBRSCN_A>;
impl<'a, REG> GBRSCN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Scanning for group B is not restarted after having been discontinued due to group A priority control.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GBRSCN_A::_0)
    }
    ///Scanning for group B is restarted after having been discontinued due to group A priority control.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GBRSCN_A::_1)
    }
}
/**Group B Single Scan Continuous Start(Enabled only when PGS = 1. Reserved when PGS = 0.)Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GBRP_A {
    ///0: Single scan for group B is not continuously activated.
    _0 = 0,
    ///1: Single scan for group B is continuously activated.
    _1 = 1,
}
impl From<GBRP_A> for bool {
    #[inline(always)]
    fn from(variant: GBRP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GBRP` reader - Group B Single Scan Continuous Start(Enabled only when PGS = 1. Reserved when PGS = 0.)Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.
pub type GBRP_R = crate::BitReader<GBRP_A>;
impl GBRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GBRP_A {
        match self.bits {
            false => GBRP_A::_0,
            true => GBRP_A::_1,
        }
    }
    ///Single scan for group B is not continuously activated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBRP_A::_0
    }
    ///Single scan for group B is continuously activated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBRP_A::_1
    }
}
///Field `GBRP` writer - Group B Single Scan Continuous Start(Enabled only when PGS = 1. Reserved when PGS = 0.)Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.
pub type GBRP_W<'a, REG> = crate::BitWriter<'a, REG, GBRP_A>;
impl<'a, REG> GBRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single scan for group B is not continuously activated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GBRP_A::_0)
    }
    ///Single scan for group B is continuously activated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GBRP_A::_1)
    }
}
impl R {
    ///Bit 0 - Group A priority control setting bit.Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\[1:0\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.
    #[inline(always)]
    pub fn pgs(&self) -> PGS_R {
        PGS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Group B Restart Setting(Enabled only when PGS = 1. Reserved when PGS = 0.)
    #[inline(always)]
    pub fn gbrscn(&self) -> GBRSCN_R {
        GBRSCN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 15 - Group B Single Scan Continuous Start(Enabled only when PGS = 1. Reserved when PGS = 0.)Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.
    #[inline(always)]
    pub fn gbrp(&self) -> GBRP_R {
        GBRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Group A priority control setting bit.Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\[1:0\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed.
    #[inline(always)]
    pub fn pgs(&mut self) -> PGS_W<ADGSPCR_SPEC> {
        PGS_W::new(self, 0)
    }
    ///Bit 1 - Group B Restart Setting(Enabled only when PGS = 1. Reserved when PGS = 0.)
    #[inline(always)]
    pub fn gbrscn(&mut self) -> GBRSCN_W<ADGSPCR_SPEC> {
        GBRSCN_W::new(self, 1)
    }
    ///Bit 15 - Group B Single Scan Continuous Start(Enabled only when PGS = 1. Reserved when PGS = 0.)Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit.
    #[inline(always)]
    pub fn gbrp(&mut self) -> GBRP_W<ADGSPCR_SPEC> {
        GBRP_W::new(self, 15)
    }
}
/**A/D Group Scan Priority Control Register

You can [`read`](crate::Reg::read) this register and get [`adgspcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgspcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADGSPCR_SPEC;
impl crate::RegisterSpec for ADGSPCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adgspcr::R`](R) reader structure
impl crate::Readable for ADGSPCR_SPEC {}
///`write(|w| ..)` method takes [`adgspcr::W`](W) writer structure
impl crate::Writable for ADGSPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADGSPCR to value 0
impl crate::Resettable for ADGSPCR_SPEC {}
