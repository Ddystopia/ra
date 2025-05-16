///Register `SDSELF` reader
pub type R = crate::R<SDSELF_SPEC>;
///Register `SDSELF` writer
pub type W = crate::W<SDSELF_SPEC>;
/**SDRAM Self-Refresh Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFEN_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<SFEN_A> for bool {
    #[inline(always)]
    fn from(variant: SFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFEN` reader - SDRAM Self-Refresh Enable
pub type SFEN_R = crate::BitReader<SFEN_A>;
impl SFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFEN_A {
        match self.bits {
            false => SFEN_A::_0,
            true => SFEN_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFEN_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFEN_A::_1
    }
}
///Field `SFEN` writer - SDRAM Self-Refresh Enable
pub type SFEN_W<'a, REG> = crate::BitWriter<'a, REG, SFEN_A>;
impl<'a, REG> SFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFEN_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFEN_A::_1)
    }
}
impl R {
    ///Bit 0 - SDRAM Self-Refresh Enable
    #[inline(always)]
    pub fn sfen(&self) -> SFEN_R {
        SFEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SDRAM Self-Refresh Enable
    #[inline(always)]
    pub fn sfen(&mut self) -> SFEN_W<SDSELF_SPEC> {
        SFEN_W::new(self, 0)
    }
}
/**SDRAM Self-Refresh Control Register

You can [`read`](crate::Reg::read) this register and get [`sdself::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdself::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDSELF_SPEC;
impl crate::RegisterSpec for SDSELF_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdself::R`](R) reader structure
impl crate::Readable for SDSELF_SPEC {}
///`write(|w| ..)` method takes [`sdself::W`](W) writer structure
impl crate::Writable for SDSELF_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDSELF to value 0
impl crate::Resettable for SDSELF_SPEC {}
