///Register `SIMR2` reader
pub type R = crate::R<SIMR2_SPEC>;
///Register `SIMR2` writer
pub type W = crate::W<SIMR2_SPEC>;
/**I2C Interrupt Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICINTM_A {
    ///0: Use ACK/NACK interrupts.
    _0 = 0,
    ///1: Use reception and transmission interrupts
    _1 = 1,
}
impl From<IICINTM_A> for bool {
    #[inline(always)]
    fn from(variant: IICINTM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICINTM` reader - I2C Interrupt Mode Select
pub type IICINTM_R = crate::BitReader<IICINTM_A>;
impl IICINTM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICINTM_A {
        match self.bits {
            false => IICINTM_A::_0,
            true => IICINTM_A::_1,
        }
    }
    ///Use ACK/NACK interrupts.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICINTM_A::_0
    }
    ///Use reception and transmission interrupts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICINTM_A::_1
    }
}
///Field `IICINTM` writer - I2C Interrupt Mode Select
pub type IICINTM_W<'a, REG> = crate::BitWriter<'a, REG, IICINTM_A>;
impl<'a, REG> IICINTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use ACK/NACK interrupts.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICINTM_A::_0)
    }
    ///Use reception and transmission interrupts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICINTM_A::_1)
    }
}
/**Clock Synchronization

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICCSC_A {
    ///0: No synchronization with the clock signal
    _0 = 0,
    ///1: Synchronization with the clock signal
    _1 = 1,
}
impl From<IICCSC_A> for bool {
    #[inline(always)]
    fn from(variant: IICCSC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICCSC` reader - Clock Synchronization
pub type IICCSC_R = crate::BitReader<IICCSC_A>;
impl IICCSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICCSC_A {
        match self.bits {
            false => IICCSC_A::_0,
            true => IICCSC_A::_1,
        }
    }
    ///No synchronization with the clock signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICCSC_A::_0
    }
    ///Synchronization with the clock signal
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICCSC_A::_1
    }
}
///Field `IICCSC` writer - Clock Synchronization
pub type IICCSC_W<'a, REG> = crate::BitWriter<'a, REG, IICCSC_A>;
impl<'a, REG> IICCSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No synchronization with the clock signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICCSC_A::_0)
    }
    ///Synchronization with the clock signal
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICCSC_A::_1)
    }
}
/**ACK Transmission Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICACKT_A {
    ///0: ACK transmission
    _0 = 0,
    ///1: NACK transmission and reception of ACK/NACK
    _1 = 1,
}
impl From<IICACKT_A> for bool {
    #[inline(always)]
    fn from(variant: IICACKT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICACKT` reader - ACK Transmission Data
pub type IICACKT_R = crate::BitReader<IICACKT_A>;
impl IICACKT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICACKT_A {
        match self.bits {
            false => IICACKT_A::_0,
            true => IICACKT_A::_1,
        }
    }
    ///ACK transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICACKT_A::_0
    }
    ///NACK transmission and reception of ACK/NACK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICACKT_A::_1
    }
}
///Field `IICACKT` writer - ACK Transmission Data
pub type IICACKT_W<'a, REG> = crate::BitWriter<'a, REG, IICACKT_A>;
impl<'a, REG> IICACKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ACK transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICACKT_A::_0)
    }
    ///NACK transmission and reception of ACK/NACK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICACKT_A::_1)
    }
}
impl R {
    ///Bit 0 - I2C Interrupt Mode Select
    #[inline(always)]
    pub fn iicintm(&self) -> IICINTM_R {
        IICINTM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock Synchronization
    #[inline(always)]
    pub fn iiccsc(&self) -> IICCSC_R {
        IICCSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ACK Transmission Data
    #[inline(always)]
    pub fn iicackt(&self) -> IICACKT_R {
        IICACKT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2C Interrupt Mode Select
    #[inline(always)]
    pub fn iicintm(&mut self) -> IICINTM_W<SIMR2_SPEC> {
        IICINTM_W::new(self, 0)
    }
    ///Bit 1 - Clock Synchronization
    #[inline(always)]
    pub fn iiccsc(&mut self) -> IICCSC_W<SIMR2_SPEC> {
        IICCSC_W::new(self, 1)
    }
    ///Bit 5 - ACK Transmission Data
    #[inline(always)]
    pub fn iicackt(&mut self) -> IICACKT_W<SIMR2_SPEC> {
        IICACKT_W::new(self, 5)
    }
}
/**I2C Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`simr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SIMR2_SPEC;
impl crate::RegisterSpec for SIMR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`simr2::R`](R) reader structure
impl crate::Readable for SIMR2_SPEC {}
///`write(|w| ..)` method takes [`simr2::W`](W) writer structure
impl crate::Writable for SIMR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIMR2 to value 0
impl crate::Resettable for SIMR2_SPEC {}
