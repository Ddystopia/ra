///Register `NMICR` reader
pub type R = crate::R<NMICR_SPEC>;
///Register `NMICR` writer
pub type W = crate::W<NMICR_SPEC>;
/**NMI Detection Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIMD_A {
    ///0: Falling edge
    _0 = 0,
    ///1: Rising edge
    _1 = 1,
}
impl From<NMIMD_A> for bool {
    #[inline(always)]
    fn from(variant: NMIMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NMIMD` reader - NMI Detection Set
pub type NMIMD_R = crate::BitReader<NMIMD_A>;
impl NMIMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NMIMD_A {
        match self.bits {
            false => NMIMD_A::_0,
            true => NMIMD_A::_1,
        }
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIMD_A::_0
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIMD_A::_1
    }
}
///Field `NMIMD` writer - NMI Detection Set
pub type NMIMD_W<'a, REG> = crate::BitWriter<'a, REG, NMIMD_A>;
impl<'a, REG> NMIMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NMIMD_A::_0)
    }
    ///Rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NMIMD_A::_1)
    }
}
/**NMI Digital Filter Sampling Clock

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCLKSEL_A {
    ///0: PCLKB
    _00 = 0,
    ///1: PCLKB/8
    _01 = 1,
    ///2: PCLKB/32
    _10 = 2,
    ///3: PCLKB/64
    _11 = 3,
}
impl From<NFCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for NFCLKSEL_A {}
///Field `NFCLKSEL` reader - NMI Digital Filter Sampling Clock
pub type NFCLKSEL_R = crate::FieldReader<NFCLKSEL_A>;
impl NFCLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFCLKSEL_A {
        match self.bits {
            0 => NFCLKSEL_A::_00,
            1 => NFCLKSEL_A::_01,
            2 => NFCLKSEL_A::_10,
            3 => NFCLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLKB
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCLKSEL_A::_00
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCLKSEL_A::_01
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCLKSEL_A::_10
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCLKSEL_A::_11
    }
}
///Field `NFCLKSEL` writer - NMI Digital Filter Sampling Clock
pub type NFCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NFCLKSEL_A, crate::Safe>;
impl<'a, REG> NFCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NFCLKSEL_A::_00)
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NFCLKSEL_A::_01)
    }
    ///PCLKB/32
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NFCLKSEL_A::_10)
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NFCLKSEL_A::_11)
    }
}
/**NMI Digital Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFLTEN_A {
    ///0: Digital filter is disabled.
    _0 = 0,
    ///1: Digital filter is enabled.
    _1 = 1,
}
impl From<NFLTEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFLTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFLTEN` reader - NMI Digital Filter Enable
pub type NFLTEN_R = crate::BitReader<NFLTEN_A>;
impl NFLTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFLTEN_A {
        match self.bits {
            false => NFLTEN_A::_0,
            true => NFLTEN_A::_1,
        }
    }
    ///Digital filter is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFLTEN_A::_0
    }
    ///Digital filter is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFLTEN_A::_1
    }
}
///Field `NFLTEN` writer - NMI Digital Filter Enable
pub type NFLTEN_W<'a, REG> = crate::BitWriter<'a, REG, NFLTEN_A>;
impl<'a, REG> NFLTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Digital filter is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFLTEN_A::_0)
    }
    ///Digital filter is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFLTEN_A::_1)
    }
}
impl R {
    ///Bit 0 - NMI Detection Set
    #[inline(always)]
    pub fn nmimd(&self) -> NMIMD_R {
        NMIMD_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - NMI Digital Filter Sampling Clock
    #[inline(always)]
    pub fn nfclksel(&self) -> NFCLKSEL_R {
        NFCLKSEL_R::new((self.bits >> 4) & 3)
    }
    ///Bit 7 - NMI Digital Filter Enable
    #[inline(always)]
    pub fn nflten(&self) -> NFLTEN_R {
        NFLTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - NMI Detection Set
    #[inline(always)]
    pub fn nmimd(&mut self) -> NMIMD_W<NMICR_SPEC> {
        NMIMD_W::new(self, 0)
    }
    ///Bits 4:5 - NMI Digital Filter Sampling Clock
    #[inline(always)]
    pub fn nfclksel(&mut self) -> NFCLKSEL_W<NMICR_SPEC> {
        NFCLKSEL_W::new(self, 4)
    }
    ///Bit 7 - NMI Digital Filter Enable
    #[inline(always)]
    pub fn nflten(&mut self) -> NFLTEN_W<NMICR_SPEC> {
        NFLTEN_W::new(self, 7)
    }
}
/**NMI Pin Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`nmicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NMICR_SPEC;
impl crate::RegisterSpec for NMICR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`nmicr::R`](R) reader structure
impl crate::Readable for NMICR_SPEC {}
///`write(|w| ..)` method takes [`nmicr::W`](W) writer structure
impl crate::Writable for NMICR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NMICR to value 0
impl crate::Resettable for NMICR_SPEC {}
