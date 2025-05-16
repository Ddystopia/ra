///Register `CMPSEL1` reader
pub type R = crate::R<CMPSEL1_SPEC>;
///Register `CMPSEL1` writer
pub type W = crate::W<CMPSEL1_SPEC>;
/**Reference voltage selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRVS_A {
    ///0: No reference voltage
    _0000 = 0,
    ///1: IVREF0 selected
    _0001 = 1,
    ///2: IVREF1 selected
    _0010 = 2,
    ///4: IVREF2 selected
    _0100 = 4,
    ///8: IVREF3 selected
    _1000 = 8,
    ///3: Setting prohibited
    OTHERS = 3,
}
impl From<CRVS_A> for u8 {
    #[inline(always)]
    fn from(variant: CRVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRVS_A {
    type Ux = u8;
}
impl crate::IsEnum for CRVS_A {}
///Field `CRVS` reader - Reference voltage selection
pub type CRVS_R = crate::FieldReader<CRVS_A>;
impl CRVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRVS_A {
        match self.bits {
            0 => CRVS_A::_0000,
            1 => CRVS_A::_0001,
            2 => CRVS_A::_0010,
            4 => CRVS_A::_0100,
            8 => CRVS_A::_1000,
            _ => CRVS_A::OTHERS,
        }
    }
    ///No reference voltage
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CRVS_A::_0000
    }
    ///IVREF0 selected
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CRVS_A::_0001
    }
    ///IVREF1 selected
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CRVS_A::_0010
    }
    ///IVREF2 selected
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CRVS_A::_0100
    }
    ///IVREF3 selected
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CRVS_A::_1000
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CRVS_A::OTHERS)
    }
}
///Field `CRVS` writer - Reference voltage selection
pub type CRVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CRVS_A, crate::Safe>;
impl<'a, REG> CRVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No reference voltage
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS_A::_0000)
    }
    ///IVREF0 selected
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS_A::_0001)
    }
    ///IVREF1 selected
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS_A::_0010)
    }
    ///IVREF2 selected
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS_A::_0100)
    }
    ///IVREF3 selected
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS_A::_1000)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CRVS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Reference voltage selection
    #[inline(always)]
    pub fn crvs(&self) -> CRVS_R {
        CRVS_R::new(self.bits & 0x0f)
    }
}
impl W {
    ///Bits 0:3 - Reference voltage selection
    #[inline(always)]
    pub fn crvs(&mut self) -> CRVS_W<CMPSEL1_SPEC> {
        CRVS_W::new(self, 0)
    }
}
/**Comparator Reference Voltage Select Register

You can [`read`](crate::Reg::read) this register and get [`cmpsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMPSEL1_SPEC;
impl crate::RegisterSpec for CMPSEL1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cmpsel1::R`](R) reader structure
impl crate::Readable for CMPSEL1_SPEC {}
///`write(|w| ..)` method takes [`cmpsel1::W`](W) writer structure
impl crate::Writable for CMPSEL1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMPSEL1 to value 0
impl crate::Resettable for CMPSEL1_SPEC {}
