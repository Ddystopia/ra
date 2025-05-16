///Register `LPSTS` reader
pub type R = crate::R<LPSTS_SPEC>;
///Register `LPSTS` writer
pub type W = crate::W<LPSTS_SPEC>;
/**UTMI SuspendM Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPENDM_A {
    ///0: UTMI suspension mode
    _0 = 0,
    ///1: UTMI normal mode
    _1 = 1,
}
impl From<SUSPENDM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPENDM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSPENDM` reader - UTMI SuspendM Control
pub type SUSPENDM_R = crate::BitReader<SUSPENDM_A>;
impl SUSPENDM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSPENDM_A {
        match self.bits {
            false => SUSPENDM_A::_0,
            true => SUSPENDM_A::_1,
        }
    }
    ///UTMI suspension mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUSPENDM_A::_0
    }
    ///UTMI normal mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUSPENDM_A::_1
    }
}
///Field `SUSPENDM` writer - UTMI SuspendM Control
pub type SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG, SUSPENDM_A>;
impl<'a, REG> SUSPENDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UTMI suspension mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPENDM_A::_0)
    }
    ///UTMI normal mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPENDM_A::_1)
    }
}
impl R {
    ///Bit 14 - UTMI SuspendM Control
    #[inline(always)]
    pub fn suspendm(&self) -> SUSPENDM_R {
        SUSPENDM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - UTMI SuspendM Control
    #[inline(always)]
    pub fn suspendm(&mut self) -> SUSPENDM_W<LPSTS_SPEC> {
        SUSPENDM_W::new(self, 14)
    }
}
/**Low Power Status Register

You can [`read`](crate::Reg::read) this register and get [`lpsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LPSTS_SPEC;
impl crate::RegisterSpec for LPSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`lpsts::R`](R) reader structure
impl crate::Readable for LPSTS_SPEC {}
///`write(|w| ..)` method takes [`lpsts::W`](W) writer structure
impl crate::Writable for LPSTS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPSTS to value 0
impl crate::Resettable for LPSTS_SPEC {}
