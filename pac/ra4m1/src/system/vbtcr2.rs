///Register `VBTCR2` reader
pub type R = crate::R<VBTCR2_SPEC>;
///Register `VBTCR2` writer
pub type W = crate::W<VBTCR2_SPEC>;
/**VBATT Pin Low Voltage Detect Enable Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTLVDEN_A {
    ///0: VBATT pin low voltage detect disable
    _0 = 0,
    ///1: VBATT pin low voltage detect enable
    _1 = 1,
}
impl From<VBTLVDEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBTLVDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBTLVDEN` reader - VBATT Pin Low Voltage Detect Enable Bit
pub type VBTLVDEN_R = crate::BitReader<VBTLVDEN_A>;
impl VBTLVDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDEN_A {
        match self.bits {
            false => VBTLVDEN_A::_0,
            true => VBTLVDEN_A::_1,
        }
    }
    ///VBATT pin low voltage detect disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTLVDEN_A::_0
    }
    ///VBATT pin low voltage detect enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTLVDEN_A::_1
    }
}
///Field `VBTLVDEN` writer - VBATT Pin Low Voltage Detect Enable Bit
pub type VBTLVDEN_W<'a, REG> = crate::BitWriter<'a, REG, VBTLVDEN_A>;
impl<'a, REG> VBTLVDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT pin low voltage detect disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDEN_A::_0)
    }
    ///VBATT pin low voltage detect enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDEN_A::_1)
    }
}
/**VBATT Pin Voltage Low Voltage Detect Level Select Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VBTLVDLVL_A {
    ///0: 2.7V
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: 2.3V
    _10 = 2,
    ///3: 2.1V
    _11 = 3,
}
impl From<VBTLVDLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: VBTLVDLVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VBTLVDLVL_A {
    type Ux = u8;
}
impl crate::IsEnum for VBTLVDLVL_A {}
///Field `VBTLVDLVL` reader - VBATT Pin Voltage Low Voltage Detect Level Select Bit
pub type VBTLVDLVL_R = crate::FieldReader<VBTLVDLVL_A>;
impl VBTLVDLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTLVDLVL_A {
        match self.bits {
            0 => VBTLVDLVL_A::_00,
            1 => VBTLVDLVL_A::_01,
            2 => VBTLVDLVL_A::_10,
            3 => VBTLVDLVL_A::_11,
            _ => unreachable!(),
        }
    }
    ///2.7V
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VBTLVDLVL_A::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VBTLVDLVL_A::_01
    }
    ///2.3V
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VBTLVDLVL_A::_10
    }
    ///2.1V
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VBTLVDLVL_A::_11
    }
}
///Field `VBTLVDLVL` writer - VBATT Pin Voltage Low Voltage Detect Level Select Bit
pub type VBTLVDLVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VBTLVDLVL_A, crate::Safe>;
impl<'a, REG> VBTLVDLVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2.7V
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_01)
    }
    ///2.3V
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_10)
    }
    ///2.1V
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(VBTLVDLVL_A::_11)
    }
}
impl R {
    ///Bit 4 - VBATT Pin Low Voltage Detect Enable Bit
    #[inline(always)]
    pub fn vbtlvden(&self) -> VBTLVDEN_R {
        VBTLVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 6:7 - VBATT Pin Voltage Low Voltage Detect Level Select Bit
    #[inline(always)]
    pub fn vbtlvdlvl(&self) -> VBTLVDLVL_R {
        VBTLVDLVL_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bit 4 - VBATT Pin Low Voltage Detect Enable Bit
    #[inline(always)]
    pub fn vbtlvden(&mut self) -> VBTLVDEN_W<VBTCR2_SPEC> {
        VBTLVDEN_W::new(self, 4)
    }
    ///Bits 6:7 - VBATT Pin Voltage Low Voltage Detect Level Select Bit
    #[inline(always)]
    pub fn vbtlvdlvl(&mut self) -> VBTLVDLVL_W<VBTCR2_SPEC> {
        VBTLVDLVL_W::new(self, 6)
    }
}
/**VBATT Control Register2

You can [`read`](crate::Reg::read) this register and get [`vbtcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTCR2_SPEC;
impl crate::RegisterSpec for VBTCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtcr2::R`](R) reader structure
impl crate::Readable for VBTCR2_SPEC {}
///`write(|w| ..)` method takes [`vbtcr2::W`](W) writer structure
impl crate::Writable for VBTCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTCR2 to value 0
impl crate::Resettable for VBTCR2_SPEC {}
