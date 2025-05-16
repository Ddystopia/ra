///Register `SIMR1` reader
pub type R = crate::R<SIMR1_SPEC>;
///Register `SIMR1` writer
pub type W = crate::W<SIMR1_SPEC>;
/**Simple I2C Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICM_A {
    ///0: Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)
    _0 = 0,
    ///1: Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)
    _1 = 1,
}
impl From<IICM_A> for bool {
    #[inline(always)]
    fn from(variant: IICM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICM` reader - Simple I2C Mode Select
pub type IICM_R = crate::BitReader<IICM_A>;
impl IICM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICM_A {
        match self.bits {
            false => IICM_A::_0,
            true => IICM_A::_1,
        }
    }
    ///Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICM_A::_0
    }
    ///Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICM_A::_1
    }
}
///Field `IICM` writer - Simple I2C Mode Select
pub type IICM_W<'a, REG> = crate::BitWriter<'a, REG, IICM_A>;
impl<'a, REG> IICM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICM_A::_0)
    }
    ///Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICM_A::_1)
    }
}
/**SSDA Delay Output SelectCycles below are of the clock signal from the on-chip baud rate generator.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IICDL_A {
    ///0: No output delay
    _00000 = 0,
    ///1: (IICDL - 1 ) to IIDCDL cycles. The delay is in the clock cycles from the on-chip baud rate generator.
    OTHERS = 1,
}
impl From<IICDL_A> for u8 {
    #[inline(always)]
    fn from(variant: IICDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IICDL_A {
    type Ux = u8;
}
impl crate::IsEnum for IICDL_A {}
///Field `IICDL` reader - SSDA Delay Output SelectCycles below are of the clock signal from the on-chip baud rate generator.
pub type IICDL_R = crate::FieldReader<IICDL_A>;
impl IICDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICDL_A {
        match self.bits {
            0 => IICDL_A::_00000,
            _ => IICDL_A::OTHERS,
        }
    }
    ///No output delay
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == IICDL_A::_00000
    }
    ///(IICDL - 1 ) to IIDCDL cycles. The delay is in the clock cycles from the on-chip baud rate generator.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), IICDL_A::OTHERS)
    }
}
///Field `IICDL` writer - SSDA Delay Output SelectCycles below are of the clock signal from the on-chip baud rate generator.
pub type IICDL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, IICDL_A, crate::Safe>;
impl<'a, REG> IICDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No output delay
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(IICDL_A::_00000)
    }
    ///(IICDL - 1 ) to IIDCDL cycles. The delay is in the clock cycles from the on-chip baud rate generator.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(IICDL_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Simple I2C Mode Select
    #[inline(always)]
    pub fn iicm(&self) -> IICM_R {
        IICM_R::new((self.bits & 1) != 0)
    }
    ///Bits 3:7 - SSDA Delay Output SelectCycles below are of the clock signal from the on-chip baud rate generator.
    #[inline(always)]
    pub fn iicdl(&self) -> IICDL_R {
        IICDL_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    ///Bit 0 - Simple I2C Mode Select
    #[inline(always)]
    pub fn iicm(&mut self) -> IICM_W<SIMR1_SPEC> {
        IICM_W::new(self, 0)
    }
    ///Bits 3:7 - SSDA Delay Output SelectCycles below are of the clock signal from the on-chip baud rate generator.
    #[inline(always)]
    pub fn iicdl(&mut self) -> IICDL_W<SIMR1_SPEC> {
        IICDL_W::new(self, 3)
    }
}
/**I2C Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`simr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SIMR1_SPEC;
impl crate::RegisterSpec for SIMR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`simr1::R`](R) reader structure
impl crate::Readable for SIMR1_SPEC {}
///`write(|w| ..)` method takes [`simr1::W`](W) writer structure
impl crate::Writable for SIMR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIMR1 to value 0
impl crate::Resettable for SIMR1_SPEC {}
