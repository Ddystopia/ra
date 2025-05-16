///Register `DPSIEGR2` reader
pub type R = crate::R<DPSIEGR2_SPEC>;
///Register `DPSIEGR2` writer
pub type W = crate::W<DPSIEGR2_SPEC>;
/**LVD1 Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD1IEG_A {
    ///0: A cancel request is generated when VCC<Vdet1 (fall) is detected
    _0 = 0,
    ///1: A cancel request is generated when VCC>=Vdet1 (rise) is detected
    _1 = 1,
}
impl From<DLVD1IEG_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD1IEG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD1IEG` reader - LVD1 Edge Select
pub type DLVD1IEG_R = crate::BitReader<DLVD1IEG_A>;
impl DLVD1IEG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLVD1IEG_A {
        match self.bits {
            false => DLVD1IEG_A::_0,
            true => DLVD1IEG_A::_1,
        }
    }
    ///A cancel request is generated when VCC<Vdet1 (fall) is detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD1IEG_A::_0
    }
    ///A cancel request is generated when VCC>=Vdet1 (rise) is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD1IEG_A::_1
    }
}
///Field `DLVD1IEG` writer - LVD1 Edge Select
pub type DLVD1IEG_W<'a, REG> = crate::BitWriter<'a, REG, DLVD1IEG_A>;
impl<'a, REG> DLVD1IEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated when VCC<Vdet1 (fall) is detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD1IEG_A::_0)
    }
    ///A cancel request is generated when VCC>=Vdet1 (rise) is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD1IEG_A::_1)
    }
}
/**LVD2 Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD2IEG_A {
    ///0: A cancel request is generated when VCC<Vdet2 (fall) is detected
    _0 = 0,
    ///1: A cancel request is generated when VCC>=Vdet2 (rise) is detected
    _1 = 1,
}
impl From<DLVD2IEG_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD2IEG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD2IEG` reader - LVD2 Edge Select
pub type DLVD2IEG_R = crate::BitReader<DLVD2IEG_A>;
impl DLVD2IEG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLVD2IEG_A {
        match self.bits {
            false => DLVD2IEG_A::_0,
            true => DLVD2IEG_A::_1,
        }
    }
    ///A cancel request is generated when VCC<Vdet2 (fall) is detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD2IEG_A::_0
    }
    ///A cancel request is generated when VCC>=Vdet2 (rise) is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD2IEG_A::_1
    }
}
///Field `DLVD2IEG` writer - LVD2 Edge Select
pub type DLVD2IEG_W<'a, REG> = crate::BitWriter<'a, REG, DLVD2IEG_A>;
impl<'a, REG> DLVD2IEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated when VCC<Vdet2 (fall) is detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD2IEG_A::_0)
    }
    ///A cancel request is generated when VCC>=Vdet2 (rise) is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD2IEG_A::_1)
    }
}
/**NMI Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNMIEG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DNMIEG_A> for bool {
    #[inline(always)]
    fn from(variant: DNMIEG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DNMIEG` reader - NMI Pin Edge Select
pub type DNMIEG_R = crate::BitReader<DNMIEG_A>;
impl DNMIEG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DNMIEG_A {
        match self.bits {
            false => DNMIEG_A::_0,
            true => DNMIEG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNMIEG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNMIEG_A::_1
    }
}
///Field `DNMIEG` writer - NMI Pin Edge Select
pub type DNMIEG_W<'a, REG> = crate::BitWriter<'a, REG, DNMIEG_A>;
impl<'a, REG> DNMIEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DNMIEG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DNMIEG_A::_1)
    }
}
impl R {
    ///Bit 0 - LVD1 Edge Select
    #[inline(always)]
    pub fn dlvd1ieg(&self) -> DLVD1IEG_R {
        DLVD1IEG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LVD2 Edge Select
    #[inline(always)]
    pub fn dlvd2ieg(&self) -> DLVD2IEG_R {
        DLVD2IEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - NMI Pin Edge Select
    #[inline(always)]
    pub fn dnmieg(&self) -> DNMIEG_R {
        DNMIEG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LVD1 Edge Select
    #[inline(always)]
    pub fn dlvd1ieg(&mut self) -> DLVD1IEG_W<DPSIEGR2_SPEC> {
        DLVD1IEG_W::new(self, 0)
    }
    ///Bit 1 - LVD2 Edge Select
    #[inline(always)]
    pub fn dlvd2ieg(&mut self) -> DLVD2IEG_W<DPSIEGR2_SPEC> {
        DLVD2IEG_W::new(self, 1)
    }
    ///Bit 4 - NMI Pin Edge Select
    #[inline(always)]
    pub fn dnmieg(&mut self) -> DNMIEG_W<DPSIEGR2_SPEC> {
        DNMIEG_W::new(self, 4)
    }
}
/**Deep Standby Interrupt Edge Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsiegr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIEGR2_SPEC;
impl crate::RegisterSpec for DPSIEGR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsiegr2::R`](R) reader structure
impl crate::Readable for DPSIEGR2_SPEC {}
///`write(|w| ..)` method takes [`dpsiegr2::W`](W) writer structure
impl crate::Writable for DPSIEGR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIEGR2 to value 0
impl crate::Resettable for DPSIEGR2_SPEC {}
