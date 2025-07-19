///Register `ECCPRCR2` reader
pub type R = crate::R<ECCPRCR2_SPEC>;
///Register `ECCPRCR2` writer
pub type W = crate::W<ECCPRCR2_SPEC>;
/**Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCPRCR2_A {
    ///0: Disable writes to the protected registers
    _0 = 0,
    ///1: Enable writes to the protected registers.
    _1 = 1,
}
impl From<ECCPRCR2_A> for bool {
    #[inline(always)]
    fn from(variant: ECCPRCR2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCPRCR2` reader - Register Write Control
pub type ECCPRCR2_R = crate::BitReader<ECCPRCR2_A>;
impl ECCPRCR2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCPRCR2_A {
        match self.bits {
            false => ECCPRCR2_A::_0,
            true => ECCPRCR2_A::_1,
        }
    }
    ///Disable writes to the protected registers
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCPRCR2_A::_0
    }
    ///Enable writes to the protected registers.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCPRCR2_A::_1
    }
}
///Field `ECCPRCR2` writer - Register Write Control
pub type ECCPRCR2_W<'a, REG> = crate::BitWriter<'a, REG, ECCPRCR2_A>;
impl<'a, REG> ECCPRCR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to the protected registers
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPRCR2_A::_0)
    }
    ///Enable writes to the protected registers.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPRCR2_A::_1)
    }
}
/**Write Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW2_A {
    ///120: These bits enable or disable writes to the ECCPRCR2 bit..
    _1111000 = 120,
    ///0: Writing to the ECCRAMPRCR2 bit is invalid.
    OTHERS = 0,
}
impl From<KW2_A> for u8 {
    #[inline(always)]
    fn from(variant: KW2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KW2_A {
    type Ux = u8;
}
impl crate::IsEnum for KW2_A {}
///Field `KW2` writer - Write Key Code
pub type KW2_W<'a, REG> = crate::FieldWriter<'a, REG, 7, KW2_A, crate::Safe>;
impl<'a, REG> KW2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///These bits enable or disable writes to the ECCPRCR2 bit..
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut crate::W<REG> {
        self.variant(KW2_A::_1111000)
    }
    ///Writing to the ECCRAMPRCR2 bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KW2_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn eccprcr2(&self) -> ECCPRCR2_R {
        ECCPRCR2_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn eccprcr2(&mut self) -> ECCPRCR2_W<'_, ECCPRCR2_SPEC> {
        ECCPRCR2_W::new(self, 0)
    }
    ///Bits 1:7 - Write Key Code
    #[inline(always)]
    pub fn kw2(&mut self) -> KW2_W<'_, ECCPRCR2_SPEC> {
        KW2_W::new(self, 1)
    }
}
/**ECC Protection Register 2

You can [`read`](crate::Reg::read) this register and get [`eccprcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECCPRCR2_SPEC;
impl crate::RegisterSpec for ECCPRCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eccprcr2::R`](R) reader structure
impl crate::Readable for ECCPRCR2_SPEC {}
///`write(|w| ..)` method takes [`eccprcr2::W`](W) writer structure
impl crate::Writable for ECCPRCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCPRCR2 to value 0
impl crate::Resettable for ECCPRCR2_SPEC {}
