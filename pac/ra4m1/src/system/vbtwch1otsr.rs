///Register `VBTWCH1OTSR` reader
pub type R = crate::R<VBTWCH1OTSR_SPEC>;
///Register `VBTWCH1OTSR` writer
pub type W = crate::W<VBTWCH1OTSR_SPEC>;
/**VBATWIO1 Output VBATWIO0 Trigger Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1VCH0TE_A {
    ///0: VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled.
    _1 = 1,
}
impl From<CH1VCH0TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH1VCH0TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1VCH0TE` reader - VBATWIO1 Output VBATWIO0 Trigger Enable
pub type CH1VCH0TE_R = crate::BitReader<CH1VCH0TE_A>;
impl CH1VCH0TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1VCH0TE_A {
        match self.bits {
            false => CH1VCH0TE_A::_0,
            true => CH1VCH0TE_A::_1,
        }
    }
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1VCH0TE_A::_0
    }
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1VCH0TE_A::_1
    }
}
///Field `CH1VCH0TE` writer - VBATWIO1 Output VBATWIO0 Trigger Enable
pub type CH1VCH0TE_W<'a, REG> = crate::BitWriter<'a, REG, CH1VCH0TE_A>;
impl<'a, REG> CH1VCH0TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VCH0TE_A::_0)
    }
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VCH0TE_A::_1)
    }
}
/**VBATWIO1 Output VBATWIO2 Trigger Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1VCH2TE_A {
    ///0: VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled.
    _1 = 1,
}
impl From<CH1VCH2TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH1VCH2TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1VCH2TE` reader - VBATWIO1 Output VBATWIO2 Trigger Enable
pub type CH1VCH2TE_R = crate::BitReader<CH1VCH2TE_A>;
impl CH1VCH2TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1VCH2TE_A {
        match self.bits {
            false => CH1VCH2TE_A::_0,
            true => CH1VCH2TE_A::_1,
        }
    }
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1VCH2TE_A::_0
    }
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1VCH2TE_A::_1
    }
}
///Field `CH1VCH2TE` writer - VBATWIO1 Output VBATWIO2 Trigger Enable
pub type CH1VCH2TE_W<'a, REG> = crate::BitWriter<'a, REG, CH1VCH2TE_A>;
impl<'a, REG> CH1VCH2TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VCH2TE_A::_0)
    }
    ///VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VCH2TE_A::_1)
    }
}
/**VBATWIO1 Output RTC Periodic Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1VRTCTE_A {
    ///0: VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled
    _1 = 1,
}
impl From<CH1VRTCTE_A> for bool {
    #[inline(always)]
    fn from(variant: CH1VRTCTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1VRTCTE` reader - VBATWIO1 Output RTC Periodic Signal Enable
pub type CH1VRTCTE_R = crate::BitReader<CH1VRTCTE_A>;
impl CH1VRTCTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1VRTCTE_A {
        match self.bits {
            false => CH1VRTCTE_A::_0,
            true => CH1VRTCTE_A::_1,
        }
    }
    ///VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1VRTCTE_A::_0
    }
    ///VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1VRTCTE_A::_1
    }
}
///Field `CH1VRTCTE` writer - VBATWIO1 Output RTC Periodic Signal Enable
pub type CH1VRTCTE_W<'a, REG> = crate::BitWriter<'a, REG, CH1VRTCTE_A>;
impl<'a, REG> CH1VRTCTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VRTCTE_A::_0)
    }
    ///VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VRTCTE_A::_1)
    }
}
/**VBATWIO1 Output RTC Alarm Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1VRTCATE_A {
    ///0: VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled.
    _1 = 1,
}
impl From<CH1VRTCATE_A> for bool {
    #[inline(always)]
    fn from(variant: CH1VRTCATE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1VRTCATE` reader - VBATWIO1 Output RTC Alarm Signal Enable
pub type CH1VRTCATE_R = crate::BitReader<CH1VRTCATE_A>;
impl CH1VRTCATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1VRTCATE_A {
        match self.bits {
            false => CH1VRTCATE_A::_0,
            true => CH1VRTCATE_A::_1,
        }
    }
    ///VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1VRTCATE_A::_0
    }
    ///VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1VRTCATE_A::_1
    }
}
///Field `CH1VRTCATE` writer - VBATWIO1 Output RTC Alarm Signal Enable
pub type CH1VRTCATE_W<'a, REG> = crate::BitWriter<'a, REG, CH1VRTCATE_A>;
impl<'a, REG> CH1VRTCATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VRTCATE_A::_0)
    }
    ///VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH1VRTCATE_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATWIO1 Output VBATWIO0 Trigger Enable
    #[inline(always)]
    pub fn ch1vch0te(&self) -> CH1VCH0TE_R {
        CH1VCH0TE_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - VBATWIO1 Output VBATWIO2 Trigger Enable
    #[inline(always)]
    pub fn ch1vch2te(&self) -> CH1VCH2TE_R {
        CH1VCH2TE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBATWIO1 Output RTC Periodic Signal Enable
    #[inline(always)]
    pub fn ch1vrtcte(&self) -> CH1VRTCTE_R {
        CH1VRTCTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VBATWIO1 Output RTC Alarm Signal Enable
    #[inline(always)]
    pub fn ch1vrtcate(&self) -> CH1VRTCATE_R {
        CH1VRTCATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATWIO1 Output VBATWIO0 Trigger Enable
    #[inline(always)]
    pub fn ch1vch0te(&mut self) -> CH1VCH0TE_W<VBTWCH1OTSR_SPEC> {
        CH1VCH0TE_W::new(self, 0)
    }
    ///Bit 2 - VBATWIO1 Output VBATWIO2 Trigger Enable
    #[inline(always)]
    pub fn ch1vch2te(&mut self) -> CH1VCH2TE_W<VBTWCH1OTSR_SPEC> {
        CH1VCH2TE_W::new(self, 2)
    }
    ///Bit 3 - VBATWIO1 Output RTC Periodic Signal Enable
    #[inline(always)]
    pub fn ch1vrtcte(&mut self) -> CH1VRTCTE_W<VBTWCH1OTSR_SPEC> {
        CH1VRTCTE_W::new(self, 3)
    }
    ///Bit 4 - VBATWIO1 Output RTC Alarm Signal Enable
    #[inline(always)]
    pub fn ch1vrtcate(&mut self) -> CH1VRTCATE_W<VBTWCH1OTSR_SPEC> {
        CH1VRTCATE_W::new(self, 4)
    }
}
/**VBATT Wakeup I/O 1 Output Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`vbtwch1otsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch1otsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTWCH1OTSR_SPEC;
impl crate::RegisterSpec for VBTWCH1OTSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtwch1otsr::R`](R) reader structure
impl crate::Readable for VBTWCH1OTSR_SPEC {}
///`write(|w| ..)` method takes [`vbtwch1otsr::W`](W) writer structure
impl crate::Writable for VBTWCH1OTSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTWCH1OTSR to value 0
impl crate::Resettable for VBTWCH1OTSR_SPEC {}
