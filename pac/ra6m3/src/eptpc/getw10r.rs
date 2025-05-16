///Register `GETW10R` reader
pub type R = crate::R<GETW10R_SPEC>;
///Register `GETW10R` writer
pub type W = crate::W<GETW10R_SPEC>;
/**Worst 10 Acquisition Directive

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GW10_A {
    ///0: The worst-10 values are not acquired.
    _0 = 0,
    ///1: Starts acquisition of the worst-10 values.
    _1 = 1,
}
impl From<GW10_A> for bool {
    #[inline(always)]
    fn from(variant: GW10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GW10` reader - Worst 10 Acquisition Directive
pub type GW10_R = crate::BitReader<GW10_A>;
impl GW10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GW10_A {
        match self.bits {
            false => GW10_A::_0,
            true => GW10_A::_1,
        }
    }
    ///The worst-10 values are not acquired.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GW10_A::_0
    }
    ///Starts acquisition of the worst-10 values.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GW10_A::_1
    }
}
///Field `GW10` writer - Worst 10 Acquisition Directive
pub type GW10_W<'a, REG> = crate::BitWriter<'a, REG, GW10_A>;
impl<'a, REG> GW10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The worst-10 values are not acquired.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GW10_A::_0)
    }
    ///Starts acquisition of the worst-10 values.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GW10_A::_1)
    }
}
impl R {
    ///Bit 0 - Worst 10 Acquisition Directive
    #[inline(always)]
    pub fn gw10(&self) -> GW10_R {
        GW10_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Worst 10 Acquisition Directive
    #[inline(always)]
    pub fn gw10(&mut self) -> GW10_W<GETW10R_SPEC> {
        GW10_W::new(self, 0)
    }
}
/**Worst 10 Acquisition Directive Register

You can [`read`](crate::Reg::read) this register and get [`getw10r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getw10r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GETW10R_SPEC;
impl crate::RegisterSpec for GETW10R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`getw10r::R`](R) reader structure
impl crate::Readable for GETW10R_SPEC {}
///`write(|w| ..)` method takes [`getw10r::W`](W) writer structure
impl crate::Writable for GETW10R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GETW10R to value 0
impl crate::Resettable for GETW10R_SPEC {}
