///Register `SDRFEN` reader
pub type R = crate::R<SDRFEN_SPEC>;
///Register `SDRFEN` writer
pub type W = crate::W<SDRFEN_SPEC>;
/**Auto-Refresh Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEN_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFEN` reader - Auto-Refresh Operation Enable
pub type RFEN_R = crate::BitReader<RFEN_A>;
impl RFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::_0,
            true => RFEN_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEN_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEN_A::_1
    }
}
///Field `RFEN` writer - Auto-Refresh Operation Enable
pub type RFEN_W<'a, REG> = crate::BitWriter<'a, REG, RFEN_A>;
impl<'a, REG> RFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFEN_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFEN_A::_1)
    }
}
impl R {
    ///Bit 0 - Auto-Refresh Operation Enable
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Auto-Refresh Operation Enable
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W<SDRFEN_SPEC> {
        RFEN_W::new(self, 0)
    }
}
/**SDRAM Auto-Refresh Control Register

You can [`read`](crate::Reg::read) this register and get [`sdrfen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrfen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDRFEN_SPEC;
impl crate::RegisterSpec for SDRFEN_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdrfen::R`](R) reader structure
impl crate::Readable for SDRFEN_SPEC {}
///`write(|w| ..)` method takes [`sdrfen::W`](W) writer structure
impl crate::Writable for SDRFEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDRFEN to value 0
impl crate::Resettable for SDRFEN_SPEC {}
