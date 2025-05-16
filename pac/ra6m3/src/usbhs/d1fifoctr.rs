///Register `D1FIFOCTR` reader
pub type R = crate::R<D1FIFOCTR_SPEC>;
///Register `D1FIFOCTR` writer
pub type W = crate::W<D1FIFOCTR_SPEC>;
///Field `DTLN` reader - Receive Data Length.Indicates the length of the receive data.
pub type DTLN_R = crate::FieldReader<u16>;
/**FIFO Port ReadyIndicates whether the FIFO port can be accessed.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDY_A {
    ///0: FIFO port access is disabled
    _0 = 0,
    ///1: FIFO port access is enabled
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRDY` reader - FIFO Port ReadyIndicates whether the FIFO port can be accessed.
pub type FRDY_R = crate::BitReader<FRDY_A>;
impl FRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRDY_A {
        match self.bits {
            false => FRDY_A::_0,
            true => FRDY_A::_1,
        }
    }
    ///FIFO port access is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDY_A::_0
    }
    ///FIFO port access is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDY_A::_1
    }
}
/**CPU Buffer Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCLR_A {
    ///0: No operation
    _0 = 0,
    ///1: Clear FIFO buffer on the CPU side.
    _1 = 1,
}
impl From<BCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCLR` writer - CPU Buffer Clear
pub type BCLR_W<'a, REG> = crate::BitWriter<'a, REG, BCLR_A>;
impl<'a, REG> BCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCLR_A::_0)
    }
    ///Clear FIFO buffer on the CPU side.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCLR_A::_1)
    }
}
/**Buffer Memory Valid Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BVAL_A {
    ///0: Invalid
    _0 = 0,
    ///1: Writing ended
    _1 = 1,
}
impl From<BVAL_A> for bool {
    #[inline(always)]
    fn from(variant: BVAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BVAL` reader - Buffer Memory Valid Flag
pub type BVAL_R = crate::BitReader<BVAL_A>;
impl BVAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BVAL_A {
        match self.bits {
            false => BVAL_A::_0,
            true => BVAL_A::_1,
        }
    }
    ///Invalid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BVAL_A::_0
    }
    ///Writing ended
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BVAL_A::_1
    }
}
///Field `BVAL` writer - Buffer Memory Valid Flag
pub type BVAL_W<'a, REG> = crate::BitWriter<'a, REG, BVAL_A>;
impl<'a, REG> BVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BVAL_A::_0)
    }
    ///Writing ended
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BVAL_A::_1)
    }
}
impl R {
    ///Bits 0:11 - Receive Data Length.Indicates the length of the receive data.
    #[inline(always)]
    pub fn dtln(&self) -> DTLN_R {
        DTLN_R::new(self.bits & 0x0fff)
    }
    ///Bit 13 - FIFO Port ReadyIndicates whether the FIFO port can be accessed.
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Buffer Memory Valid Flag
    #[inline(always)]
    pub fn bval(&self) -> BVAL_R {
        BVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - CPU Buffer Clear
    #[inline(always)]
    pub fn bclr(&mut self) -> BCLR_W<D1FIFOCTR_SPEC> {
        BCLR_W::new(self, 14)
    }
    ///Bit 15 - Buffer Memory Valid Flag
    #[inline(always)]
    pub fn bval(&mut self) -> BVAL_W<D1FIFOCTR_SPEC> {
        BVAL_W::new(self, 15)
    }
}
/**D1FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`d1fifoctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1FIFOCTR_SPEC;
impl crate::RegisterSpec for D1FIFOCTR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`d1fifoctr::R`](R) reader structure
impl crate::Readable for D1FIFOCTR_SPEC {}
///`write(|w| ..)` method takes [`d1fifoctr::W`](W) writer structure
impl crate::Writable for D1FIFOCTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFOCTR to value 0
impl crate::Resettable for D1FIFOCTR_SPEC {}
