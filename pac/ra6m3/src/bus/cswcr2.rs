///Register `CS%sWCR2` reader
pub type R = crate::R<CSWCR2_SPEC>;
///Register `CS%sWCR2` writer
pub type W = crate::W<CSWCR2_SPEC>;
/**Read-Access CS Extension Cycle Select

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSROFF_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of CSROFF clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSROFF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSROFF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSROFF_A {
    type Ux = u8;
}
impl crate::IsEnum for CSROFF_A {}
///Field `CSROFF` reader - Read-Access CS Extension Cycle Select
pub type CSROFF_R = crate::FieldReader<CSROFF_A>;
impl CSROFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSROFF_A {
        match self.bits {
            0 => CSROFF_A::_0X0,
            _ => CSROFF_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSROFF_A::_0X0
    }
    ///Wait with a length of CSROFF clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSROFF_A::OTHERS)
    }
}
///Field `CSROFF` writer - Read-Access CS Extension Cycle Select
pub type CSROFF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CSROFF_A, crate::Safe>;
impl<'a, REG> CSROFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSROFF_A::_0X0)
    }
    ///Wait with a length of CSROFF clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSROFF_A::OTHERS)
    }
}
/**Write-Access CS Extension Cycle Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSWOFF_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of CSWOFF clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSWOFF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSWOFF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSWOFF_A {
    type Ux = u8;
}
impl crate::IsEnum for CSWOFF_A {}
///Field `CSWOFF` reader - Write-Access CS Extension Cycle Select
pub type CSWOFF_R = crate::FieldReader<CSWOFF_A>;
impl CSWOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSWOFF_A {
        match self.bits {
            0 => CSWOFF_A::_0X0,
            _ => CSWOFF_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSWOFF_A::_0X0
    }
    ///Wait with a length of CSWOFF clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSWOFF_A::OTHERS)
    }
}
///Field `CSWOFF` writer - Write-Access CS Extension Cycle Select
pub type CSWOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CSWOFF_A, crate::Safe>;
impl<'a, REG> CSWOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSWOFF_A::_0X0)
    }
    ///Wait with a length of CSWOFF clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSWOFF_A::OTHERS)
    }
}
/**Write Data Output Extension Cycle Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOFF_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of WDOFF clock cycle is inserted.
    OTHERS = 1,
}
impl From<WDOFF_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOFF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDOFF_A {
    type Ux = u8;
}
impl crate::IsEnum for WDOFF_A {}
///Field `WDOFF` reader - Write Data Output Extension Cycle Select
pub type WDOFF_R = crate::FieldReader<WDOFF_A>;
impl WDOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDOFF_A {
        match self.bits {
            0 => WDOFF_A::_0X0,
            _ => WDOFF_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WDOFF_A::_0X0
    }
    ///Wait with a length of WDOFF clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), WDOFF_A::OTHERS)
    }
}
///Field `WDOFF` writer - Write Data Output Extension Cycle Select
pub type WDOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDOFF_A, crate::Safe>;
impl<'a, REG> WDOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDOFF_A::_0X0)
    }
    ///Wait with a length of WDOFF clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(WDOFF_A::OTHERS)
    }
}
/**Address Cycle Wait Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWAIT_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of AWAIT clock cycle is inserted.
    OTHERS = 1,
}
impl From<AWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: AWAIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWAIT_A {
    type Ux = u8;
}
impl crate::IsEnum for AWAIT_A {}
///Field `AWAIT` reader - Address Cycle Wait Select
pub type AWAIT_R = crate::FieldReader<AWAIT_A>;
impl AWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWAIT_A {
        match self.bits {
            0 => AWAIT_A::_0X0,
            _ => AWAIT_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == AWAIT_A::_0X0
    }
    ///Wait with a length of AWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), AWAIT_A::OTHERS)
    }
}
///Field `AWAIT` writer - Address Cycle Wait Select
pub type AWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, AWAIT_A, crate::Safe>;
impl<'a, REG> AWAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWAIT_A::_0X0)
    }
    ///Wait with a length of AWAIT clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(AWAIT_A::OTHERS)
    }
}
/**RD Assert Wait Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDON_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of RDON clock cycle is inserted.
    OTHERS = 1,
}
impl From<RDON_A> for u8 {
    #[inline(always)]
    fn from(variant: RDON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDON_A {
    type Ux = u8;
}
impl crate::IsEnum for RDON_A {}
///Field `RDON` reader - RD Assert Wait Select
pub type RDON_R = crate::FieldReader<RDON_A>;
impl RDON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDON_A {
        match self.bits {
            0 => RDON_A::_0X0,
            _ => RDON_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RDON_A::_0X0
    }
    ///Wait with a length of RDON clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RDON_A::OTHERS)
    }
}
///Field `RDON` writer - RD Assert Wait Select
pub type RDON_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RDON_A, crate::Safe>;
impl<'a, REG> RDON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RDON_A::_0X0)
    }
    ///Wait with a length of RDON clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RDON_A::OTHERS)
    }
}
/**WR Assert Wait Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRON_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of WRON clock cycle is inserted.
    OTHERS = 1,
}
impl From<WRON_A> for u8 {
    #[inline(always)]
    fn from(variant: WRON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRON_A {
    type Ux = u8;
}
impl crate::IsEnum for WRON_A {}
///Field `WRON` reader - WR Assert Wait Select
pub type WRON_R = crate::FieldReader<WRON_A>;
impl WRON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRON_A {
        match self.bits {
            0 => WRON_A::_0X0,
            _ => WRON_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WRON_A::_0X0
    }
    ///Wait with a length of WRON clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), WRON_A::OTHERS)
    }
}
///Field `WRON` writer - WR Assert Wait Select
pub type WRON_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WRON_A, crate::Safe>;
impl<'a, REG> WRON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WRON_A::_0X0)
    }
    ///Wait with a length of WRON clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(WRON_A::OTHERS)
    }
}
/**Write Data Output Wait Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDON_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of WDON clock cycle is inserted.
    OTHERS = 1,
}
impl From<WDON_A> for u8 {
    #[inline(always)]
    fn from(variant: WDON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDON_A {
    type Ux = u8;
}
impl crate::IsEnum for WDON_A {}
///Field `WDON` reader - Write Data Output Wait Select
pub type WDON_R = crate::FieldReader<WDON_A>;
impl WDON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDON_A {
        match self.bits {
            0 => WDON_A::_0X0,
            _ => WDON_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WDON_A::_0X0
    }
    ///Wait with a length of WDON clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), WDON_A::OTHERS)
    }
}
///Field `WDON` writer - Write Data Output Wait Select
pub type WDON_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDON_A, crate::Safe>;
impl<'a, REG> WDON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDON_A::_0X0)
    }
    ///Wait with a length of WDON clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(WDON_A::OTHERS)
    }
}
/**CS Assert Wait Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSON_A {
    ///0: No wait is inserted.
    _0X0 = 0,
    ///1: Wait with a length of CSON clock cycle is inserted.
    OTHERS = 1,
}
impl From<CSON_A> for u8 {
    #[inline(always)]
    fn from(variant: CSON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSON_A {
    type Ux = u8;
}
impl crate::IsEnum for CSON_A {}
///Field `CSON` reader - CS Assert Wait Select
pub type CSON_R = crate::FieldReader<CSON_A>;
impl CSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSON_A {
        match self.bits {
            0 => CSON_A::_0X0,
            _ => CSON_A::OTHERS,
        }
    }
    ///No wait is inserted.
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSON_A::_0X0
    }
    ///Wait with a length of CSON clock cycle is inserted.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CSON_A::OTHERS)
    }
}
///Field `CSON` writer - CS Assert Wait Select
pub type CSON_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CSON_A, crate::Safe>;
impl<'a, REG> CSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No wait is inserted.
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSON_A::_0X0)
    }
    ///Wait with a length of CSON clock cycle is inserted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CSON_A::OTHERS)
    }
}
impl R {
    ///Bits 0:2 - Read-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn csroff(&self) -> CSROFF_R {
        CSROFF_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Write-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn cswoff(&self) -> CSWOFF_R {
        CSWOFF_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Write Data Output Extension Cycle Select
    #[inline(always)]
    pub fn wdoff(&self) -> WDOFF_R {
        WDOFF_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:13 - Address Cycle Wait Select
    #[inline(always)]
    pub fn await_(&self) -> AWAIT_R {
        AWAIT_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - RD Assert Wait Select
    #[inline(always)]
    pub fn rdon(&self) -> RDON_R {
        RDON_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - WR Assert Wait Select
    #[inline(always)]
    pub fn wron(&self) -> WRON_R {
        WRON_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Write Data Output Wait Select
    #[inline(always)]
    pub fn wdon(&self) -> WDON_R {
        WDON_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - CS Assert Wait Select
    #[inline(always)]
    pub fn cson(&self) -> CSON_R {
        CSON_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Read-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn csroff(&mut self) -> CSROFF_W<CSWCR2_SPEC> {
        CSROFF_W::new(self, 0)
    }
    ///Bits 4:6 - Write-Access CS Extension Cycle Select
    #[inline(always)]
    pub fn cswoff(&mut self) -> CSWOFF_W<CSWCR2_SPEC> {
        CSWOFF_W::new(self, 4)
    }
    ///Bits 8:10 - Write Data Output Extension Cycle Select
    #[inline(always)]
    pub fn wdoff(&mut self) -> WDOFF_W<CSWCR2_SPEC> {
        WDOFF_W::new(self, 8)
    }
    ///Bits 12:13 - Address Cycle Wait Select
    #[inline(always)]
    pub fn await_(&mut self) -> AWAIT_W<CSWCR2_SPEC> {
        AWAIT_W::new(self, 12)
    }
    ///Bits 16:18 - RD Assert Wait Select
    #[inline(always)]
    pub fn rdon(&mut self) -> RDON_W<CSWCR2_SPEC> {
        RDON_W::new(self, 16)
    }
    ///Bits 20:22 - WR Assert Wait Select
    #[inline(always)]
    pub fn wron(&mut self) -> WRON_W<CSWCR2_SPEC> {
        WRON_W::new(self, 20)
    }
    ///Bits 24:26 - Write Data Output Wait Select
    #[inline(always)]
    pub fn wdon(&mut self) -> WDON_W<CSWCR2_SPEC> {
        WDON_W::new(self, 24)
    }
    ///Bits 28:30 - CS Assert Wait Select
    #[inline(always)]
    pub fn cson(&mut self) -> CSON_W<CSWCR2_SPEC> {
        CSON_W::new(self, 28)
    }
}
/**CS%s Wait Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cswcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSWCR2_SPEC;
impl crate::RegisterSpec for CSWCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cswcr2::R`](R) reader structure
impl crate::Readable for CSWCR2_SPEC {}
///`write(|w| ..)` method takes [`cswcr2::W`](W) writer structure
impl crate::Writable for CSWCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sWCR2 to value 0x07
impl crate::Resettable for CSWCR2_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
