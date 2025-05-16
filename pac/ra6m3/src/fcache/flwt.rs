///Register `FLWT` reader
pub type R = crate::R<FLWT_SPEC>;
///Register `FLWT` writer
pub type W = crate::W<FLWT_SPEC>;
/**Flash Wait Cycle

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLWT_A {
    ///0: 0 wait (ICLK<=80MHz)
    _000 = 0,
    ///1: 1 wait (80MHz < ICLK <=160MHz)
    _001 = 1,
    ///2: 2 waits (160MHz < ICLK <=240MHz)
    _010 = 2,
    ///3: Setting prohibited
    OTHERS = 3,
}
impl From<FLWT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLWT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLWT_A {
    type Ux = u8;
}
impl crate::IsEnum for FLWT_A {}
///Field `FLWT` reader - Flash Wait Cycle
pub type FLWT_R = crate::FieldReader<FLWT_A>;
impl FLWT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLWT_A {
        match self.bits {
            0 => FLWT_A::_000,
            1 => FLWT_A::_001,
            2 => FLWT_A::_010,
            _ => FLWT_A::OTHERS,
        }
    }
    ///0 wait (ICLK<=80MHz)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FLWT_A::_000
    }
    ///1 wait (80MHz < ICLK <=160MHz)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FLWT_A::_001
    }
    ///2 waits (160MHz < ICLK <=240MHz)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FLWT_A::_010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), FLWT_A::OTHERS)
    }
}
///Field `FLWT` writer - Flash Wait Cycle
pub type FLWT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FLWT_A, crate::Safe>;
impl<'a, REG> FLWT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///0 wait (ICLK<=80MHz)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(FLWT_A::_000)
    }
    ///1 wait (80MHz < ICLK <=160MHz)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(FLWT_A::_001)
    }
    ///2 waits (160MHz < ICLK <=240MHz)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(FLWT_A::_010)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(FLWT_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Flash Wait Cycle
    #[inline(always)]
    pub fn flwt(&self) -> FLWT_R {
        FLWT_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - Flash Wait Cycle
    #[inline(always)]
    pub fn flwt(&mut self) -> FLWT_W<FLWT_SPEC> {
        FLWT_W::new(self, 0)
    }
}
/**Flash Wait Cycle Register

You can [`read`](crate::Reg::read) this register and get [`flwt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flwt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FLWT_SPEC;
impl crate::RegisterSpec for FLWT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`flwt::R`](R) reader structure
impl crate::Readable for FLWT_SPEC {}
///`write(|w| ..)` method takes [`flwt::W`](W) writer structure
impl crate::Writable for FLWT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLWT to value 0
impl crate::Resettable for FLWT_SPEC {}
