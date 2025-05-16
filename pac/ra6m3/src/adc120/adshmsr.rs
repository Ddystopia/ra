///Register `ADSHMSR` reader
pub type R = crate::R<ADSHMSR_SPEC>;
///Register `ADSHMSR` writer
pub type W = crate::W<ADSHMSR_SPEC>;
/**Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD_A {
    ///0: Sampling by channel-dedicated sample-and-hold circuit is disable.
    _0 = 0,
    ///1: Sampling by channel-dedicated sample-and-hold circuit is enable.
    _1 = 1,
}
impl From<SHMD_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHMD` reader - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select
pub type SHMD_R = crate::BitReader<SHMD_A>;
impl SHMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHMD_A {
        match self.bits {
            false => SHMD_A::_0,
            true => SHMD_A::_1,
        }
    }
    ///Sampling by channel-dedicated sample-and-hold circuit is disable.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD_A::_0
    }
    ///Sampling by channel-dedicated sample-and-hold circuit is enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD_A::_1
    }
}
///Field `SHMD` writer - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select
pub type SHMD_W<'a, REG> = crate::BitWriter<'a, REG, SHMD_A>;
impl<'a, REG> SHMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling by channel-dedicated sample-and-hold circuit is disable.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHMD_A::_0)
    }
    ///Sampling by channel-dedicated sample-and-hold circuit is enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHMD_A::_1)
    }
}
impl R {
    ///Bit 0 - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select
    #[inline(always)]
    pub fn shmd(&self) -> SHMD_R {
        SHMD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select
    #[inline(always)]
    pub fn shmd(&mut self) -> SHMD_W<ADSHMSR_SPEC> {
        SHMD_W::new(self, 0)
    }
}
/**A/D Sample and Hold Operation Mode Select Register

You can [`read`](crate::Reg::read) this register and get [`adshmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adshmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSHMSR_SPEC;
impl crate::RegisterSpec for ADSHMSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adshmsr::R`](R) reader structure
impl crate::Readable for ADSHMSR_SPEC {}
///`write(|w| ..)` method takes [`adshmsr::W`](W) writer structure
impl crate::Writable for ADSHMSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSHMSR to value 0
impl crate::Resettable for ADSHMSR_SPEC {}
