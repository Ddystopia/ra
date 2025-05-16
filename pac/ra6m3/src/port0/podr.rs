///Register `PODR` reader
pub type R = crate::R<PODR_SPEC>;
///Register `PODR` writer
pub type W = crate::W<PODR_SPEC>;
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PODR_A {
    ///0: Low output
    _0 = 0,
    ///1: High output.
    _1 = 1,
}
impl From<PODR_A> for u16 {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PODR_A {
    type Ux = u16;
}
impl crate::IsEnum for PODR_A {}
///Field `PODR` reader - Pmn Output Data
pub type PODR_R = crate::FieldReader<PODR_A>;
impl PODR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PODR_A> {
        match self.bits {
            0 => Some(PODR_A::_0),
            1 => Some(PODR_A::_1),
            _ => None,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    ///High output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
///Field `PODR` writer - Pmn Output Data
pub type PODR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PODR_A>;
impl<'a, REG> PODR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_0)
    }
    ///High output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_1)
    }
}
impl R {
    ///Bits 0:15 - Pmn Output Data
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Pmn Output Data
    #[inline(always)]
    pub fn podr(&mut self) -> PODR_W<PODR_SPEC> {
        PODR_W::new(self, 0)
    }
}
/**Output data register

You can [`read`](crate::Reg::read) this register and get [`podr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PODR_SPEC;
impl crate::RegisterSpec for PODR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`podr::R`](R) reader structure
impl crate::Readable for PODR_SPEC {}
///`write(|w| ..)` method takes [`podr::W`](W) writer structure
impl crate::Writable for PODR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PODR to value 0
impl crate::Resettable for PODR_SPEC {}
