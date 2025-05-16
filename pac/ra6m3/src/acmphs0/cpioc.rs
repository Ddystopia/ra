///Register `CPIOC` reader
pub type R = crate::R<CPIOC_SPEC>;
///Register `CPIOC` writer
pub type W = crate::W<CPIOC_SPEC>;
/**Comparator output selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOE_A {
    ///0: VCOUT pin output of the comparator is disabled (the output signal is low level).
    _0 = 0,
    ///1: VCOUT pin output of the comparator is enabled
    _1 = 1,
}
impl From<CPOE_A> for bool {
    #[inline(always)]
    fn from(variant: CPOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOE` reader - Comparator output selection
pub type CPOE_R = crate::BitReader<CPOE_A>;
impl CPOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPOE_A {
        match self.bits {
            false => CPOE_A::_0,
            true => CPOE_A::_1,
        }
    }
    ///VCOUT pin output of the comparator is disabled (the output signal is low level).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOE_A::_0
    }
    ///VCOUT pin output of the comparator is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOE_A::_1
    }
}
///Field `CPOE` writer - Comparator output selection
pub type CPOE_W<'a, REG> = crate::BitWriter<'a, REG, CPOE_A>;
impl<'a, REG> CPOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VCOUT pin output of the comparator is disabled (the output signal is low level).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOE_A::_0)
    }
    ///VCOUT pin output of the comparator is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOE_A::_1)
    }
}
/**Internal Vref enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    ///0: Internal Vref disable
    _0 = 0,
    ///1: Internal Vref enable
    _1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - Internal Vref enable
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::_0,
            true => VREFEN_A::_1,
        }
    }
    ///Internal Vref disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFEN_A::_0
    }
    ///Internal Vref enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFEN_A::_1
    }
}
///Field `VREFEN` writer - Internal Vref enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN_A>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal Vref disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::_0)
    }
    ///Internal Vref enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::_1)
    }
}
impl R {
    ///Bit 0 - Comparator output selection
    #[inline(always)]
    pub fn cpoe(&self) -> CPOE_R {
        CPOE_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Internal Vref enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator output selection
    #[inline(always)]
    pub fn cpoe(&mut self) -> CPOE_W<CPIOC_SPEC> {
        CPOE_W::new(self, 0)
    }
    ///Bit 7 - Internal Vref enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<CPIOC_SPEC> {
        VREFEN_W::new(self, 7)
    }
}
/**Comparator Output Control Register

You can [`read`](crate::Reg::read) this register and get [`cpioc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpioc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPIOC_SPEC;
impl crate::RegisterSpec for CPIOC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cpioc::R`](R) reader structure
impl crate::Readable for CPIOC_SPEC {}
///`write(|w| ..)` method takes [`cpioc::W`](W) writer structure
impl crate::Writable for CPIOC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPIOC to value 0
impl crate::Resettable for CPIOC_SPEC {}
