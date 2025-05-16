///Register `SRAMPRCR` reader
pub type R = crate::R<SRAMPRCR_SPEC>;
///Register `SRAMPRCR` writer
pub type W = crate::W<SRAMPRCR_SPEC>;
/**Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMPRCR_A {
    ///0: Disable writes to protected registers
    _0 = 0,
    ///1: Enable writes to protected registers
    _1 = 1,
}
impl From<SRAMPRCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMPRCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMPRCR` reader - Register Write Control
pub type SRAMPRCR_R = crate::BitReader<SRAMPRCR_A>;
impl SRAMPRCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAMPRCR_A {
        match self.bits {
            false => SRAMPRCR_A::_0,
            true => SRAMPRCR_A::_1,
        }
    }
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAMPRCR_A::_0
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAMPRCR_A::_1
    }
}
///Field `SRAMPRCR` writer - Register Write Control
pub type SRAMPRCR_W<'a, REG> = crate::BitWriter<'a, REG, SRAMPRCR_A>;
impl<'a, REG> SRAMPRCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMPRCR_A::_0)
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMPRCR_A::_1)
    }
}
/**Write Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KW_A {
    ///120: Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b.
    _1111000 = 120,
    ///0: Writing to the RAMPRCR bit is invalid.
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
    ///Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b.
    #[inline(always)]
    pub fn _1111000(self) -> &'a mut crate::W<REG> {
        self.variant(KW_A::_1111000)
    }
    ///Writing to the RAMPRCR bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KW_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn sramprcr(&self) -> SRAMPRCR_R {
        SRAMPRCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn sramprcr(&mut self) -> SRAMPRCR_W<SRAMPRCR_SPEC> {
        SRAMPRCR_W::new(self, 0)
    }
    ///Bits 1:7 - Write Key Code
    #[inline(always)]
    pub fn kw(&mut self) -> KW_W<SRAMPRCR_SPEC> {
        KW_W::new(self, 1)
    }
}
/**SRAM Protection Register

You can [`read`](crate::Reg::read) this register and get [`sramprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRAMPRCR_SPEC;
impl crate::RegisterSpec for SRAMPRCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sramprcr::R`](R) reader structure
impl crate::Readable for SRAMPRCR_SPEC {}
///`write(|w| ..)` method takes [`sramprcr::W`](W) writer structure
impl crate::Writable for SRAMPRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRAMPRCR to value 0
impl crate::Resettable for SRAMPRCR_SPEC {}
