///Register `MCTL_RX[%s]` reader
pub type R = crate::R<MCTL_RX_SPEC>;
///Register `MCTL_RX[%s]` writer
pub type W = crate::W<MCTL_RX_SPEC>;
/**Reception Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEWDATA_A {
    ///0: No data has been received or 0 is written to the NEWDATA bit
    _0 = 0,
    ///1: A new message is being stored or has been stored to the mailbox
    _1 = 1,
}
impl From<NEWDATA_A> for bool {
    #[inline(always)]
    fn from(variant: NEWDATA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NEWDATA` reader - Reception Complete Flag
pub type NEWDATA_R = crate::BitReader<NEWDATA_A>;
impl NEWDATA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NEWDATA_A {
        match self.bits {
            false => NEWDATA_A::_0,
            true => NEWDATA_A::_1,
        }
    }
    ///No data has been received or 0 is written to the NEWDATA bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NEWDATA_A::_0
    }
    ///A new message is being stored or has been stored to the mailbox
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NEWDATA_A::_1
    }
}
///Field `NEWDATA` writer - Reception Complete Flag
pub type NEWDATA_W<'a, REG> = crate::BitWriter<'a, REG, NEWDATA_A>;
impl<'a, REG> NEWDATA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No data has been received or 0 is written to the NEWDATA bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NEWDATA_A::_0)
    }
    ///A new message is being stored or has been stored to the mailbox
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NEWDATA_A::_1)
    }
}
/**Reception-in-Progress Status Flag (Receive mailbox setting enabled)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVALDATA_A {
    ///0: Message valid
    _0 = 0,
    ///1: Message being updated
    _1 = 1,
}
impl From<INVALDATA_A> for bool {
    #[inline(always)]
    fn from(variant: INVALDATA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INVALDATA` reader - Reception-in-Progress Status Flag (Receive mailbox setting enabled)
pub type INVALDATA_R = crate::BitReader<INVALDATA_A>;
impl INVALDATA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INVALDATA_A {
        match self.bits {
            false => INVALDATA_A::_0,
            true => INVALDATA_A::_1,
        }
    }
    ///Message valid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVALDATA_A::_0
    }
    ///Message being updated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVALDATA_A::_1
    }
}
/**Message Lost Flag(Receive mailbox setting enabled)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSGLOST_A {
    ///0: Message is not overwritten or overrun
    _0 = 0,
    ///1: Message is overwritten or overrun
    _1 = 1,
}
impl From<MSGLOST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGLOST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSGLOST` reader - Message Lost Flag(Receive mailbox setting enabled)
pub type MSGLOST_R = crate::BitReader<MSGLOST_A>;
impl MSGLOST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSGLOST_A {
        match self.bits {
            false => MSGLOST_A::_0,
            true => MSGLOST_A::_1,
        }
    }
    ///Message is not overwritten or overrun
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSGLOST_A::_0
    }
    ///Message is overwritten or overrun
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSGLOST_A::_1
    }
}
///Field `MSGLOST` writer - Message Lost Flag(Receive mailbox setting enabled)
pub type MSGLOST_W<'a, REG> = crate::BitWriter<'a, REG, MSGLOST_A>;
impl<'a, REG> MSGLOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Message is not overwritten or overrun
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSGLOST_A::_0)
    }
    ///Message is overwritten or overrun
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSGLOST_A::_1)
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
    ///Bit 0 - Reception Complete Flag
    #[inline(always)]
    pub fn newdata(&self) -> NEWDATA_R {
        NEWDATA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reception-in-Progress Status Flag (Receive mailbox setting enabled)
    #[inline(always)]
    pub fn invaldata(&self) -> INVALDATA_R {
        INVALDATA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Message Lost Flag(Receive mailbox setting enabled)
    #[inline(always)]
    pub fn msglost(&self) -> MSGLOST_R {
        MSGLOST_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 0 - Reception Complete Flag
    #[inline(always)]
    pub fn newdata(&mut self) -> NEWDATA_W<MCTL_RX_SPEC> {
        NEWDATA_W::new(self, 0)
    }
    ///Bit 2 - Message Lost Flag(Receive mailbox setting enabled)
    #[inline(always)]
    pub fn msglost(&mut self) -> MSGLOST_W<MCTL_RX_SPEC> {
        MSGLOST_W::new(self, 2)
    }
    ///Bit 4 - One-Shot Enable
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W<MCTL_RX_SPEC> {
        ONESHOT_W::new(self, 4)
    }
    ///Bit 6 - Receive Mailbox Request
    #[inline(always)]
    pub fn recreq(&mut self) -> RECREQ_W<MCTL_RX_SPEC> {
        RECREQ_W::new(self, 6)
    }
    ///Bit 7 - Transmit Mailbox Request
    #[inline(always)]
    pub fn trmreq(&mut self) -> TRMREQ_W<MCTL_RX_SPEC> {
        TRMREQ_W::new(self, 7)
    }
}
/**Message Control Register for Receive

You can [`read`](crate::Reg::read) this register and get [`mctl_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MCTL_RX_SPEC;
impl crate::RegisterSpec for MCTL_RX_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mctl_rx::R`](R) reader structure
impl crate::Readable for MCTL_RX_SPEC {}
///`write(|w| ..)` method takes [`mctl_rx::W`](W) writer structure
impl crate::Writable for MCTL_RX_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCTL_RX[%s] to value 0
impl crate::Resettable for MCTL_RX_SPEC {}
