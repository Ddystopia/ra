///Register `ADSHCR` reader
pub type R = crate::R<ADSHCR_SPEC>;
///Register `ADSHCR` writer
pub type W = crate::W<ADSHCR_SPEC>;
///Field `SSTSH` reader - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)
pub type SSTSH_R = crate::FieldReader;
///Field `SSTSH` writer - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)
pub type SSTSH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**AN100 sample-and-hold circuit Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHANS0_A {
    ///0: Bypass the sample-and-hold circuit.
    _0 = 0,
    ///1: Use the sample-and-hold circuit.
    _1 = 1,
}
impl From<SHANS0_A> for bool {
    #[inline(always)]
    fn from(variant: SHANS0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHANS0` reader - AN100 sample-and-hold circuit Select
pub type SHANS0_R = crate::BitReader<SHANS0_A>;
impl SHANS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHANS0_A {
        match self.bits {
            false => SHANS0_A::_0,
            true => SHANS0_A::_1,
        }
    }
    ///Bypass the sample-and-hold circuit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHANS0_A::_0
    }
    ///Use the sample-and-hold circuit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHANS0_A::_1
    }
}
///Field `SHANS0` writer - AN100 sample-and-hold circuit Select
pub type SHANS0_W<'a, REG> = crate::BitWriter<'a, REG, SHANS0_A>;
impl<'a, REG> SHANS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass the sample-and-hold circuit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHANS0_A::_0)
    }
    ///Use the sample-and-hold circuit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHANS0_A::_1)
    }
}
/**AN101 sample-and-hold circuit Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHANS1_A {
    ///0: Bypass the sample-and-hold circuit.
    _0 = 0,
    ///1: Use the sample-and-hold circuit.
    _1 = 1,
}
impl From<SHANS1_A> for bool {
    #[inline(always)]
    fn from(variant: SHANS1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHANS1` reader - AN101 sample-and-hold circuit Select
pub type SHANS1_R = crate::BitReader<SHANS1_A>;
impl SHANS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHANS1_A {
        match self.bits {
            false => SHANS1_A::_0,
            true => SHANS1_A::_1,
        }
    }
    ///Bypass the sample-and-hold circuit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHANS1_A::_0
    }
    ///Use the sample-and-hold circuit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHANS1_A::_1
    }
}
///Field `SHANS1` writer - AN101 sample-and-hold circuit Select
pub type SHANS1_W<'a, REG> = crate::BitWriter<'a, REG, SHANS1_A>;
impl<'a, REG> SHANS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass the sample-and-hold circuit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHANS1_A::_0)
    }
    ///Use the sample-and-hold circuit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHANS1_A::_1)
    }
}
/**AN102 sample-and-hold circuit Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHANS2_A {
    ///0: Bypass the sample-and-hold circuit.
    _0 = 0,
    ///1: Use the sample-and-hold circuit.
    _1 = 1,
}
impl From<SHANS2_A> for bool {
    #[inline(always)]
    fn from(variant: SHANS2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHANS2` reader - AN102 sample-and-hold circuit Select
pub type SHANS2_R = crate::BitReader<SHANS2_A>;
impl SHANS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHANS2_A {
        match self.bits {
            false => SHANS2_A::_0,
            true => SHANS2_A::_1,
        }
    }
    ///Bypass the sample-and-hold circuit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHANS2_A::_0
    }
    ///Use the sample-and-hold circuit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHANS2_A::_1
    }
}
///Field `SHANS2` writer - AN102 sample-and-hold circuit Select
pub type SHANS2_W<'a, REG> = crate::BitWriter<'a, REG, SHANS2_A>;
impl<'a, REG> SHANS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass the sample-and-hold circuit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHANS2_A::_0)
    }
    ///Use the sample-and-hold circuit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHANS2_A::_1)
    }
}
impl R {
    ///Bits 0:7 - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)
    #[inline(always)]
    pub fn sstsh(&self) -> SSTSH_R {
        SSTSH_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - AN100 sample-and-hold circuit Select
    #[inline(always)]
    pub fn shans0(&self) -> SHANS0_R {
        SHANS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AN101 sample-and-hold circuit Select
    #[inline(always)]
    pub fn shans1(&self) -> SHANS1_R {
        SHANS1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AN102 sample-and-hold circuit Select
    #[inline(always)]
    pub fn shans2(&self) -> SHANS2_R {
        SHANS2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting Set the sampling time (4 to 255 states)
    #[inline(always)]
    pub fn sstsh(&mut self) -> SSTSH_W<ADSHCR_SPEC> {
        SSTSH_W::new(self, 0)
    }
    ///Bit 8 - AN100 sample-and-hold circuit Select
    #[inline(always)]
    pub fn shans0(&mut self) -> SHANS0_W<ADSHCR_SPEC> {
        SHANS0_W::new(self, 8)
    }
    ///Bit 9 - AN101 sample-and-hold circuit Select
    #[inline(always)]
    pub fn shans1(&mut self) -> SHANS1_W<ADSHCR_SPEC> {
        SHANS1_W::new(self, 9)
    }
    ///Bit 10 - AN102 sample-and-hold circuit Select
    #[inline(always)]
    pub fn shans2(&mut self) -> SHANS2_W<ADSHCR_SPEC> {
        SHANS2_W::new(self, 10)
    }
}
/**A/D Sample and Hold Circuit Control Register

You can [`read`](crate::Reg::read) this register and get [`adshcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adshcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSHCR_SPEC;
impl crate::RegisterSpec for ADSHCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adshcr::R`](R) reader structure
impl crate::Readable for ADSHCR_SPEC {}
///`write(|w| ..)` method takes [`adshcr::W`](W) writer structure
impl crate::Writable for ADSHCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSHCR to value 0x18
impl crate::Resettable for ADSHCR_SPEC {
    const RESET_VALUE: u16 = 0x18;
}
