///Register `SD_STOP` reader
pub type R = crate::R<SD_STOP_SPEC>;
///Register `SD_STOP` writer
pub type W = crate::W<SD_STOP_SPEC>;
///Field `STP` reader - Stop- When STP is set to 1 during multiple block transfer, CMD12 is issued to halt the transfer through the SD host interface.However, if a command sequence is halted because of a communications error or timeout, CMD12 is not issued. Although continued buffer access is possible even after STP has been set to 1, the buffer access error bit (ERR5 or ERR4) in SD_INFO2 will be set accordingly.- When STP has been set to 1 during transfer for single block write, the access end flag is set when SD_BUF becomes empty, and CMD12 is not issued. If SD_BUF does contain data, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP has been set to 1 during transfer for single block read, the access end flag is set immediately after setting of the STP bit and CMD12 is not issued.- When STP is set to 1 during reception of the busy state after an R1b response, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP is set to 1 after a command sequence has been completed, CMD12 is not issued and the access end flag is not set.- Set STP to 1 after the response end flag has been set.- Set STP to 0 after the response end flag has been set.
pub type STP_R = crate::BitReader;
///Field `STP` writer - Stop- When STP is set to 1 during multiple block transfer, CMD12 is issued to halt the transfer through the SD host interface.However, if a command sequence is halted because of a communications error or timeout, CMD12 is not issued. Although continued buffer access is possible even after STP has been set to 1, the buffer access error bit (ERR5 or ERR4) in SD_INFO2 will be set accordingly.- When STP has been set to 1 during transfer for single block write, the access end flag is set when SD_BUF becomes empty, and CMD12 is not issued. If SD_BUF does contain data, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP has been set to 1 during transfer for single block read, the access end flag is set immediately after setting of the STP bit and CMD12 is not issued.- When STP is set to 1 during reception of the busy state after an R1b response, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP is set to 1 after a command sequence has been completed, CMD12 is not issued and the access end flag is not set.- Set STP to 1 after the response end flag has been set.- Set STP to 0 after the response end flag has been set.
pub type STP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Block Count EnableSet SEC to 1 at multiple block transfer.When SD_CMD is set as follows to start the command sequence while SEC is set to 1, CMD12 is automatically issued to stop multi-block transfer with the number of blocks which is set to SD_SECCNT.1. CMD18 or CMD25 in normal mode (SD_CMD\[10:8\] = 000)2. SD_CMD\[15:13\] = 001 in extended mode (CMD12 is automatically issued, multiple block transfer)When the command sequence is halted because of a communications error or timeout, CMD12 is not automatically issued.NOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_A {
    ///0: Disables SD_SECCNT setting value.
    _0 = 0,
    ///1: Enables SD_SECCNT setting value.
    _1 = 1,
}
impl From<SEC_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEC` reader - Block Count EnableSet SEC to 1 at multiple block transfer.When SD_CMD is set as follows to start the command sequence while SEC is set to 1, CMD12 is automatically issued to stop multi-block transfer with the number of blocks which is set to SD_SECCNT.1. CMD18 or CMD25 in normal mode (SD_CMD\[10:8\] = 000)2. SD_CMD\[15:13\] = 001 in extended mode (CMD12 is automatically issued, multiple block transfer)When the command sequence is halted because of a communications error or timeout, CMD12 is not automatically issued.NOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
pub type SEC_R = crate::BitReader<SEC_A>;
impl SEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEC_A {
        match self.bits {
            false => SEC_A::_0,
            true => SEC_A::_1,
        }
    }
    ///Disables SD_SECCNT setting value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEC_A::_0
    }
    ///Enables SD_SECCNT setting value.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEC_A::_1
    }
}
///Field `SEC` writer - Block Count EnableSet SEC to 1 at multiple block transfer.When SD_CMD is set as follows to start the command sequence while SEC is set to 1, CMD12 is automatically issued to stop multi-block transfer with the number of blocks which is set to SD_SECCNT.1. CMD18 or CMD25 in normal mode (SD_CMD\[10:8\] = 000)2. SD_CMD\[15:13\] = 001 in extended mode (CMD12 is automatically issued, multiple block transfer)When the command sequence is halted because of a communications error or timeout, CMD12 is not automatically issued.NOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG, SEC_A>;
impl<'a, REG> SEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables SD_SECCNT setting value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEC_A::_0)
    }
    ///Enables SD_SECCNT setting value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEC_A::_1)
    }
}
impl R {
    ///Bit 0 - Stop- When STP is set to 1 during multiple block transfer, CMD12 is issued to halt the transfer through the SD host interface.However, if a command sequence is halted because of a communications error or timeout, CMD12 is not issued. Although continued buffer access is possible even after STP has been set to 1, the buffer access error bit (ERR5 or ERR4) in SD_INFO2 will be set accordingly.- When STP has been set to 1 during transfer for single block write, the access end flag is set when SD_BUF becomes empty, and CMD12 is not issued. If SD_BUF does contain data, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP has been set to 1 during transfer for single block read, the access end flag is set immediately after setting of the STP bit and CMD12 is not issued.- When STP is set to 1 during reception of the busy state after an R1b response, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP is set to 1 after a command sequence has been completed, CMD12 is not issued and the access end flag is not set.- Set STP to 1 after the response end flag has been set.- Set STP to 0 after the response end flag has been set.
    #[inline(always)]
    pub fn stp(&self) -> STP_R {
        STP_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Block Count EnableSet SEC to 1 at multiple block transfer.When SD_CMD is set as follows to start the command sequence while SEC is set to 1, CMD12 is automatically issued to stop multi-block transfer with the number of blocks which is set to SD_SECCNT.1. CMD18 or CMD25 in normal mode (SD_CMD\[10:8\] = 000)2. SD_CMD\[15:13\] = 001 in extended mode (CMD12 is automatically issued, multiple block transfer)When the command sequence is halted because of a communications error or timeout, CMD12 is not automatically issued.NOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Stop- When STP is set to 1 during multiple block transfer, CMD12 is issued to halt the transfer through the SD host interface.However, if a command sequence is halted because of a communications error or timeout, CMD12 is not issued. Although continued buffer access is possible even after STP has been set to 1, the buffer access error bit (ERR5 or ERR4) in SD_INFO2 will be set accordingly.- When STP has been set to 1 during transfer for single block write, the access end flag is set when SD_BUF becomes empty, and CMD12 is not issued. If SD_BUF does contain data, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP has been set to 1 during transfer for single block read, the access end flag is set immediately after setting of the STP bit and CMD12 is not issued.- When STP is set to 1 during reception of the busy state after an R1b response, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP is set to 1 after a command sequence has been completed, CMD12 is not issued and the access end flag is not set.- Set STP to 1 after the response end flag has been set.- Set STP to 0 after the response end flag has been set.
    #[inline(always)]
    pub fn stp(&mut self) -> STP_W<SD_STOP_SPEC> {
        STP_W::new(self, 0)
    }
    ///Bit 8 - Block Count EnableSet SEC to 1 at multiple block transfer.When SD_CMD is set as follows to start the command sequence while SEC is set to 1, CMD12 is automatically issued to stop multi-block transfer with the number of blocks which is set to SD_SECCNT.1. CMD18 or CMD25 in normal mode (SD_CMD\[10:8\] = 000)2. SD_CMD\[15:13\] = 001 in extended mode (CMD12 is automatically issued, multiple block transfer)When the command sequence is halted because of a communications error or timeout, CMD12 is not automatically issued.NOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<SD_STOP_SPEC> {
        SEC_W::new(self, 8)
    }
}
/**Data Stop Register

You can [`read`](crate::Reg::read) this register and get [`sd_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_STOP_SPEC;
impl crate::RegisterSpec for SD_STOP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_stop::R`](R) reader structure
impl crate::Readable for SD_STOP_SPEC {}
///`write(|w| ..)` method takes [`sd_stop::W`](W) writer structure
impl crate::Writable for SD_STOP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_STOP to value 0
impl crate::Resettable for SD_STOP_SPEC {}
