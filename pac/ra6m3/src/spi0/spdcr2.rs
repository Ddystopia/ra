///Register `SPDCR2` reader
pub type R = crate::R<SPDCR2_SPEC>;
///Register `SPDCR2` writer
pub type W = crate::W<SPDCR2_SPEC>;
/**Byte Swap Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYSW_A {
    ///0: Byte Swap Operating Mode disabled
    _0 = 0,
    ///1: Byte Swap Operating Mode enabled
    _1 = 1,
}
impl From<BYSW_A> for bool {
    #[inline(always)]
    fn from(variant: BYSW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BYSW` reader - Byte Swap Operating Mode Select
pub type BYSW_R = crate::BitReader<BYSW_A>;
impl BYSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BYSW_A {
        match self.bits {
            false => BYSW_A::_0,
            true => BYSW_A::_1,
        }
    }
    ///Byte Swap Operating Mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYSW_A::_0
    }
    ///Byte Swap Operating Mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYSW_A::_1
    }
}
///Field `BYSW` writer - Byte Swap Operating Mode Select
pub type BYSW_W<'a, REG> = crate::BitWriter<'a, REG, BYSW_A>;
impl<'a, REG> BYSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Byte Swap Operating Mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BYSW_A::_0)
    }
    ///Byte Swap Operating Mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BYSW_A::_1)
    }
}
impl R {
    ///Bit 0 - Byte Swap Operating Mode Select
    #[inline(always)]
    pub fn bysw(&self) -> BYSW_R {
        BYSW_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Byte Swap Operating Mode Select
    #[inline(always)]
    pub fn bysw(&mut self) -> BYSW_W<SPDCR2_SPEC> {
        BYSW_W::new(self, 0)
    }
}
/**SPI Data Control Register 2

You can [`read`](crate::Reg::read) this register and get [`spdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPDCR2_SPEC;
impl crate::RegisterSpec for SPDCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spdcr2::R`](R) reader structure
impl crate::Readable for SPDCR2_SPEC {}
///`write(|w| ..)` method takes [`spdcr2::W`](W) writer structure
impl crate::Writable for SPDCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDCR2 to value 0
impl crate::Resettable for SPDCR2_SPEC {}
