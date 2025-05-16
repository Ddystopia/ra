///Register `SDAMOD` reader
pub type R = crate::R<SDAMOD_SPEC>;
///Register `SDAMOD` writer
pub type W = crate::W<SDAMOD_SPEC>;
/**Continuous Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<BE_A> for bool {
    #[inline(always)]
    fn from(variant: BE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BE` reader - Continuous Access Enable
pub type BE_R = crate::BitReader<BE_A>;
impl BE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BE_A {
        match self.bits {
            false => BE_A::_0,
            true => BE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BE_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BE_A::_1
    }
}
///Field `BE` writer - Continuous Access Enable
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG, BE_A>;
impl<'a, REG> BE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BE_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BE_A::_1)
    }
}
impl R {
    ///Bit 0 - Continuous Access Enable
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Continuous Access Enable
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<SDAMOD_SPEC> {
        BE_W::new(self, 0)
    }
}
/**SDRAM Access Mode Register

You can [`read`](crate::Reg::read) this register and get [`sdamod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdamod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDAMOD_SPEC;
impl crate::RegisterSpec for SDAMOD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdamod::R`](R) reader structure
impl crate::Readable for SDAMOD_SPEC {}
///`write(|w| ..)` method takes [`sdamod::W`](W) writer structure
impl crate::Writable for SDAMOD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDAMOD to value 0
impl crate::Resettable for SDAMOD_SPEC {}
