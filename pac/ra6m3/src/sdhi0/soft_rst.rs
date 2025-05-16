///Register `SOFT_RST` reader
pub type R = crate::R<SOFT_RST_SPEC>;
///Register `SOFT_RST` writer
pub type W = crate::W<SOFT_RST_SPEC>;
/**Software Reset of SD I/F Unit

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDRST_A {
    ///0: Reset
    _0 = 0,
    ///1: Reset released
    _1 = 1,
}
impl From<SDRST_A> for bool {
    #[inline(always)]
    fn from(variant: SDRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDRST` reader - Software Reset of SD I/F Unit
pub type SDRST_R = crate::BitReader<SDRST_A>;
impl SDRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDRST_A {
        match self.bits {
            false => SDRST_A::_0,
            true => SDRST_A::_1,
        }
    }
    ///Reset
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDRST_A::_0
    }
    ///Reset released
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDRST_A::_1
    }
}
///Field `SDRST` writer - Software Reset of SD I/F Unit
pub type SDRST_W<'a, REG> = crate::BitWriter<'a, REG, SDRST_A>;
impl<'a, REG> SDRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDRST_A::_0)
    }
    ///Reset released
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDRST_A::_1)
    }
}
impl R {
    ///Bit 0 - Software Reset of SD I/F Unit
    #[inline(always)]
    pub fn sdrst(&self) -> SDRST_R {
        SDRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software Reset of SD I/F Unit
    #[inline(always)]
    pub fn sdrst(&mut self) -> SDRST_W<SOFT_RST_SPEC> {
        SDRST_W::new(self, 0)
    }
}
/**Software Reset Register

You can [`read`](crate::Reg::read) this register and get [`soft_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SOFT_RST_SPEC;
impl crate::RegisterSpec for SOFT_RST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`soft_rst::R`](R) reader structure
impl crate::Readable for SOFT_RST_SPEC {}
///`write(|w| ..)` method takes [`soft_rst::W`](W) writer structure
impl crate::Writable for SOFT_RST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOFT_RST to value 0x07
impl crate::Resettable for SOFT_RST_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
