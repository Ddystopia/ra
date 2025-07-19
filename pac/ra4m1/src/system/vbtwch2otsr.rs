///Register `VBTWCH2OTSR` reader
pub type R = crate::R<VBTWCH2OTSR_SPEC>;
///Register `VBTWCH2OTSR` writer
pub type W = crate::W<VBTWCH2OTSR_SPEC>;
/**VBATWIO2 Output VBATWIO0 Trigger Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VCH0TE_A {
    ///0: VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled.
    _1 = 1,
}
impl From<CH2VCH0TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VCH0TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH2VCH0TE` reader - VBATWIO2 Output VBATWIO0 Trigger Enable
pub type CH2VCH0TE_R = crate::BitReader<CH2VCH0TE_A>;
impl CH2VCH0TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH2VCH0TE_A {
        match self.bits {
            false => CH2VCH0TE_A::_0,
            true => CH2VCH0TE_A::_1,
        }
    }
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VCH0TE_A::_0
    }
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VCH0TE_A::_1
    }
}
///Field `CH2VCH0TE` writer - VBATWIO2 Output VBATWIO0 Trigger Enable
pub type CH2VCH0TE_W<'a, REG> = crate::BitWriter<'a, REG, CH2VCH0TE_A>;
impl<'a, REG> CH2VCH0TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH0TE_A::_0)
    }
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH0TE_A::_1)
    }
}
/**VBATWIO2 Output VBATWIO1 Trigger Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VCH1TE_A {
    ///0: VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled.
    _1 = 1,
}
impl From<CH2VCH1TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VCH1TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH2VCH1TE` reader - VBATWIO2 Output VBATWIO1 Trigger Enable
pub type CH2VCH1TE_R = crate::BitReader<CH2VCH1TE_A>;
impl CH2VCH1TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH2VCH1TE_A {
        match self.bits {
            false => CH2VCH1TE_A::_0,
            true => CH2VCH1TE_A::_1,
        }
    }
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VCH1TE_A::_0
    }
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VCH1TE_A::_1
    }
}
///Field `CH2VCH1TE` writer - VBATWIO2 Output VBATWIO1 Trigger Enable
pub type CH2VCH1TE_W<'a, REG> = crate::BitWriter<'a, REG, CH2VCH1TE_A>;
impl<'a, REG> CH2VCH1TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH1TE_A::_0)
    }
    ///VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH1TE_A::_1)
    }
}
/**VBATWIO2 Output RTC Periodic Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VRTCTE_A {
    ///0: VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled.
    _1 = 1,
}
impl From<CH2VRTCTE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VRTCTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH2VRTCTE` reader - VBATWIO2 Output RTC Periodic Signal Enable
pub type CH2VRTCTE_R = crate::BitReader<CH2VRTCTE_A>;
impl CH2VRTCTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH2VRTCTE_A {
        match self.bits {
            false => CH2VRTCTE_A::_0,
            true => CH2VRTCTE_A::_1,
        }
    }
    ///VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VRTCTE_A::_0
    }
    ///VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VRTCTE_A::_1
    }
}
///Field `CH2VRTCTE` writer - VBATWIO2 Output RTC Periodic Signal Enable
pub type CH2VRTCTE_W<'a, REG> = crate::BitWriter<'a, REG, CH2VRTCTE_A>;
impl<'a, REG> CH2VRTCTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCTE_A::_0)
    }
    ///VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCTE_A::_1)
    }
}
/**VBATWIO2 Output RTC Alarm Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VRTCATE_A {
    ///0: VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled
    _0 = 0,
    ///1: VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled.
    _1 = 1,
}
impl From<CH2VRTCATE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VRTCATE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH2VRTCATE` reader - VBATWIO2 Output RTC Alarm Signal Enable
pub type CH2VRTCATE_R = crate::BitReader<CH2VRTCATE_A>;
impl CH2VRTCATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH2VRTCATE_A {
        match self.bits {
            false => CH2VRTCATE_A::_0,
            true => CH2VRTCATE_A::_1,
        }
    }
    ///VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VRTCATE_A::_0
    }
    ///VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VRTCATE_A::_1
    }
}
///Field `CH2VRTCATE` writer - VBATWIO2 Output RTC Alarm Signal Enable
pub type CH2VRTCATE_W<'a, REG> = crate::BitWriter<'a, REG, CH2VRTCATE_A>;
impl<'a, REG> CH2VRTCATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCATE_A::_0)
    }
    ///VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCATE_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATWIO2 Output VBATWIO0 Trigger Enable
    #[inline(always)]
    pub fn ch2vch0te(&self) -> CH2VCH0TE_R {
        CH2VCH0TE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBATWIO2 Output VBATWIO1 Trigger Enable
    #[inline(always)]
    pub fn ch2vch1te(&self) -> CH2VCH1TE_R {
        CH2VCH1TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - VBATWIO2 Output RTC Periodic Signal Enable
    #[inline(always)]
    pub fn ch2vrtcte(&self) -> CH2VRTCTE_R {
        CH2VRTCTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VBATWIO2 Output RTC Alarm Signal Enable
    #[inline(always)]
    pub fn ch2vrtcate(&self) -> CH2VRTCATE_R {
        CH2VRTCATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATWIO2 Output VBATWIO0 Trigger Enable
    #[inline(always)]
    pub fn ch2vch0te(&mut self) -> CH2VCH0TE_W<'_, VBTWCH2OTSR_SPEC> {
        CH2VCH0TE_W::new(self, 0)
    }
    ///Bit 1 - VBATWIO2 Output VBATWIO1 Trigger Enable
    #[inline(always)]
    pub fn ch2vch1te(&mut self) -> CH2VCH1TE_W<'_, VBTWCH2OTSR_SPEC> {
        CH2VCH1TE_W::new(self, 1)
    }
    ///Bit 3 - VBATWIO2 Output RTC Periodic Signal Enable
    #[inline(always)]
    pub fn ch2vrtcte(&mut self) -> CH2VRTCTE_W<'_, VBTWCH2OTSR_SPEC> {
        CH2VRTCTE_W::new(self, 3)
    }
    ///Bit 4 - VBATWIO2 Output RTC Alarm Signal Enable
    #[inline(always)]
    pub fn ch2vrtcate(&mut self) -> CH2VRTCATE_W<'_, VBTWCH2OTSR_SPEC> {
        CH2VRTCATE_W::new(self, 4)
    }
}
/**VBATT Wakeup I/O 2 Output Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`vbtwch2otsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch2otsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTWCH2OTSR_SPEC;
impl crate::RegisterSpec for VBTWCH2OTSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtwch2otsr::R`](R) reader structure
impl crate::Readable for VBTWCH2OTSR_SPEC {}
///`write(|w| ..)` method takes [`vbtwch2otsr::W`](W) writer structure
impl crate::Writable for VBTWCH2OTSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTWCH2OTSR to value 0
impl crate::Resettable for VBTWCH2OTSR_SPEC {}
