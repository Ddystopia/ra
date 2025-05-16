///Register `MMPUACA%s` reader
pub type R = crate::R<MMPUACA_SPEC>;
///Register `MMPUACA%s` writer
pub type W = crate::W<MMPUACA_SPEC>;
/**Region enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    ///0: Group m Region n unit is disabled
    _0 = 0,
    ///1: Group m Region n unit is enabled
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - Region enable
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    ///Group m Region n unit is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    ///Group m Region n unit is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
///Field `ENABLE` writer - Region enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Group m Region n unit is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    ///Group m Region n unit is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_1)
    }
}
/**Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_A {
    ///0: Read permission
    _0 = 0,
    ///1: Read protection
    _1 = 1,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RP` reader - Read protection
pub type RP_R = crate::BitReader<RP_A>;
impl RP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::_0,
            true => RP_A::_1,
        }
    }
    ///Read permission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_A::_0
    }
    ///Read protection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_A::_1
    }
}
///Field `RP` writer - Read protection
pub type RP_W<'a, REG> = crate::BitWriter<'a, REG, RP_A>;
impl<'a, REG> RP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read permission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RP_A::_0)
    }
    ///Read protection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RP_A::_1)
    }
}
/**Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    ///0: Write permission
    _0 = 0,
    ///1: Write protection
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WP` reader - Write protection
pub type WP_R = crate::BitReader<WP_A>;
impl WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    ///Write permission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    ///Write protection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
///Field `WP` writer - Write protection
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG, WP_A>;
impl<'a, REG> WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write permission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_0)
    }
    ///Write protection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_1)
    }
}
impl R {
    ///Bit 0 - Region enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read protection
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write protection
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Region enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<MMPUACA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - Read protection
    #[inline(always)]
    pub fn rp(&mut self) -> RP_W<MMPUACA_SPEC> {
        RP_W::new(self, 1)
    }
    ///Bit 2 - Write protection
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<MMPUACA_SPEC> {
        WP_W::new(self, 2)
    }
}
/**Group A Region %s Access Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuaca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuaca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUACA_SPEC;
impl crate::RegisterSpec for MMPUACA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mmpuaca::R`](R) reader structure
impl crate::Readable for MMPUACA_SPEC {}
///`write(|w| ..)` method takes [`mmpuaca::W`](W) writer structure
impl crate::Writable for MMPUACA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUACA%s to value 0
impl crate::Resettable for MMPUACA_SPEC {}
