///Register `GTSOTR` reader
pub type R = crate::R<GTSOTR_SPEC>;
///Register `GTSOTR` writer
pub type W = crate::W<GTSOTR_SPEC>;
/**Output Protection Function Temporary Release

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOTR_A {
    ///0: Do not release protected state
    _0 = 0,
    ///1: Release protected state
    _1 = 1,
}
impl From<SOTR_A> for bool {
    #[inline(always)]
    fn from(variant: SOTR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOTR` reader - Output Protection Function Temporary Release
pub type SOTR_R = crate::BitReader<SOTR_A>;
impl SOTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOTR_A {
        match self.bits {
            false => SOTR_A::_0,
            true => SOTR_A::_1,
        }
    }
    ///Do not release protected state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOTR_A::_0
    }
    ///Release protected state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOTR_A::_1
    }
}
///Field `SOTR` writer - Output Protection Function Temporary Release
pub type SOTR_W<'a, REG> = crate::BitWriter<'a, REG, SOTR_A>;
impl<'a, REG> SOTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not release protected state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOTR_A::_0)
    }
    ///Release protected state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOTR_A::_1)
    }
}
impl R {
    ///Bit 0 - Output Protection Function Temporary Release
    #[inline(always)]
    pub fn sotr(&self) -> SOTR_R {
        SOTR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Output Protection Function Temporary Release
    #[inline(always)]
    pub fn sotr(&mut self) -> SOTR_W<GTSOTR_SPEC> {
        SOTR_W::new(self, 0)
    }
}
/**General PWM Timer Output Protection Function Temporary Release Register

You can [`read`](crate::Reg::read) this register and get [`gtsotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtsotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTSOTR_SPEC;
impl crate::RegisterSpec for GTSOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtsotr::R`](R) reader structure
impl crate::Readable for GTSOTR_SPEC {}
///`write(|w| ..)` method takes [`gtsotr::W`](W) writer structure
impl crate::Writable for GTSOTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSOTR to value 0
impl crate::Resettable for GTSOTR_SPEC {}
