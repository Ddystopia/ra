///Register `FCR` reader
pub type R = crate::R<FCR_SPEC>;
///Register `FCR` writer
pub type W = crate::W<FCR_SPEC>;
/**FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FM_A {
    ///0: Non-FIFO mode
    _0 = 0,
    ///1: FIFO mode
    _1 = 1,
}
impl From<FM_A> for bool {
    #[inline(always)]
    fn from(variant: FM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FM` reader - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
pub type FM_R = crate::BitReader<FM_A>;
impl FM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FM_A {
        match self.bits {
            false => FM_A::_0,
            true => FM_A::_1,
        }
    }
    ///Non-FIFO mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FM_A::_0
    }
    ///FIFO mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FM_A::_1
    }
}
///Field `FM` writer - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
pub type FM_W<'a, REG> = crate::BitWriter<'a, REG, FM_A>;
impl<'a, REG> FM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-FIFO mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FM_A::_0)
    }
    ///FIFO mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FM_A::_1)
    }
}
/**Receive FIFO Data Register Reset(Valid only in FCR.FM=1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    ///0: The number of data stored in FRDRH and FRDRL register are NOT made 0
    _0 = 0,
    ///1: The number of data stored in FRDRH and FRDRL register are made 0
    _1 = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRST` reader - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)
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
    ///The number of data stored in FRDRH and FRDRL register are NOT made 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRST_A::_0
    }
    ///The number of data stored in FRDRH and FRDRL register are made 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRST_A::_1
    }
}
///Field `RFRST` writer - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)
pub type RFRST_W<'a, REG> = crate::BitWriter<'a, REG, RFRST_A>;
impl<'a, REG> RFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The number of data stored in FRDRH and FRDRL register are NOT made 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_0)
    }
    ///The number of data stored in FRDRH and FRDRL register are made 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_1)
    }
}
/**Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    ///0: The number of data stored in FTDRH and FTDRL register are NOT made 0
    _0 = 0,
    ///1: The number of data stored in FTDRH and FTDRL register are made 0
    _1 = 1,
}
impl From<TFRST_A> for bool {
    #[inline(always)]
    fn from(variant: TFRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFRST` reader - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)
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
    ///The number of data stored in FTDRH and FTDRL register are NOT made 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFRST_A::_0
    }
    ///The number of data stored in FTDRH and FTDRL register are made 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFRST_A::_1
    }
}
///Field `TFRST` writer - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)
pub type TFRST_W<'a, REG> = crate::BitWriter<'a, REG, TFRST_A>;
impl<'a, REG> TFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The number of data stored in FTDRH and FTDRL register are NOT made 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_0)
    }
    ///The number of data stored in FTDRH and FTDRL register are made 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_1)
    }
}
/**Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRES_A {
    ///0: reception data full interrupt (RXI)
    _0 = 0,
    ///1: receive error interrupt (ERI)
    _1 = 1,
}
impl From<DRES_A> for bool {
    #[inline(always)]
    fn from(variant: DRES_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRES` reader - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)
pub type DRES_R = crate::BitReader<DRES_A>;
impl DRES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRES_A {
        match self.bits {
            false => DRES_A::_0,
            true => DRES_A::_1,
        }
    }
    ///reception data full interrupt (RXI)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRES_A::_0
    }
    ///receive error interrupt (ERI)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRES_A::_1
    }
}
///Field `DRES` writer - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)
pub type DRES_W<'a, REG> = crate::BitWriter<'a, REG, DRES_A>;
impl<'a, REG> DRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///reception data full interrupt (RXI)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::_0)
    }
    ///receive error interrupt (ERI)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::_1)
    }
}
/**Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTRG_A {
    ///0: Trigger number 0
    _0000 = 0,
    ///1: Triger number n (n= 0-15)
    OTHERS = 1,
}
impl From<TTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: TTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for TTRG_A {}
///Field `TTRG` reader - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
pub type TTRG_R = crate::FieldReader<TTRG_A>;
impl TTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TTRG_A {
        match self.bits {
            0 => TTRG_A::_0000,
            _ => TTRG_A::OTHERS,
        }
    }
    ///Trigger number 0
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TTRG_A::_0000
    }
    ///Triger number n (n= 0-15)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TTRG_A::OTHERS)
    }
}
///Field `TTRG` writer - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
pub type TTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TTRG_A, crate::Safe>;
impl<'a, REG> TTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger number 0
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::_0000)
    }
    ///Triger number n (n= 0-15)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::OTHERS)
    }
}
/**Receive FIFO data trigger number

Value on reset: 8*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTRG_A {
    ///0: Trigger number 0
    _0000 = 0,
    ///1: Triger number n (n= 0-15)
    OTHERS = 1,
}
impl From<RTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for RTRG_A {}
///Field `RTRG` reader - Receive FIFO data trigger number
pub type RTRG_R = crate::FieldReader<RTRG_A>;
impl RTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTRG_A {
        match self.bits {
            0 => RTRG_A::_0000,
            _ => RTRG_A::OTHERS,
        }
    }
    ///Trigger number 0
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RTRG_A::_0000
    }
    ///Triger number n (n= 0-15)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RTRG_A::OTHERS)
    }
}
///Field `RTRG` writer - Receive FIFO data trigger number
pub type RTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RTRG_A, crate::Safe>;
impl<'a, REG> RTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger number 0
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::_0000)
    }
    ///Triger number n (n= 0-15)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::OTHERS)
    }
}
/**RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)

Value on reset: 15*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTRG_A {
    ///0: Trigger number 0
    _0000 = 0,
    ///1: Triger number n (n= 0-15)
    OTHERS = 1,
}
impl From<RSTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for RSTRG_A {}
///Field `RSTRG` reader - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
pub type RSTRG_R = crate::FieldReader<RSTRG_A>;
impl RSTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSTRG_A {
        match self.bits {
            0 => RSTRG_A::_0000,
            _ => RSTRG_A::OTHERS,
        }
    }
    ///Trigger number 0
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RSTRG_A::_0000
    }
    ///Triger number n (n= 0-15)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RSTRG_A::OTHERS)
    }
}
///Field `RSTRG` writer - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
pub type RSTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RSTRG_A, crate::Safe>;
impl<'a, REG> RSTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger number 0
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(RSTRG_A::_0000)
    }
    ///Triger number n (n= 0-15)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RSTRG_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
    #[inline(always)]
    pub fn fm(&self) -> FM_R {
        FM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Receive FIFO data trigger number
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
    #[inline(always)]
    pub fn rstrg(&self) -> RSTRG_R {
        RSTRG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
    #[inline(always)]
    pub fn fm(&mut self) -> FM_W<FCR_SPEC> {
        FM_W::new(self, 0)
    }
    ///Bit 1 - Receive FIFO Data Register Reset(Valid only in FCR.FM=1)
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W<FCR_SPEC> {
        RFRST_W::new(self, 1)
    }
    ///Bit 2 - Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)
    #[inline(always)]
    pub fn tfrst(&mut self) -> TFRST_W<FCR_SPEC> {
        TFRST_W::new(self, 2)
    }
    ///Bit 3 - Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)
    #[inline(always)]
    pub fn dres(&mut self) -> DRES_W<FCR_SPEC> {
        DRES_W::new(self, 3)
    }
    ///Bits 4:7 - Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
    #[inline(always)]
    pub fn ttrg(&mut self) -> TTRG_W<FCR_SPEC> {
        TTRG_W::new(self, 4)
    }
    ///Bits 8:11 - Receive FIFO data trigger number
    #[inline(always)]
    pub fn rtrg(&mut self) -> RTRG_W<FCR_SPEC> {
        RTRG_W::new(self, 8)
    }
    ///Bits 12:15 - RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)
    #[inline(always)]
    pub fn rstrg(&mut self) -> RSTRG_W<FCR_SPEC> {
        RSTRG_W::new(self, 12)
    }
}
/**FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`fcr::R`](R) reader structure
impl crate::Readable for FCR_SPEC {}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0xf800
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: u16 = 0xf800;
}
