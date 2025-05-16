///Register `CS%sWCR1` reader
pub type R = crate::R<CSWCR1_SPEC>;
///Register `CS%sWCR1` writer
pub type W = crate::W<CSWCR1_SPEC>;
/**Page Write Cycle Wait SelectNOTE: The CSPWWAIT value is valid only when the PWENB bit in CSnMOD is set to 1.

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSPWWAIT_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of CSPWWAIT clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSPWWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSPWWAIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSPWWAIT_A {
    type Ux = u8;
}
impl crate::IsEnum for CSPWWAIT_A {}
///Field `CSPWWAIT` reader - Page Write Cycle Wait SelectNOTE: The CSPWWAIT value is valid only when the PWENB bit in CSnMOD is set to 1.
pub type CSPWWAIT_R = crate::FieldReader<CSPWWAIT_A>;
impl CSPWWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSPWWAIT_A {
        match self.bits {
            0 => CSPWWAIT_A::_0X0,
            _ => CSPWWAIT_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSPWWAIT_A::_0X0
    }
    ///Wait with a length of CSPWWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSPWWAIT_A::OTHERS)
    }
}
///Field `CSPWWAIT` writer - Page Write Cycle Wait SelectNOTE: The CSPWWAIT value is valid only when the PWENB bit in CSnMOD is set to 1.
pub type CSPWWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CSPWWAIT_A, crate::Safe>;
impl<'a, REG> CSPWWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSPWWAIT_A::_0X0)
    }
    ///Wait with a length of CSPWWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSPWWAIT_A::OTHERS)
    }
}
/**Page Read Cycle Wait SelectNOTE: The CSPRWAIT value is valid only when the PRENB bit in CSnMOD is set to 1.

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSPRWAIT_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of CSPRWAIT clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSPRWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSPRWAIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSPRWAIT_A {
    type Ux = u8;
}
impl crate::IsEnum for CSPRWAIT_A {}
///Field `CSPRWAIT` reader - Page Read Cycle Wait SelectNOTE: The CSPRWAIT value is valid only when the PRENB bit in CSnMOD is set to 1.
pub type CSPRWAIT_R = crate::FieldReader<CSPRWAIT_A>;
impl CSPRWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSPRWAIT_A {
        match self.bits {
            0 => CSPRWAIT_A::_0X0,
            _ => CSPRWAIT_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSPRWAIT_A::_0X0
    }
    ///Wait with a length of CSPRWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSPRWAIT_A::OTHERS)
    }
}
///Field `CSPRWAIT` writer - Page Read Cycle Wait SelectNOTE: The CSPRWAIT value is valid only when the PRENB bit in CSnMOD is set to 1.
pub type CSPRWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CSPRWAIT_A, crate::Safe>;
impl<'a, REG> CSPRWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSPRWAIT_A::_0X0)
    }
    ///Wait with a length of CSPRWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSPRWAIT_A::OTHERS)
    }
}
/**Normal Write Cycle Wait Select

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSWWAIT_A {
    ///0: No wait is inserted.
    _0X00 = 0,
    ///1: Wait with a length of CSWWAIT clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSWWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSWWAIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSWWAIT_A {
    type Ux = u8;
}
impl crate::IsEnum for CSWWAIT_A {}
///Field `CSWWAIT` reader - Normal Write Cycle Wait Select
pub type CSWWAIT_R = crate::FieldReader<CSWWAIT_A>;
impl CSWWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSWWAIT_A {
        match self.bits {
            0 => CSWWAIT_A::_0X00,
            _ => CSWWAIT_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CSWWAIT_A::_0X00
    }
    ///Wait with a length of CSWWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSWWAIT_A::OTHERS)
    }
}
///Field `CSWWAIT` writer - Normal Write Cycle Wait Select
pub type CSWWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CSWWAIT_A, crate::Safe>;
impl<'a, REG> CSWWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(CSWWAIT_A::_0X00)
    }
    ///Wait with a length of CSWWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSWWAIT_A::OTHERS)
    }
}
/**Normal Read Cycle Wait Select

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSRWAIT_A {
    ///0: No wait is inserted.
    _0X00 = 0,
    ///1: Wait with a length of CSRWAIT clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSRWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRWAIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSRWAIT_A {
    type Ux = u8;
}
impl crate::IsEnum for CSRWAIT_A {}
///Field `CSRWAIT` reader - Normal Read Cycle Wait Select
pub type CSRWAIT_R = crate::FieldReader<CSRWAIT_A>;
impl CSRWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSRWAIT_A {
        match self.bits {
            0 => CSRWAIT_A::_0X00,
            _ => CSRWAIT_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CSRWAIT_A::_0X00
    }
    ///Wait with a length of CSRWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSRWAIT_A::OTHERS)
    }
}
///Field `CSRWAIT` writer - Normal Read Cycle Wait Select
pub type CSRWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CSRWAIT_A, crate::Safe>;
impl<'a, REG> CSRWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(CSRWAIT_A::_0X00)
    }
    ///Wait with a length of CSRWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSRWAIT_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Page Write Cycle Wait SelectNOTE: The CSPWWAIT value is valid only when the PWENB bit in CSnMOD is set to 1.
    #[inline(always)]
    pub fn cspwwait(&self) -> CSPWWAIT_R {
        CSPWWAIT_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - Page Read Cycle Wait SelectNOTE: The CSPRWAIT value is valid only when the PRENB bit in CSnMOD is set to 1.
    #[inline(always)]
    pub fn csprwait(&self) -> CSPRWAIT_R {
        CSPRWAIT_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:20 - Normal Write Cycle Wait Select
    #[inline(always)]
    pub fn cswwait(&self) -> CSWWAIT_R {
        CSWWAIT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - Normal Read Cycle Wait Select
    #[inline(always)]
    pub fn csrwait(&self) -> CSRWAIT_R {
        CSRWAIT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:2 - Page Write Cycle Wait SelectNOTE: The CSPWWAIT value is valid only when the PWENB bit in CSnMOD is set to 1.
    #[inline(always)]
    pub fn cspwwait(&mut self) -> CSPWWAIT_W<CSWCR1_SPEC> {
        CSPWWAIT_W::new(self, 0)
    }
    ///Bits 8:10 - Page Read Cycle Wait SelectNOTE: The CSPRWAIT value is valid only when the PRENB bit in CSnMOD is set to 1.
    #[inline(always)]
    pub fn csprwait(&mut self) -> CSPRWAIT_W<CSWCR1_SPEC> {
        CSPRWAIT_W::new(self, 8)
    }
    ///Bits 16:20 - Normal Write Cycle Wait Select
    #[inline(always)]
    pub fn cswwait(&mut self) -> CSWWAIT_W<CSWCR1_SPEC> {
        CSWWAIT_W::new(self, 16)
    }
    ///Bits 24:28 - Normal Read Cycle Wait Select
    #[inline(always)]
    pub fn csrwait(&mut self) -> CSRWAIT_W<CSWCR1_SPEC> {
        CSRWAIT_W::new(self, 24)
    }
}
/**CS%s Wait Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cswcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSWCR1_SPEC;
impl crate::RegisterSpec for CSWCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cswcr1::R`](R) reader structure
impl crate::Readable for CSWCR1_SPEC {}
///`write(|w| ..)` method takes [`cswcr1::W`](W) writer structure
impl crate::Writable for CSWCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sWCR1 to value 0x0707_0707
impl crate::Resettable for CSWCR1_SPEC {
    const RESET_VALUE: u32 = 0x0707_0707;
}
