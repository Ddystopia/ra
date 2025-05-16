///Register `FWEPROR` reader
pub type R = crate::R<FWEPROR_SPEC>;
///Register `FWEPROR` writer
pub type W = crate::W<FWEPROR_SPEC>;
/**Flash Programming and Erasure

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLWE_A {
    ///0: Prohibits programming and erasure of the code flash, data flash or blank checking.
    _00 = 0,
    ///1: Permits programming and erasure of the code flash, data flash or blank checking.
    _01 = 1,
    ///2: Prohibits programming and erasure of the code flash, data flash or blank checking.
    _10 = 2,
    ///3: Prohibits programming and erasure of the code flash, data flash or blank checking.
    _11 = 3,
}
impl From<FLWE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLWE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLWE_A {
    type Ux = u8;
}
impl crate::IsEnum for FLWE_A {}
///Field `FLWE` reader - Flash Programming and Erasure
pub type FLWE_R = crate::FieldReader<FLWE_A>;
impl FLWE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLWE_A {
        match self.bits {
            0 => FLWE_A::_00,
            1 => FLWE_A::_01,
            2 => FLWE_A::_10,
            3 => FLWE_A::_11,
            _ => unreachable!(),
        }
    }
    ///Prohibits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FLWE_A::_00
    }
    ///Permits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FLWE_A::_01
    }
    ///Prohibits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FLWE_A::_10
    }
    ///Prohibits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FLWE_A::_11
    }
}
///Field `FLWE` writer - Flash Programming and Erasure
pub type FLWE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FLWE_A, crate::Safe>;
impl<'a, REG> FLWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prohibits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(FLWE_A::_00)
    }
    ///Permits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(FLWE_A::_01)
    }
    ///Prohibits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(FLWE_A::_10)
    }
    ///Prohibits programming and erasure of the code flash, data flash or blank checking.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(FLWE_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Flash Programming and Erasure
    #[inline(always)]
    pub fn flwe(&self) -> FLWE_R {
        FLWE_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - Flash Programming and Erasure
    #[inline(always)]
    pub fn flwe(&mut self) -> FLWE_W<FWEPROR_SPEC> {
        FLWE_W::new(self, 0)
    }
}
/**Flash P/E Protect Register

You can [`read`](crate::Reg::read) this register and get [`fwepror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwepror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FWEPROR_SPEC;
impl crate::RegisterSpec for FWEPROR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`fwepror::R`](R) reader structure
impl crate::Readable for FWEPROR_SPEC {}
///`write(|w| ..)` method takes [`fwepror::W`](W) writer structure
impl crate::Writable for FWEPROR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FWEPROR to value 0x02
impl crate::Resettable for FWEPROR_SPEC {
    const RESET_VALUE: u8 = 0x02;
}
