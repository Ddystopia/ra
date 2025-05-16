///Register `RCR1` reader
pub type R = crate::R<RCR1_SPEC>;
///Register `RCR1` writer
pub type W = crate::W<RCR1_SPEC>;
/**Alarm Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIE_A {
    ///0: An alarm interrupt request is disabled.
    _0 = 0,
    ///1: An alarm interrupt request is enabled.
    _1 = 1,
}
impl From<AIE_A> for bool {
    #[inline(always)]
    fn from(variant: AIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AIE` reader - Alarm Interrupt Enable
pub type AIE_R = crate::BitReader<AIE_A>;
impl AIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AIE_A {
        match self.bits {
            false => AIE_A::_0,
            true => AIE_A::_1,
        }
    }
    ///An alarm interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AIE_A::_0
    }
    ///An alarm interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AIE_A::_1
    }
}
///Field `AIE` writer - Alarm Interrupt Enable
pub type AIE_W<'a, REG> = crate::BitWriter<'a, REG, AIE_A>;
impl<'a, REG> AIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///An alarm interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AIE_A::_0)
    }
    ///An alarm interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AIE_A::_1)
    }
}
/**Carry Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIE_A {
    ///0: A carry interrupt request is disabled.
    _0 = 0,
    ///1: A carry interrupt request is enabled.
    _1 = 1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CIE` reader - Carry Interrupt Enable
pub type CIE_R = crate::BitReader<CIE_A>;
impl CIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::_0,
            true => CIE_A::_1,
        }
    }
    ///A carry interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIE_A::_0
    }
    ///A carry interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIE_A::_1
    }
}
///Field `CIE` writer - Carry Interrupt Enable
pub type CIE_W<'a, REG> = crate::BitWriter<'a, REG, CIE_A>;
impl<'a, REG> CIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A carry interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CIE_A::_0)
    }
    ///A carry interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CIE_A::_1)
    }
}
/**Periodic Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIE_A {
    ///0: A periodic interrupt request is disabled.
    _0 = 0,
    ///1: A periodic interrupt request is enabled.
    _1 = 1,
}
impl From<PIE_A> for bool {
    #[inline(always)]
    fn from(variant: PIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIE` reader - Periodic Interrupt Enable
pub type PIE_R = crate::BitReader<PIE_A>;
impl PIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIE_A {
        match self.bits {
            false => PIE_A::_0,
            true => PIE_A::_1,
        }
    }
    ///A periodic interrupt request is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIE_A::_0
    }
    ///A periodic interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIE_A::_1
    }
}
///Field `PIE` writer - Periodic Interrupt Enable
pub type PIE_W<'a, REG> = crate::BitWriter<'a, REG, PIE_A>;
impl<'a, REG> PIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A periodic interrupt request is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIE_A::_0)
    }
    ///A periodic interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIE_A::_1)
    }
}
/**RTCOUT Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOS_A {
    ///0: RTCOUT outputs 1 Hz.
    _0 = 0,
    ///1: RTCOUT outputs 64 Hz.
    _1 = 1,
}
impl From<RTCOS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCOS` reader - RTCOUT Output Select
pub type RTCOS_R = crate::BitReader<RTCOS_A>;
impl RTCOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCOS_A {
        match self.bits {
            false => RTCOS_A::_0,
            true => RTCOS_A::_1,
        }
    }
    ///RTCOUT outputs 1 Hz.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCOS_A::_0
    }
    ///RTCOUT outputs 64 Hz.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCOS_A::_1
    }
}
///Field `RTCOS` writer - RTCOUT Output Select
pub type RTCOS_W<'a, REG> = crate::BitWriter<'a, REG, RTCOS_A>;
impl<'a, REG> RTCOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTCOUT outputs 1 Hz.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOS_A::_0)
    }
    ///RTCOUT outputs 64 Hz.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCOS_A::_1)
    }
}
/**Periodic Interrupt Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PES_A {
    ///6: A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1).
    _0110 = 6,
    ///7: A periodic interrupt is generated every 1/128 second.
    _0111 = 7,
    ///8: A periodic interrupt is generated every 1/64 second.
    _1000 = 8,
    ///9: A periodic interrupt is generated every 1/32 second.
    _1001 = 9,
    ///10: A periodic interrupt is generated every 1/16 second.
    _1010 = 10,
    ///11: A periodic interrupt is generated every 1/8 second.
    _1011 = 11,
    ///12: A periodic interrupt is generated every 1/4 second.
    _1100 = 12,
    ///13: A periodic interrupt is generated every 1/2 second.
    _1101 = 13,
    ///14: A periodic interrupt is generated every 1 second.
    _1110 = 14,
    ///15: A periodic interrupt is generated every 2 seconds.
    _1111 = 15,
    ///0: No periodic interrupts are generated.
    OTHERS = 0,
}
impl From<PES_A> for u8 {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PES_A {
    type Ux = u8;
}
impl crate::IsEnum for PES_A {}
///Field `PES` reader - Periodic Interrupt Select
pub type PES_R = crate::FieldReader<PES_A>;
impl PES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PES_A {
        match self.bits {
            6 => PES_A::_0110,
            7 => PES_A::_0111,
            8 => PES_A::_1000,
            9 => PES_A::_1001,
            10 => PES_A::_1010,
            11 => PES_A::_1011,
            12 => PES_A::_1100,
            13 => PES_A::_1101,
            14 => PES_A::_1110,
            15 => PES_A::_1111,
            _ => PES_A::OTHERS,
        }
    }
    ///A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1).
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PES_A::_0110
    }
    ///A periodic interrupt is generated every 1/128 second.
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PES_A::_0111
    }
    ///A periodic interrupt is generated every 1/64 second.
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PES_A::_1000
    }
    ///A periodic interrupt is generated every 1/32 second.
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PES_A::_1001
    }
    ///A periodic interrupt is generated every 1/16 second.
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PES_A::_1010
    }
    ///A periodic interrupt is generated every 1/8 second.
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PES_A::_1011
    }
    ///A periodic interrupt is generated every 1/4 second.
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PES_A::_1100
    }
    ///A periodic interrupt is generated every 1/2 second.
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PES_A::_1101
    }
    ///A periodic interrupt is generated every 1 second.
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PES_A::_1110
    }
    ///A periodic interrupt is generated every 2 seconds.
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PES_A::_1111
    }
    ///No periodic interrupts are generated.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PES_A::OTHERS)
    }
}
///Field `PES` writer - Periodic Interrupt Select
pub type PES_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PES_A, crate::Safe>;
impl<'a, REG> PES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1).
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_0110)
    }
    ///A periodic interrupt is generated every 1/128 second.
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_0111)
    }
    ///A periodic interrupt is generated every 1/64 second.
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1000)
    }
    ///A periodic interrupt is generated every 1/32 second.
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1001)
    }
    ///A periodic interrupt is generated every 1/16 second.
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1010)
    }
    ///A periodic interrupt is generated every 1/8 second.
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1011)
    }
    ///A periodic interrupt is generated every 1/4 second.
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1100)
    }
    ///A periodic interrupt is generated every 1/2 second.
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1101)
    }
    ///A periodic interrupt is generated every 1 second.
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1110)
    }
    ///A periodic interrupt is generated every 2 seconds.
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::_1111)
    }
    ///No periodic interrupts are generated.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PES_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Alarm Interrupt Enable
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Carry Interrupt Enable
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Periodic Interrupt Enable
    #[inline(always)]
    pub fn pie(&self) -> PIE_R {
        PIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTCOUT Output Select
    #[inline(always)]
    pub fn rtcos(&self) -> RTCOS_R {
        RTCOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Periodic Interrupt Select
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    ///Bit 0 - Alarm Interrupt Enable
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W<RCR1_SPEC> {
        AIE_W::new(self, 0)
    }
    ///Bit 1 - Carry Interrupt Enable
    #[inline(always)]
    pub fn cie(&mut self) -> CIE_W<RCR1_SPEC> {
        CIE_W::new(self, 1)
    }
    ///Bit 2 - Periodic Interrupt Enable
    #[inline(always)]
    pub fn pie(&mut self) -> PIE_W<RCR1_SPEC> {
        PIE_W::new(self, 2)
    }
    ///Bit 3 - RTCOUT Output Select
    #[inline(always)]
    pub fn rtcos(&mut self) -> RTCOS_W<RCR1_SPEC> {
        RTCOS_W::new(self, 3)
    }
    ///Bits 4:7 - Periodic Interrupt Select
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W<RCR1_SPEC> {
        PES_W::new(self, 4)
    }
}
/**RTC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`rcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RCR1_SPEC;
impl crate::RegisterSpec for RCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rcr1::R`](R) reader structure
impl crate::Readable for RCR1_SPEC {}
///`write(|w| ..)` method takes [`rcr1::W`](W) writer structure
impl crate::Writable for RCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR1 to value 0
impl crate::Resettable for RCR1_SPEC {}
