///Register `VBTLVDICR` reader
pub type R = crate::R<VBTLVDICR_SPEC>;
///Register `VBTLVDICR` writer
pub type W = crate::W<VBTLVDICR_SPEC>;
/**VBATT Pin Low Voltage Detect Interrupt Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTLVDIE_A {
    ///0: VBATT Pin Low Voltage Detect Interrupt Disable
    _0 = 0,
    ///1: VBATT Pin Low Voltage Detect Interrupt Enable
    _1 = 1,
}
impl From<VBTLVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: VBTLVDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBTLVDIE` reader - VBATT Pin Low Voltage Detect Interrupt Enable bit
pub type VBTLVDIE_R = crate::BitReader<VBTLVDIE_A>;
impl VBTLVDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDIE_A {
        match self.bits {
            false => VBTLVDIE_A::_0,
            true => VBTLVDIE_A::_1,
        }
    }
    ///VBATT Pin Low Voltage Detect Interrupt Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTLVDIE_A::_0
    }
    ///VBATT Pin Low Voltage Detect Interrupt Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTLVDIE_A::_1
    }
}
///Field `VBTLVDIE` writer - VBATT Pin Low Voltage Detect Interrupt Enable bit
pub type VBTLVDIE_W<'a, REG> = crate::BitWriter<'a, REG, VBTLVDIE_A>;
impl<'a, REG> VBTLVDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT Pin Low Voltage Detect Interrupt Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDIE_A::_0)
    }
    ///VBATT Pin Low Voltage Detect Interrupt Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDIE_A::_1)
    }
}
/**Pin Low Voltage Detect Interrupt Select bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTLVDISEL_A {
    ///0: Non Maskable Interrupt
    _0 = 0,
    ///1: Maskable Interrupt
    _1 = 1,
}
impl From<VBTLVDISEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBTLVDISEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBTLVDISEL` reader - Pin Low Voltage Detect Interrupt Select bit
pub type VBTLVDISEL_R = crate::BitReader<VBTLVDISEL_A>;
impl VBTLVDISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDISEL_A {
        match self.bits {
            false => VBTLVDISEL_A::_0,
            true => VBTLVDISEL_A::_1,
        }
    }
    ///Non Maskable Interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTLVDISEL_A::_0
    }
    ///Maskable Interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTLVDISEL_A::_1
    }
}
///Field `VBTLVDISEL` writer - Pin Low Voltage Detect Interrupt Select bit
pub type VBTLVDISEL_W<'a, REG> = crate::BitWriter<'a, REG, VBTLVDISEL_A>;
impl<'a, REG> VBTLVDISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non Maskable Interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDISEL_A::_0)
    }
    ///Maskable Interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDISEL_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATT Pin Low Voltage Detect Interrupt Enable bit
    #[inline(always)]
    pub fn vbtlvdie(&self) -> VBTLVDIE_R {
        VBTLVDIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pin Low Voltage Detect Interrupt Select bit
    #[inline(always)]
    pub fn vbtlvdisel(&self) -> VBTLVDISEL_R {
        VBTLVDISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATT Pin Low Voltage Detect Interrupt Enable bit
    #[inline(always)]
    pub fn vbtlvdie(&mut self) -> VBTLVDIE_W<VBTLVDICR_SPEC> {
        VBTLVDIE_W::new(self, 0)
    }
    ///Bit 1 - Pin Low Voltage Detect Interrupt Select bit
    #[inline(always)]
    pub fn vbtlvdisel(&mut self) -> VBTLVDISEL_W<VBTLVDICR_SPEC> {
        VBTLVDISEL_W::new(self, 1)
    }
}
/**VBATT Pin Low Voltage Detect Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtlvdicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtlvdicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTLVDICR_SPEC;
impl crate::RegisterSpec for VBTLVDICR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtlvdicr::R`](R) reader structure
impl crate::Readable for VBTLVDICR_SPEC {}
///`write(|w| ..)` method takes [`vbtlvdicr::W`](W) writer structure
impl crate::Writable for VBTLVDICR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTLVDICR to value 0
impl crate::Resettable for VBTLVDICR_SPEC {}
