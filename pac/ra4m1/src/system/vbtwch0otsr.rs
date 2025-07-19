///Register `VBTWCH0OTSR` reader
pub type R = crate::R<VBTWCH0OTSR_SPEC>;
///Register `VBTWCH0OTSR` writer
pub type W = crate::W<VBTWCH0OTSR_SPEC>;
/**VBATWIO0 Output VBATWIO1 Trigger Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VCH1TE_A {
    ///0: VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled.
    _1 = 1,
}
impl From<CH0VCH1TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VCH1TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH0VCH1TE` reader - VBATWIO0 Output VBATWIO1 Trigger Enable
pub type CH0VCH1TE_R = crate::BitReader<CH0VCH1TE_A>;
impl CH0VCH1TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH0VCH1TE_A {
        match self.bits {
            false => CH0VCH1TE_A::_0,
            true => CH0VCH1TE_A::_1,
        }
    }
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VCH1TE_A::_0
    }
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VCH1TE_A::_1
    }
}
///Field `CH0VCH1TE` writer - VBATWIO0 Output VBATWIO1 Trigger Enable
pub type CH0VCH1TE_W<'a, REG> = crate::BitWriter<'a, REG, CH0VCH1TE_A>;
impl<'a, REG> CH0VCH1TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VCH1TE_A::_0)
    }
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VCH1TE_A::_1)
    }
}
/**VBATWIO0 Output VBATWIO2 Trigger Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VCH2TE_A {
    ///0: VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled.
    _1 = 1,
}
impl From<CH0VCH2TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VCH2TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH0VCH2TE` reader - VBATWIO0 Output VBATWIO2 Trigger Enable
pub type CH0VCH2TE_R = crate::BitReader<CH0VCH2TE_A>;
impl CH0VCH2TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH0VCH2TE_A {
        match self.bits {
            false => CH0VCH2TE_A::_0,
            true => CH0VCH2TE_A::_1,
        }
    }
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VCH2TE_A::_0
    }
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VCH2TE_A::_1
    }
}
///Field `CH0VCH2TE` writer - VBATWIO0 Output VBATWIO2 Trigger Enable
pub type CH0VCH2TE_W<'a, REG> = crate::BitWriter<'a, REG, CH0VCH2TE_A>;
impl<'a, REG> CH0VCH2TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VCH2TE_A::_0)
    }
    ///VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VCH2TE_A::_1)
    }
}
/**VBATWIO0 Output RTC Periodic Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VRTCTE_A {
    ///0: VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled.
    _1 = 1,
}
impl From<CH0VRTCTE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VRTCTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH0VRTCTE` reader - VBATWIO0 Output RTC Periodic Signal Enable
pub type CH0VRTCTE_R = crate::BitReader<CH0VRTCTE_A>;
impl CH0VRTCTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH0VRTCTE_A {
        match self.bits {
            false => CH0VRTCTE_A::_0,
            true => CH0VRTCTE_A::_1,
        }
    }
    ///VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VRTCTE_A::_0
    }
    ///VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VRTCTE_A::_1
    }
}
///Field `CH0VRTCTE` writer - VBATWIO0 Output RTC Periodic Signal Enable
pub type CH0VRTCTE_W<'a, REG> = crate::BitWriter<'a, REG, CH0VRTCTE_A>;
impl<'a, REG> CH0VRTCTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VRTCTE_A::_0)
    }
    ///VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VRTCTE_A::_1)
    }
}
/**VBATWIO0 Output RTC Alarm Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0VRTCATE_A {
    ///0: VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled.
    _1 = 1,
}
impl From<CH0VRTCATE_A> for bool {
    #[inline(always)]
    fn from(variant: CH0VRTCATE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH0VRTCATE` reader - VBATWIO0 Output RTC Alarm Signal Enable
pub type CH0VRTCATE_R = crate::BitReader<CH0VRTCATE_A>;
impl CH0VRTCATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH0VRTCATE_A {
        match self.bits {
            false => CH0VRTCATE_A::_0,
            true => CH0VRTCATE_A::_1,
        }
    }
    ///VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0VRTCATE_A::_0
    }
    ///VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0VRTCATE_A::_1
    }
}
///Field `CH0VRTCATE` writer - VBATWIO0 Output RTC Alarm Signal Enable
pub type CH0VRTCATE_W<'a, REG> = crate::BitWriter<'a, REG, CH0VRTCATE_A>;
impl<'a, REG> CH0VRTCATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VRTCATE_A::_0)
    }
    ///VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH0VRTCATE_A::_1)
    }
}
impl R {
    ///Bit 1 - VBATWIO0 Output VBATWIO1 Trigger Enable
    #[inline(always)]
    pub fn ch0vch1te(&self) -> CH0VCH1TE_R {
        CH0VCH1TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBATWIO0 Output VBATWIO2 Trigger Enable
    #[inline(always)]
    pub fn ch0vch2te(&self) -> CH0VCH2TE_R {
        CH0VCH2TE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBATWIO0 Output RTC Periodic Signal Enable
    #[inline(always)]
    pub fn ch0vrtcte(&self) -> CH0VRTCTE_R {
        CH0VRTCTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VBATWIO0 Output RTC Alarm Signal Enable
    #[inline(always)]
    pub fn ch0vrtcate(&self) -> CH0VRTCATE_R {
        CH0VRTCATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - VBATWIO0 Output VBATWIO1 Trigger Enable
    #[inline(always)]
    pub fn ch0vch1te(&mut self) -> CH0VCH1TE_W<'_, VBTWCH0OTSR_SPEC> {
        CH0VCH1TE_W::new(self, 1)
    }
    ///Bit 2 - VBATWIO0 Output VBATWIO2 Trigger Enable
    #[inline(always)]
    pub fn ch0vch2te(&mut self) -> CH0VCH2TE_W<'_, VBTWCH0OTSR_SPEC> {
        CH0VCH2TE_W::new(self, 2)
    }
    ///Bit 3 - VBATWIO0 Output RTC Periodic Signal Enable
    #[inline(always)]
    pub fn ch0vrtcte(&mut self) -> CH0VRTCTE_W<'_, VBTWCH0OTSR_SPEC> {
        CH0VRTCTE_W::new(self, 3)
    }
    ///Bit 4 - VBATWIO0 Output RTC Alarm Signal Enable
    #[inline(always)]
    pub fn ch0vrtcate(&mut self) -> CH0VRTCATE_W<'_, VBTWCH0OTSR_SPEC> {
        CH0VRTCATE_W::new(self, 4)
    }
}
/**VBATT Wakeup I/O 0 Output Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`vbtwch0otsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch0otsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTWCH0OTSR_SPEC;
impl crate::RegisterSpec for VBTWCH0OTSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtwch0otsr::R`](R) reader structure
impl crate::Readable for VBTWCH0OTSR_SPEC {}
///`write(|w| ..)` method takes [`vbtwch0otsr::W`](W) writer structure
impl crate::Writable for VBTWCH0OTSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTWCH0OTSR to value 0
impl crate::Resettable for VBTWCH0OTSR_SPEC {}
