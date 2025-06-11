///Register `SSLND` reader
pub type R = crate::R<SSLND_SPEC>;
///Register `SSLND` writer
pub type W = crate::W<SSLND_SPEC>;
/**SSL Negation Delay Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLNDL_A {
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
impl From<SLNDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SLNDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLNDL_A {
    type Ux = u8;
}
impl crate::IsEnum for SLNDL_A {}
///Field `SLNDL` reader - SSL Negation Delay Setting
pub type SLNDL_R = crate::FieldReader<SLNDL_A>;
impl SLNDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLNDL_A {
        match self.bits {
            0 => SLNDL_A::_000,
            1 => SLNDL_A::_001,
            2 => SLNDL_A::_010,
            3 => SLNDL_A::_011,
            4 => SLNDL_A::_100,
            5 => SLNDL_A::_101,
            6 => SLNDL_A::_110,
            7 => SLNDL_A::_111,
            _ => unreachable!(),
        }
    }
    ///1 RSPCK
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SLNDL_A::_000
    }
    ///2 RSPCK
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SLNDL_A::_001
    }
    ///3 RSPCK
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SLNDL_A::_010
    }
    ///4 RSPCK
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SLNDL_A::_011
    }
    ///5 RSPCK
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SLNDL_A::_100
    }
    ///6 RSPCK
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SLNDL_A::_101
    }
    ///7 RSPCK
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SLNDL_A::_110
    }
    ///8 RSPCK
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SLNDL_A::_111
    }
}
///Field `SLNDL` writer - SSL Negation Delay Setting
pub type SLNDL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SLNDL_A, crate::Safe>;
impl<'a, REG> SLNDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RSPCK
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_000)
    }
    ///2 RSPCK
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_001)
    }
    ///3 RSPCK
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_010)
    }
    ///4 RSPCK
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_011)
    }
    ///5 RSPCK
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_100)
    }
    ///6 RSPCK
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_101)
    }
    ///7 RSPCK
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_110)
    }
    ///8 RSPCK
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_111)
    }
}
impl R {
    ///Bits 0:2 - SSL Negation Delay Setting
    #[inline(always)]
    pub fn slndl(&self) -> SLNDL_R {
        SLNDL_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - SSL Negation Delay Setting
    #[inline(always)]
    pub fn slndl(&mut self) -> SLNDL_W<SSLND_SPEC> {
        SLNDL_W::new(self, 0)
    }
}
/**SPI Slave Select Negation Delay Register

You can [`read`](crate::Reg::read) this register and get [`sslnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSLND_SPEC;
impl crate::RegisterSpec for SSLND_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sslnd::R`](R) reader structure
impl crate::Readable for SSLND_SPEC {}
///`write(|w| ..)` method takes [`sslnd::W`](W) writer structure
impl crate::Writable for SSLND_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSLND to value 0
impl crate::Resettable for SSLND_SPEC {}
