///Register `MCTL_TX[%s]` reader
pub type R = crate::R<MCTL_TX_SPEC>;
///Register `MCTL_TX[%s]` writer
pub type W = crate::W<MCTL_TX_SPEC>;
/**Transmission Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENTDATA_A {
    ///0: Transmission is not completed
    _0 = 0,
    ///1: Transmission is completed
    _1 = 1,
}
impl From<SENTDATA_A> for bool {
    #[inline(always)]
    fn from(variant: SENTDATA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SENTDATA` reader - Transmission Complete Flag
pub type SENTDATA_R = crate::BitReader<SENTDATA_A>;
impl SENTDATA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SENTDATA_A {
        match self.bits {
            false => SENTDATA_A::_0,
            true => SENTDATA_A::_1,
        }
    }
    ///Transmission is not completed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SENTDATA_A::_0
    }
    ///Transmission is completed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SENTDATA_A::_1
    }
}
///Field `SENTDATA` writer - Transmission Complete Flag
pub type SENTDATA_W<'a, REG> = crate::BitWriter<'a, REG, SENTDATA_A>;
impl<'a, REG> SENTDATA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission is not completed
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SENTDATA_A::_0)
    }
    ///Transmission is completed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SENTDATA_A::_1)
    }
}
/**Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMACTIVE_A {
    ///0: Transmission is pending or transmission is not requested
    _0 = 0,
    ///1: From acceptance of transmission request to completion of transmission, or error/arbitration-lost
    _1 = 1,
}
impl From<TRMACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: TRMACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRMACTIVE` reader - Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)
pub type TRMACTIVE_R = crate::BitReader<TRMACTIVE_A>;
impl TRMACTIVE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRMACTIVE_A {
        match self.bits {
            false => TRMACTIVE_A::_0,
            true => TRMACTIVE_A::_1,
        }
    }
    ///Transmission is pending or transmission is not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMACTIVE_A::_0
    }
    ///From acceptance of transmission request to completion of transmission, or error/arbitration-lost
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMACTIVE_A::_1
    }
}
/**Transmission Abort Complete Flag (Transmit mailbox setting enabled)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMABT_A {
    ///0: Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested
    _0 = 0,
    ///1: Transmission abort is completed
    _1 = 1,
}
impl From<TRMABT_A> for bool {
    #[inline(always)]
    fn from(variant: TRMABT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRMABT` reader - Transmission Abort Complete Flag (Transmit mailbox setting enabled)
pub type TRMABT_R = crate::BitReader<TRMABT_A>;
impl TRMABT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRMABT_A {
        match self.bits {
            false => TRMABT_A::_0,
            true => TRMABT_A::_1,
        }
    }
    ///Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMABT_A::_0
    }
    ///Transmission abort is completed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMABT_A::_1
    }
}
///Field `TRMABT` writer - Transmission Abort Complete Flag (Transmit mailbox setting enabled)
pub type TRMABT_W<'a, REG> = crate::BitWriter<'a, REG, TRMABT_A>;
impl<'a, REG> TRMABT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRMABT_A::_0)
    }
    ///Transmission abort is completed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRMABT_A::_1)
    }
}
/**One-Shot Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONESHOT_A {
    ///0: One-shot reception or one-shot transmission disabled
    _0 = 0,
    ///1: One-shot reception or one-shot transmission enabled
    _1 = 1,
}
impl From<ONESHOT_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ONESHOT` reader - One-Shot Enable
pub type ONESHOT_R = crate::BitReader<ONESHOT_A>;
impl ONESHOT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ONESHOT_A {
        match self.bits {
            false => ONESHOT_A::_0,
            true => ONESHOT_A::_1,
        }
    }
    ///One-shot reception or one-shot transmission disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONESHOT_A::_0
    }
    ///One-shot reception or one-shot transmission enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONESHOT_A::_1
    }
}
///Field `ONESHOT` writer - One-Shot Enable
pub type ONESHOT_W<'a, REG> = crate::BitWriter<'a, REG, ONESHOT_A>;
impl<'a, REG> ONESHOT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One-shot reception or one-shot transmission disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ONESHOT_A::_0)
    }
    ///One-shot reception or one-shot transmission enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ONESHOT_A::_1)
    }
}
/**Receive Mailbox Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECREQ_A {
    ///0: Not configured for reception
    _0 = 0,
    ///1: Configured for reception
    _1 = 1,
}
impl From<RECREQ_A> for bool {
    #[inline(always)]
    fn from(variant: RECREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RECREQ` reader - Receive Mailbox Request
pub type RECREQ_R = crate::BitReader<RECREQ_A>;
impl RECREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RECREQ_A {
        match self.bits {
            false => RECREQ_A::_0,
            true => RECREQ_A::_1,
        }
    }
    ///Not configured for reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECREQ_A::_0
    }
    ///Configured for reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECREQ_A::_1
    }
}
///Field `RECREQ` writer - Receive Mailbox Request
pub type RECREQ_W<'a, REG> = crate::BitWriter<'a, REG, RECREQ_A>;
impl<'a, REG> RECREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not configured for reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RECREQ_A::_0)
    }
    ///Configured for reception
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RECREQ_A::_1)
    }
}
/**Transmit Mailbox Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRMREQ_A {
    ///0: Not configured for transmission
    _0 = 0,
    ///1: Configured for transmission
    _1 = 1,
}
impl From<TRMREQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRMREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRMREQ` reader - Transmit Mailbox Request
pub type TRMREQ_R = crate::BitReader<TRMREQ_A>;
impl TRMREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRMREQ_A {
        match self.bits {
            false => TRMREQ_A::_0,
            true => TRMREQ_A::_1,
        }
    }
    ///Not configured for transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRMREQ_A::_0
    }
    ///Configured for transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRMREQ_A::_1
    }
}
///Field `TRMREQ` writer - Transmit Mailbox Request
pub type TRMREQ_W<'a, REG> = crate::BitWriter<'a, REG, TRMREQ_A>;
impl<'a, REG> TRMREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not configured for transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRMREQ_A::_0)
    }
    ///Configured for transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRMREQ_A::_1)
    }
}
impl R {
    ///Bit 0 - Transmission Complete Flag
    #[inline(always)]
    pub fn sentdata(&self) -> SENTDATA_R {
        SENTDATA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)
    #[inline(always)]
    pub fn trmactive(&self) -> TRMACTIVE_R {
        TRMACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmission Abort Complete Flag (Transmit mailbox setting enabled)
    #[inline(always)]
    pub fn trmabt(&self) -> TRMABT_R {
        TRMABT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - One-Shot Enable
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Receive Mailbox Request
    #[inline(always)]
    pub fn recreq(&self) -> RECREQ_R {
        RECREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Mailbox Request
    #[inline(always)]
    pub fn trmreq(&self) -> TRMREQ_R {
        TRMREQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmission Complete Flag
    #[inline(always)]
    pub fn sentdata(&mut self) -> SENTDATA_W<MCTL_TX_SPEC> {
        SENTDATA_W::new(self, 0)
    }
    ///Bit 2 - Transmission Abort Complete Flag (Transmit mailbox setting enabled)
    #[inline(always)]
    pub fn trmabt(&mut self) -> TRMABT_W<MCTL_TX_SPEC> {
        TRMABT_W::new(self, 2)
    }
    ///Bit 4 - One-Shot Enable
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W<MCTL_TX_SPEC> {
        ONESHOT_W::new(self, 4)
    }
    ///Bit 6 - Receive Mailbox Request
    #[inline(always)]
    pub fn recreq(&mut self) -> RECREQ_W<MCTL_TX_SPEC> {
        RECREQ_W::new(self, 6)
    }
    ///Bit 7 - Transmit Mailbox Request
    #[inline(always)]
    pub fn trmreq(&mut self) -> TRMREQ_W<MCTL_TX_SPEC> {
        TRMREQ_W::new(self, 7)
    }
}
/**Message Control Register for Transmit

You can [`read`](crate::Reg::read) this register and get [`mctl_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MCTL_TX_SPEC;
impl crate::RegisterSpec for MCTL_TX_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mctl_tx::R`](R) reader structure
impl crate::Readable for MCTL_TX_SPEC {}
///`write(|w| ..)` method takes [`mctl_tx::W`](W) writer structure
impl crate::Writable for MCTL_TX_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCTL_TX[%s] to value 0
impl crate::Resettable for MCTL_TX_SPEC {}
