///Register `SSIFCR` reader
pub type R = crate::R<SSIFCR_SPEC>;
///Register `SSIFCR` writer
pub type W = crate::W<SSIFCR_SPEC>;
/**Receive FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    ///0: Clears a receive data FIFO reset condition
    _0 = 0,
    ///1: Sets a receive data FIFO reset condition.
    _1 = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRST` reader - Receive FIFO Data Register Reset
pub type RFRST_R = crate::BitReader<RFRST_A>;
impl RFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::_0,
            true => RFRST_A::_1,
        }
    }
    ///Clears a receive data FIFO reset condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRST_A::_0
    }
    ///Sets a receive data FIFO reset condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRST_A::_1
    }
}
///Field `RFRST` writer - Receive FIFO Data Register Reset
pub type RFRST_W<'a, REG> = crate::BitWriter<'a, REG, RFRST_A>;
impl<'a, REG> RFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears a receive data FIFO reset condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_0)
    }
    ///Sets a receive data FIFO reset condition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_1)
    }
}
/**Transmit FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    ///0: Clears a transmit data FIFO reset condition
    _0 = 0,
    ///1: Sets a transmit data FIFO reset condition.
    _1 = 1,
}
impl From<TFRST_A> for bool {
    #[inline(always)]
    fn from(variant: TFRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFRST` reader - Transmit FIFO Data Register Reset
pub type TFRST_R = crate::BitReader<TFRST_A>;
impl TFRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFRST_A {
        match self.bits {
            false => TFRST_A::_0,
            true => TFRST_A::_1,
        }
    }
    ///Clears a transmit data FIFO reset condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFRST_A::_0
    }
    ///Sets a transmit data FIFO reset condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFRST_A::_1
    }
}
///Field `TFRST` writer - Transmit FIFO Data Register Reset
pub type TFRST_W<'a, REG> = crate::BitWriter<'a, REG, TFRST_A>;
impl<'a, REG> TFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears a transmit data FIFO reset condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_0)
    }
    ///Sets a transmit data FIFO reset condition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_1)
    }
}
/**Receive Data Full Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    ///0: Disables receive data full interrupts
    _0 = 0,
    ///1: Enables receive data full interrupts.
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Data Full Interrupt Output Enable
pub type RIE_R = crate::BitReader<RIE_A>;
impl RIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    ///Disables receive data full interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    ///Enables receive data full interrupts.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
///Field `RIE` writer - Receive Data Full Interrupt Output Enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG, RIE_A>;
impl<'a, REG> RIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables receive data full interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_0)
    }
    ///Enables receive data full interrupts.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RIE_A::_1)
    }
}
/**Transmit Data Empty Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    ///0: Disables transmit data empty interrupts
    _0 = 0,
    ///1: Enables transmit data empty interrupts.
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Data Empty Interrupt Output Enable
pub type TIE_R = crate::BitReader<TIE_A>;
impl TIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    ///Disables transmit data empty interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    ///Enables transmit data empty interrupts.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
///Field `TIE` writer - Transmit Data Empty Interrupt Output Enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmit data empty interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_0)
    }
    ///Enables transmit data empty interrupts.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::_1)
    }
}
/**Byte Swap Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSW_A {
    ///0: Disables byte swap
    _0 = 0,
    ///1: Enables byte swap
    _1 = 1,
}
impl From<BSW_A> for bool {
    #[inline(always)]
    fn from(variant: BSW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSW` reader - Byte Swap Enable
pub type BSW_R = crate::BitReader<BSW_A>;
impl BSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSW_A {
        match self.bits {
            false => BSW_A::_0,
            true => BSW_A::_1,
        }
    }
    ///Disables byte swap
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSW_A::_0
    }
    ///Enables byte swap
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSW_A::_1
    }
}
///Field `BSW` writer - Byte Swap Enable
pub type BSW_W<'a, REG> = crate::BitWriter<'a, REG, BSW_A>;
impl<'a, REG> BSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables byte swap
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSW_A::_0)
    }
    ///Enables byte swap
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSW_A::_1)
    }
}
/**Software Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIRST_A {
    ///0: Clears a software reset condition
    _0 = 0,
    ///1: Sets a software reset condition.
    _1 = 1,
}
impl From<SSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: SSIRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSIRST` reader - Software Reset
pub type SSIRST_R = crate::BitReader<SSIRST_A>;
impl SSIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSIRST_A {
        match self.bits {
            false => SSIRST_A::_0,
            true => SSIRST_A::_1,
        }
    }
    ///Clears a software reset condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIRST_A::_0
    }
    ///Sets a software reset condition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIRST_A::_1
    }
}
///Field `SSIRST` writer - Software Reset
pub type SSIRST_W<'a, REG> = crate::BitWriter<'a, REG, SSIRST_A>;
impl<'a, REG> SSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears a software reset condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSIRST_A::_0)
    }
    ///Sets a software reset condition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSIRST_A::_1)
    }
}
/**AUDIO_MCK Enable in Mastermode Communication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUCKE_A {
    ///0: Disables supply of AUDIO_MCK
    _0 = 0,
    ///1: Enables supply of AUDIO_MCK.
    _1 = 1,
}
impl From<AUCKE_A> for bool {
    #[inline(always)]
    fn from(variant: AUCKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUCKE` reader - AUDIO_MCK Enable in Mastermode Communication
pub type AUCKE_R = crate::BitReader<AUCKE_A>;
impl AUCKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUCKE_A {
        match self.bits {
            false => AUCKE_A::_0,
            true => AUCKE_A::_1,
        }
    }
    ///Disables supply of AUDIO_MCK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUCKE_A::_0
    }
    ///Enables supply of AUDIO_MCK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUCKE_A::_1
    }
}
///Field `AUCKE` writer - AUDIO_MCK Enable in Mastermode Communication
pub type AUCKE_W<'a, REG> = crate::BitWriter<'a, REG, AUCKE_A>;
impl<'a, REG> AUCKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables supply of AUDIO_MCK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AUCKE_A::_0)
    }
    ///Enables supply of AUDIO_MCK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AUCKE_A::_1)
    }
}
impl R {
    ///Bit 0 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive Data Full Interrupt Output Enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit Data Empty Interrupt Output Enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - Byte Swap Enable
    #[inline(always)]
    pub fn bsw(&self) -> BSW_R {
        BSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Software Reset
    #[inline(always)]
    pub fn ssirst(&self) -> SSIRST_R {
        SSIRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - AUDIO_MCK Enable in Mastermode Communication
    #[inline(always)]
    pub fn aucke(&self) -> AUCKE_R {
        AUCKE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W<SSIFCR_SPEC> {
        RFRST_W::new(self, 0)
    }
    ///Bit 1 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&mut self) -> TFRST_W<SSIFCR_SPEC> {
        TFRST_W::new(self, 1)
    }
    ///Bit 2 - Receive Data Full Interrupt Output Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<SSIFCR_SPEC> {
        RIE_W::new(self, 2)
    }
    ///Bit 3 - Transmit Data Empty Interrupt Output Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<SSIFCR_SPEC> {
        TIE_W::new(self, 3)
    }
    ///Bit 11 - Byte Swap Enable
    #[inline(always)]
    pub fn bsw(&mut self) -> BSW_W<SSIFCR_SPEC> {
        BSW_W::new(self, 11)
    }
    ///Bit 16 - Software Reset
    #[inline(always)]
    pub fn ssirst(&mut self) -> SSIRST_W<SSIFCR_SPEC> {
        SSIRST_W::new(self, 16)
    }
    ///Bit 31 - AUDIO_MCK Enable in Mastermode Communication
    #[inline(always)]
    pub fn aucke(&mut self) -> AUCKE_W<SSIFCR_SPEC> {
        AUCKE_W::new(self, 31)
    }
}
/**FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`ssifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSIFCR_SPEC;
impl crate::RegisterSpec for SSIFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssifcr::R`](R) reader structure
impl crate::Readable for SSIFCR_SPEC {}
///`write(|w| ..)` method takes [`ssifcr::W`](W) writer structure
impl crate::Writable for SSIFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIFCR to value 0
impl crate::Resettable for SSIFCR_SPEC {}
