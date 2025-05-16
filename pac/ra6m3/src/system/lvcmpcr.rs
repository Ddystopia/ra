///Register `LVCMPCR` reader
pub type R = crate::R<LVCMPCR_SPEC>;
///Register `LVCMPCR` writer
pub type W = crate::W<LVCMPCR_SPEC>;
/**Voltage Detection 1 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1E_A {
    ///0: Voltage detection 1 circuit disabled
    _0 = 0,
    ///1: Voltage detection 1 circuit enabled
    _1 = 1,
}
impl From<LVD1E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1E` reader - Voltage Detection 1 Enable
pub type LVD1E_R = crate::BitReader<LVD1E_A>;
impl LVD1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1E_A {
        match self.bits {
            false => LVD1E_A::_0,
            true => LVD1E_A::_1,
        }
    }
    ///Voltage detection 1 circuit disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1E_A::_0
    }
    ///Voltage detection 1 circuit enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1E_A::_1
    }
}
///Field `LVD1E` writer - Voltage Detection 1 Enable
pub type LVD1E_W<'a, REG> = crate::BitWriter<'a, REG, LVD1E_A>;
impl<'a, REG> LVD1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage detection 1 circuit disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1E_A::_0)
    }
    ///Voltage detection 1 circuit enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1E_A::_1)
    }
}
/**Voltage Detection 2 Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2E_A {
    ///0: Voltage detection 2 circuit disabled
    _0 = 0,
    ///1: Voltage detection 2 circuit enabled
    _1 = 1,
}
impl From<LVD2E_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2E` reader - Voltage Detection 2 Enable
pub type LVD2E_R = crate::BitReader<LVD2E_A>;
impl LVD2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2E_A {
        match self.bits {
            false => LVD2E_A::_0,
            true => LVD2E_A::_1,
        }
    }
    ///Voltage detection 2 circuit disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2E_A::_0
    }
    ///Voltage detection 2 circuit enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2E_A::_1
    }
}
///Field `LVD2E` writer - Voltage Detection 2 Enable
pub type LVD2E_W<'a, REG> = crate::BitWriter<'a, REG, LVD2E_A>;
impl<'a, REG> LVD2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage detection 2 circuit disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2E_A::_0)
    }
    ///Voltage detection 2 circuit enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2E_A::_1)
    }
}
impl R {
    ///Bit 5 - Voltage Detection 1 Enable
    #[inline(always)]
    pub fn lvd1e(&self) -> LVD1E_R {
        LVD1E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Voltage Detection 2 Enable
    #[inline(always)]
    pub fn lvd2e(&self) -> LVD2E_R {
        LVD2E_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - Voltage Detection 1 Enable
    #[inline(always)]
    pub fn lvd1e(&mut self) -> LVD1E_W<LVCMPCR_SPEC> {
        LVD1E_W::new(self, 5)
    }
    ///Bit 6 - Voltage Detection 2 Enable
    #[inline(always)]
    pub fn lvd2e(&mut self) -> LVD2E_W<LVCMPCR_SPEC> {
        LVD2E_W::new(self, 6)
    }
}
/**Voltage Monitor Circuit Control Register

You can [`read`](crate::Reg::read) this register and get [`lvcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVCMPCR_SPEC;
impl crate::RegisterSpec for LVCMPCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lvcmpcr::R`](R) reader structure
impl crate::Readable for LVCMPCR_SPEC {}
///`write(|w| ..)` method takes [`lvcmpcr::W`](W) writer structure
impl crate::Writable for LVCMPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVCMPCR to value 0
impl crate::Resettable for LVCMPCR_SPEC {}
