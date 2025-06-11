///Register `DAVREFCR` reader
pub type R = crate::R<DAVREFCR_SPEC>;
///Register `DAVREFCR` writer
pub type W = crate::W<DAVREFCR_SPEC>;
/**D/A Reference Voltage Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_A {
    ///0: Not selected
    _000 = 0,
    ///1: AVCC0/AVSS0
    _001 = 1,
    ///3: Internal reference voltage/AVSS0
    _011 = 3,
    ///6: VREFH/VREFL
    _110 = 6,
    ///2: Setting prohibited
    OTHERS = 2,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REF_A {
    type Ux = u8;
}
impl crate::IsEnum for REF_A {}
///Field `REF` reader - D/A Reference Voltage Select
pub type REF_R = crate::FieldReader<REF_A>;
impl REF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REF_A {
        match self.bits {
            0 => REF_A::_000,
            1 => REF_A::_001,
            3 => REF_A::_011,
            6 => REF_A::_110,
            _ => REF_A::OTHERS,
        }
    }
    ///Not selected
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == REF_A::_000
    }
    ///AVCC0/AVSS0
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == REF_A::_001
    }
    ///Internal reference voltage/AVSS0
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == REF_A::_011
    }
    ///VREFH/VREFL
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == REF_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), REF_A::OTHERS)
    }
}
///Field `REF` writer - D/A Reference Voltage Select
pub type REF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, REF_A, crate::Safe>;
impl<'a, REG> REF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Not selected
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_000)
    }
    ///AVCC0/AVSS0
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_001)
    }
    ///Internal reference voltage/AVSS0
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_011)
    }
    ///VREFH/VREFL
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - D/A Reference Voltage Select
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - D/A Reference Voltage Select
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W<DAVREFCR_SPEC> {
        REF_W::new(self, 0)
    }
}
/**D/A VREF Control Register

You can [`read`](crate::Reg::read) this register and get [`davrefcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`davrefcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAVREFCR_SPEC;
impl crate::RegisterSpec for DAVREFCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`davrefcr::R`](R) reader structure
impl crate::Readable for DAVREFCR_SPEC {}
///`write(|w| ..)` method takes [`davrefcr::W`](W) writer structure
impl crate::Writable for DAVREFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAVREFCR to value 0
impl crate::Resettable for DAVREFCR_SPEC {}
