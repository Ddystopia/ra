///Register `GTDTCR` reader
pub type R = crate::R<GTDTCR_SPEC>;
///Register `GTDTCR` writer
pub type W = crate::W<GTDTCR_SPEC>;
/**Negative-Phase Waveform Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    ///0: Set GTCCRB without using GTDVU and GTDVD.
    _0 = 0,
    ///1: Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB.
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDE` reader - Negative-Phase Waveform Setting
pub type TDE_R = crate::BitReader<TDE_A>;
impl TDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    ///Set GTCCRB without using GTDVU and GTDVD.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    ///Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
///Field `TDE` writer - Negative-Phase Waveform Setting
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG, TDE_A>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set GTCCRB without using GTDVU and GTDVD.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    ///Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
impl R {
    ///Bit 0 - Negative-Phase Waveform Setting
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Negative-Phase Waveform Setting
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<GTDTCR_SPEC> {
        TDE_W::new(self, 0)
    }
}
/**General PWM Timer Dead Time Control Register

You can [`read`](crate::Reg::read) this register and get [`gtdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDTCR_SPEC;
impl crate::RegisterSpec for GTDTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdtcr::R`](R) reader structure
impl crate::Readable for GTDTCR_SPEC {}
///`write(|w| ..)` method takes [`gtdtcr::W`](W) writer structure
impl crate::Writable for GTDTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDTCR to value 0
impl crate::Resettable for GTDTCR_SPEC {}
