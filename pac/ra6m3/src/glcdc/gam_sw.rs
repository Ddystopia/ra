///Register `GAM_SW` reader
pub type R = crate::R<GAM_SW_SPEC>;
///Register `GAM_SW` writer
pub type W = crate::W<GAM_SW_SPEC>;
/**Gamma correction on/off control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAMON_A {
    ///1: Turns on gamma correction.
    _1 = 1,
    ///0: Turns off gamma correction.
    _0 = 0,
}
impl From<GAMON_A> for bool {
    #[inline(always)]
    fn from(variant: GAMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GAMON` reader - Gamma correction on/off control
pub type GAMON_R = crate::BitReader<GAMON_A>;
impl GAMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GAMON_A {
        match self.bits {
            true => GAMON_A::_1,
            false => GAMON_A::_0,
        }
    }
    ///Turns on gamma correction.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAMON_A::_1
    }
    ///Turns off gamma correction.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAMON_A::_0
    }
}
///Field `GAMON` writer - Gamma correction on/off control
pub type GAMON_W<'a, REG> = crate::BitWriter<'a, REG, GAMON_A>;
impl<'a, REG> GAMON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Turns on gamma correction.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GAMON_A::_1)
    }
    ///Turns off gamma correction.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GAMON_A::_0)
    }
}
impl R {
    ///Bit 0 - Gamma correction on/off control
    #[inline(always)]
    pub fn gamon(&self) -> GAMON_R {
        GAMON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Gamma correction on/off control
    #[inline(always)]
    pub fn gamon(&mut self) -> GAMON_W<GAM_SW_SPEC> {
        GAMON_W::new(self, 0)
    }
}
/**Gamma Correction Block Function Switch Register

You can [`read`](crate::Reg::read) this register and get [`gam_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_SW_SPEC;
impl crate::RegisterSpec for GAM_SW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_sw::R`](R) reader structure
impl crate::Readable for GAM_SW_SPEC {}
///`write(|w| ..)` method takes [`gam_sw::W`](W) writer structure
impl crate::Writable for GAM_SW_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAM_SW to value 0
impl crate::Resettable for GAM_SW_SPEC {}
