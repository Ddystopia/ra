///Register `SELSR0` reader
pub type R = crate::R<SELSR0_SPEC>;
///Register `SELSR0` writer
pub type W = crate::W<SELSR0_SPEC>;
/**SYS Event Link Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SELS_A {
    ///0: Disable event output to the associated low-power mode module
    _000000000 = 0,
    ///1: Event signal number to be linked.
    OTHERS = 1,
}
impl From<SELS_A> for u16 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELS_A {
    type Ux = u16;
}
impl crate::IsEnum for SELS_A {}
///Field `SELS` reader - SYS Event Link Select
pub type SELS_R = crate::FieldReader<SELS_A>;
impl SELS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SELS_A {
        match self.bits {
            0 => SELS_A::_000000000,
            _ => SELS_A::OTHERS,
        }
    }
    ///Disable event output to the associated low-power mode module
    #[inline(always)]
    pub fn is_000000000(&self) -> bool {
        *self == SELS_A::_000000000
    }
    ///Event signal number to be linked.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SELS_A::OTHERS)
    }
}
///Field `SELS` writer - SYS Event Link Select
pub type SELS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, SELS_A, crate::Safe>;
impl<'a, REG> SELS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Disable event output to the associated low-power mode module
    #[inline(always)]
    pub fn _000000000(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::_000000000)
    }
    ///Event signal number to be linked.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::OTHERS)
    }
}
impl R {
    ///Bits 0:8 - SYS Event Link Select
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(self.bits & 0x01ff)
    }
}
impl W {
    ///Bits 0:8 - SYS Event Link Select
    #[inline(always)]
    pub fn sels(&mut self) -> SELS_W<SELSR0_SPEC> {
        SELS_W::new(self, 0)
    }
}
/**SYS Event Link Setting Register

You can [`read`](crate::Reg::read) this register and get [`selsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SELSR0_SPEC;
impl crate::RegisterSpec for SELSR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`selsr0::R`](R) reader structure
impl crate::Readable for SELSR0_SPEC {}
///`write(|w| ..)` method takes [`selsr0::W`](W) writer structure
impl crate::Writable for SELSR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SELSR0 to value 0
impl crate::Resettable for SELSR0_SPEC {}
