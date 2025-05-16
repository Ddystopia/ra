///Register `ECCPRCR` reader
pub type R = crate::R<ECCPRCR_SPEC>;
///Register `ECCPRCR` writer
pub type W = crate::W<ECCPRCR_SPEC>;
/**ECCRAMETST Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCPRCR_A {
    ///0: Disable writes to protected registers
    _0 = 0,
    ///1: Enable writes to protected registers
    _1 = 1,
}
impl From<ECCPRCR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCPRCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCPRCR` reader - ECCRAMETST Register Write Control
pub type ECCPRCR_R = crate::BitReader<ECCPRCR_A>;
impl ECCPRCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCPRCR_A {
        match self.bits {
            false => ECCPRCR_A::_0,
            true => ECCPRCR_A::_1,
        }
    }
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCPRCR_A::_0
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCPRCR_A::_1
    }
}
///Field `ECCPRCR` writer - ECCRAMETST Register Write Control
pub type ECCPRCR_W<'a, REG> = crate::BitWriter<'a, REG, ECCPRCR_A>;
impl<'a, REG> ECCPRCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPRCR_A::_0)
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPRCR_A::_1)
    }
}
/**Write Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW_A {
    ///120: Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b.
    _1111000 = 120,
    ///0: Writing to the ECCRAMPRCR bit is invalid.
    OTHERS = 0,
}
impl From<KW_A> for u8 {
    #[inline(always)]
    fn from(variant: KW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KW_A {
    type Ux = u8;
}
impl crate::IsEnum for KW_A {}
///Field `KW` writer - Write Key Code
pub type KW_W<'a, REG> = crate::FieldWriter<'a, REG, 7, KW_A, crate::Safe>;
impl<'a, REG> KW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b.
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut crate::W<REG> {
        self.variant(KW_A::_1111000)
    }
    ///Writing to the ECCRAMPRCR bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KW_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - ECCRAMETST Register Write Control
    #[inline(always)]
    pub fn eccprcr(&self) -> ECCPRCR_R {
        ECCPRCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ECCRAMETST Register Write Control
    #[inline(always)]
    pub fn eccprcr(&mut self) -> ECCPRCR_W<ECCPRCR_SPEC> {
        ECCPRCR_W::new(self, 0)
    }
    ///Bits 1:7 - Write Key Code
    #[inline(always)]
    pub fn kw(&mut self) -> KW_W<ECCPRCR_SPEC> {
        KW_W::new(self, 1)
    }
}
/**ECCRAM Protection Register

You can [`read`](crate::Reg::read) this register and get [`eccprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECCPRCR_SPEC;
impl crate::RegisterSpec for ECCPRCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eccprcr::R`](R) reader structure
impl crate::Readable for ECCPRCR_SPEC {}
///`write(|w| ..)` method takes [`eccprcr::W`](W) writer structure
impl crate::Writable for ECCPRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCPRCR to value 0
impl crate::Resettable for ECCPRCR_SPEC {}
