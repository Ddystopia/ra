///Register `LVD%sSR` reader
pub type R = crate::R<LVDSR_SPEC>;
///Register `LVD%sSR` writer
pub type W = crate::W<LVDSR_SPEC>;
/**Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DET_A {
    ///0: Not detected
    _0 = 0,
    ///1: Vdet1 passage detection
    _1 = 1,
}
impl From<DET_A> for bool {
    #[inline(always)]
    fn from(variant: DET_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DET` reader - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DET_R = crate::BitReader<DET_A>;
impl DET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DET_A {
        match self.bits {
            false => DET_A::_0,
            true => DET_A::_1,
        }
    }
    ///Not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DET_A::_0
    }
    ///Vdet1 passage detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DET_A::_1
    }
}
///Field `DET` writer - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.
pub type DET_W<'a, REG> = crate::BitWriter0C<'a, REG, DET_A>;
impl<'a, REG> DET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DET_A::_0)
    }
    ///Vdet1 passage detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DET_A::_1)
    }
}
/**Voltage Monitor 1 Signal Monitor Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MON_A {
    ///0: VCC < Vdet
    _0 = 0,
    ///1: VCC >= Vdet or MON bit is disabled
    _1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MON` reader - Voltage Monitor 1 Signal Monitor Flag
pub type MON_R = crate::BitReader<MON_A>;
impl MON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::_0,
            true => MON_A::_1,
        }
    }
    ///VCC < Vdet
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MON_A::_0
    }
    ///VCC >= Vdet or MON bit is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MON_A::_1
    }
}
impl R {
    ///Bit 0 - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage Monitor 1 Signal Monitor Flag
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.
    #[inline(always)]
    pub fn det(&mut self) -> DET_W<LVDSR_SPEC> {
        DET_W::new(self, 0)
    }
}
/**Voltage Monitor %s Circuit Status Register

You can [`read`](crate::Reg::read) this register and get [`lvdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVDSR_SPEC;
impl crate::RegisterSpec for LVDSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lvdsr::R`](R) reader structure
impl crate::Readable for LVDSR_SPEC {}
///`write(|w| ..)` method takes [`lvdsr::W`](W) writer structure
impl crate::Writable for LVDSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
///`reset()` method sets LVD%sSR to value 0x02
impl crate::Resettable for LVDSR_SPEC {
    const RESET_VALUE: u8 = 0x02;
}
