///Register `TRIMD` reader
pub type R = crate::R<TRIMD_SPEC>;
///Register `TRIMD` writer
pub type W = crate::W<TRIMD_SPEC>;
/**Transmit Interrupt EnableSet the EESR.TWB flag to 1 in the mode selected by the TIM bit to notify an interrupt.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIS_A {
    ///0: Transmit Interrupt is disabled.
    _0 = 0,
    ///1: Transmit Interrupt is enabled.
    _1 = 1,
}
impl From<TIS_A> for bool {
    #[inline(always)]
    fn from(variant: TIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIS` reader - Transmit Interrupt EnableSet the EESR.TWB flag to 1 in the mode selected by the TIM bit to notify an interrupt.
pub type TIS_R = crate::BitReader<TIS_A>;
impl TIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIS_A {
        match self.bits {
            false => TIS_A::_0,
            true => TIS_A::_1,
        }
    }
    ///Transmit Interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIS_A::_0
    }
    ///Transmit Interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIS_A::_1
    }
}
///Field `TIS` writer - Transmit Interrupt EnableSet the EESR.TWB flag to 1 in the mode selected by the TIM bit to notify an interrupt.
pub type TIS_W<'a, REG> = crate::BitWriter<'a, REG, TIS_A>;
impl<'a, REG> TIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit Interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIS_A::_0)
    }
    ///Transmit Interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIS_A::_1)
    }
}
/**Transmit Interrupt Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM_A {
    ///0: Transmission complete interrupt mode: An interrupt occurs when a frame has been transmitted.
    _0 = 0,
    ///1: Write-back complete interrupt mode: An interrupt occurs when write-back to the transmit descriptor has been completed.
    _1 = 1,
}
impl From<TIM_A> for bool {
    #[inline(always)]
    fn from(variant: TIM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM` reader - Transmit Interrupt Mode
pub type TIM_R = crate::BitReader<TIM_A>;
impl TIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM_A {
        match self.bits {
            false => TIM_A::_0,
            true => TIM_A::_1,
        }
    }
    ///Transmission complete interrupt mode: An interrupt occurs when a frame has been transmitted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIM_A::_0
    }
    ///Write-back complete interrupt mode: An interrupt occurs when write-back to the transmit descriptor has been completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIM_A::_1
    }
}
///Field `TIM` writer - Transmit Interrupt Mode
pub type TIM_W<'a, REG> = crate::BitWriter<'a, REG, TIM_A>;
impl<'a, REG> TIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission complete interrupt mode: An interrupt occurs when a frame has been transmitted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM_A::_0)
    }
    ///Write-back complete interrupt mode: An interrupt occurs when write-back to the transmit descriptor has been completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM_A::_1)
    }
}
impl R {
    ///Bit 0 - Transmit Interrupt EnableSet the EESR.TWB flag to 1 in the mode selected by the TIM bit to notify an interrupt.
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Transmit Interrupt Mode
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit Interrupt EnableSet the EESR.TWB flag to 1 in the mode selected by the TIM bit to notify an interrupt.
    #[inline(always)]
    pub fn tis(&mut self) -> TIS_W<TRIMD_SPEC> {
        TIS_W::new(self, 0)
    }
    ///Bit 4 - Transmit Interrupt Mode
    #[inline(always)]
    pub fn tim(&mut self) -> TIM_W<TRIMD_SPEC> {
        TIM_W::new(self, 4)
    }
}
/**Transmit Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`trimd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trimd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TRIMD_SPEC;
impl crate::RegisterSpec for TRIMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`trimd::R`](R) reader structure
impl crate::Readable for TRIMD_SPEC {}
///`write(|w| ..)` method takes [`trimd::W`](W) writer structure
impl crate::Writable for TRIMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRIMD to value 0
impl crate::Resettable for TRIMD_SPEC {}
