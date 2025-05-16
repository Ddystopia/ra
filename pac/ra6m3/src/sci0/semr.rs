///Register `SEMR` reader
pub type R = crate::R<SEMR_SPEC>;
///Register `SEMR` writer
pub type W = crate::W<SEMR_SPEC>;
/**Bit Modulation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRME_A {
    ///0: Bit rate modulation function is disabled.
    _0 = 0,
    ///1: Bit rate modulation function is enabled.
    _1 = 1,
}
impl From<BRME_A> for bool {
    #[inline(always)]
    fn from(variant: BRME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRME` reader - Bit Modulation Enable
pub type BRME_R = crate::BitReader<BRME_A>;
impl BRME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRME_A {
        match self.bits {
            false => BRME_A::_0,
            true => BRME_A::_1,
        }
    }
    ///Bit rate modulation function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRME_A::_0
    }
    ///Bit rate modulation function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRME_A::_1
    }
}
///Field `BRME` writer - Bit Modulation Enable
pub type BRME_W<'a, REG> = crate::BitWriter<'a, REG, BRME_A>;
impl<'a, REG> BRME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bit rate modulation function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRME_A::_0)
    }
    ///Bit rate modulation function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRME_A::_1)
    }
}
/**Asynchronous Mode Extended Base Clock Select1(Valid only in asynchronous mode and SCR.CKE\[1\]=0)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABCSE_A {
    ///0: Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR.
    _0 = 0,
    ///1: Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator.
    _1 = 1,
}
impl From<ABCSE_A> for bool {
    #[inline(always)]
    fn from(variant: ABCSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ABCSE` reader - Asynchronous Mode Extended Base Clock Select1(Valid only in asynchronous mode and SCR.CKE\[1\]=0)
pub type ABCSE_R = crate::BitReader<ABCSE_A>;
impl ABCSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABCSE_A {
        match self.bits {
            false => ABCSE_A::_0,
            true => ABCSE_A::_1,
        }
    }
    ///Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCSE_A::_0
    }
    ///Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCSE_A::_1
    }
}
///Field `ABCSE` writer - Asynchronous Mode Extended Base Clock Select1(Valid only in asynchronous mode and SCR.CKE\[1\]=0)
pub type ABCSE_W<'a, REG> = crate::BitWriter<'a, REG, ABCSE_A>;
impl<'a, REG> ABCSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ABCSE_A::_0)
    }
    ///Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ABCSE_A::_1)
    }
}
/**Asynchronous Mode Base Clock Select(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABCS_A {
    ///0: Selects 16 base clock cycles for 1-bit period.
    _0 = 0,
    ///1: Selects 8 base clock cycles for 1-bit period.
    _1 = 1,
}
impl From<ABCS_A> for bool {
    #[inline(always)]
    fn from(variant: ABCS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ABCS` reader - Asynchronous Mode Base Clock Select(Valid only in asynchronous mode)
pub type ABCS_R = crate::BitReader<ABCS_A>;
impl ABCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABCS_A {
        match self.bits {
            false => ABCS_A::_0,
            true => ABCS_A::_1,
        }
    }
    ///Selects 16 base clock cycles for 1-bit period.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCS_A::_0
    }
    ///Selects 8 base clock cycles for 1-bit period.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCS_A::_1
    }
}
///Field `ABCS` writer - Asynchronous Mode Base Clock Select(Valid only in asynchronous mode)
pub type ABCS_W<'a, REG> = crate::BitWriter<'a, REG, ABCS_A>;
impl<'a, REG> ABCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selects 16 base clock cycles for 1-bit period.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ABCS_A::_0)
    }
    ///Selects 8 base clock cycles for 1-bit period.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ABCS_A::_1)
    }
}
/**Digital Noise Filter Function Enable(The NFEN bit should be 0 without simple I2C mode and asynchronous mode.)In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    ///0: Noise cancellation function for the RXDn/TXDn input signal is disabled.
    _0 = 0,
    ///1: Noise cancellation function for the RXDn/TXDn input signal is enabled.
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFEN` reader - Digital Noise Filter Function Enable(The NFEN bit should be 0 without simple I2C mode and asynchronous mode.)In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.
pub type NFEN_R = crate::BitReader<NFEN_A>;
impl NFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    ///Noise cancellation function for the RXDn/TXDn input signal is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    ///Noise cancellation function for the RXDn/TXDn input signal is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
///Field `NFEN` writer - Digital Noise Filter Function Enable(The NFEN bit should be 0 without simple I2C mode and asynchronous mode.)In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.
pub type NFEN_W<'a, REG> = crate::BitWriter<'a, REG, NFEN_A>;
impl<'a, REG> NFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Noise cancellation function for the RXDn/TXDn input signal is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_0)
    }
    ///Noise cancellation function for the RXDn/TXDn input signal is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_1)
    }
}
/**Baud Rate Generator Double-Speed Mode Select(Only valid the CKE\[1\] bit in SCR is 0 in asynchronous mode).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGDM_A {
    ///0: Baud rate generator outputs the clock with normal frequency.
    _0 = 0,
    ///1: Baud rate generator outputs the clock with doubled frequency.
    _1 = 1,
}
impl From<BGDM_A> for bool {
    #[inline(always)]
    fn from(variant: BGDM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BGDM` reader - Baud Rate Generator Double-Speed Mode Select(Only valid the CKE\[1\] bit in SCR is 0 in asynchronous mode).
pub type BGDM_R = crate::BitReader<BGDM_A>;
impl BGDM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BGDM_A {
        match self.bits {
            false => BGDM_A::_0,
            true => BGDM_A::_1,
        }
    }
    ///Baud rate generator outputs the clock with normal frequency.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGDM_A::_0
    }
    ///Baud rate generator outputs the clock with doubled frequency.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGDM_A::_1
    }
}
///Field `BGDM` writer - Baud Rate Generator Double-Speed Mode Select(Only valid the CKE\[1\] bit in SCR is 0 in asynchronous mode).
pub type BGDM_W<'a, REG> = crate::BitWriter<'a, REG, BGDM_A>;
impl<'a, REG> BGDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Baud rate generator outputs the clock with normal frequency.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BGDM_A::_0)
    }
    ///Baud rate generator outputs the clock with doubled frequency.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BGDM_A::_1)
    }
}
/**Asynchronous Start Bit Edge Detection Select(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDESEL_A {
    ///0: The low level on the RXDn pin is detected as the start bit.
    _0 = 0,
    ///1: A falling edge on the RXDn pin is detected as the start bit.
    _1 = 1,
}
impl From<RXDESEL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDESEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDESEL` reader - Asynchronous Start Bit Edge Detection Select(Valid only in asynchronous mode)
pub type RXDESEL_R = crate::BitReader<RXDESEL_A>;
impl RXDESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDESEL_A {
        match self.bits {
            false => RXDESEL_A::_0,
            true => RXDESEL_A::_1,
        }
    }
    ///The low level on the RXDn pin is detected as the start bit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDESEL_A::_0
    }
    ///A falling edge on the RXDn pin is detected as the start bit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDESEL_A::_1
    }
}
///Field `RXDESEL` writer - Asynchronous Start Bit Edge Detection Select(Valid only in asynchronous mode)
pub type RXDESEL_W<'a, REG> = crate::BitWriter<'a, REG, RXDESEL_A>;
impl<'a, REG> RXDESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The low level on the RXDn pin is detected as the start bit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDESEL_A::_0)
    }
    ///A falling edge on the RXDn pin is detected as the start bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDESEL_A::_1)
    }
}
impl R {
    ///Bit 2 - Bit Modulation Enable
    #[inline(always)]
    pub fn brme(&self) -> BRME_R {
        BRME_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Asynchronous Mode Extended Base Clock Select1(Valid only in asynchronous mode and SCR.CKE\[1\]=0)
    #[inline(always)]
    pub fn abcse(&self) -> ABCSE_R {
        ABCSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Asynchronous Mode Base Clock Select(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn abcs(&self) -> ABCS_R {
        ABCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Digital Noise Filter Function Enable(The NFEN bit should be 0 without simple I2C mode and asynchronous mode.)In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Baud Rate Generator Double-Speed Mode Select(Only valid the CKE\[1\] bit in SCR is 0 in asynchronous mode).
    #[inline(always)]
    pub fn bgdm(&self) -> BGDM_R {
        BGDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Asynchronous Start Bit Edge Detection Select(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn rxdesel(&self) -> RXDESEL_R {
        RXDESEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Bit Modulation Enable
    #[inline(always)]
    pub fn brme(&mut self) -> BRME_W<SEMR_SPEC> {
        BRME_W::new(self, 2)
    }
    ///Bit 3 - Asynchronous Mode Extended Base Clock Select1(Valid only in asynchronous mode and SCR.CKE\[1\]=0)
    #[inline(always)]
    pub fn abcse(&mut self) -> ABCSE_W<SEMR_SPEC> {
        ABCSE_W::new(self, 3)
    }
    ///Bit 4 - Asynchronous Mode Base Clock Select(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn abcs(&mut self) -> ABCS_W<SEMR_SPEC> {
        ABCS_W::new(self, 4)
    }
    ///Bit 5 - Digital Noise Filter Function Enable(The NFEN bit should be 0 without simple I2C mode and asynchronous mode.)In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.
    #[inline(always)]
    pub fn nfen(&mut self) -> NFEN_W<SEMR_SPEC> {
        NFEN_W::new(self, 5)
    }
    ///Bit 6 - Baud Rate Generator Double-Speed Mode Select(Only valid the CKE\[1\] bit in SCR is 0 in asynchronous mode).
    #[inline(always)]
    pub fn bgdm(&mut self) -> BGDM_W<SEMR_SPEC> {
        BGDM_W::new(self, 6)
    }
    ///Bit 7 - Asynchronous Start Bit Edge Detection Select(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn rxdesel(&mut self) -> RXDESEL_W<SEMR_SPEC> {
        RXDESEL_W::new(self, 7)
    }
}
/**Serial Extended Mode Register

You can [`read`](crate::Reg::read) this register and get [`semr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SEMR_SPEC;
impl crate::RegisterSpec for SEMR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`semr::R`](R) reader structure
impl crate::Readable for SEMR_SPEC {}
///`write(|w| ..)` method takes [`semr::W`](W) writer structure
impl crate::Writable for SEMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEMR to value 0
impl crate::Resettable for SEMR_SPEC {}
