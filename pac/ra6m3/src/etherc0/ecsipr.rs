///Register `ECSIPR` reader
pub type R = crate::R<ECSIPR_SPEC>;
///Register `ECSIPR` writer
pub type W = crate::W<ECSIPR_SPEC>;
/**False Carrier Detect Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICDIP_A {
    ///0: Notification of the false carrier detect interrupt is disabled.
    _0 = 0,
    ///1: Notification of the false carrier detect interrupt is enabled.
    _1 = 1,
}
impl From<ICDIP_A> for bool {
    #[inline(always)]
    fn from(variant: ICDIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ICDIP` reader - False Carrier Detect Interrupt Enable
pub type ICDIP_R = crate::BitReader<ICDIP_A>;
impl ICDIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICDIP_A {
        match self.bits {
            false => ICDIP_A::_0,
            true => ICDIP_A::_1,
        }
    }
    ///Notification of the false carrier detect interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICDIP_A::_0
    }
    ///Notification of the false carrier detect interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICDIP_A::_1
    }
}
///Field `ICDIP` writer - False Carrier Detect Interrupt Enable
pub type ICDIP_W<'a, REG> = crate::BitWriter<'a, REG, ICDIP_A>;
impl<'a, REG> ICDIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Notification of the false carrier detect interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ICDIP_A::_0)
    }
    ///Notification of the false carrier detect interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ICDIP_A::_1)
    }
}
/**Magic Packet Detect Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDIP_A {
    ///0: Notification of the Magic Packet detect interrupt is disabled.
    _0 = 0,
    ///1: Notification of the Magic Packet detect interrupt is enabled.
    _1 = 1,
}
impl From<MPDIP_A> for bool {
    #[inline(always)]
    fn from(variant: MPDIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPDIP` reader - Magic Packet Detect Interrupt Enable
pub type MPDIP_R = crate::BitReader<MPDIP_A>;
impl MPDIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPDIP_A {
        match self.bits {
            false => MPDIP_A::_0,
            true => MPDIP_A::_1,
        }
    }
    ///Notification of the Magic Packet detect interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDIP_A::_0
    }
    ///Notification of the Magic Packet detect interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDIP_A::_1
    }
}
///Field `MPDIP` writer - Magic Packet Detect Interrupt Enable
pub type MPDIP_W<'a, REG> = crate::BitWriter<'a, REG, MPDIP_A>;
impl<'a, REG> MPDIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Notification of the Magic Packet detect interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPDIP_A::_0)
    }
    ///Notification of the Magic Packet detect interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPDIP_A::_1)
    }
}
/**LINK Signal Change Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCHNGIP_A {
    ///0: Notification of ETn_LINKSTA signal change interrupt is disabled.
    _0 = 0,
    ///1: Notification of ETn_LINKSTA signal change interrupt is enabled.
    _1 = 1,
}
impl From<LCHNGIP_A> for bool {
    #[inline(always)]
    fn from(variant: LCHNGIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCHNGIP` reader - LINK Signal Change Interrupt Enable
pub type LCHNGIP_R = crate::BitReader<LCHNGIP_A>;
impl LCHNGIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCHNGIP_A {
        match self.bits {
            false => LCHNGIP_A::_0,
            true => LCHNGIP_A::_1,
        }
    }
    ///Notification of ETn_LINKSTA signal change interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCHNGIP_A::_0
    }
    ///Notification of ETn_LINKSTA signal change interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCHNGIP_A::_1
    }
}
///Field `LCHNGIP` writer - LINK Signal Change Interrupt Enable
pub type LCHNGIP_W<'a, REG> = crate::BitWriter<'a, REG, LCHNGIP_A>;
impl<'a, REG> LCHNGIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Notification of ETn_LINKSTA signal change interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCHNGIP_A::_0)
    }
    ///Notification of ETn_LINKSTA signal change interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCHNGIP_A::_1)
    }
}
/**PAUSE Frame Retransmit Over Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSRTOIP_A {
    ///0: Notification of PAUSE frame retransmit over interrupt is disabled.
    _0 = 0,
    ///1: Notification of PAUSE frame retransmit over interrupt is enabled.
    _1 = 1,
}
impl From<PSRTOIP_A> for bool {
    #[inline(always)]
    fn from(variant: PSRTOIP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSRTOIP` reader - PAUSE Frame Retransmit Over Interrupt Enable
pub type PSRTOIP_R = crate::BitReader<PSRTOIP_A>;
impl PSRTOIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSRTOIP_A {
        match self.bits {
            false => PSRTOIP_A::_0,
            true => PSRTOIP_A::_1,
        }
    }
    ///Notification of PAUSE frame retransmit over interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSRTOIP_A::_0
    }
    ///Notification of PAUSE frame retransmit over interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSRTOIP_A::_1
    }
}
///Field `PSRTOIP` writer - PAUSE Frame Retransmit Over Interrupt Enable
pub type PSRTOIP_W<'a, REG> = crate::BitWriter<'a, REG, PSRTOIP_A>;
impl<'a, REG> PSRTOIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Notification of PAUSE frame retransmit over interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSRTOIP_A::_0)
    }
    ///Notification of PAUSE frame retransmit over interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSRTOIP_A::_1)
    }
}
/**Continuous Broadcast Frame Reception Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFSIPR_A {
    ///0: Notification of continuous broadcast frame reception interrupt is disabled.
    _0 = 0,
    ///1: Notification of continuous broadcast frame reception interrupt is enabled.
    _1 = 1,
}
impl From<BFSIPR_A> for bool {
    #[inline(always)]
    fn from(variant: BFSIPR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BFSIPR` reader - Continuous Broadcast Frame Reception Interrupt Enable
pub type BFSIPR_R = crate::BitReader<BFSIPR_A>;
impl BFSIPR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BFSIPR_A {
        match self.bits {
            false => BFSIPR_A::_0,
            true => BFSIPR_A::_1,
        }
    }
    ///Notification of continuous broadcast frame reception interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFSIPR_A::_0
    }
    ///Notification of continuous broadcast frame reception interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFSIPR_A::_1
    }
}
///Field `BFSIPR` writer - Continuous Broadcast Frame Reception Interrupt Enable
pub type BFSIPR_W<'a, REG> = crate::BitWriter<'a, REG, BFSIPR_A>;
impl<'a, REG> BFSIPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Notification of continuous broadcast frame reception interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BFSIPR_A::_0)
    }
    ///Notification of continuous broadcast frame reception interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BFSIPR_A::_1)
    }
}
impl R {
    ///Bit 0 - False Carrier Detect Interrupt Enable
    #[inline(always)]
    pub fn icdip(&self) -> ICDIP_R {
        ICDIP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet Detect Interrupt Enable
    #[inline(always)]
    pub fn mpdip(&self) -> MPDIP_R {
        MPDIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LINK Signal Change Interrupt Enable
    #[inline(always)]
    pub fn lchngip(&self) -> LCHNGIP_R {
        LCHNGIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Interrupt Enable
    #[inline(always)]
    pub fn psrtoip(&self) -> PSRTOIP_R {
        PSRTOIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn bfsipr(&self) -> BFSIPR_R {
        BFSIPR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - False Carrier Detect Interrupt Enable
    #[inline(always)]
    pub fn icdip(&mut self) -> ICDIP_W<ECSIPR_SPEC> {
        ICDIP_W::new(self, 0)
    }
    ///Bit 1 - Magic Packet Detect Interrupt Enable
    #[inline(always)]
    pub fn mpdip(&mut self) -> MPDIP_W<ECSIPR_SPEC> {
        MPDIP_W::new(self, 1)
    }
    ///Bit 2 - LINK Signal Change Interrupt Enable
    #[inline(always)]
    pub fn lchngip(&mut self) -> LCHNGIP_W<ECSIPR_SPEC> {
        LCHNGIP_W::new(self, 2)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Interrupt Enable
    #[inline(always)]
    pub fn psrtoip(&mut self) -> PSRTOIP_W<ECSIPR_SPEC> {
        PSRTOIP_W::new(self, 4)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Interrupt Enable
    #[inline(always)]
    pub fn bfsipr(&mut self) -> BFSIPR_W<ECSIPR_SPEC> {
        BFSIPR_W::new(self, 5)
    }
}
/**ETHERC Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ecsipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECSIPR_SPEC;
impl crate::RegisterSpec for ECSIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ecsipr::R`](R) reader structure
impl crate::Readable for ECSIPR_SPEC {}
///`write(|w| ..)` method takes [`ecsipr::W`](W) writer structure
impl crate::Writable for ECSIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECSIPR to value 0
impl crate::Resettable for ECSIPR_SPEC {}
