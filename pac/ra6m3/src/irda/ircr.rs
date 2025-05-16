///Register `IRCR` reader
pub type R = crate::R<IRCR_SPEC>;
///Register `IRCR` writer
pub type W = crate::W<IRCR_SPEC>;
/**IRRXD Polarity Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRRXINV_A {
    ///0: IRRXD input is used as received data as is.
    _0 = 0,
    ///1: IRRXD input is used as received data after the polarity is inverted.
    _1 = 1,
}
impl From<IRRXINV_A> for bool {
    #[inline(always)]
    fn from(variant: IRRXINV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRRXINV` reader - IRRXD Polarity Switching
pub type IRRXINV_R = crate::BitReader<IRRXINV_A>;
impl IRRXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRRXINV_A {
        match self.bits {
            false => IRRXINV_A::_0,
            true => IRRXINV_A::_1,
        }
    }
    ///IRRXD input is used as received data as is.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRRXINV_A::_0
    }
    ///IRRXD input is used as received data after the polarity is inverted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRRXINV_A::_1
    }
}
///Field `IRRXINV` writer - IRRXD Polarity Switching
pub type IRRXINV_W<'a, REG> = crate::BitWriter<'a, REG, IRRXINV_A>;
impl<'a, REG> IRRXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IRRXD input is used as received data as is.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRRXINV_A::_0)
    }
    ///IRRXD input is used as received data after the polarity is inverted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRRXINV_A::_1)
    }
}
/**IRTXD Polarity Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRTXINV_A {
    ///0: Data to be transmitted is output to IRTXD as is.
    _0 = 0,
    ///1: Data to be transmitted is output to IRTXD after the polarity is inverted.
    _1 = 1,
}
impl From<IRTXINV_A> for bool {
    #[inline(always)]
    fn from(variant: IRTXINV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRTXINV` reader - IRTXD Polarity Switching
pub type IRTXINV_R = crate::BitReader<IRTXINV_A>;
impl IRTXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRTXINV_A {
        match self.bits {
            false => IRTXINV_A::_0,
            true => IRTXINV_A::_1,
        }
    }
    ///Data to be transmitted is output to IRTXD as is.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRTXINV_A::_0
    }
    ///Data to be transmitted is output to IRTXD after the polarity is inverted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRTXINV_A::_1
    }
}
///Field `IRTXINV` writer - IRTXD Polarity Switching
pub type IRTXINV_W<'a, REG> = crate::BitWriter<'a, REG, IRTXINV_A>;
impl<'a, REG> IRTXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data to be transmitted is output to IRTXD as is.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRTXINV_A::_0)
    }
    ///Data to be transmitted is output to IRTXD after the polarity is inverted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRTXINV_A::_1)
    }
}
/**IrDA Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRE_A {
    ///0: Serial I/O pins are used for normal serial communication.
    _0 = 0,
    ///1: Serial I/O pins are used for IrDA data communication.
    _1 = 1,
}
impl From<IRE_A> for bool {
    #[inline(always)]
    fn from(variant: IRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRE` reader - IrDA Enable
pub type IRE_R = crate::BitReader<IRE_A>;
impl IRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRE_A {
        match self.bits {
            false => IRE_A::_0,
            true => IRE_A::_1,
        }
    }
    ///Serial I/O pins are used for normal serial communication.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRE_A::_0
    }
    ///Serial I/O pins are used for IrDA data communication.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRE_A::_1
    }
}
///Field `IRE` writer - IrDA Enable
pub type IRE_W<'a, REG> = crate::BitWriter<'a, REG, IRE_A>;
impl<'a, REG> IRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Serial I/O pins are used for normal serial communication.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRE_A::_0)
    }
    ///Serial I/O pins are used for IrDA data communication.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRE_A::_1)
    }
}
impl R {
    ///Bit 2 - IRRXD Polarity Switching
    #[inline(always)]
    pub fn irrxinv(&self) -> IRRXINV_R {
        IRRXINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRTXD Polarity Switching
    #[inline(always)]
    pub fn irtxinv(&self) -> IRTXINV_R {
        IRTXINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - IrDA Enable
    #[inline(always)]
    pub fn ire(&self) -> IRE_R {
        IRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - IRRXD Polarity Switching
    #[inline(always)]
    pub fn irrxinv(&mut self) -> IRRXINV_W<IRCR_SPEC> {
        IRRXINV_W::new(self, 2)
    }
    ///Bit 3 - IRTXD Polarity Switching
    #[inline(always)]
    pub fn irtxinv(&mut self) -> IRTXINV_W<IRCR_SPEC> {
        IRTXINV_W::new(self, 3)
    }
    ///Bit 7 - IrDA Enable
    #[inline(always)]
    pub fn ire(&mut self) -> IRE_W<IRCR_SPEC> {
        IRE_W::new(self, 7)
    }
}
/**IrDA Control Register

You can [`read`](crate::Reg::read) this register and get [`ircr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ircr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IRCR_SPEC;
impl crate::RegisterSpec for IRCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ircr::R`](R) reader structure
impl crate::Readable for IRCR_SPEC {}
///`write(|w| ..)` method takes [`ircr::W`](W) writer structure
impl crate::Writable for IRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRCR to value 0
impl crate::Resettable for IRCR_SPEC {}
