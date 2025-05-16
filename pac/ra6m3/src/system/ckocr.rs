///Register `CKOCR` reader
pub type R = crate::R<CKOCR_SPEC>;
///Register `CKOCR` writer
pub type W = crate::W<CKOCR_SPEC>;
/**Clock out source select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKOSEL_A {
    ///0: HOCO
    _000 = 0,
    ///1: MOCO
    _001 = 1,
    ///2: LOCO
    _010 = 2,
    ///3: MOSC
    _011 = 3,
    ///4: SOSC
    _100 = 4,
    ///5: Setting prohibited
    OTHERS = 5,
}
impl From<CKOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKOSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CKOSEL_A {}
///Field `CKOSEL` reader - Clock out source select
pub type CKOSEL_R = crate::FieldReader<CKOSEL_A>;
impl CKOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKOSEL_A {
        match self.bits {
            0 => CKOSEL_A::_000,
            1 => CKOSEL_A::_001,
            2 => CKOSEL_A::_010,
            3 => CKOSEL_A::_011,
            4 => CKOSEL_A::_100,
            _ => CKOSEL_A::OTHERS,
        }
    }
    ///HOCO
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKOSEL_A::_000
    }
    ///MOCO
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKOSEL_A::_001
    }
    ///LOCO
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKOSEL_A::_010
    }
    ///MOSC
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKOSEL_A::_011
    }
    ///SOSC
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKOSEL_A::_100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CKOSEL_A::OTHERS)
    }
}
///Field `CKOSEL` writer - Clock out source select
pub type CKOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CKOSEL_A, crate::Safe>;
impl<'a, REG> CKOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HOCO
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSEL_A::_000)
    }
    ///MOCO
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSEL_A::_001)
    }
    ///LOCO
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSEL_A::_010)
    }
    ///MOSC
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSEL_A::_011)
    }
    ///SOSC
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSEL_A::_100)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CKOSEL_A::OTHERS)
    }
}
/**Clock out input frequency Division Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKODIV_A {
    ///0: /1
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /8
    _011 = 3,
    ///4: /16
    _100 = 4,
    ///5: /32
    _101 = 5,
    ///6: /64
    _110 = 6,
    ///7: /128
    _111 = 7,
}
impl From<CKODIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKODIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKODIV_A {
    type Ux = u8;
}
impl crate::IsEnum for CKODIV_A {}
///Field `CKODIV` reader - Clock out input frequency Division Select
pub type CKODIV_R = crate::FieldReader<CKODIV_A>;
impl CKODIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKODIV_A {
        match self.bits {
            0 => CKODIV_A::_000,
            1 => CKODIV_A::_001,
            2 => CKODIV_A::_010,
            3 => CKODIV_A::_011,
            4 => CKODIV_A::_100,
            5 => CKODIV_A::_101,
            6 => CKODIV_A::_110,
            7 => CKODIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKODIV_A::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKODIV_A::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKODIV_A::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKODIV_A::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKODIV_A::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKODIV_A::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CKODIV_A::_110
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CKODIV_A::_111
    }
}
///Field `CKODIV` writer - Clock out input frequency Division Select
pub type CKODIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CKODIV_A, crate::Safe>;
impl<'a, REG> CKODIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_110)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(CKODIV_A::_111)
    }
}
/**Clock out enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKOEN_A {
    ///0: Disable clock out
    _0 = 0,
    ///1: Enable clock out
    _1 = 1,
}
impl From<CKOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKOEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKOEN` reader - Clock out enable
pub type CKOEN_R = crate::BitReader<CKOEN_A>;
impl CKOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKOEN_A {
        match self.bits {
            false => CKOEN_A::_0,
            true => CKOEN_A::_1,
        }
    }
    ///Disable clock out
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKOEN_A::_0
    }
    ///Enable clock out
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKOEN_A::_1
    }
}
///Field `CKOEN` writer - Clock out enable
pub type CKOEN_W<'a, REG> = crate::BitWriter<'a, REG, CKOEN_A>;
impl<'a, REG> CKOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable clock out
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CKOEN_A::_0)
    }
    ///Enable clock out
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CKOEN_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Clock out source select
    #[inline(always)]
    pub fn ckosel(&self) -> CKOSEL_R {
        CKOSEL_R::new(self.bits & 7)
    }
    ///Bits 4:6 - Clock out input frequency Division Select
    #[inline(always)]
    pub fn ckodiv(&self) -> CKODIV_R {
        CKODIV_R::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - Clock out enable
    #[inline(always)]
    pub fn ckoen(&self) -> CKOEN_R {
        CKOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Clock out source select
    #[inline(always)]
    pub fn ckosel(&mut self) -> CKOSEL_W<CKOCR_SPEC> {
        CKOSEL_W::new(self, 0)
    }
    ///Bits 4:6 - Clock out input frequency Division Select
    #[inline(always)]
    pub fn ckodiv(&mut self) -> CKODIV_W<CKOCR_SPEC> {
        CKODIV_W::new(self, 4)
    }
    ///Bit 7 - Clock out enable
    #[inline(always)]
    pub fn ckoen(&mut self) -> CKOEN_W<CKOCR_SPEC> {
        CKOEN_W::new(self, 7)
    }
}
/**Clock Out Control Register

You can [`read`](crate::Reg::read) this register and get [`ckocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CKOCR_SPEC;
impl crate::RegisterSpec for CKOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ckocr::R`](R) reader structure
impl crate::Readable for CKOCR_SPEC {}
///`write(|w| ..)` method takes [`ckocr::W`](W) writer structure
impl crate::Writable for CKOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKOCR to value 0
impl crate::Resettable for CKOCR_SPEC {}
