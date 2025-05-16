///Register `DTCST` reader
pub type R = crate::R<DTCST_SPEC>;
///Register `DTCST` writer
pub type W = crate::W<DTCST_SPEC>;
/**DTC Module Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCST_A {
    ///0: DTC module stop
    _0 = 0,
    ///1: DTC module start
    _1 = 1,
}
impl From<DTCST_A> for bool {
    #[inline(always)]
    fn from(variant: DTCST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCST` reader - DTC Module Start
pub type DTCST_R = crate::BitReader<DTCST_A>;
impl DTCST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCST_A {
        match self.bits {
            false => DTCST_A::_0,
            true => DTCST_A::_1,
        }
    }
    ///DTC module stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCST_A::_0
    }
    ///DTC module start
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCST_A::_1
    }
}
///Field `DTCST` writer - DTC Module Start
pub type DTCST_W<'a, REG> = crate::BitWriter<'a, REG, DTCST_A>;
impl<'a, REG> DTCST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTC module stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCST_A::_0)
    }
    ///DTC module start
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCST_A::_1)
    }
}
impl R {
    ///Bit 0 - DTC Module Start
    #[inline(always)]
    pub fn dtcst(&self) -> DTCST_R {
        DTCST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DTC Module Start
    #[inline(always)]
    pub fn dtcst(&mut self) -> DTCST_W<DTCST_SPEC> {
        DTCST_W::new(self, 0)
    }
}
/**DTC Module Start Register

You can [`read`](crate::Reg::read) this register and get [`dtcst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DTCST_SPEC;
impl crate::RegisterSpec for DTCST_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dtcst::R`](R) reader structure
impl crate::Readable for DTCST_SPEC {}
///`write(|w| ..)` method takes [`dtcst::W`](W) writer structure
impl crate::Writable for DTCST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTCST to value 0
impl crate::Resettable for DTCST_SPEC {}
