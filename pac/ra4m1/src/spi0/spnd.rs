///Register `SPND` reader
pub type R = crate::R<SPND_SPEC>;
///Register `SPND` writer
pub type W = crate::W<SPND_SPEC>;
/**SPI Next-Access Delay Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPNDL_A {
    ///0: 1 RSPCK + 2 PCLK
    _000 = 0,
    ///1: 2 RSPCK + 2 PCLK
    _001 = 1,
    ///2: 3 RSPCK + 2 PCLK
    _010 = 2,
    ///3: 4 RSPCK + 2 PCLK
    _011 = 3,
    ///4: 5 RSPCK + 2 PCLK
    _100 = 4,
    ///5: 6 RSPCK + 2 PCLK
    _101 = 5,
    ///6: 7 RSPCK + 2 PCLK
    _110 = 6,
    ///7: 8 RSPCK + 2 PCLK
    _111 = 7,
}
impl From<SPNDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPNDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPNDL_A {
    type Ux = u8;
}
impl crate::IsEnum for SPNDL_A {}
///Field `SPNDL` reader - SPI Next-Access Delay Setting
pub type SPNDL_R = crate::FieldReader<SPNDL_A>;
impl SPNDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPNDL_A {
        match self.bits {
            0 => SPNDL_A::_000,
            1 => SPNDL_A::_001,
            2 => SPNDL_A::_010,
            3 => SPNDL_A::_011,
            4 => SPNDL_A::_100,
            5 => SPNDL_A::_101,
            6 => SPNDL_A::_110,
            7 => SPNDL_A::_111,
            _ => unreachable!(),
        }
    }
    ///1 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPNDL_A::_000
    }
    ///2 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPNDL_A::_001
    }
    ///3 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPNDL_A::_010
    }
    ///4 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPNDL_A::_011
    }
    ///5 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPNDL_A::_100
    }
    ///6 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPNDL_A::_101
    }
    ///7 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPNDL_A::_110
    }
    ///8 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPNDL_A::_111
    }
}
///Field `SPNDL` writer - SPI Next-Access Delay Setting
pub type SPNDL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPNDL_A, crate::Safe>;
impl<'a, REG> SPNDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_000)
    }
    ///2 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_001)
    }
    ///3 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_010)
    }
    ///4 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_011)
    }
    ///5 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_100)
    }
    ///6 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_101)
    }
    ///7 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_110)
    }
    ///8 RSPCK + 2 PCLK
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_111)
    }
}
impl R {
    ///Bits 0:2 - SPI Next-Access Delay Setting
    #[inline(always)]
    pub fn spndl(&self) -> SPNDL_R {
        SPNDL_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - SPI Next-Access Delay Setting
    #[inline(always)]
    pub fn spndl(&mut self) -> SPNDL_W<SPND_SPEC> {
        SPNDL_W::new(self, 0)
    }
}
/**SPI Next-Access Delay Register

You can [`read`](crate::Reg::read) this register and get [`spnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPND_SPEC;
impl crate::RegisterSpec for SPND_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spnd::R`](R) reader structure
impl crate::Readable for SPND_SPEC {}
///`write(|w| ..)` method takes [`spnd::W`](W) writer structure
impl crate::Writable for SPND_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPND to value 0
impl crate::Resettable for SPND_SPEC {}
