///Register `SFMCST` reader
pub type R = crate::R<SFMCST_SPEC>;
///Register `SFMCST` writer
pub type W = crate::W<SFMCST_SPEC>;
/**SPI bus cycle completion state in direct communication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMBSY_A {
    ///0: There is no serial transfer being processed.
    _0 = 0,
    ///1: There is a serial transfer being processed.
    _1 = 1,
}
impl From<COMBSY_A> for bool {
    #[inline(always)]
    fn from(variant: COMBSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COMBSY` reader - SPI bus cycle completion state in direct communication
pub type COMBSY_R = crate::BitReader<COMBSY_A>;
impl COMBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMBSY_A {
        match self.bits {
            false => COMBSY_A::_0,
            true => COMBSY_A::_1,
        }
    }
    ///There is no serial transfer being processed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMBSY_A::_0
    }
    ///There is a serial transfer being processed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMBSY_A::_1
    }
}
/**Status of ROM access detection in the direct communication modeNOTE: Writing of 0 only is possible. Writing of 1 is ignored.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EROMR_A {
    ///0: ROM access is not detected in direct communication mode
    _0 = 0,
    ///1: ROM access is detected in direct communication mode
    _1 = 1,
}
impl From<EROMR_A> for bool {
    #[inline(always)]
    fn from(variant: EROMR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EROMR` reader - Status of ROM access detection in the direct communication modeNOTE: Writing of 0 only is possible. Writing of 1 is ignored.
pub type EROMR_R = crate::BitReader<EROMR_A>;
impl EROMR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EROMR_A {
        match self.bits {
            false => EROMR_A::_0,
            true => EROMR_A::_1,
        }
    }
    ///ROM access is not detected in direct communication mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EROMR_A::_0
    }
    ///ROM access is detected in direct communication mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EROMR_A::_1
    }
}
impl R {
    ///Bit 0 - SPI bus cycle completion state in direct communication
    #[inline(always)]
    pub fn combsy(&self) -> COMBSY_R {
        COMBSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Status of ROM access detection in the direct communication modeNOTE: Writing of 0 only is possible. Writing of 1 is ignored.
    #[inline(always)]
    pub fn eromr(&self) -> EROMR_R {
        EROMR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
/**Communication Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmcst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMCST_SPEC;
impl crate::RegisterSpec for SFMCST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmcst::R`](R) reader structure
impl crate::Readable for SFMCST_SPEC {}
///`write(|w| ..)` method takes [`sfmcst::W`](W) writer structure
impl crate::Writable for SFMCST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCST to value 0
impl crate::Resettable for SFMCST_SPEC {}
