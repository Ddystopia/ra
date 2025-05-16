///Register `SPCKD` reader
pub type R = crate::R<SPCKD_SPEC>;
///Register `SPCKD` writer
pub type W = crate::W<SPCKD_SPEC>;
/**RSPCK Delay Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCKDL_A {
    ///0: 1 RSPCK
    _000 = 0,
    ///1: 2 RSPCK
    _001 = 1,
    ///2: 3 RSPCK
    _010 = 2,
    ///3: 4 RSPCK
    _011 = 3,
    ///4: 5 RSPCK
    _100 = 4,
    ///5: 6 RSPCK
    _101 = 5,
    ///6: 7 RSPCK
    _110 = 6,
    ///7: 8 RSPCK
    _111 = 7,
}
impl From<SCKDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCKDL_A {
    type Ux = u8;
}
impl crate::IsEnum for SCKDL_A {}
///Field `SCKDL` reader - RSPCK Delay Setting
pub type SCKDL_R = crate::FieldReader<SCKDL_A>;
impl SCKDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCKDL_A {
        match self.bits {
            0 => SCKDL_A::_000,
            1 => SCKDL_A::_001,
            2 => SCKDL_A::_010,
            3 => SCKDL_A::_011,
            4 => SCKDL_A::_100,
            5 => SCKDL_A::_101,
            6 => SCKDL_A::_110,
            7 => SCKDL_A::_111,
            _ => unreachable!(),
        }
    }
    ///1 RSPCK
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCKDL_A::_000
    }
    ///2 RSPCK
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCKDL_A::_001
    }
    ///3 RSPCK
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCKDL_A::_010
    }
    ///4 RSPCK
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCKDL_A::_011
    }
    ///5 RSPCK
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SCKDL_A::_100
    }
    ///6 RSPCK
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SCKDL_A::_101
    }
    ///7 RSPCK
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SCKDL_A::_110
    }
    ///8 RSPCK
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SCKDL_A::_111
    }
}
///Field `SCKDL` writer - RSPCK Delay Setting
pub type SCKDL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SCKDL_A, crate::Safe>;
impl<'a, REG> SCKDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RSPCK
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_000)
    }
    ///2 RSPCK
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_001)
    }
    ///3 RSPCK
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_010)
    }
    ///4 RSPCK
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_011)
    }
    ///5 RSPCK
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_100)
    }
    ///6 RSPCK
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_101)
    }
    ///7 RSPCK
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_110)
    }
    ///8 RSPCK
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_111)
    }
}
impl R {
    ///Bits 0:2 - RSPCK Delay Setting
    #[inline(always)]
    pub fn sckdl(&self) -> SCKDL_R {
        SCKDL_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - RSPCK Delay Setting
    #[inline(always)]
    pub fn sckdl(&mut self) -> SCKDL_W<SPCKD_SPEC> {
        SCKDL_W::new(self, 0)
    }
}
/**SPI Clock Delay Register

You can [`read`](crate::Reg::read) this register and get [`spckd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spckd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPCKD_SPEC;
impl crate::RegisterSpec for SPCKD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spckd::R`](R) reader structure
impl crate::Readable for SPCKD_SPEC {}
///`write(|w| ..)` method takes [`spckd::W`](W) writer structure
impl crate::Writable for SPCKD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCKD to value 0
impl crate::Resettable for SPCKD_SPEC {}
