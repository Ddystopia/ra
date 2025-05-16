///Register `ELSR%s` reader
pub type R = crate::R<ELSR_SPEC>;
///Register `ELSR%s` writer
pub type W = crate::W<ELSR_SPEC>;
/**Event Link Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ELS_A {
    ///0: Event output to the corresponding peripheral module is disabled.
    _0X000 = 0,
    ///1: Set the number for the event signal to be linked.
    OTHERS = 1,
}
impl From<ELS_A> for u16 {
    #[inline(always)]
    fn from(variant: ELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ELS_A {
    type Ux = u16;
}
impl crate::IsEnum for ELS_A {}
///Field `ELS` reader - Event Link Select
pub type ELS_R = crate::FieldReader<ELS_A>;
impl ELS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELS_A {
        match self.bits {
            0 => ELS_A::_0X000,
            _ => ELS_A::OTHERS,
        }
    }
    ///Event output to the corresponding peripheral module is disabled.
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == ELS_A::_0X000
    }
    ///Set the number for the event signal to be linked.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), ELS_A::OTHERS)
    }
}
///Field `ELS` writer - Event Link Select
pub type ELS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, ELS_A, crate::Safe>;
impl<'a, REG> ELS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Event output to the corresponding peripheral module is disabled.
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(ELS_A::_0X000)
    }
    ///Set the number for the event signal to be linked.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ELS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:8 - Event Link Select
    #[inline(always)]
    pub fn els(&self) -> ELS_R {
        ELS_R::new(self.bits & 0x01ff)
    }
}
impl W {
    ///Bits 0:8 - Event Link Select
    #[inline(always)]
    pub fn els(&mut self) -> ELS_W<ELSR_SPEC> {
        ELS_W::new(self, 0)
    }
}
/**Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`elsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ELSR_SPEC;
impl crate::RegisterSpec for ELSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`elsr::R`](R) reader structure
impl crate::Readable for ELSR_SPEC {}
///`write(|w| ..)` method takes [`elsr::W`](W) writer structure
impl crate::Writable for ELSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELSR%s to value 0
impl crate::Resettable for ELSR_SPEC {}
