///Register `TRCKCR` reader
pub type R = crate::R<TRCKCR_SPEC>;
///Register `TRCKCR` writer
pub type W = crate::W<TRCKCR_SPEC>;
/**Trace Clock operating frequency select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRCK_A {
    ///0: /1
    _0000 = 0,
    ///1: /2
    _0001 = 1,
    ///2: /4
    _0010 = 2,
    ///3: Setting prohibited
    OTHERS = 3,
}
impl From<TRCK_A> for u8 {
    #[inline(always)]
    fn from(variant: TRCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRCK_A {
    type Ux = u8;
}
impl crate::IsEnum for TRCK_A {}
///Field `TRCK` reader - Trace Clock operating frequency select
pub type TRCK_R = crate::FieldReader<TRCK_A>;
impl TRCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRCK_A {
        match self.bits {
            0 => TRCK_A::_0000,
            1 => TRCK_A::_0001,
            2 => TRCK_A::_0010,
            _ => TRCK_A::OTHERS,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TRCK_A::_0000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TRCK_A::_0001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TRCK_A::_0010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TRCK_A::OTHERS)
    }
}
///Field `TRCK` writer - Trace Clock operating frequency select
pub type TRCK_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TRCK_A, crate::Safe>;
impl<'a, REG> TRCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(TRCK_A::_0000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(TRCK_A::_0001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(TRCK_A::_0010)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TRCK_A::OTHERS)
    }
}
/**Trace Clock operating Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRCKEN_A {
    ///0: Disable operation
    _0 = 0,
    ///1: Enable operation
    _1 = 1,
}
impl From<TRCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRCKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRCKEN` reader - Trace Clock operating Enable
pub type TRCKEN_R = crate::BitReader<TRCKEN_A>;
impl TRCKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRCKEN_A {
        match self.bits {
            false => TRCKEN_A::_0,
            true => TRCKEN_A::_1,
        }
    }
    ///Disable operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRCKEN_A::_0
    }
    ///Enable operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRCKEN_A::_1
    }
}
///Field `TRCKEN` writer - Trace Clock operating Enable
pub type TRCKEN_W<'a, REG> = crate::BitWriter<'a, REG, TRCKEN_A>;
impl<'a, REG> TRCKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRCKEN_A::_0)
    }
    ///Enable operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRCKEN_A::_1)
    }
}
impl R {
    ///Bits 0:3 - Trace Clock operating frequency select
    #[inline(always)]
    pub fn trck(&self) -> TRCK_R {
        TRCK_R::new(self.bits & 0x0f)
    }
    ///Bit 7 - Trace Clock operating Enable
    #[inline(always)]
    pub fn trcken(&self) -> TRCKEN_R {
        TRCKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Trace Clock operating frequency select
    #[inline(always)]
    pub fn trck(&mut self) -> TRCK_W<TRCKCR_SPEC> {
        TRCK_W::new(self, 0)
    }
    ///Bit 7 - Trace Clock operating Enable
    #[inline(always)]
    pub fn trcken(&mut self) -> TRCKEN_W<TRCKCR_SPEC> {
        TRCKEN_W::new(self, 7)
    }
}
/**Trace Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`trckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TRCKCR_SPEC;
impl crate::RegisterSpec for TRCKCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`trckcr::R`](R) reader structure
impl crate::Readable for TRCKCR_SPEC {}
///`write(|w| ..)` method takes [`trckcr::W`](W) writer structure
impl crate::Writable for TRCKCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TRCKCR to value 0x01
impl crate::Resettable for TRCKCR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
