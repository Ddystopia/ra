///Register `BKRACR` reader
pub type R = crate::R<BKRACR_SPEC>;
///Register `BKRACR` writer
pub type W = crate::W<BKRACR_SPEC>;
/**Backup Register Access Control Register

Value on reset: 6*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKRACS_A {
    ///0: Access control disable. When System clock source is SOSC or LOCO.
    _000 = 0,
    ///6: Access control enable. System clock source is other than SOSC or LOCO.
    _110 = 6,
    ///1: Setting prohibited
    OTHERS = 1,
}
impl From<BKRACS_A> for u8 {
    #[inline(always)]
    fn from(variant: BKRACS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BKRACS_A {
    type Ux = u8;
}
impl crate::IsEnum for BKRACS_A {}
///Field `BKRACS` reader - Backup Register Access Control Register
pub type BKRACS_R = crate::FieldReader<BKRACS_A>;
impl BKRACS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKRACS_A {
        match self.bits {
            0 => BKRACS_A::_000,
            6 => BKRACS_A::_110,
            _ => BKRACS_A::OTHERS,
        }
    }
    ///Access control disable. When System clock source is SOSC or LOCO.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BKRACS_A::_000
    }
    ///Access control enable. System clock source is other than SOSC or LOCO.
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BKRACS_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), BKRACS_A::OTHERS)
    }
}
///Field `BKRACS` writer - Backup Register Access Control Register
pub type BKRACS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BKRACS_A, crate::Safe>;
impl<'a, REG> BKRACS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Access control disable. When System clock source is SOSC or LOCO.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(BKRACS_A::_000)
    }
    ///Access control enable. System clock source is other than SOSC or LOCO.
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(BKRACS_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(BKRACS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Backup Register Access Control Register
    #[inline(always)]
    pub fn bkracs(&self) -> BKRACS_R {
        BKRACS_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - Backup Register Access Control Register
    #[inline(always)]
    pub fn bkracs(&mut self) -> BKRACS_W<BKRACR_SPEC> {
        BKRACS_W::new(self, 0)
    }
}
/**Backup Register Access Control Register

You can [`read`](crate::Reg::read) this register and get [`bkracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BKRACR_SPEC;
impl crate::RegisterSpec for BKRACR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bkracr::R`](R) reader structure
impl crate::Readable for BKRACR_SPEC {}
///`write(|w| ..)` method takes [`bkracr::W`](W) writer structure
impl crate::Writable for BKRACR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKRACR to value 0x06
impl crate::Resettable for BKRACR_SPEC {
    const RESET_VALUE: u8 = 0x06;
}
