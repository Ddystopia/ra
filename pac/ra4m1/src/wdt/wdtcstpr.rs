///Register `WDTCSTPR` reader
pub type R = crate::R<WDTCSTPR_SPEC>;
///Register `WDTCSTPR` writer
pub type W = crate::W<WDTCSTPR_SPEC>;
/**Sleep-Mode Count Stop Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLCSTP_A {
    ///0: Count stop is disabled.
    _0 = 0,
    ///1: Count is stopped at a transition to sleep mode.
    _1 = 1,
}
impl From<SLCSTP_A> for bool {
    #[inline(always)]
    fn from(variant: SLCSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLCSTP` reader - Sleep-Mode Count Stop Control
pub type SLCSTP_R = crate::BitReader<SLCSTP_A>;
impl SLCSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLCSTP_A {
        match self.bits {
            false => SLCSTP_A::_0,
            true => SLCSTP_A::_1,
        }
    }
    ///Count stop is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLCSTP_A::_0
    }
    ///Count is stopped at a transition to sleep mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLCSTP_A::_1
    }
}
///Field `SLCSTP` writer - Sleep-Mode Count Stop Control
pub type SLCSTP_W<'a, REG> = crate::BitWriter<'a, REG, SLCSTP_A>;
impl<'a, REG> SLCSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count stop is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLCSTP_A::_0)
    }
    ///Count is stopped at a transition to sleep mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLCSTP_A::_1)
    }
}
impl R {
    ///Bit 7 - Sleep-Mode Count Stop Control
    #[inline(always)]
    pub fn slcstp(&self) -> SLCSTP_R {
        SLCSTP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - Sleep-Mode Count Stop Control
    #[inline(always)]
    pub fn slcstp(&mut self) -> SLCSTP_W<WDTCSTPR_SPEC> {
        SLCSTP_W::new(self, 7)
    }
}
/**WDT Count Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcstpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcstpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDTCSTPR_SPEC;
impl crate::RegisterSpec for WDTCSTPR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`wdtcstpr::R`](R) reader structure
impl crate::Readable for WDTCSTPR_SPEC {}
///`write(|w| ..)` method takes [`wdtcstpr::W`](W) writer structure
impl crate::Writable for WDTCSTPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTCSTPR to value 0x80
impl crate::Resettable for WDTCSTPR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
