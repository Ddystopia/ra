///Register `RSTSR2` reader
pub type R = crate::R<RSTSR2_SPEC>;
///Register `RSTSR2` writer
pub type W = crate::W<RSTSR2_SPEC>;
/**Cold/Warm Start Determination Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWSF_A {
    ///0: Cold start
    _0 = 0,
    ///1: Warm start
    _1 = 1,
}
impl From<CWSF_A> for bool {
    #[inline(always)]
    fn from(variant: CWSF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CWSF` reader - Cold/Warm Start Determination Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CWSF_R = crate::BitReader<CWSF_A>;
impl CWSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CWSF_A {
        match self.bits {
            false => CWSF_A::_0,
            true => CWSF_A::_1,
        }
    }
    ///Cold start
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWSF_A::_0
    }
    ///Warm start
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWSF_A::_1
    }
}
///Field `CWSF` writer - Cold/Warm Start Determination Flag
pub type CWSF_W<'a, REG> = crate::BitWriter1S<'a, REG, CWSF_A>;
impl<'a, REG> CWSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cold start
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CWSF_A::_0)
    }
    ///Warm start
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CWSF_A::_1)
    }
}
impl R {
    ///Bit 0 - Cold/Warm Start Determination Flag
    #[inline(always)]
    pub fn cwsf(&self) -> CWSF_R {
        CWSF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Cold/Warm Start Determination Flag
    #[inline(always)]
    pub fn cwsf(&mut self) -> CWSF_W<RSTSR2_SPEC> {
        CWSF_W::new(self, 0)
    }
}
/**Reset Status Register 2

You can [`read`](crate::Reg::read) this register and get [`rstsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSTSR2_SPEC;
impl crate::RegisterSpec for RSTSR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rstsr2::R`](R) reader structure
impl crate::Readable for RSTSR2_SPEC {}
///`write(|w| ..)` method takes [`rstsr2::W`](W) writer structure
impl crate::Writable for RSTSR2_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
///`reset()` method sets RSTSR2 to value 0
impl crate::Resettable for RSTSR2_SPEC {}
