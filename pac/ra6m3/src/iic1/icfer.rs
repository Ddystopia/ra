///Register `ICFER` reader
pub type R = crate::R<ICFER_SPEC>;
///Register `ICFER` writer
pub type W = crate::W<ICFER_SPEC>;
/**Timeout Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOE_A {
    ///0: The timeout function is disabled.
    _0 = 0,
    ///1: The timeout function is enabled.
    _1 = 1,
}
impl From<TMOE_A> for bool {
    #[inline(always)]
    fn from(variant: TMOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TMOE` reader - Timeout Function Enable
pub type TMOE_R = crate::BitReader<TMOE_A>;
impl TMOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TMOE_A {
        match self.bits {
            false => TMOE_A::_0,
            true => TMOE_A::_1,
        }
    }
    ///The timeout function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOE_A::_0
    }
    ///The timeout function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOE_A::_1
    }
}
///Field `TMOE` writer - Timeout Function Enable
pub type TMOE_W<'a, REG> = crate::BitWriter<'a, REG, TMOE_A>;
impl<'a, REG> TMOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The timeout function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOE_A::_0)
    }
    ///The timeout function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOE_A::_1)
    }
}
/**Master Arbitration-Lost Detection Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MALE_A {
    ///0: Master arbitration-lost detection is disabled.
    _0 = 0,
    ///1: Master arbitration-lost detection is enabled.
    _1 = 1,
}
impl From<MALE_A> for bool {
    #[inline(always)]
    fn from(variant: MALE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MALE` reader - Master Arbitration-Lost Detection Enable
pub type MALE_R = crate::BitReader<MALE_A>;
impl MALE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MALE_A {
        match self.bits {
            false => MALE_A::_0,
            true => MALE_A::_1,
        }
    }
    ///Master arbitration-lost detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MALE_A::_0
    }
    ///Master arbitration-lost detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MALE_A::_1
    }
}
///Field `MALE` writer - Master Arbitration-Lost Detection Enable
pub type MALE_W<'a, REG> = crate::BitWriter<'a, REG, MALE_A>;
impl<'a, REG> MALE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master arbitration-lost detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MALE_A::_0)
    }
    ///Master arbitration-lost detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MALE_A::_1)
    }
}
/**NACK Transmission Arbitration-Lost Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NALE_A {
    ///0: NACK transmission arbitration-lost detection is disabled.
    _0 = 0,
    ///1: NACK transmission arbitration-lost detection is enabled.
    _1 = 1,
}
impl From<NALE_A> for bool {
    #[inline(always)]
    fn from(variant: NALE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NALE` reader - NACK Transmission Arbitration-Lost Detection Enable
pub type NALE_R = crate::BitReader<NALE_A>;
impl NALE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NALE_A {
        match self.bits {
            false => NALE_A::_0,
            true => NALE_A::_1,
        }
    }
    ///NACK transmission arbitration-lost detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NALE_A::_0
    }
    ///NACK transmission arbitration-lost detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NALE_A::_1
    }
}
///Field `NALE` writer - NACK Transmission Arbitration-Lost Detection Enable
pub type NALE_W<'a, REG> = crate::BitWriter<'a, REG, NALE_A>;
impl<'a, REG> NALE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NACK transmission arbitration-lost detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NALE_A::_0)
    }
    ///NACK transmission arbitration-lost detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NALE_A::_1)
    }
}
/**Slave Arbitration-Lost Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SALE_A {
    ///0: Slave arbitration-lost detection is disabled.
    _0 = 0,
    ///1: Slave arbitration-lost detection is enabled.
    _1 = 1,
}
impl From<SALE_A> for bool {
    #[inline(always)]
    fn from(variant: SALE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SALE` reader - Slave Arbitration-Lost Detection Enable
pub type SALE_R = crate::BitReader<SALE_A>;
impl SALE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SALE_A {
        match self.bits {
            false => SALE_A::_0,
            true => SALE_A::_1,
        }
    }
    ///Slave arbitration-lost detection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SALE_A::_0
    }
    ///Slave arbitration-lost detection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SALE_A::_1
    }
}
///Field `SALE` writer - Slave Arbitration-Lost Detection Enable
pub type SALE_W<'a, REG> = crate::BitWriter<'a, REG, SALE_A>;
impl<'a, REG> SALE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave arbitration-lost detection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SALE_A::_0)
    }
    ///Slave arbitration-lost detection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SALE_A::_1)
    }
}
/**NACK Reception Transfer Suspension Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKE_A {
    ///0: Transfer operation is not suspended during NACK reception (transfer suspension disabled).
    _0 = 0,
    ///1: Transfer operation is suspended during NACK reception (transfer suspension enabled).
    _1 = 1,
}
impl From<NACKE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKE` reader - NACK Reception Transfer Suspension Enable
pub type NACKE_R = crate::BitReader<NACKE_A>;
impl NACKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACKE_A {
        match self.bits {
            false => NACKE_A::_0,
            true => NACKE_A::_1,
        }
    }
    ///Transfer operation is not suspended during NACK reception (transfer suspension disabled).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKE_A::_0
    }
    ///Transfer operation is suspended during NACK reception (transfer suspension enabled).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKE_A::_1
    }
}
///Field `NACKE` writer - NACK Reception Transfer Suspension Enable
pub type NACKE_W<'a, REG> = crate::BitWriter<'a, REG, NACKE_A>;
impl<'a, REG> NACKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer operation is not suspended during NACK reception (transfer suspension disabled).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NACKE_A::_0)
    }
    ///Transfer operation is suspended during NACK reception (transfer suspension enabled).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKE_A::_1)
    }
}
/**Digital Noise Filter Circuit Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFE_A {
    ///0: No digital noise filter circuit is used.
    _0 = 0,
    ///1: A digital noise filter circuit is used.
    _1 = 1,
}
impl From<NFE_A> for bool {
    #[inline(always)]
    fn from(variant: NFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFE` reader - Digital Noise Filter Circuit Enable
pub type NFE_R = crate::BitReader<NFE_A>;
impl NFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFE_A {
        match self.bits {
            false => NFE_A::_0,
            true => NFE_A::_1,
        }
    }
    ///No digital noise filter circuit is used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFE_A::_0
    }
    ///A digital noise filter circuit is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFE_A::_1
    }
}
///Field `NFE` writer - Digital Noise Filter Circuit Enable
pub type NFE_W<'a, REG> = crate::BitWriter<'a, REG, NFE_A>;
impl<'a, REG> NFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No digital noise filter circuit is used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFE_A::_0)
    }
    ///A digital noise filter circuit is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFE_A::_1)
    }
}
/**SCL Synchronous Circuit Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLE_A {
    ///0: No SCL synchronous circuit is used.
    _0 = 0,
    ///1: An SCL synchronous circuit is used.
    _1 = 1,
}
impl From<SCLE_A> for bool {
    #[inline(always)]
    fn from(variant: SCLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCLE` reader - SCL Synchronous Circuit Enable
pub type SCLE_R = crate::BitReader<SCLE_A>;
impl SCLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCLE_A {
        match self.bits {
            false => SCLE_A::_0,
            true => SCLE_A::_1,
        }
    }
    ///No SCL synchronous circuit is used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLE_A::_0
    }
    ///An SCL synchronous circuit is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLE_A::_1
    }
}
///Field `SCLE` writer - SCL Synchronous Circuit Enable
pub type SCLE_W<'a, REG> = crate::BitWriter<'a, REG, SCLE_A>;
impl<'a, REG> SCLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SCL synchronous circuit is used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCLE_A::_0)
    }
    ///An SCL synchronous circuit is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCLE_A::_1)
    }
}
/**Fast-mode Plus Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPE_A {
    ///0: No Fm+ slope control circuit is used for the SCLn pin and SDAn pin.
    _0 = 0,
    ///1: An Fm+ slope control circuit is used for the SCLn pin and SDAn pin.
    _1 = 1,
}
impl From<FMPE_A> for bool {
    #[inline(always)]
    fn from(variant: FMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPE` reader - Fast-mode Plus Enable
pub type FMPE_R = crate::BitReader<FMPE_A>;
impl FMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPE_A {
        match self.bits {
            false => FMPE_A::_0,
            true => FMPE_A::_1,
        }
    }
    ///No Fm+ slope control circuit is used for the SCLn pin and SDAn pin.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FMPE_A::_0
    }
    ///An Fm+ slope control circuit is used for the SCLn pin and SDAn pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FMPE_A::_1
    }
}
///Field `FMPE` writer - Fast-mode Plus Enable
pub type FMPE_W<'a, REG> = crate::BitWriter<'a, REG, FMPE_A>;
impl<'a, REG> FMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Fm+ slope control circuit is used for the SCLn pin and SDAn pin.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FMPE_A::_0)
    }
    ///An Fm+ slope control circuit is used for the SCLn pin and SDAn pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FMPE_A::_1)
    }
}
impl R {
    ///Bit 0 - Timeout Function Enable
    #[inline(always)]
    pub fn tmoe(&self) -> TMOE_R {
        TMOE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Master Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn male(&self) -> MALE_R {
        MALE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NACK Transmission Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn nale(&self) -> NALE_R {
        NALE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Slave Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn sale(&self) -> SALE_R {
        SALE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACK Reception Transfer Suspension Enable
    #[inline(always)]
    pub fn nacke(&self) -> NACKE_R {
        NACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Digital Noise Filter Circuit Enable
    #[inline(always)]
    pub fn nfe(&self) -> NFE_R {
        NFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SCL Synchronous Circuit Enable
    #[inline(always)]
    pub fn scle(&self) -> SCLE_R {
        SCLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Fast-mode Plus Enable
    #[inline(always)]
    pub fn fmpe(&self) -> FMPE_R {
        FMPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timeout Function Enable
    #[inline(always)]
    pub fn tmoe(&mut self) -> TMOE_W<ICFER_SPEC> {
        TMOE_W::new(self, 0)
    }
    ///Bit 1 - Master Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn male(&mut self) -> MALE_W<ICFER_SPEC> {
        MALE_W::new(self, 1)
    }
    ///Bit 2 - NACK Transmission Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn nale(&mut self) -> NALE_W<ICFER_SPEC> {
        NALE_W::new(self, 2)
    }
    ///Bit 3 - Slave Arbitration-Lost Detection Enable
    #[inline(always)]
    pub fn sale(&mut self) -> SALE_W<ICFER_SPEC> {
        SALE_W::new(self, 3)
    }
    ///Bit 4 - NACK Reception Transfer Suspension Enable
    #[inline(always)]
    pub fn nacke(&mut self) -> NACKE_W<ICFER_SPEC> {
        NACKE_W::new(self, 4)
    }
    ///Bit 5 - Digital Noise Filter Circuit Enable
    #[inline(always)]
    pub fn nfe(&mut self) -> NFE_W<ICFER_SPEC> {
        NFE_W::new(self, 5)
    }
    ///Bit 6 - SCL Synchronous Circuit Enable
    #[inline(always)]
    pub fn scle(&mut self) -> SCLE_W<ICFER_SPEC> {
        SCLE_W::new(self, 6)
    }
    ///Bit 7 - Fast-mode Plus Enable
    #[inline(always)]
    pub fn fmpe(&mut self) -> FMPE_W<ICFER_SPEC> {
        FMPE_W::new(self, 7)
    }
}
/**I2C Bus Function Enable Register

You can [`read`](crate::Reg::read) this register and get [`icfer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICFER_SPEC;
impl crate::RegisterSpec for ICFER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icfer::R`](R) reader structure
impl crate::Readable for ICFER_SPEC {}
///`write(|w| ..)` method takes [`icfer::W`](W) writer structure
impl crate::Writable for ICFER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICFER to value 0x72
impl crate::Resettable for ICFER_SPEC {
    const RESET_VALUE: u8 = 0x72;
}
