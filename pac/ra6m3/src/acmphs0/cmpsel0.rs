///Register `CMPSEL0` reader
pub type R = crate::R<CMPSEL0_SPEC>;
///Register `CMPSEL0` writer
pub type W = crate::W<CMPSEL0_SPEC>;
/**Comparator input selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPSEL_A {
    ///0: No input
    _0000 = 0,
    ///1: IVCMP0 selected
    _0001 = 1,
    ///2: IVCMP1 selected
    _0010 = 2,
    ///4: IVCMP2 selected
    _0100 = 4,
    ///8: IVCMP3 selected
    _1000 = 8,
    ///3: Setting prohibited
    OTHERS = 3,
}
impl From<CMPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CMPSEL_A {}
///Field `CMPSEL` reader - Comparator input selection
pub type CMPSEL_R = crate::FieldReader<CMPSEL_A>;
impl CMPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSEL_A {
        match self.bits {
            0 => CMPSEL_A::_0000,
            1 => CMPSEL_A::_0001,
            2 => CMPSEL_A::_0010,
            4 => CMPSEL_A::_0100,
            8 => CMPSEL_A::_1000,
            _ => CMPSEL_A::OTHERS,
        }
    }
    ///No input
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CMPSEL_A::_0000
    }
    ///IVCMP0 selected
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CMPSEL_A::_0001
    }
    ///IVCMP1 selected
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CMPSEL_A::_0010
    }
    ///IVCMP2 selected
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CMPSEL_A::_0100
    }
    ///IVCMP3 selected
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CMPSEL_A::_1000
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CMPSEL_A::OTHERS)
    }
}
///Field `CMPSEL` writer - Comparator input selection
pub type CMPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CMPSEL_A, crate::Safe>;
impl<'a, REG> CMPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No input
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL_A::_0000)
    }
    ///IVCMP0 selected
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL_A::_0001)
    }
    ///IVCMP1 selected
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL_A::_0010)
    }
    ///IVCMP2 selected
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL_A::_0100)
    }
    ///IVCMP3 selected
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL_A::_1000)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSEL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Comparator input selection
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(self.bits & 0x0f)
    }
}
impl W {
    ///Bits 0:3 - Comparator input selection
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CMPSEL_W<CMPSEL0_SPEC> {
        CMPSEL_W::new(self, 0)
    }
}
/**Comparator Input Select Register

You can [`read`](crate::Reg::read) this register and get [`cmpsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMPSEL0_SPEC;
impl crate::RegisterSpec for CMPSEL0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cmpsel0::R`](R) reader structure
impl crate::Readable for CMPSEL0_SPEC {}
///`write(|w| ..)` method takes [`cmpsel0::W`](W) writer structure
impl crate::Writable for CMPSEL0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMPSEL0 to value 0
impl crate::Resettable for CMPSEL0_SPEC {}
