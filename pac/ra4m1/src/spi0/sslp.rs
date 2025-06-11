///Register `SSLP` reader
pub type R = crate::R<SSLP_SPEC>;
///Register `SSLP` writer
pub type W = crate::W<SSLP_SPEC>;
/**SSL0 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL0P_A {
    ///0: SSL0 signal is active low
    _0 = 0,
    ///1: SSL0 signal is active high
    _1 = 1,
}
impl From<SSL0P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL0P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL0P` reader - SSL0 Signal Polarity Setting
pub type SSL0P_R = crate::BitReader<SSL0P_A>;
impl SSL0P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSL0P_A {
        match self.bits {
            false => SSL0P_A::_0,
            true => SSL0P_A::_1,
        }
    }
    ///SSL0 signal is active low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL0P_A::_0
    }
    ///SSL0 signal is active high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL0P_A::_1
    }
}
///Field `SSL0P` writer - SSL0 Signal Polarity Setting
pub type SSL0P_W<'a, REG> = crate::BitWriter<'a, REG, SSL0P_A>;
impl<'a, REG> SSL0P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSL0 signal is active low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSL0P_A::_0)
    }
    ///SSL0 signal is active high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSL0P_A::_1)
    }
}
/**SSL1 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL1P_A {
    ///0: SSL1 signal is active low
    _0 = 0,
    ///1: SSL1 signal is active high
    _1 = 1,
}
impl From<SSL1P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL1P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL1P` reader - SSL1 Signal Polarity Setting
pub type SSL1P_R = crate::BitReader<SSL1P_A>;
impl SSL1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSL1P_A {
        match self.bits {
            false => SSL1P_A::_0,
            true => SSL1P_A::_1,
        }
    }
    ///SSL1 signal is active low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL1P_A::_0
    }
    ///SSL1 signal is active high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL1P_A::_1
    }
}
///Field `SSL1P` writer - SSL1 Signal Polarity Setting
pub type SSL1P_W<'a, REG> = crate::BitWriter<'a, REG, SSL1P_A>;
impl<'a, REG> SSL1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSL1 signal is active low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSL1P_A::_0)
    }
    ///SSL1 signal is active high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSL1P_A::_1)
    }
}
/**SSL2 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL2P_A {
    ///0: SSL2 signal is active low
    _0 = 0,
    ///1: SSL2 signal is active high
    _1 = 1,
}
impl From<SSL2P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL2P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL2P` reader - SSL2 Signal Polarity Setting
pub type SSL2P_R = crate::BitReader<SSL2P_A>;
impl SSL2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSL2P_A {
        match self.bits {
            false => SSL2P_A::_0,
            true => SSL2P_A::_1,
        }
    }
    ///SSL2 signal is active low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL2P_A::_0
    }
    ///SSL2 signal is active high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL2P_A::_1
    }
}
///Field `SSL2P` writer - SSL2 Signal Polarity Setting
pub type SSL2P_W<'a, REG> = crate::BitWriter<'a, REG, SSL2P_A>;
impl<'a, REG> SSL2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSL2 signal is active low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSL2P_A::_0)
    }
    ///SSL2 signal is active high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSL2P_A::_1)
    }
}
/**SSL3 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL3P_A {
    ///0: SSL3 signal is active low
    _0 = 0,
    ///1: SSL3 signal is active high
    _1 = 1,
}
impl From<SSL3P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL3P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL3P` reader - SSL3 Signal Polarity Setting
pub type SSL3P_R = crate::BitReader<SSL3P_A>;
impl SSL3P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSL3P_A {
        match self.bits {
            false => SSL3P_A::_0,
            true => SSL3P_A::_1,
        }
    }
    ///SSL3 signal is active low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL3P_A::_0
    }
    ///SSL3 signal is active high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL3P_A::_1
    }
}
///Field `SSL3P` writer - SSL3 Signal Polarity Setting
pub type SSL3P_W<'a, REG> = crate::BitWriter<'a, REG, SSL3P_A>;
impl<'a, REG> SSL3P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSL3 signal is active low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSL3P_A::_0)
    }
    ///SSL3 signal is active high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSL3P_A::_1)
    }
}
impl R {
    ///Bit 0 - SSL0 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl0p(&self) -> SSL0P_R {
        SSL0P_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SSL1 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl1p(&self) -> SSL1P_R {
        SSL1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SSL2 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl2p(&self) -> SSL2P_R {
        SSL2P_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SSL3 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl3p(&self) -> SSL3P_R {
        SSL3P_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SSL0 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl0p(&mut self) -> SSL0P_W<SSLP_SPEC> {
        SSL0P_W::new(self, 0)
    }
    ///Bit 1 - SSL1 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl1p(&mut self) -> SSL1P_W<SSLP_SPEC> {
        SSL1P_W::new(self, 1)
    }
    ///Bit 2 - SSL2 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl2p(&mut self) -> SSL2P_W<SSLP_SPEC> {
        SSL2P_W::new(self, 2)
    }
    ///Bit 3 - SSL3 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl3p(&mut self) -> SSL3P_W<SSLP_SPEC> {
        SSL3P_W::new(self, 3)
    }
}
/**SPI Slave Select Polarity Register

You can [`read`](crate::Reg::read) this register and get [`sslp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSLP_SPEC;
impl crate::RegisterSpec for SSLP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sslp::R`](R) reader structure
impl crate::Readable for SSLP_SPEC {}
///`write(|w| ..)` method takes [`sslp::W`](W) writer structure
impl crate::Writable for SSLP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSLP to value 0
impl crate::Resettable for SSLP_SPEC {}
