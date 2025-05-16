///Register `SPTR` reader
pub type R = crate::R<SPTR_SPEC>;
///Register `SPTR` writer
pub type W = crate::W<SPTR_SPEC>;
/**Serial input data monitor bit(The state of the RXD terminal is shown.)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMON_A {
    ///0: RXD terminal is the Low level.
    _0 = 0,
    ///1: RXD terminal is the High level.
    _1 = 1,
}
impl From<RXDMON_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMON` reader - Serial input data monitor bit(The state of the RXD terminal is shown.)
pub type RXDMON_R = crate::BitReader<RXDMON_A>;
impl RXDMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMON_A {
        match self.bits {
            false => RXDMON_A::_0,
            true => RXDMON_A::_1,
        }
    }
    ///RXD terminal is the Low level.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDMON_A::_0
    }
    ///RXD terminal is the High level.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDMON_A::_1
    }
}
/**Serial port break data select bit(The output level of TxD terminal is selected when SCR.TE = 0.)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2DT_A {
    ///0: Low level is output in TxD terminal.
    _0 = 0,
    ///1: High level is output in TxD terminal.
    _1 = 1,
}
impl From<SPB2DT_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2DT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPB2DT` reader - Serial port break data select bit(The output level of TxD terminal is selected when SCR.TE = 0.)
pub type SPB2DT_R = crate::BitReader<SPB2DT_A>;
impl SPB2DT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPB2DT_A {
        match self.bits {
            false => SPB2DT_A::_0,
            true => SPB2DT_A::_1,
        }
    }
    ///Low level is output in TxD terminal.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2DT_A::_0
    }
    ///High level is output in TxD terminal.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2DT_A::_1
    }
}
///Field `SPB2DT` writer - Serial port break data select bit(The output level of TxD terminal is selected when SCR.TE = 0.)
pub type SPB2DT_W<'a, REG> = crate::BitWriter<'a, REG, SPB2DT_A>;
impl<'a, REG> SPB2DT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low level is output in TxD terminal.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2DT_A::_0)
    }
    ///High level is output in TxD terminal.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2DT_A::_1)
    }
}
/**Serial port break I/O bit(It's selected whether the value of SPB2DT is output to TxD terminal.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPB2IO_A {
    ///0: The value of SPB2DT bit isn't output in TxD terminal.
    _0 = 0,
    ///1: The value of SPB2DT bit is output in TxD terminal.
    _1 = 1,
}
impl From<SPB2IO_A> for bool {
    #[inline(always)]
    fn from(variant: SPB2IO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPB2IO` reader - Serial port break I/O bit(It's selected whether the value of SPB2DT is output to TxD terminal.)
pub type SPB2IO_R = crate::BitReader<SPB2IO_A>;
impl SPB2IO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPB2IO_A {
        match self.bits {
            false => SPB2IO_A::_0,
            true => SPB2IO_A::_1,
        }
    }
    ///The value of SPB2DT bit isn't output in TxD terminal.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPB2IO_A::_0
    }
    ///The value of SPB2DT bit is output in TxD terminal.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPB2IO_A::_1
    }
}
///Field `SPB2IO` writer - Serial port break I/O bit(It's selected whether the value of SPB2DT is output to TxD terminal.)
pub type SPB2IO_W<'a, REG> = crate::BitWriter<'a, REG, SPB2IO_A>;
impl<'a, REG> SPB2IO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The value of SPB2DT bit isn't output in TxD terminal.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2IO_A::_0)
    }
    ///The value of SPB2DT bit is output in TxD terminal.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPB2IO_A::_1)
    }
}
impl R {
    ///Bit 0 - Serial input data monitor bit(The state of the RXD terminal is shown.)
    #[inline(always)]
    pub fn rxdmon(&self) -> RXDMON_R {
        RXDMON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Serial port break data select bit(The output level of TxD terminal is selected when SCR.TE = 0.)
    #[inline(always)]
    pub fn spb2dt(&self) -> SPB2DT_R {
        SPB2DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Serial port break I/O bit(It's selected whether the value of SPB2DT is output to TxD terminal.)
    #[inline(always)]
    pub fn spb2io(&self) -> SPB2IO_R {
        SPB2IO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Serial port break data select bit(The output level of TxD terminal is selected when SCR.TE = 0.)
    #[inline(always)]
    pub fn spb2dt(&mut self) -> SPB2DT_W<SPTR_SPEC> {
        SPB2DT_W::new(self, 1)
    }
    ///Bit 2 - Serial port break I/O bit(It's selected whether the value of SPB2DT is output to TxD terminal.)
    #[inline(always)]
    pub fn spb2io(&mut self) -> SPB2IO_W<SPTR_SPEC> {
        SPB2IO_W::new(self, 2)
    }
}
/**Serial Port Register

You can [`read`](crate::Reg::read) this register and get [`sptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPTR_SPEC;
impl crate::RegisterSpec for SPTR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sptr::R`](R) reader structure
impl crate::Readable for SPTR_SPEC {}
///`write(|w| ..)` method takes [`sptr::W`](W) writer structure
impl crate::Writable for SPTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPTR to value 0x03
impl crate::Resettable for SPTR_SPEC {
    const RESET_VALUE: u8 = 0x03;
}
