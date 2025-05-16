///Register `SPPCR` reader
pub type R = crate::R<SPPCR_SPEC>;
///Register `SPPCR` writer
pub type W = crate::W<SPPCR_SPEC>;
/**RSPI Loopback

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP_A {
    ///0: Normal mode
    _0 = 0,
    ///1: Loopback mode (data is inverted for transmission)
    _1 = 1,
}
impl From<SPLP_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPLP` reader - RSPI Loopback
pub type SPLP_R = crate::BitReader<SPLP_A>;
impl SPLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPLP_A {
        match self.bits {
            false => SPLP_A::_0,
            true => SPLP_A::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP_A::_0
    }
    ///Loopback mode (data is inverted for transmission)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP_A::_1
    }
}
///Field `SPLP` writer - RSPI Loopback
pub type SPLP_W<'a, REG> = crate::BitWriter<'a, REG, SPLP_A>;
impl<'a, REG> SPLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP_A::_0)
    }
    ///Loopback mode (data is inverted for transmission)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP_A::_1)
    }
}
/**RSPI Loopback 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLP2_A {
    ///0: Normal mode
    _0 = 0,
    ///1: Loopback mode (data is not inverted for transmission)
    _1 = 1,
}
impl From<SPLP2_A> for bool {
    #[inline(always)]
    fn from(variant: SPLP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPLP2` reader - RSPI Loopback 2
pub type SPLP2_R = crate::BitReader<SPLP2_A>;
impl SPLP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPLP2_A {
        match self.bits {
            false => SPLP2_A::_0,
            true => SPLP2_A::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPLP2_A::_0
    }
    ///Loopback mode (data is not inverted for transmission)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPLP2_A::_1
    }
}
///Field `SPLP2` writer - RSPI Loopback 2
pub type SPLP2_W<'a, REG> = crate::BitWriter<'a, REG, SPLP2_A>;
impl<'a, REG> SPLP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP2_A::_0)
    }
    ///Loopback mode (data is not inverted for transmission)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPLP2_A::_1)
    }
}
/**MOSI Idle Fixed Value

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFV_A {
    ///0: The level output on the MOSIn pin during MOSI idling corresponds to low.
    _0 = 0,
    ///1: The level output on the MOSIn pin during MOSI idling corresponds to high.
    _1 = 1,
}
impl From<MOIFV_A> for bool {
    #[inline(always)]
    fn from(variant: MOIFV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MOIFV` reader - MOSI Idle Fixed Value
pub type MOIFV_R = crate::BitReader<MOIFV_A>;
impl MOIFV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOIFV_A {
        match self.bits {
            false => MOIFV_A::_0,
            true => MOIFV_A::_1,
        }
    }
    ///The level output on the MOSIn pin during MOSI idling corresponds to low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOIFV_A::_0
    }
    ///The level output on the MOSIn pin during MOSI idling corresponds to high.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOIFV_A::_1
    }
}
///Field `MOIFV` writer - MOSI Idle Fixed Value
pub type MOIFV_W<'a, REG> = crate::BitWriter<'a, REG, MOIFV_A>;
impl<'a, REG> MOIFV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The level output on the MOSIn pin during MOSI idling corresponds to low.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFV_A::_0)
    }
    ///The level output on the MOSIn pin during MOSI idling corresponds to high.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFV_A::_1)
    }
}
/**MOSI Idle Value Fixing Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOIFE_A {
    ///0: MOSI output value equals final data from previous transfer
    _0 = 0,
    ///1: MOSI output value equals the value set in the MOIFV bit
    _1 = 1,
}
impl From<MOIFE_A> for bool {
    #[inline(always)]
    fn from(variant: MOIFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MOIFE` reader - MOSI Idle Value Fixing Enable
pub type MOIFE_R = crate::BitReader<MOIFE_A>;
impl MOIFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOIFE_A {
        match self.bits {
            false => MOIFE_A::_0,
            true => MOIFE_A::_1,
        }
    }
    ///MOSI output value equals final data from previous transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOIFE_A::_0
    }
    ///MOSI output value equals the value set in the MOIFV bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOIFE_A::_1
    }
}
///Field `MOIFE` writer - MOSI Idle Value Fixing Enable
pub type MOIFE_W<'a, REG> = crate::BitWriter<'a, REG, MOIFE_A>;
impl<'a, REG> MOIFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MOSI output value equals final data from previous transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFE_A::_0)
    }
    ///MOSI output value equals the value set in the MOIFV bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOIFE_A::_1)
    }
}
impl R {
    ///Bit 0 - RSPI Loopback
    #[inline(always)]
    pub fn splp(&self) -> SPLP_R {
        SPLP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RSPI Loopback 2
    #[inline(always)]
    pub fn splp2(&self) -> SPLP2_R {
        SPLP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - MOSI Idle Fixed Value
    #[inline(always)]
    pub fn moifv(&self) -> MOIFV_R {
        MOIFV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MOSI Idle Value Fixing Enable
    #[inline(always)]
    pub fn moife(&self) -> MOIFE_R {
        MOIFE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RSPI Loopback
    #[inline(always)]
    pub fn splp(&mut self) -> SPLP_W<SPPCR_SPEC> {
        SPLP_W::new(self, 0)
    }
    ///Bit 1 - RSPI Loopback 2
    #[inline(always)]
    pub fn splp2(&mut self) -> SPLP2_W<SPPCR_SPEC> {
        SPLP2_W::new(self, 1)
    }
    ///Bit 4 - MOSI Idle Fixed Value
    #[inline(always)]
    pub fn moifv(&mut self) -> MOIFV_W<SPPCR_SPEC> {
        MOIFV_W::new(self, 4)
    }
    ///Bit 5 - MOSI Idle Value Fixing Enable
    #[inline(always)]
    pub fn moife(&mut self) -> MOIFE_W<SPPCR_SPEC> {
        MOIFE_W::new(self, 5)
    }
}
/**RSPI Pin Control Register

You can [`read`](crate::Reg::read) this register and get [`sppcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPPCR_SPEC;
impl crate::RegisterSpec for SPPCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sppcr::R`](R) reader structure
impl crate::Readable for SPPCR_SPEC {}
///`write(|w| ..)` method takes [`sppcr::W`](W) writer structure
impl crate::Writable for SPPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPPCR to value 0
impl crate::Resettable for SPPCR_SPEC {}
