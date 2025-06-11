///Register `CAICR` reader
pub type R = crate::R<CAICR_SPEC>;
///Register `CAICR` writer
pub type W = crate::W<CAICR_SPEC>;
/**Frequency Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERRIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<FERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FERRIE` reader - Frequency Error Interrupt Request Enable
pub type FERRIE_R = crate::BitReader<FERRIE_A>;
impl FERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FERRIE_A {
        match self.bits {
            false => FERRIE_A::_0,
            true => FERRIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FERRIE_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FERRIE_A::_1
    }
}
///Field `FERRIE` writer - Frequency Error Interrupt Request Enable
pub type FERRIE_W<'a, REG> = crate::BitWriter<'a, REG, FERRIE_A>;
impl<'a, REG> FERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FERRIE_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FERRIE_A::_1)
    }
}
/**Measurement End Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MENDIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<MENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: MENDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MENDIE` reader - Measurement End Interrupt Request Enable
pub type MENDIE_R = crate::BitReader<MENDIE_A>;
impl MENDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MENDIE_A {
        match self.bits {
            false => MENDIE_A::_0,
            true => MENDIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MENDIE_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MENDIE_A::_1
    }
}
///Field `MENDIE` writer - Measurement End Interrupt Request Enable
pub type MENDIE_W<'a, REG> = crate::BitWriter<'a, REG, MENDIE_A>;
impl<'a, REG> MENDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MENDIE_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MENDIE_A::_1)
    }
}
/**Overflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<OVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVFIE` reader - Overflow Interrupt Request Enable
pub type OVFIE_R = crate::BitReader<OVFIE_A>;
impl OVFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVFIE_A {
        match self.bits {
            false => OVFIE_A::_0,
            true => OVFIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFIE_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFIE_A::_1
    }
}
///Field `OVFIE` writer - Overflow Interrupt Request Enable
pub type OVFIE_W<'a, REG> = crate::BitWriter<'a, REG, OVFIE_A>;
impl<'a, REG> OVFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIE_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIE_A::_1)
    }
}
/**FERRF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERRFCL_A {
    ///0: No effect on operations
    _0 = 0,
    ///1: Clears the FERRF flag
    _1 = 1,
}
impl From<FERRFCL_A> for bool {
    #[inline(always)]
    fn from(variant: FERRFCL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FERRFCL` writer - FERRF Clear
pub type FERRFCL_W<'a, REG> = crate::BitWriter<'a, REG, FERRFCL_A>;
impl<'a, REG> FERRFCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FERRFCL_A::_0)
    }
    ///Clears the FERRF flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FERRFCL_A::_1)
    }
}
/**MENDF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MENDFCL_A {
    ///0: No effect on operations
    _0 = 0,
    ///1: Clears the MENDF flag
    _1 = 1,
}
impl From<MENDFCL_A> for bool {
    #[inline(always)]
    fn from(variant: MENDFCL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MENDFCL` writer - MENDF Clear
pub type MENDFCL_W<'a, REG> = crate::BitWriter<'a, REG, MENDFCL_A>;
impl<'a, REG> MENDFCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MENDFCL_A::_0)
    }
    ///Clears the MENDF flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MENDFCL_A::_1)
    }
}
/**OVFF Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFFCL_A {
    ///0: No effect on operations
    _0 = 0,
    ///1: Clears the OVFF flag
    _1 = 1,
}
impl From<OVFFCL_A> for bool {
    #[inline(always)]
    fn from(variant: OVFFCL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVFFCL` writer - OVFF Clear
pub type OVFFCL_W<'a, REG> = crate::BitWriter<'a, REG, OVFFCL_A>;
impl<'a, REG> OVFFCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVFFCL_A::_0)
    }
    ///Clears the OVFF flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVFFCL_A::_1)
    }
}
impl R {
    ///Bit 0 - Frequency Error Interrupt Request Enable
    #[inline(always)]
    pub fn ferrie(&self) -> FERRIE_R {
        FERRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Measurement End Interrupt Request Enable
    #[inline(always)]
    pub fn mendie(&self) -> MENDIE_R {
        MENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Frequency Error Interrupt Request Enable
    #[inline(always)]
    pub fn ferrie(&mut self) -> FERRIE_W<CAICR_SPEC> {
        FERRIE_W::new(self, 0)
    }
    ///Bit 1 - Measurement End Interrupt Request Enable
    #[inline(always)]
    pub fn mendie(&mut self) -> MENDIE_W<CAICR_SPEC> {
        MENDIE_W::new(self, 1)
    }
    ///Bit 2 - Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn ovfie(&mut self) -> OVFIE_W<CAICR_SPEC> {
        OVFIE_W::new(self, 2)
    }
    ///Bit 4 - FERRF Clear
    #[inline(always)]
    pub fn ferrfcl(&mut self) -> FERRFCL_W<CAICR_SPEC> {
        FERRFCL_W::new(self, 4)
    }
    ///Bit 5 - MENDF Clear
    #[inline(always)]
    pub fn mendfcl(&mut self) -> MENDFCL_W<CAICR_SPEC> {
        MENDFCL_W::new(self, 5)
    }
    ///Bit 6 - OVFF Clear
    #[inline(always)]
    pub fn ovffcl(&mut self) -> OVFFCL_W<CAICR_SPEC> {
        OVFFCL_W::new(self, 6)
    }
}
/**CAC Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`caicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CAICR_SPEC;
impl crate::RegisterSpec for CAICR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`caicr::R`](R) reader structure
impl crate::Readable for CAICR_SPEC {}
///`write(|w| ..)` method takes [`caicr::W`](W) writer structure
impl crate::Writable for CAICR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAICR to value 0
impl crate::Resettable for CAICR_SPEC {}
