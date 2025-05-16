///Register `DMCRB` reader
pub type R = crate::R<DMCRB_SPEC>;
///Register `DMCRB` writer
pub type W = crate::W<DMCRB_SPEC>;
/**Specifies the number of block transfer operations or repeat transfer operations.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DMCRB_A {
    ///0: 65,536 blocks
    _0000 = 0,
    ///1: DMCRB blocks
    OTHERS = 1,
}
impl From<DMCRB_A> for u16 {
    #[inline(always)]
    fn from(variant: DMCRB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMCRB_A {
    type Ux = u16;
}
impl crate::IsEnum for DMCRB_A {}
///Field `DMCRB` reader - Specifies the number of block transfer operations or repeat transfer operations.
pub type DMCRB_R = crate::FieldReader<DMCRB_A>;
impl DMCRB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMCRB_A {
        match self.bits {
            0 => DMCRB_A::_0000,
            _ => DMCRB_A::OTHERS,
        }
    }
    ///65,536 blocks
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DMCRB_A::_0000
    }
    ///DMCRB blocks
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DMCRB_A::OTHERS)
    }
}
///Field `DMCRB` writer - Specifies the number of block transfer operations or repeat transfer operations.
pub type DMCRB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, DMCRB_A, crate::Safe>;
impl<'a, REG> DMCRB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///65,536 blocks
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(DMCRB_A::_0000)
    }
    ///DMCRB blocks
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DMCRB_A::OTHERS)
    }
}
impl R {
    ///Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations.
    #[inline(always)]
    pub fn dmcrb(&self) -> DMCRB_R {
        DMCRB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations.
    #[inline(always)]
    pub fn dmcrb(&mut self) -> DMCRB_W<DMCRB_SPEC> {
        DMCRB_W::new(self, 0)
    }
}
/**DMA Block Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMCRB_SPEC;
impl crate::RegisterSpec for DMCRB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dmcrb::R`](R) reader structure
impl crate::Readable for DMCRB_SPEC {}
///`write(|w| ..)` method takes [`dmcrb::W`](W) writer structure
impl crate::Writable for DMCRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMCRB to value 0
impl crate::Resettable for DMCRB_SPEC {}
