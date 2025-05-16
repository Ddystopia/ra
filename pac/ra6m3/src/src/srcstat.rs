///Register `SRCSTAT` reader
pub type R = crate::R<SRCSTAT_SPEC>;
///Register `SRCSTAT` writer
pub type W = crate::W<SRCSTAT_SPEC>;
/**Output Data FIFO Full Interrupt Request Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OINT_A {
    ///0: Number of data units in the output FIFO has not become equal to or greater than the specified triggering number.
    _0 = 0,
    ///1: Number of data units in the output FIFO has become equal to or greater than the specified triggering number.
    _1 = 1,
}
impl From<OINT_A> for bool {
    #[inline(always)]
    fn from(variant: OINT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OINT` reader - Output Data FIFO Full Interrupt Request Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OINT_R = crate::BitReader<OINT_A>;
impl OINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OINT_A {
        match self.bits {
            false => OINT_A::_0,
            true => OINT_A::_1,
        }
    }
    ///Number of data units in the output FIFO has not become equal to or greater than the specified triggering number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OINT_A::_0
    }
    ///Number of data units in the output FIFO has become equal to or greater than the specified triggering number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OINT_A::_1
    }
}
///Field `OINT` writer - Output Data FIFO Full Interrupt Request Flag
pub type OINT_W<'a, REG> = crate::BitWriter0C<'a, REG, OINT_A>;
impl<'a, REG> OINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data units in the output FIFO has not become equal to or greater than the specified triggering number.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OINT_A::_0)
    }
    ///Number of data units in the output FIFO has become equal to or greater than the specified triggering number.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OINT_A::_1)
    }
}
/**Input Data FIFO Empty Interrupt Request Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IINT_A {
    ///0: Number of data units in the input FIFO has not become equal to or smaller than the specified triggering number.
    _0 = 0,
    ///1: Number of data units in the input FIFO has become equal to or smaller than the specified triggering number.
    _1 = 1,
}
impl From<IINT_A> for bool {
    #[inline(always)]
    fn from(variant: IINT_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `IINT` reader - Input Data FIFO Empty Interrupt Request Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type IINT_R = crate::BitReader<IINT_A>;
impl IINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IINT_A {
        match self.bits {
            false => IINT_A::_0,
            true => IINT_A::_1,
        }
    }
    ///Number of data units in the input FIFO has not become equal to or smaller than the specified triggering number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IINT_A::_0
    }
    ///Number of data units in the input FIFO has become equal to or smaller than the specified triggering number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IINT_A::_1
    }
}
///Field `IINT` writer - Input Data FIFO Empty Interrupt Request Flag
pub type IINT_W<'a, REG> = crate::BitWriter0C<'a, REG, IINT_A>;
impl<'a, REG> IINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data units in the input FIFO has not become equal to or smaller than the specified triggering number.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IINT_A::_0)
    }
    ///Number of data units in the input FIFO has become equal to or smaller than the specified triggering number.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IINT_A::_1)
    }
}
/**Output Data FIFO Overwrite Interrupt Request Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVF_A {
    ///0: Next data conversion processing is not completed.
    _0 = 0,
    ///1: Next data conversion processing is completed.
    _1 = 1,
}
impl From<OVF_A> for bool {
    #[inline(always)]
    fn from(variant: OVF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OVF` reader - Output Data FIFO Overwrite Interrupt Request Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OVF_R = crate::BitReader<OVF_A>;
impl OVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVF_A {
        match self.bits {
            false => OVF_A::_0,
            true => OVF_A::_1,
        }
    }
    ///Next data conversion processing is not completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVF_A::_0
    }
    ///Next data conversion processing is completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVF_A::_1
    }
}
///Field `OVF` writer - Output Data FIFO Overwrite Interrupt Request Flag
pub type OVF_W<'a, REG> = crate::BitWriter0C<'a, REG, OVF_A>;
impl<'a, REG> OVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Next data conversion processing is not completed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVF_A::_0)
    }
    ///Next data conversion processing is completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVF_A::_1)
    }
}
/**Output FIFO Underflow Interrupt Request Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDF_A {
    ///0: Output data FIFO has not been read out.
    _0 = 0,
    ///1: Output data FIFO has been read out.
    _1 = 1,
}
impl From<UDF_A> for bool {
    #[inline(always)]
    fn from(variant: UDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `UDF` reader - Output FIFO Underflow Interrupt Request Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type UDF_R = crate::BitReader<UDF_A>;
impl UDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDF_A {
        match self.bits {
            false => UDF_A::_0,
            true => UDF_A::_1,
        }
    }
    ///Output data FIFO has not been read out.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDF_A::_0
    }
    ///Output data FIFO has been read out.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDF_A::_1
    }
}
///Field `UDF` writer - Output FIFO Underflow Interrupt Request Flag
pub type UDF_W<'a, REG> = crate::BitWriter0C<'a, REG, UDF_A>;
impl<'a, REG> UDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output data FIFO has not been read out.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UDF_A::_0)
    }
    ///Output data FIFO has been read out.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UDF_A::_1)
    }
}
/**Flush Processing Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLF_A {
    ///0: Flash processing is completed.
    _0 = 0,
    ///1: Flash processing is in progress.
    _1 = 1,
}
impl From<FLF_A> for bool {
    #[inline(always)]
    fn from(variant: FLF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLF` reader - Flush Processing Status Flag
pub type FLF_R = crate::BitReader<FLF_A>;
impl FLF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLF_A {
        match self.bits {
            false => FLF_A::_0,
            true => FLF_A::_1,
        }
    }
    ///Flash processing is completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLF_A::_0
    }
    ///Flash processing is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLF_A::_1
    }
}
/**Conversion End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEF_A {
    ///0: All of the output data has not been read out.
    _0 = 0,
    ///1: All of the output data has been read out.
    _1 = 1,
}
impl From<CEF_A> for bool {
    #[inline(always)]
    fn from(variant: CEF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CEF` reader - Conversion End Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CEF_R = crate::BitReader<CEF_A>;
impl CEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEF_A {
        match self.bits {
            false => CEF_A::_0,
            true => CEF_A::_1,
        }
    }
    ///All of the output data has not been read out.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEF_A::_0
    }
    ///All of the output data has been read out.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEF_A::_1
    }
}
///Field `CEF` writer - Conversion End Flag
pub type CEF_W<'a, REG> = crate::BitWriter0C<'a, REG, CEF_A>;
impl<'a, REG> CEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All of the output data has not been read out.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CEF_A::_0)
    }
    ///All of the output data has been read out.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CEF_A::_1)
    }
}
/**Input FIFO Data Count

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFDN_A {
    ///0: The value of IFDN indicatethe number of data units in the input FIFO.
    IFDN = 0,
}
impl From<IFDN_A> for u8 {
    #[inline(always)]
    fn from(variant: IFDN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFDN_A {
    type Ux = u8;
}
impl crate::IsEnum for IFDN_A {}
///Field `IFDN` reader - Input FIFO Data Count
pub type IFDN_R = crate::FieldReader<IFDN_A>;
impl IFDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IFDN_A {
        match self.bits {
            _ => IFDN_A::IFDN,
        }
    }
    ///The value of IFDN indicatethe number of data units in the input FIFO.
    #[inline(always)]
    pub fn is_ifdn(&self) -> bool {
        matches!(self.variant(), IFDN_A::IFDN)
    }
}
/**Output FIFO Data Count

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OFDN_A {
    ///0: The value of OFDN indicatethe number of data units in the output FIFO.
    OFDN = 0,
}
impl From<OFDN_A> for u8 {
    #[inline(always)]
    fn from(variant: OFDN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OFDN_A {
    type Ux = u8;
}
impl crate::IsEnum for OFDN_A {}
///Field `OFDN` reader - Output FIFO Data Count
pub type OFDN_R = crate::FieldReader<OFDN_A>;
impl OFDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFDN_A {
        match self.bits {
            _ => OFDN_A::OFDN,
        }
    }
    ///The value of OFDN indicatethe number of data units in the output FIFO.
    #[inline(always)]
    pub fn is_ofdn(&self) -> bool {
        matches!(self.variant(), OFDN_A::OFDN)
    }
}
impl R {
    ///Bit 0 - Output Data FIFO Full Interrupt Request Flag
    #[inline(always)]
    pub fn oint(&self) -> OINT_R {
        OINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input Data FIFO Empty Interrupt Request Flag
    #[inline(always)]
    pub fn iint(&self) -> IINT_R {
        IINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output Data FIFO Overwrite Interrupt Request Flag
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output FIFO Underflow Interrupt Request Flag
    #[inline(always)]
    pub fn udf(&self) -> UDF_R {
        UDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flush Processing Status Flag
    #[inline(always)]
    pub fn flf(&self) -> FLF_R {
        FLF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Conversion End Flag
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 7:10 - Input FIFO Data Count
    #[inline(always)]
    pub fn ifdn(&self) -> IFDN_R {
        IFDN_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bits 11:15 - Output FIFO Data Count
    #[inline(always)]
    pub fn ofdn(&self) -> OFDN_R {
        OFDN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - Output Data FIFO Full Interrupt Request Flag
    #[inline(always)]
    pub fn oint(&mut self) -> OINT_W<SRCSTAT_SPEC> {
        OINT_W::new(self, 0)
    }
    ///Bit 1 - Input Data FIFO Empty Interrupt Request Flag
    #[inline(always)]
    pub fn iint(&mut self) -> IINT_W<SRCSTAT_SPEC> {
        IINT_W::new(self, 1)
    }
    ///Bit 2 - Output Data FIFO Overwrite Interrupt Request Flag
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W<SRCSTAT_SPEC> {
        OVF_W::new(self, 2)
    }
    ///Bit 3 - Output FIFO Underflow Interrupt Request Flag
    #[inline(always)]
    pub fn udf(&mut self) -> UDF_W<SRCSTAT_SPEC> {
        UDF_W::new(self, 3)
    }
    ///Bit 5 - Conversion End Flag
    #[inline(always)]
    pub fn cef(&mut self) -> CEF_W<SRCSTAT_SPEC> {
        CEF_W::new(self, 5)
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`srcstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCSTAT_SPEC;
impl crate::RegisterSpec for SRCSTAT_SPEC {
    type Ux = u16;
}
///`read()` method returns [`srcstat::R`](R) reader structure
impl crate::Readable for SRCSTAT_SPEC {}
///`write(|w| ..)` method takes [`srcstat::W`](W) writer structure
impl crate::Writable for SRCSTAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x2f;
}
///`reset()` method sets SRCSTAT to value 0x02
impl crate::Resettable for SRCSTAT_SPEC {
    const RESET_VALUE: u16 = 0x02;
}
