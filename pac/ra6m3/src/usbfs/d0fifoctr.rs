///Register `D0FIFOCTR` reader
pub type R = crate::R<D0FIFOCTR_SPEC>;
///Register `D0FIFOCTR` writer
pub type W = crate::W<D0FIFOCTR_SPEC>;
///Field `DTLN` reader - Receive Data LengthIndicates the length of the receive data.
pub type DTLN_R = crate::FieldReader<u16>;
/**FIFO Port Ready

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDY_A {
    ///0: FIFO port access is disabled.
    _0 = 0,
    ///1: FIFO port access is enabled.
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRDY` reader - FIFO Port Ready
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
    ///FIFO port access is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDY_A::_0
    }
    ///FIFO port access is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDY_A::_1
    }
}
/**CPU Buffer Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCLR_A {
    ///0: Invalid
    _0 = 0,
    ///1: Clears the buffer memory on the CPU side
    _1 = 1,
}
impl From<BCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCLR` reader - CPU Buffer Clear
pub type BCLR_R = crate::BitReader<BCLR_A>;
impl BCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCLR_A {
        match self.bits {
            false => BCLR_A::_0,
            true => BCLR_A::_1,
        }
    }
    ///Invalid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCLR_A::_0
    }
    ///Clears the buffer memory on the CPU side
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCLR_A::_1
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
    ///Bits 0:8 - Receive Data LengthIndicates the length of the receive data.
    #[inline(always)]
    pub fn dtln(&self) -> DTLN_R {
        DTLN_R::new(self.bits & 0x01ff)
    }
    ///Bit 13 - FIFO Port Ready
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU Buffer Clear
    #[inline(always)]
    pub fn bclr(&self) -> BCLR_R {
        BCLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Buffer Memory Valid Flag
    #[inline(always)]
    pub fn bval(&self) -> BVAL_R {
        BVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - Buffer Memory Valid Flag
    #[inline(always)]
    pub fn bval(&mut self) -> BVAL_W<D0FIFOCTR_SPEC> {
        BVAL_W::new(self, 15)
    }
}
/**D0FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`d0fifoctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D0FIFOCTR_SPEC;
impl crate::RegisterSpec for D0FIFOCTR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`d0fifoctr::R`](R) reader structure
impl crate::Readable for D0FIFOCTR_SPEC {}
///`write(|w| ..)` method takes [`d0fifoctr::W`](W) writer structure
impl crate::Writable for D0FIFOCTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D0FIFOCTR to value 0
impl crate::Resettable for D0FIFOCTR_SPEC {}
