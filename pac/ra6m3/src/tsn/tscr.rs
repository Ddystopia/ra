///Register `TSCR` reader
pub type R = crate::R<TSCR_SPEC>;
///Register `TSCR` writer
pub type W = crate::W<TSCR_SPEC>;
/**Temperature Sensor Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOE_A {
    ///0: Disables output from the temperature sensor to the 12-bit A/D converter.
    _0 = 0,
    ///1: Enables output from the temperature sensor to the 12-bit A/D converter.
    _1 = 1,
}
impl From<TSOE_A> for bool {
    #[inline(always)]
    fn from(variant: TSOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSOE` reader - Temperature Sensor Enable
pub type TSOE_R = crate::BitReader<TSOE_A>;
impl TSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSOE_A {
        match self.bits {
            false => TSOE_A::_0,
            true => TSOE_A::_1,
        }
    }
    ///Disables output from the temperature sensor to the 12-bit A/D converter.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSOE_A::_0
    }
    ///Enables output from the temperature sensor to the 12-bit A/D converter.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSOE_A::_1
    }
}
///Field `TSOE` writer - Temperature Sensor Enable
pub type TSOE_W<'a, REG> = crate::BitWriter<'a, REG, TSOE_A>;
impl<'a, REG> TSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables output from the temperature sensor to the 12-bit A/D converter.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSOE_A::_0)
    }
    ///Enables output from the temperature sensor to the 12-bit A/D converter.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSOE_A::_1)
    }
}
/**Temperature Sensor Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN_A {
    ///0: Stops the temperature sensor.
    _0 = 0,
    ///1: Starts the temperature sensor.
    _1 = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEN` reader - Temperature Sensor Output Enable
pub type TSEN_R = crate::BitReader<TSEN_A>;
impl TSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::_0,
            true => TSEN_A::_1,
        }
    }
    ///Stops the temperature sensor.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSEN_A::_0
    }
    ///Starts the temperature sensor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSEN_A::_1
    }
}
///Field `TSEN` writer - Temperature Sensor Output Enable
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN_A>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops the temperature sensor.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN_A::_0)
    }
    ///Starts the temperature sensor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN_A::_1)
    }
}
impl R {
    ///Bit 4 - Temperature Sensor Enable
    #[inline(always)]
    pub fn tsoe(&self) -> TSOE_R {
        TSOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Temperature Sensor Output Enable
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Temperature Sensor Enable
    #[inline(always)]
    pub fn tsoe(&mut self) -> TSOE_W<TSCR_SPEC> {
        TSOE_W::new(self, 4)
    }
    ///Bit 7 - Temperature Sensor Output Enable
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<TSCR_SPEC> {
        TSEN_W::new(self, 7)
    }
}
/**Temperature Sensor Control Register

You can [`read`](crate::Reg::read) this register and get [`tscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSCR_SPEC;
impl crate::RegisterSpec for TSCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tscr::R`](R) reader structure
impl crate::Readable for TSCR_SPEC {}
///`write(|w| ..)` method takes [`tscr::W`](W) writer structure
impl crate::Writable for TSCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCR to value 0
impl crate::Resettable for TSCR_SPEC {}
