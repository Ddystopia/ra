///Register `DMCNT` reader
pub type R = crate::R<DMCNT_SPEC>;
///Register `DMCNT` writer
pub type W = crate::W<DMCNT_SPEC>;
/**DMA Transfer Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<DTE_A> for bool {
    #[inline(always)]
    fn from(variant: DTE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DTE` reader - DMA Transfer Enable

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DTE_R = crate::BitReader<DTE_A>;
impl DTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTE_A {
        match self.bits {
            false => DTE_A::_0,
            true => DTE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTE_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTE_A::_1
    }
}
///Field `DTE` writer - DMA Transfer Enable
pub type DTE_W<'a, REG> = crate::BitWriter<'a, REG, DTE_A>;
impl<'a, REG> DTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTE_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTE_A::_1)
    }
}
impl R {
    ///Bit 0 - DMA Transfer Enable
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA Transfer Enable
    #[inline(always)]
    pub fn dte(&mut self) -> DTE_W<DMCNT_SPEC> {
        DTE_W::new(self, 0)
    }
}
/**DMA Transfer Enable Register

You can [`read`](crate::Reg::read) this register and get [`dmcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMCNT_SPEC;
impl crate::RegisterSpec for DMCNT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dmcnt::R`](R) reader structure
impl crate::Readable for DMCNT_SPEC {}
///`write(|w| ..)` method takes [`dmcnt::W`](W) writer structure
impl crate::Writable for DMCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMCNT to value 0
impl crate::Resettable for DMCNT_SPEC {}
