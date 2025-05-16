///Register `BCFRR` reader
pub type R = crate::R<BCFRR_SPEC>;
///Register `BCFRR` writer
pub type W = crate::W<BCFRR_SPEC>;
/**Broadcast Frame Continuous Receive Count Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BCF_A {
    ///0: Number of receptions is unlimited.
    _0000H = 0,
    ///1: Receive (BFC) frame.
    OTHERS = 1,
}
impl From<BCF_A> for u16 {
    #[inline(always)]
    fn from(variant: BCF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCF_A {
    type Ux = u16;
}
impl crate::IsEnum for BCF_A {}
///Field `BCF` reader - Broadcast Frame Continuous Receive Count Setting
pub type BCF_R = crate::FieldReader<BCF_A>;
impl BCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCF_A {
        match self.bits {
            0 => BCF_A::_0000H,
            _ => BCF_A::OTHERS,
        }
    }
    ///Number of receptions is unlimited.
    #[inline(always)]
    pub fn is_0000h(&self) -> bool {
        *self == BCF_A::_0000H
    }
    ///Receive (BFC) frame.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), BCF_A::OTHERS)
    }
}
///Field `BCF` writer - Broadcast Frame Continuous Receive Count Setting
pub type BCF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, BCF_A, crate::Safe>;
impl<'a, REG> BCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Number of receptions is unlimited.
    #[inline(always)]
    pub fn _0000h(self) -> &'a mut crate::W<REG> {
        self.variant(BCF_A::_0000H)
    }
    ///Receive (BFC) frame.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(BCF_A::OTHERS)
    }
}
impl R {
    ///Bits 0:15 - Broadcast Frame Continuous Receive Count Setting
    #[inline(always)]
    pub fn bcf(&self) -> BCF_R {
        BCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Broadcast Frame Continuous Receive Count Setting
    #[inline(always)]
    pub fn bcf(&mut self) -> BCF_W<BCFRR_SPEC> {
        BCF_W::new(self, 0)
    }
}
/**Broadcast Frame Receive Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`bcfrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcfrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCFRR_SPEC;
impl crate::RegisterSpec for BCFRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bcfrr::R`](R) reader structure
impl crate::Readable for BCFRR_SPEC {}
///`write(|w| ..)` method takes [`bcfrr::W`](W) writer structure
impl crate::Writable for BCFRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCFRR to value 0
impl crate::Resettable for BCFRR_SPEC {}
