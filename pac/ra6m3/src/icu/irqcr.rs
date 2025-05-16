///Register `IRQCR%s` reader
pub type R = crate::R<IRQCR_SPEC>;
///Register `IRQCR%s` writer
pub type W = crate::W<IRQCR_SPEC>;
/**IRQ Detection Sense Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRQMD_A {
    ///0: Falling edge
    _00 = 0,
    ///1: Rising edge
    _01 = 1,
    ///2: Rising and falling edges
    _10 = 2,
    ///3: Low level
    _11 = 3,
}
impl From<IRQMD_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IRQMD_A {
    type Ux = u8;
}
impl crate::IsEnum for IRQMD_A {}
///Field `IRQMD` reader - IRQ Detection Sense Select
pub type IRQMD_R = crate::FieldReader<IRQMD_A>;
impl IRQMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQMD_A {
        match self.bits {
            0 => IRQMD_A::_00,
            1 => IRQMD_A::_01,
            2 => IRQMD_A::_10,
            3 => IRQMD_A::_11,
            _ => unreachable!(),
        }
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IRQMD_A::_00
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IRQMD_A::_01
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IRQMD_A::_10
    }
    ///Low level
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IRQMD_A::_11
    }
}
///Field `IRQMD` writer - IRQ Detection Sense Select
pub type IRQMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IRQMD_A, crate::Safe>;
impl<'a, REG> IRQMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Falling edge
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IRQMD_A::_00)
    }
    ///Rising edge
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IRQMD_A::_01)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IRQMD_A::_10)
    }
    ///Low level
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IRQMD_A::_11)
    }
}
/**IRQ Digital Filter Sampling Clock

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCLKSEL_A {
    ///0: PCLKB
    _00 = 0,
    ///1: PCLKB/8
    _01 = 1,
    ///2: PCLKB/32
    _10 = 2,
    ///3: PCLKB/64
    _11 = 3,
}
impl From<FCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for FCLKSEL_A {}
///Field `FCLKSEL` reader - IRQ Digital Filter Sampling Clock
pub type FCLKSEL_R = crate::FieldReader<FCLKSEL_A>;
impl FCLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FCLKSEL_A {
        match self.bits {
            0 => FCLKSEL_A::_00,
            1 => FCLKSEL_A::_01,
            2 => FCLKSEL_A::_10,
            3 => FCLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLKB
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FCLKSEL_A::_00
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FCLKSEL_A::_01
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FCLKSEL_A::_10
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FCLKSEL_A::_11
    }
}
///Field `FCLKSEL` writer - IRQ Digital Filter Sampling Clock
pub type FCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FCLKSEL_A, crate::Safe>;
impl<'a, REG> FCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(FCLKSEL_A::_00)
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(FCLKSEL_A::_01)
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(FCLKSEL_A::_10)
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(FCLKSEL_A::_11)
    }
}
/**IRQ Digital Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTEN_A {
    ///0: Digital filter is disabled.
    _0 = 0,
    ///1: Digital filter is enabled.
    _1 = 1,
}
impl From<FLTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTEN` reader - IRQ Digital Filter Enable
pub type FLTEN_R = crate::BitReader<FLTEN_A>;
impl FLTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLTEN_A {
        match self.bits {
            false => FLTEN_A::_0,
            true => FLTEN_A::_1,
        }
    }
    ///Digital filter is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLTEN_A::_0
    }
    ///Digital filter is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLTEN_A::_1
    }
}
///Field `FLTEN` writer - IRQ Digital Filter Enable
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG, FLTEN_A>;
impl<'a, REG> FLTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Digital filter is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLTEN_A::_0)
    }
    ///Digital filter is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLTEN_A::_1)
    }
}
impl R {
    ///Bits 0:1 - IRQ Detection Sense Select
    #[inline(always)]
    pub fn irqmd(&self) -> IRQMD_R {
        IRQMD_R::new(self.bits & 3)
    }
    ///Bits 4:5 - IRQ Digital Filter Sampling Clock
    #[inline(always)]
    pub fn fclksel(&self) -> FCLKSEL_R {
        FCLKSEL_R::new((self.bits >> 4) & 3)
    }
    ///Bit 7 - IRQ Digital Filter Enable
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - IRQ Detection Sense Select
    #[inline(always)]
    pub fn irqmd(&mut self) -> IRQMD_W<IRQCR_SPEC> {
        IRQMD_W::new(self, 0)
    }
    ///Bits 4:5 - IRQ Digital Filter Sampling Clock
    #[inline(always)]
    pub fn fclksel(&mut self) -> FCLKSEL_W<IRQCR_SPEC> {
        FCLKSEL_W::new(self, 4)
    }
    ///Bit 7 - IRQ Digital Filter Enable
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<IRQCR_SPEC> {
        FLTEN_W::new(self, 7)
    }
}
/**IRQ Control Register %s

You can [`read`](crate::Reg::read) this register and get [`irqcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IRQCR_SPEC;
impl crate::RegisterSpec for IRQCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`irqcr::R`](R) reader structure
impl crate::Readable for IRQCR_SPEC {}
///`write(|w| ..)` method takes [`irqcr::W`](W) writer structure
impl crate::Writable for IRQCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQCR%s to value 0
impl crate::Resettable for IRQCR_SPEC {}
