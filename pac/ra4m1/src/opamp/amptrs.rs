///Register `AMPTRS` reader
pub type R = crate::R<AMPTRS_SPEC>;
///Register `AMPTRS` writer
pub type W = crate::W<AMPTRS_SPEC>;
/**ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMPTRS_A {
    ///0: Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3
    _00 = 0,
    ///1: Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1
    _01 = 1,
    ///2: Setting prohibited
    _10 = 2,
    ///3: Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0
    _11 = 3,
}
impl From<AMPTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: AMPTRS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AMPTRS_A {
    type Ux = u8;
}
impl crate::IsEnum for AMPTRS_A {}
///Field `AMPTRS` reader - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register.
pub type AMPTRS_R = crate::FieldReader<AMPTRS_A>;
impl AMPTRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AMPTRS_A {
        match self.bits {
            0 => AMPTRS_A::_00,
            1 => AMPTRS_A::_01,
            2 => AMPTRS_A::_10,
            3 => AMPTRS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AMPTRS_A::_00
    }
    ///Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AMPTRS_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AMPTRS_A::_10
    }
    ///Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AMPTRS_A::_11
    }
}
///Field `AMPTRS` writer - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register.
pub type AMPTRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, AMPTRS_A, crate::Safe>;
impl<'a, REG> AMPTRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRS_A::_00)
    }
    ///Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRS_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRS_A::_10)
    }
    ///Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(AMPTRS_A::_11)
    }
}
impl R {
    ///Bits 0:1 - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register.
    #[inline(always)]
    pub fn amptrs(&self) -> AMPTRS_R {
        AMPTRS_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register.
    #[inline(always)]
    pub fn amptrs(&mut self) -> AMPTRS_W<AMPTRS_SPEC> {
        AMPTRS_W::new(self, 0)
    }
}
/**Operational Amplifier Activation Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`amptrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AMPTRS_SPEC;
impl crate::RegisterSpec for AMPTRS_SPEC {
    type Ux = u8;
}
///`read()` method returns [`amptrs::R`](R) reader structure
impl crate::Readable for AMPTRS_SPEC {}
///`write(|w| ..)` method takes [`amptrs::W`](W) writer structure
impl crate::Writable for AMPTRS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AMPTRS to value 0
impl crate::Resettable for AMPTRS_SPEC {}
