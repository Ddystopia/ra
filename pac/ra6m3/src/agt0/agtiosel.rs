///Register `AGTIOSEL` reader
pub type R = crate::R<AGTIOSEL_SPEC>;
///Register `AGTIOSEL` writer
pub type W = crate::W<AGTIOSEL_SPEC>;
/**AGTIO pin select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    ///0: AGTIO_A can not be used as AGTIO input pin in deep software standby mode
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: AGTIO_B can be used as AGTIO input pin in deep software standby mode. AGTIO_B is input only. It is not possible to output.
    _10 = 2,
    ///3: AGTIO_C can be used as AGTIO input pin in deep software standby mode. AGTIO_C is input only. It is not possible to output.
    _11 = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SEL_A {}
///Field `SEL` reader - AGTIO pin select
pub type SEL_R = crate::FieldReader<SEL_A>;
impl SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::_00,
            1 => SEL_A::_01,
            2 => SEL_A::_10,
            3 => SEL_A::_11,
            _ => unreachable!(),
        }
    }
    ///AGTIO_A can not be used as AGTIO input pin in deep software standby mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEL_A::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEL_A::_01
    }
    ///AGTIO_B can be used as AGTIO input pin in deep software standby mode. AGTIO_B is input only. It is not possible to output.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL_A::_10
    }
    ///AGTIO_C can be used as AGTIO input pin in deep software standby mode. AGTIO_C is input only. It is not possible to output.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL_A::_11
    }
}
///Field `SEL` writer - AGTIO pin select
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SEL_A, crate::Safe>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AGTIO_A can not be used as AGTIO input pin in deep software standby mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_01)
    }
    ///AGTIO_B can be used as AGTIO input pin in deep software standby mode. AGTIO_B is input only. It is not possible to output.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_10)
    }
    ///AGTIO_C can be used as AGTIO input pin in deep software standby mode. AGTIO_C is input only. It is not possible to output.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_11)
    }
}
/**AGTIO input enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIES_A {
    ///0: external event input disable during software standby mode
    _0 = 0,
    ///1: external event input enable during software standby mode
    _1 = 1,
}
impl From<TIES_A> for bool {
    #[inline(always)]
    fn from(variant: TIES_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIES` reader - AGTIO input enable
pub type TIES_R = crate::BitReader<TIES_A>;
impl TIES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIES_A {
        match self.bits {
            false => TIES_A::_0,
            true => TIES_A::_1,
        }
    }
    ///external event input disable during software standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIES_A::_0
    }
    ///external event input enable during software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIES_A::_1
    }
}
///Field `TIES` writer - AGTIO input enable
pub type TIES_W<'a, REG> = crate::BitWriter<'a, REG, TIES_A>;
impl<'a, REG> TIES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///external event input disable during software standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TIES_A::_0)
    }
    ///external event input enable during software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TIES_A::_1)
    }
}
impl R {
    ///Bits 0:1 - AGTIO pin select
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(self.bits & 3)
    }
    ///Bit 4 - AGTIO input enable
    #[inline(always)]
    pub fn ties(&self) -> TIES_R {
        TIES_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - AGTIO pin select
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<AGTIOSEL_SPEC> {
        SEL_W::new(self, 0)
    }
    ///Bit 4 - AGTIO input enable
    #[inline(always)]
    pub fn ties(&mut self) -> TIES_W<AGTIOSEL_SPEC> {
        TIES_W::new(self, 4)
    }
}
/**AGT Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtiosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtiosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTIOSEL_SPEC;
impl crate::RegisterSpec for AGTIOSEL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtiosel::R`](R) reader structure
impl crate::Readable for AGTIOSEL_SPEC {}
///`write(|w| ..)` method takes [`agtiosel::W`](W) writer structure
impl crate::Writable for AGTIOSEL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTIOSEL to value 0
impl crate::Resettable for AGTIOSEL_SPEC {}
