///Register `RDMLR` reader
pub type R = crate::R<RDMLR_SPEC>;
///Register `RDMLR` writer
pub type W = crate::W<RDMLR_SPEC>;
/**Random Number Generation Counter

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RMD_A {
    ///0: Normal operation
    _00000H = 0,
    ///1: Setting prohibited
    OTHERS = 1,
}
impl From<RMD_A> for u32 {
    #[inline(always)]
    fn from(variant: RMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RMD_A {
    type Ux = u32;
}
impl crate::IsEnum for RMD_A {}
///Field `RMD` reader - Random Number Generation Counter
pub type RMD_R = crate::FieldReader<RMD_A>;
impl RMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMD_A {
        match self.bits {
            0 => RMD_A::_00000H,
            _ => RMD_A::OTHERS,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_00000h(&self) -> bool {
        *self == RMD_A::_00000H
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RMD_A::OTHERS)
    }
}
///Field `RMD` writer - Random Number Generation Counter
pub type RMD_W<'a, REG> = crate::FieldWriter<'a, REG, 20, RMD_A, crate::Safe>;
impl<'a, REG> RMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Normal operation
    #[inline(always)]
    pub fn _00000h(self) -> &'a mut crate::W<REG> {
        self.variant(RMD_A::_00000H)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RMD_A::OTHERS)
    }
}
impl R {
    ///Bits 0:19 - Random Number Generation Counter
    #[inline(always)]
    pub fn rmd(&self) -> RMD_R {
        RMD_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Random Number Generation Counter
    #[inline(always)]
    pub fn rmd(&mut self) -> RMD_W<RDMLR_SPEC> {
        RMD_W::new(self, 0)
    }
}
/**Random Number Generation Counter Upper Limit Setting Register

You can [`read`](crate::Reg::read) this register and get [`rdmlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdmlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDMLR_SPEC;
impl crate::RegisterSpec for RDMLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdmlr::R`](R) reader structure
impl crate::Readable for RDMLR_SPEC {}
///`write(|w| ..)` method takes [`rdmlr::W`](W) writer structure
impl crate::Writable for RDMLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDMLR to value 0
impl crate::Resettable for RDMLR_SPEC {}
