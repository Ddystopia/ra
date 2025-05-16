///Register `AGTIOC` reader
pub type R = crate::R<AGTIOC_SPEC>;
///Register `AGTIOC` writer
pub type W = crate::W<AGTIOC_SPEC>;
///Field `TEDGSEL` reader - I/O polarity switchFunction varies depending on the operating mode.
pub type TEDGSEL_R = crate::BitReader;
///Field `TEDGSEL` writer - I/O polarity switchFunction varies depending on the operating mode.
pub type TEDGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**AGTO output enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOE_A {
    ///0: AGTO output disabled (port)
    _0 = 0,
    ///1: AGTO output enabled
    _1 = 1,
}
impl From<TOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOE` reader - AGTO output enable
pub type TOE_R = crate::BitReader<TOE_A>;
impl TOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOE_A {
        match self.bits {
            false => TOE_A::_0,
            true => TOE_A::_1,
        }
    }
    ///AGTO output disabled (port)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOE_A::_0
    }
    ///AGTO output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOE_A::_1
    }
}
///Field `TOE` writer - AGTO output enable
pub type TOE_W<'a, REG> = crate::BitWriter<'a, REG, TOE_A>;
impl<'a, REG> TOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTO output disabled (port)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TOE_A::_0)
    }
    ///AGTO output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TOE_A::_1)
    }
}
/**AGTIO input filter select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIPF_A {
    ///0: No filter
    _00 = 0,
    ///1: Filter sampled at PCLKB
    _01 = 1,
    ///2: Filter sampled at PCLKB/8
    _10 = 2,
    ///3: Filter sampled at PCLKB/32
    _11 = 3,
}
impl From<TIPF_A> for u8 {
    #[inline(always)]
    fn from(variant: TIPF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIPF_A {
    type Ux = u8;
}
impl crate::IsEnum for TIPF_A {}
///Field `TIPF` reader - AGTIO input filter select
pub type TIPF_R = crate::FieldReader<TIPF_A>;
impl TIPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIPF_A {
        match self.bits {
            0 => TIPF_A::_00,
            1 => TIPF_A::_01,
            2 => TIPF_A::_10,
            3 => TIPF_A::_11,
            _ => unreachable!(),
        }
    }
    ///No filter
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TIPF_A::_00
    }
    ///Filter sampled at PCLKB
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TIPF_A::_01
    }
    ///Filter sampled at PCLKB/8
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TIPF_A::_10
    }
    ///Filter sampled at PCLKB/32
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TIPF_A::_11
    }
}
///Field `TIPF` writer - AGTIO input filter select
pub type TIPF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TIPF_A, crate::Safe>;
impl<'a, REG> TIPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TIPF_A::_00)
    }
    ///Filter sampled at PCLKB
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TIPF_A::_01)
    }
    ///Filter sampled at PCLKB/8
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TIPF_A::_10)
    }
    ///Filter sampled at PCLKB/32
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TIPF_A::_11)
    }
}
/**AGTIO count control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIOGT_A {
    ///0: Event is always counted
    _00 = 0,
    ///1: Event is counted during polarity period specified for AGTEE
    _01 = 1,
    ///2: Setting prohibited
    OTHERS = 2,
}
impl From<TIOGT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIOGT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIOGT_A {
    type Ux = u8;
}
impl crate::IsEnum for TIOGT_A {}
///Field `TIOGT` reader - AGTIO count control
pub type TIOGT_R = crate::FieldReader<TIOGT_A>;
impl TIOGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIOGT_A {
        match self.bits {
            0 => TIOGT_A::_00,
            1 => TIOGT_A::_01,
            _ => TIOGT_A::OTHERS,
        }
    }
    ///Event is always counted
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TIOGT_A::_00
    }
    ///Event is counted during polarity period specified for AGTEE
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TIOGT_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TIOGT_A::OTHERS)
    }
}
///Field `TIOGT` writer - AGTIO count control
pub type TIOGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TIOGT_A, crate::Safe>;
impl<'a, REG> TIOGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Event is always counted
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TIOGT_A::_00)
    }
    ///Event is counted during polarity period specified for AGTEE
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TIOGT_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TIOGT_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - I/O polarity switchFunction varies depending on the operating mode.
    #[inline(always)]
    pub fn tedgsel(&self) -> TEDGSEL_R {
        TEDGSEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - AGTO output enable
    #[inline(always)]
    pub fn toe(&self) -> TOE_R {
        TOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - AGTIO input filter select
    #[inline(always)]
    pub fn tipf(&self) -> TIPF_R {
        TIPF_R::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - AGTIO count control
    #[inline(always)]
    pub fn tiogt(&self) -> TIOGT_R {
        TIOGT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bit 0 - I/O polarity switchFunction varies depending on the operating mode.
    #[inline(always)]
    pub fn tedgsel(&mut self) -> TEDGSEL_W<AGTIOC_SPEC> {
        TEDGSEL_W::new(self, 0)
    }
    ///Bit 2 - AGTO output enable
    #[inline(always)]
    pub fn toe(&mut self) -> TOE_W<AGTIOC_SPEC> {
        TOE_W::new(self, 2)
    }
    ///Bits 4:5 - AGTIO input filter select
    #[inline(always)]
    pub fn tipf(&mut self) -> TIPF_W<AGTIOC_SPEC> {
        TIPF_W::new(self, 4)
    }
    ///Bits 6:7 - AGTIO count control
    #[inline(always)]
    pub fn tiogt(&mut self) -> TIOGT_W<AGTIOC_SPEC> {
        TIOGT_W::new(self, 6)
    }
}
/**AGT I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`agtioc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtioc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTIOC_SPEC;
impl crate::RegisterSpec for AGTIOC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtioc::R`](R) reader structure
impl crate::Readable for AGTIOC_SPEC {}
///`write(|w| ..)` method takes [`agtioc::W`](W) writer structure
impl crate::Writable for AGTIOC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTIOC to value 0
impl crate::Resettable for AGTIOC_SPEC {}
