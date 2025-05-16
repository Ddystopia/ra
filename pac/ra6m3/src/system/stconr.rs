///Register `STCONR` reader
pub type R = crate::R<STCONR_SPEC>;
///Register `STCONR` writer
pub type W = crate::W<STCONR_SPEC>;
/**SSTBY condition bit

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STCON_A {
    ///0: set this value in case of transferring to Software Standby Mode in using HOCO.
    _00 = 0,
    ///3: set this value in case of transferring to Software Standby Mode in using expect for HOCO.
    _11 = 3,
}
impl From<STCON_A> for u8 {
    #[inline(always)]
    fn from(variant: STCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STCON_A {
    type Ux = u8;
}
impl crate::IsEnum for STCON_A {}
///Field `STCON` reader - SSTBY condition bit
pub type STCON_R = crate::FieldReader<STCON_A>;
impl STCON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STCON_A> {
        match self.bits {
            0 => Some(STCON_A::_00),
            3 => Some(STCON_A::_11),
            _ => None,
        }
    }
    ///set this value in case of transferring to Software Standby Mode in using HOCO.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == STCON_A::_00
    }
    ///set this value in case of transferring to Software Standby Mode in using expect for HOCO.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == STCON_A::_11
    }
}
///Field `STCON` writer - SSTBY condition bit
pub type STCON_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STCON_A>;
impl<'a, REG> STCON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///set this value in case of transferring to Software Standby Mode in using HOCO.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(STCON_A::_00)
    }
    ///set this value in case of transferring to Software Standby Mode in using expect for HOCO.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(STCON_A::_11)
    }
}
impl R {
    ///Bits 0:1 - SSTBY condition bit
    #[inline(always)]
    pub fn stcon(&self) -> STCON_R {
        STCON_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - SSTBY condition bit
    #[inline(always)]
    pub fn stcon(&mut self) -> STCON_W<STCONR_SPEC> {
        STCON_W::new(self, 0)
    }
}
/**Standby Condition Register

You can [`read`](crate::Reg::read) this register and get [`stconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STCONR_SPEC;
impl crate::RegisterSpec for STCONR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`stconr::R`](R) reader structure
impl crate::Readable for STCONR_SPEC {}
///`write(|w| ..)` method takes [`stconr::W`](W) writer structure
impl crate::Writable for STCONR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STCONR to value 0xc3
impl crate::Resettable for STCONR_SPEC {
    const RESET_VALUE: u8 = 0xc3;
}
