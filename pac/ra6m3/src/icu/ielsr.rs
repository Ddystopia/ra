///Register `IELSR%s` reader
pub type R = crate::R<IELSR_SPEC>;
///Register `IELSR%s` writer
pub type W = crate::W<IELSR_SPEC>;
/**Event selection to NVIC

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IELS_A {
    ///0: Nothing is selected
    _0X000 = 0,
    ///1: See Event Table
    OTHERS = 1,
}
impl From<IELS_A> for u16 {
    #[inline(always)]
    fn from(variant: IELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IELS_A {
    type Ux = u16;
}
impl crate::IsEnum for IELS_A {}
///Field `IELS` reader - Event selection to NVIC
pub type IELS_R = crate::FieldReader<IELS_A>;
impl IELS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IELS_A {
        match self.bits {
            0 => IELS_A::_0X000,
            _ => IELS_A::OTHERS,
        }
    }
    ///Nothing is selected
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == IELS_A::_0X000
    }
    ///See Event Table
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), IELS_A::OTHERS)
    }
}
///Field `IELS` writer - Event selection to NVIC
pub type IELS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, IELS_A, crate::Safe>;
impl<'a, REG> IELS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Nothing is selected
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(IELS_A::_0X000)
    }
    ///See Event Table
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(IELS_A::OTHERS)
    }
}
/**Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_A {
    ///0: No interrupt request is generated
    _0 = 0,
    ///1: An interrupt request is generated ( "1" write to the IR bit is prohibited. )
    _1 = 1,
}
impl From<IR_A> for bool {
    #[inline(always)]
    fn from(variant: IR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IR` reader - Interrupt Status Flag
pub type IR_R = crate::BitReader<IR_A>;
impl IR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IR_A {
        match self.bits {
            false => IR_A::_0,
            true => IR_A::_1,
        }
    }
    ///No interrupt request is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IR_A::_0
    }
    ///An interrupt request is generated ( "1" write to the IR bit is prohibited. )
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IR_A::_1
    }
}
///Field `IR` writer - Interrupt Status Flag
pub type IR_W<'a, REG> = crate::BitWriter<'a, REG, IR_A>;
impl<'a, REG> IR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt request is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IR_A::_0)
    }
    ///An interrupt request is generated ( "1" write to the IR bit is prohibited. )
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IR_A::_1)
    }
}
/**DTC Activation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCE_A {
    ///0: DTC activation is disabled
    _0 = 0,
    ///1: DTC activation is enabled
    _1 = 1,
}
impl From<DTCE_A> for bool {
    #[inline(always)]
    fn from(variant: DTCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCE` reader - DTC Activation Enable
pub type DTCE_R = crate::BitReader<DTCE_A>;
impl DTCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCE_A {
        match self.bits {
            false => DTCE_A::_0,
            true => DTCE_A::_1,
        }
    }
    ///DTC activation is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCE_A::_0
    }
    ///DTC activation is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCE_A::_1
    }
}
///Field `DTCE` writer - DTC Activation Enable
pub type DTCE_W<'a, REG> = crate::BitWriter<'a, REG, DTCE_A>;
impl<'a, REG> DTCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DTC activation is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCE_A::_0)
    }
    ///DTC activation is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCE_A::_1)
    }
}
impl R {
    ///Bits 0:8 - Event selection to NVIC
    #[inline(always)]
    pub fn iels(&self) -> IELS_R {
        IELS_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 16 - Interrupt Status Flag
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - DTC Activation Enable
    #[inline(always)]
    pub fn dtce(&self) -> DTCE_R {
        DTCE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - Event selection to NVIC
    #[inline(always)]
    pub fn iels(&mut self) -> IELS_W<IELSR_SPEC> {
        IELS_W::new(self, 0)
    }
    ///Bit 16 - Interrupt Status Flag
    #[inline(always)]
    pub fn ir(&mut self) -> IR_W<IELSR_SPEC> {
        IR_W::new(self, 16)
    }
    ///Bit 24 - DTC Activation Enable
    #[inline(always)]
    pub fn dtce(&mut self) -> DTCE_W<IELSR_SPEC> {
        DTCE_W::new(self, 24)
    }
}
/**INT Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`ielsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IELSR_SPEC;
impl crate::RegisterSpec for IELSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ielsr::R`](R) reader structure
impl crate::Readable for IELSR_SPEC {}
///`write(|w| ..)` method takes [`ielsr::W`](W) writer structure
impl crate::Writable for IELSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IELSR%s to value 0
impl crate::Resettable for IELSR_SPEC {}
