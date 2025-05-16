///Register `ELCR` reader
pub type R = crate::R<ELCR_SPEC>;
///Register `ELCR` writer
pub type W = crate::W<ELCR_SPEC>;
/**All Event Link Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELCON_A {
    ///0: Disable ELC function
    _0 = 0,
    ///1: Enable ELC function.
    _1 = 1,
}
impl From<ELCON_A> for bool {
    #[inline(always)]
    fn from(variant: ELCON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELCON` reader - All Event Link Enable
pub type ELCON_R = crate::BitReader<ELCON_A>;
impl ELCON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELCON_A {
        match self.bits {
            false => ELCON_A::_0,
            true => ELCON_A::_1,
        }
    }
    ///Disable ELC function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELCON_A::_0
    }
    ///Enable ELC function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELCON_A::_1
    }
}
///Field `ELCON` writer - All Event Link Enable
pub type ELCON_W<'a, REG> = crate::BitWriter<'a, REG, ELCON_A>;
impl<'a, REG> ELCON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ELC function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELCON_A::_0)
    }
    ///Enable ELC function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELCON_A::_1)
    }
}
impl R {
    ///Bit 7 - All Event Link Enable
    #[inline(always)]
    pub fn elcon(&self) -> ELCON_R {
        ELCON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - All Event Link Enable
    #[inline(always)]
    pub fn elcon(&mut self) -> ELCON_W<ELCR_SPEC> {
        ELCON_W::new(self, 7)
    }
}
/**Event Link Controller Register

You can [`read`](crate::Reg::read) this register and get [`elcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ELCR_SPEC;
impl crate::RegisterSpec for ELCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`elcr::R`](R) reader structure
impl crate::Readable for ELCR_SPEC {}
///`write(|w| ..)` method takes [`elcr::W`](W) writer structure
impl crate::Writable for ELCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELCR to value 0
impl crate::Resettable for ELCR_SPEC {}
