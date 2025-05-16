///Register `CRCCR1` reader
pub type R = crate::R<CRCCR1_SPEC>;
///Register `CRCCR1` writer
pub type W = crate::W<CRCCR1_SPEC>;
/**Snoop-on-write/read switch bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSWR_A {
    ///0: Snoop-on-read
    _0 = 0,
    ///1: Snoop-on-write
    _1 = 1,
}
impl From<CRCSWR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSWR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCSWR` reader - Snoop-on-write/read switch bit
pub type CRCSWR_R = crate::BitReader<CRCSWR_A>;
impl CRCSWR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCSWR_A {
        match self.bits {
            false => CRCSWR_A::_0,
            true => CRCSWR_A::_1,
        }
    }
    ///Snoop-on-read
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCSWR_A::_0
    }
    ///Snoop-on-write
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCSWR_A::_1
    }
}
///Field `CRCSWR` writer - Snoop-on-write/read switch bit
pub type CRCSWR_W<'a, REG> = crate::BitWriter<'a, REG, CRCSWR_A>;
impl<'a, REG> CRCSWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Snoop-on-read
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSWR_A::_0)
    }
    ///Snoop-on-write
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSWR_A::_1)
    }
}
/**Snoop enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSEN_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<CRCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCSEN` reader - Snoop enable bit
pub type CRCSEN_R = crate::BitReader<CRCSEN_A>;
impl CRCSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCSEN_A {
        match self.bits {
            false => CRCSEN_A::_0,
            true => CRCSEN_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCSEN_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCSEN_A::_1
    }
}
///Field `CRCSEN` writer - Snoop enable bit
pub type CRCSEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCSEN_A>;
impl<'a, REG> CRCSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSEN_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSEN_A::_1)
    }
}
impl R {
    ///Bit 6 - Snoop-on-write/read switch bit
    #[inline(always)]
    pub fn crcswr(&self) -> CRCSWR_R {
        CRCSWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Snoop enable bit
    #[inline(always)]
    pub fn crcsen(&self) -> CRCSEN_R {
        CRCSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - Snoop-on-write/read switch bit
    #[inline(always)]
    pub fn crcswr(&mut self) -> CRCSWR_W<CRCCR1_SPEC> {
        CRCSWR_W::new(self, 6)
    }
    ///Bit 7 - Snoop enable bit
    #[inline(always)]
    pub fn crcsen(&mut self) -> CRCSEN_W<CRCCR1_SPEC> {
        CRCSEN_W::new(self, 7)
    }
}
/**CRC Control Register1

You can [`read`](crate::Reg::read) this register and get [`crccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCCR1_SPEC;
impl crate::RegisterSpec for CRCCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`crccr1::R`](R) reader structure
impl crate::Readable for CRCCR1_SPEC {}
///`write(|w| ..)` method takes [`crccr1::W`](W) writer structure
impl crate::Writable for CRCCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCCR1 to value 0
impl crate::Resettable for CRCCR1_SPEC {}
