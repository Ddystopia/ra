///Register `BUSWAIT` reader
pub type R = crate::R<BUSWAIT_SPEC>;
///Register `BUSWAIT` writer
pub type W = crate::W<BUSWAIT_SPEC>;
/**CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)

Value on reset: 15*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BWAIT_A {
    ///0: BWAIT wait(s) ( BWAIT + 2 access cycles )
    BWAIT = 0,
}
impl From<BWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: BWAIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BWAIT_A {
    type Ux = u8;
}
impl crate::IsEnum for BWAIT_A {}
///Field `BWAIT` reader - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)
pub type BWAIT_R = crate::FieldReader<BWAIT_A>;
impl BWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BWAIT_A {
        match self.bits {
            _ => BWAIT_A::BWAIT,
        }
    }
    ///BWAIT wait(s) ( BWAIT + 2 access cycles )
    #[inline(always)]
    pub fn is_bwait(&self) -> bool {
        matches!(self.variant(), BWAIT_A::BWAIT)
    }
}
///Field `BWAIT` writer - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)
pub type BWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BWAIT_A, crate::Safe>;
impl<'a, REG> BWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///BWAIT wait(s) ( BWAIT + 2 access cycles )
    #[inline(always)]
    pub fn bwait(self) -> &'a mut crate::W<REG> {
        self.variant(BWAIT_A::BWAIT)
    }
}
impl R {
    ///Bits 0:3 - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)
    #[inline(always)]
    pub fn bwait(&self) -> BWAIT_R {
        BWAIT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - CPU Bus Access Wait Specification BWAIT waits (BWAIT+2 access cycles)
    #[inline(always)]
    pub fn bwait(&mut self) -> BWAIT_W<BUSWAIT_SPEC> {
        BWAIT_W::new(self, 0)
    }
}
/**CPU Bus Wait Register

You can [`read`](crate::Reg::read) this register and get [`buswait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buswait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUSWAIT_SPEC;
impl crate::RegisterSpec for BUSWAIT_SPEC {
    type Ux = u16;
}
///`read()` method returns [`buswait::R`](R) reader structure
impl crate::Readable for BUSWAIT_SPEC {}
///`write(|w| ..)` method takes [`buswait::W`](W) writer structure
impl crate::Writable for BUSWAIT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSWAIT to value 0x0f
impl crate::Resettable for BUSWAIT_SPEC {
    const RESET_VALUE: u16 = 0x0f;
}
