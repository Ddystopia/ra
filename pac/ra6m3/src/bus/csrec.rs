///Register `CS%sREC` reader
pub type R = crate::R<CSREC_SPEC>;
///Register `CS%sREC` writer
pub type W = crate::W<CSREC_SPEC>;
/**Read Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RRCV_A {
    ///0: No recovery cycle is inserted.
    _0X0 = 0,
    ///1: RRCV recovery cycle is inserted.
    OTHERS = 1,
}
impl From<RRCV_A> for u8 {
    #[inline(always)]
    fn from(variant: RRCV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RRCV_A {
    type Ux = u8;
}
impl crate::IsEnum for RRCV_A {}
///Field `RRCV` reader - Read Recovery
pub type RRCV_R = crate::FieldReader<RRCV_A>;
impl RRCV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRCV_A {
        match self.bits {
            0 => RRCV_A::_0X0,
            _ => RRCV_A::OTHERS,
        }
    }
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RRCV_A::_0X0
    }
    ///RRCV recovery cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RRCV_A::OTHERS)
    }
}
///Field `RRCV` writer - Read Recovery
pub type RRCV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RRCV_A, crate::Safe>;
impl<'a, REG> RRCV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RRCV_A::_0X0)
    }
    ///RRCV recovery cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RRCV_A::OTHERS)
    }
}
/**Write Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRCV_A {
    ///0: No recovery cycle is inserted.
    _0X0 = 0,
    ///1: WRCV recovery cycle is inserted.
    OTHERS = 1,
}
impl From<WRCV_A> for u8 {
    #[inline(always)]
    fn from(variant: WRCV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRCV_A {
    type Ux = u8;
}
impl crate::IsEnum for WRCV_A {}
///Field `WRCV` reader - Write Recovery
pub type WRCV_R = crate::FieldReader<WRCV_A>;
impl WRCV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRCV_A {
        match self.bits {
            0 => WRCV_A::_0X0,
            _ => WRCV_A::OTHERS,
        }
    }
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WRCV_A::_0X0
    }
    ///WRCV recovery cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), WRCV_A::OTHERS)
    }
}
///Field `WRCV` writer - Write Recovery
pub type WRCV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, WRCV_A, crate::Safe>;
impl<'a, REG> WRCV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No recovery cycle is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WRCV_A::_0X0)
    }
    ///WRCV recovery cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(WRCV_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Read Recovery
    #[inline(always)]
    pub fn rrcv(&self) -> RRCV_R {
        RRCV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Write Recovery
    #[inline(always)]
    pub fn wrcv(&self) -> WRCV_R {
        WRCV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Read Recovery
    #[inline(always)]
    pub fn rrcv(&mut self) -> RRCV_W<CSREC_SPEC> {
        RRCV_W::new(self, 0)
    }
    ///Bits 8:11 - Write Recovery
    #[inline(always)]
    pub fn wrcv(&mut self) -> WRCV_W<CSREC_SPEC> {
        WRCV_W::new(self, 8)
    }
}
/**CS%s Recovery Cycle Register

You can [`read`](crate::Reg::read) this register and get [`csrec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSREC_SPEC;
impl crate::RegisterSpec for CSREC_SPEC {
    type Ux = u16;
}
///`read()` method returns [`csrec::R`](R) reader structure
impl crate::Readable for CSREC_SPEC {}
///`write(|w| ..)` method takes [`csrec::W`](W) writer structure
impl crate::Writable for CSREC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sREC to value 0
impl crate::Resettable for CSREC_SPEC {}
