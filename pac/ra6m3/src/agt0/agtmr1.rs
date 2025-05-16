///Register `AGTMR1` reader
pub type R = crate::R<AGTMR1_SPEC>;
///Register `AGTMR1` writer
pub type W = crate::W<AGTMR1_SPEC>;
/**AGT operating mode select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMOD_A {
    ///0: Timer mode
    _000 = 0,
    ///1: Pulse output mode
    _001 = 1,
    ///2: Event counter mode
    _010 = 2,
    ///3: Pulse width measurement mode
    _011 = 3,
    ///4: Pulse period measurement mode
    _100 = 4,
    ///5: Setting prohibited
    OTHERS = 5,
}
impl From<TMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for TMOD_A {}
///Field `TMOD` reader - AGT operating mode select
pub type TMOD_R = crate::FieldReader<TMOD_A>;
impl TMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TMOD_A {
        match self.bits {
            0 => TMOD_A::_000,
            1 => TMOD_A::_001,
            2 => TMOD_A::_010,
            3 => TMOD_A::_011,
            4 => TMOD_A::_100,
            _ => TMOD_A::OTHERS,
        }
    }
    ///Timer mode
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TMOD_A::_000
    }
    ///Pulse output mode
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TMOD_A::_001
    }
    ///Event counter mode
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TMOD_A::_010
    }
    ///Pulse width measurement mode
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TMOD_A::_011
    }
    ///Pulse period measurement mode
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TMOD_A::_100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TMOD_A::OTHERS)
    }
}
///Field `TMOD` writer - AGT operating mode select
pub type TMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TMOD_A, crate::Safe>;
impl<'a, REG> TMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer mode
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::_000)
    }
    ///Pulse output mode
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::_001)
    }
    ///Event counter mode
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::_010)
    }
    ///Pulse width measurement mode
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::_011)
    }
    ///Pulse period measurement mode
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::_100)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::OTHERS)
    }
}
/**AGTIO edge polarity select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEDGPL_A {
    ///0: One edge
    _0 = 0,
    ///1: Both edges
    _1 = 1,
}
impl From<TEDGPL_A> for bool {
    #[inline(always)]
    fn from(variant: TEDGPL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEDGPL` reader - AGTIO edge polarity select
pub type TEDGPL_R = crate::BitReader<TEDGPL_A>;
impl TEDGPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEDGPL_A {
        match self.bits {
            false => TEDGPL_A::_0,
            true => TEDGPL_A::_1,
        }
    }
    ///One edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEDGPL_A::_0
    }
    ///Both edges
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEDGPL_A::_1
    }
}
///Field `TEDGPL` writer - AGTIO edge polarity select
pub type TEDGPL_W<'a, REG> = crate::BitWriter<'a, REG, TEDGPL_A>;
impl<'a, REG> TEDGPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEDGPL_A::_0)
    }
    ///Both edges
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEDGPL_A::_1)
    }
}
/**AGT count source select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCK_A {
    ///0: PCLKB
    _000 = 0,
    ///1: PCLKB/8
    _001 = 1,
    ///3: PCLKB/2
    _011 = 3,
    ///4: Divided clock LOCO specified by AGTMR2.CKS bit.
    _100 = 4,
    ///5: Underflow event signal from AGT
    _101 = 5,
    ///6: Divided clock fSUB specified by AGTMR2.CKS bit.
    _110 = 6,
    ///2: Setting prohibited
    OTHERS = 2,
}
impl From<TCK_A> for u8 {
    #[inline(always)]
    fn from(variant: TCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCK_A {
    type Ux = u8;
}
impl crate::IsEnum for TCK_A {}
///Field `TCK` reader - AGT count source select
pub type TCK_R = crate::FieldReader<TCK_A>;
impl TCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCK_A {
        match self.bits {
            0 => TCK_A::_000,
            1 => TCK_A::_001,
            3 => TCK_A::_011,
            4 => TCK_A::_100,
            5 => TCK_A::_101,
            6 => TCK_A::_110,
            _ => TCK_A::OTHERS,
        }
    }
    ///PCLKB
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TCK_A::_000
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TCK_A::_001
    }
    ///PCLKB/2
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TCK_A::_011
    }
    ///Divided clock LOCO specified by AGTMR2.CKS bit.
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TCK_A::_100
    }
    ///Underflow event signal from AGT
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TCK_A::_101
    }
    ///Divided clock fSUB specified by AGTMR2.CKS bit.
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TCK_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TCK_A::OTHERS)
    }
}
///Field `TCK` writer - AGT count source select
pub type TCK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TCK_A, crate::Safe>;
impl<'a, REG> TCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::_000)
    }
    ///PCLKB/8
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::_001)
    }
    ///PCLKB/2
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::_011)
    }
    ///Divided clock LOCO specified by AGTMR2.CKS bit.
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::_100)
    }
    ///Underflow event signal from AGT
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::_101)
    }
    ///Divided clock fSUB specified by AGTMR2.CKS bit.
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TCK_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - AGT operating mode select
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(self.bits & 7)
    }
    ///Bit 3 - AGTIO edge polarity select
    #[inline(always)]
    pub fn tedgpl(&self) -> TEDGPL_R {
        TEDGPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - AGT count source select
    #[inline(always)]
    pub fn tck(&self) -> TCK_R {
        TCK_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    ///Bits 0:2 - AGT operating mode select
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W<AGTMR1_SPEC> {
        TMOD_W::new(self, 0)
    }
    ///Bit 3 - AGTIO edge polarity select
    #[inline(always)]
    pub fn tedgpl(&mut self) -> TEDGPL_W<AGTMR1_SPEC> {
        TEDGPL_W::new(self, 3)
    }
    ///Bits 4:6 - AGT count source select
    #[inline(always)]
    pub fn tck(&mut self) -> TCK_W<AGTMR1_SPEC> {
        TCK_W::new(self, 4)
    }
}
/**AGT Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`agtmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTMR1_SPEC;
impl crate::RegisterSpec for AGTMR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtmr1::R`](R) reader structure
impl crate::Readable for AGTMR1_SPEC {}
///`write(|w| ..)` method takes [`agtmr1::W`](W) writer structure
impl crate::Writable for AGTMR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTMR1 to value 0
impl crate::Resettable for AGTMR1_SPEC {}
