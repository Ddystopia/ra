///Register `LVD%sCR1` reader
pub type R = crate::R<LVDCR1_SPEC>;
///Register `LVD%sCR1` writer
pub type W = crate::W<LVDCR1_SPEC>;
/**Voltage Monitor Interrupt Generation Condition Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDTSEL_A {
    ///0: Generate when VCC>=Vdet (rise) is detected
    _00 = 0,
    ///1: Generate when VCC<Vdet (drop) is detected
    _01 = 1,
    ///2: Generate when drop and rise are detected
    _10 = 2,
    ///3: Settings prohibited
    _11 = 3,
}
impl From<IDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IDTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDTSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for IDTSEL_A {}
///Field `IDTSEL` reader - Voltage Monitor Interrupt Generation Condition Select
pub type IDTSEL_R = crate::FieldReader<IDTSEL_A>;
impl IDTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDTSEL_A {
        match self.bits {
            0 => IDTSEL_A::_00,
            1 => IDTSEL_A::_01,
            2 => IDTSEL_A::_10,
            3 => IDTSEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///Generate when VCC>=Vdet (rise) is detected
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDTSEL_A::_00
    }
    ///Generate when VCC<Vdet (drop) is detected
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDTSEL_A::_01
    }
    ///Generate when drop and rise are detected
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDTSEL_A::_10
    }
    ///Settings prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDTSEL_A::_11
    }
}
///Field `IDTSEL` writer - Voltage Monitor Interrupt Generation Condition Select
pub type IDTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IDTSEL_A, crate::Safe>;
impl<'a, REG> IDTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Generate when VCC>=Vdet (rise) is detected
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_00)
    }
    ///Generate when VCC<Vdet (drop) is detected
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_01)
    }
    ///Generate when drop and rise are detected
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_10)
    }
    ///Settings prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IDTSEL_A::_11)
    }
}
/**Voltage Monitor Interrupt Type Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQSEL_A {
    ///0: Non-maskable interrupt
    _0 = 0,
    ///1: Maskable interrupt
    _1 = 1,
}
impl From<IRQSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRQSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQSEL` reader - Voltage Monitor Interrupt Type Select
pub type IRQSEL_R = crate::BitReader<IRQSEL_A>;
impl IRQSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQSEL_A {
        match self.bits {
            false => IRQSEL_A::_0,
            true => IRQSEL_A::_1,
        }
    }
    ///Non-maskable interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQSEL_A::_0
    }
    ///Maskable interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQSEL_A::_1
    }
}
///Field `IRQSEL` writer - Voltage Monitor Interrupt Type Select
pub type IRQSEL_W<'a, REG> = crate::BitWriter<'a, REG, IRQSEL_A>;
impl<'a, REG> IRQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-maskable interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQSEL_A::_0)
    }
    ///Maskable interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQSEL_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Voltage Monitor Interrupt Generation Condition Select
    #[inline(always)]
    pub fn idtsel(&self) -> IDTSEL_R {
        IDTSEL_R::new(self.bits & 3)
    }
    ///Bit 2 - Voltage Monitor Interrupt Type Select
    #[inline(always)]
    pub fn irqsel(&self) -> IRQSEL_R {
        IRQSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Voltage Monitor Interrupt Generation Condition Select
    #[inline(always)]
    pub fn idtsel(&mut self) -> IDTSEL_W<LVDCR1_SPEC> {
        IDTSEL_W::new(self, 0)
    }
    ///Bit 2 - Voltage Monitor Interrupt Type Select
    #[inline(always)]
    pub fn irqsel(&mut self) -> IRQSEL_W<LVDCR1_SPEC> {
        IRQSEL_W::new(self, 2)
    }
}
/**Voltage Monitor %s Circuit Control Register 1

You can [`read`](crate::Reg::read) this register and get [`lvdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVDCR1_SPEC;
impl crate::RegisterSpec for LVDCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lvdcr1::R`](R) reader structure
impl crate::Readable for LVDCR1_SPEC {}
///`write(|w| ..)` method takes [`lvdcr1::W`](W) writer structure
impl crate::Writable for LVDCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD%sCR1 to value 0x01
impl crate::Resettable for LVDCR1_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
