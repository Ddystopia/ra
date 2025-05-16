///Register `ICSER` reader
pub type R = crate::R<ICSER_SPEC>;
///Register `ICSER` writer
pub type W = crate::W<ICSER_SPEC>;
/**Slave Address Register 0 Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAR0E_A {
    ///0: Slave address in SARL0 and SARU0 is disabled.
    _0 = 0,
    ///1: Slave address in SARL0 and SARU0 is enabled.
    _1 = 1,
}
impl From<SAR0E_A> for bool {
    #[inline(always)]
    fn from(variant: SAR0E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SAR0E` reader - Slave Address Register 0 Enable
pub type SAR0E_R = crate::BitReader<SAR0E_A>;
impl SAR0E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAR0E_A {
        match self.bits {
            false => SAR0E_A::_0,
            true => SAR0E_A::_1,
        }
    }
    ///Slave address in SARL0 and SARU0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAR0E_A::_0
    }
    ///Slave address in SARL0 and SARU0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAR0E_A::_1
    }
}
///Field `SAR0E` writer - Slave Address Register 0 Enable
pub type SAR0E_W<'a, REG> = crate::BitWriter<'a, REG, SAR0E_A>;
impl<'a, REG> SAR0E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address in SARL0 and SARU0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SAR0E_A::_0)
    }
    ///Slave address in SARL0 and SARU0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SAR0E_A::_1)
    }
}
/**Slave Address Register 1 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAR1E_A {
    ///0: Slave address in SARL1 and SARU1 is disabled.
    _0 = 0,
    ///1: Slave address in SARL1 and SARU1 is enabled.
    _1 = 1,
}
impl From<SAR1E_A> for bool {
    #[inline(always)]
    fn from(variant: SAR1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SAR1E` reader - Slave Address Register 1 Enable
pub type SAR1E_R = crate::BitReader<SAR1E_A>;
impl SAR1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAR1E_A {
        match self.bits {
            false => SAR1E_A::_0,
            true => SAR1E_A::_1,
        }
    }
    ///Slave address in SARL1 and SARU1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAR1E_A::_0
    }
    ///Slave address in SARL1 and SARU1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAR1E_A::_1
    }
}
///Field `SAR1E` writer - Slave Address Register 1 Enable
pub type SAR1E_W<'a, REG> = crate::BitWriter<'a, REG, SAR1E_A>;
impl<'a, REG> SAR1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address in SARL1 and SARU1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SAR1E_A::_0)
    }
    ///Slave address in SARL1 and SARU1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SAR1E_A::_1)
    }
}
/**Slave Address Register 2 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAR2E_A {
    ///0: Slave address in SARL2 and SARU2 is disabled.
    _0 = 0,
    ///1: Slave address in SARL2 and SARU2 is enabled
    _1 = 1,
}
impl From<SAR2E_A> for bool {
    #[inline(always)]
    fn from(variant: SAR2E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SAR2E` reader - Slave Address Register 2 Enable
pub type SAR2E_R = crate::BitReader<SAR2E_A>;
impl SAR2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAR2E_A {
        match self.bits {
            false => SAR2E_A::_0,
            true => SAR2E_A::_1,
        }
    }
    ///Slave address in SARL2 and SARU2 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAR2E_A::_0
    }
    ///Slave address in SARL2 and SARU2 is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAR2E_A::_1
    }
}
///Field `SAR2E` writer - Slave Address Register 2 Enable
pub type SAR2E_W<'a, REG> = crate::BitWriter<'a, REG, SAR2E_A>;
impl<'a, REG> SAR2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address in SARL2 and SARU2 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SAR2E_A::_0)
    }
    ///Slave address in SARL2 and SARU2 is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SAR2E_A::_1)
    }
}
/**General Call Address Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCAE_A {
    ///0: General call address detection is disabled.
    _0 = 0,
    ///1: General call address detection is enabled.
    _1 = 1,
}
impl From<GCAE_A> for bool {
    #[inline(always)]
    fn from(variant: GCAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GCAE` reader - General Call Address Enable
pub type GCAE_R = crate::BitReader<GCAE_A>;
impl GCAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCAE_A {
        match self.bits {
            false => GCAE_A::_0,
            true => GCAE_A::_1,
        }
    }
    ///General call address detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCAE_A::_0
    }
    ///General call address detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCAE_A::_1
    }
}
///Field `GCAE` writer - General Call Address Enable
pub type GCAE_W<'a, REG> = crate::BitWriter<'a, REG, GCAE_A>;
impl<'a, REG> GCAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///General call address detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GCAE_A::_0)
    }
    ///General call address detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GCAE_A::_1)
    }
}
/**Device-ID Address Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIDE_A {
    ///0: Device-ID address detection is disabled.
    _0 = 0,
    ///1: Device-ID address detection is enabled.
    _1 = 1,
}
impl From<DIDE_A> for bool {
    #[inline(always)]
    fn from(variant: DIDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIDE` reader - Device-ID Address Detection Enable
pub type DIDE_R = crate::BitReader<DIDE_A>;
impl DIDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIDE_A {
        match self.bits {
            false => DIDE_A::_0,
            true => DIDE_A::_1,
        }
    }
    ///Device-ID address detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIDE_A::_0
    }
    ///Device-ID address detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIDE_A::_1
    }
}
///Field `DIDE` writer - Device-ID Address Detection Enable
pub type DIDE_W<'a, REG> = crate::BitWriter<'a, REG, DIDE_A>;
impl<'a, REG> DIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Device-ID address detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIDE_A::_0)
    }
    ///Device-ID address detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIDE_A::_1)
    }
}
/**Host Address Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOAE_A {
    ///0: Host address detection is disabled.
    _0 = 0,
    ///1: Host address detection is enabled.
    _1 = 1,
}
impl From<HOAE_A> for bool {
    #[inline(always)]
    fn from(variant: HOAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HOAE` reader - Host Address Enable
pub type HOAE_R = crate::BitReader<HOAE_A>;
impl HOAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HOAE_A {
        match self.bits {
            false => HOAE_A::_0,
            true => HOAE_A::_1,
        }
    }
    ///Host address detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOAE_A::_0
    }
    ///Host address detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOAE_A::_1
    }
}
///Field `HOAE` writer - Host Address Enable
pub type HOAE_W<'a, REG> = crate::BitWriter<'a, REG, HOAE_A>;
impl<'a, REG> HOAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Host address detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HOAE_A::_0)
    }
    ///Host address detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HOAE_A::_1)
    }
}
impl R {
    ///Bit 0 - Slave Address Register 0 Enable
    #[inline(always)]
    pub fn sar0e(&self) -> SAR0E_R {
        SAR0E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Slave Address Register 1 Enable
    #[inline(always)]
    pub fn sar1e(&self) -> SAR1E_R {
        SAR1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Slave Address Register 2 Enable
    #[inline(always)]
    pub fn sar2e(&self) -> SAR2E_R {
        SAR2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - General Call Address Enable
    #[inline(always)]
    pub fn gcae(&self) -> GCAE_R {
        GCAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Device-ID Address Detection Enable
    #[inline(always)]
    pub fn dide(&self) -> DIDE_R {
        DIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Host Address Enable
    #[inline(always)]
    pub fn hoae(&self) -> HOAE_R {
        HOAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Slave Address Register 0 Enable
    #[inline(always)]
    pub fn sar0e(&mut self) -> SAR0E_W<ICSER_SPEC> {
        SAR0E_W::new(self, 0)
    }
    ///Bit 1 - Slave Address Register 1 Enable
    #[inline(always)]
    pub fn sar1e(&mut self) -> SAR1E_W<ICSER_SPEC> {
        SAR1E_W::new(self, 1)
    }
    ///Bit 2 - Slave Address Register 2 Enable
    #[inline(always)]
    pub fn sar2e(&mut self) -> SAR2E_W<ICSER_SPEC> {
        SAR2E_W::new(self, 2)
    }
    ///Bit 3 - General Call Address Enable
    #[inline(always)]
    pub fn gcae(&mut self) -> GCAE_W<ICSER_SPEC> {
        GCAE_W::new(self, 3)
    }
    ///Bit 5 - Device-ID Address Detection Enable
    #[inline(always)]
    pub fn dide(&mut self) -> DIDE_W<ICSER_SPEC> {
        DIDE_W::new(self, 5)
    }
    ///Bit 7 - Host Address Enable
    #[inline(always)]
    pub fn hoae(&mut self) -> HOAE_W<ICSER_SPEC> {
        HOAE_W::new(self, 7)
    }
}
/**I2C Bus Status Enable Register

You can [`read`](crate::Reg::read) this register and get [`icser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICSER_SPEC;
impl crate::RegisterSpec for ICSER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icser::R`](R) reader structure
impl crate::Readable for ICSER_SPEC {}
///`write(|w| ..)` method takes [`icser::W`](W) writer structure
impl crate::Writable for ICSER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSER to value 0x09
impl crate::Resettable for ICSER_SPEC {
    const RESET_VALUE: u8 = 0x09;
}
