///Register `SMR` reader
pub type R = crate::R<SMR_SPEC>;
///Register `SMR` writer
pub type W = crate::W<SMR_SPEC>;
/**Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    ///0: PCLK clock
    _00 = 0,
    ///1: PCLK/4 clock
    _01 = 1,
    ///2: PCLK/16 clock
    _10 = 2,
    ///3: PCLK/64 clock
    _11 = 3,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKS_A {
    type Ux = u8;
}
impl crate::IsEnum for CKS_A {}
///Field `CKS` reader - Clock Select
pub type CKS_R = crate::FieldReader<CKS_A>;
impl CKS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_00,
            1 => CKS_A::_01,
            2 => CKS_A::_10,
            3 => CKS_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK clock
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKS_A::_00
    }
    ///PCLK/4 clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKS_A::_01
    }
    ///PCLK/16 clock
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKS_A::_10
    }
    ///PCLK/64 clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKS_A::_11
    }
}
///Field `CKS` writer - Clock Select
pub type CKS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKS_A, crate::Safe>;
impl<'a, REG> CKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_00)
    }
    ///PCLK/4 clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_01)
    }
    ///PCLK/16 clock
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_10)
    }
    ///PCLK/64 clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_11)
    }
}
/**Multi-Processor Mode(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MP_A {
    ///0: Multi-processor communications function is disabled
    _0 = 0,
    ///1: Multi-processor communications function is enabled
    _1 = 1,
}
impl From<MP_A> for bool {
    #[inline(always)]
    fn from(variant: MP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MP` reader - Multi-Processor Mode(Valid only in asynchronous mode)
pub type MP_R = crate::BitReader<MP_A>;
impl MP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MP_A {
        match self.bits {
            false => MP_A::_0,
            true => MP_A::_1,
        }
    }
    ///Multi-processor communications function is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MP_A::_0
    }
    ///Multi-processor communications function is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MP_A::_1
    }
}
///Field `MP` writer - Multi-Processor Mode(Valid only in asynchronous mode)
pub type MP_W<'a, REG> = crate::BitWriter<'a, REG, MP_A>;
impl<'a, REG> MP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Multi-processor communications function is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MP_A::_0)
    }
    ///Multi-processor communications function is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MP_A::_1)
    }
}
/**Stop Bit Length(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    ///0: 1 stop bit
    _0 = 0,
    ///1: 2 stop bits
    _1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` reader - Stop Bit Length(Valid only in asynchronous mode)
pub type STOP_R = crate::BitReader<STOP_A>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_0,
            true => STOP_A::_1,
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOP_A::_0
    }
    ///2 stop bits
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOP_A::_1
    }
}
///Field `STOP` writer - Stop Bit Length(Valid only in asynchronous mode)
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP_A>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///1 stop bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_0)
    }
    ///2 stop bits
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_1)
    }
}
/**Parity Mode (Valid only when the PE bit is 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    ///0: Selects even parity
    _0 = 0,
    ///1: Selects odd parity
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - Parity Mode (Valid only when the PE bit is 1)
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    ///Selects even parity
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    ///Selects odd parity
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
///Field `PM` writer - Parity Mode (Valid only when the PE bit is 1)
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM_A>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selects even parity
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_0)
    }
    ///Selects odd parity
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_1)
    }
}
/**Parity Enable(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    ///0: Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )
    _0 = 0,
    ///1: The parity bit is added (transmitting) / The parity bit is checked (receiving)
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Parity Enable(Valid only in asynchronous mode)
pub type PE_R = crate::BitReader<PE_A>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    ///Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    ///The parity bit is added (transmitting) / The parity bit is checked (receiving)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
///Field `PE` writer - Parity Enable(Valid only in asynchronous mode)
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE_A>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_0)
    }
    ///The parity bit is added (transmitting) / The parity bit is checked (receiving)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_1)
    }
}
/**Character Length(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHR_A {
    ///0: Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)
    _0 = 0,
    ///1: Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)
    _1 = 1,
}
impl From<CHR_A> for bool {
    #[inline(always)]
    fn from(variant: CHR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHR` reader - Character Length(Valid only in asynchronous mode)
pub type CHR_R = crate::BitReader<CHR_A>;
impl CHR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHR_A {
        match self.bits {
            false => CHR_A::_0,
            true => CHR_A::_1,
        }
    }
    ///Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHR_A::_0
    }
    ///Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHR_A::_1
    }
}
///Field `CHR` writer - Character Length(Valid only in asynchronous mode)
pub type CHR_W<'a, REG> = crate::BitWriter<'a, REG, CHR_A>;
impl<'a, REG> CHR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 8bit data length(SCMR.CHR1=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CHR_A::_0)
    }
    ///Transmit/receive in 9-bit data length(SCMR.CHR1=0) / in 7bit data length(SCMR.CHR1=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CHR_A::_1)
    }
}
/**Communications Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CM_A {
    ///0: Asynchronous mode or simple I2C mode
    _0 = 0,
    ///1: Clock synchronous mode
    _1 = 1,
}
impl From<CM_A> for bool {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CM` reader - Communications Mode
pub type CM_R = crate::BitReader<CM_A>;
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CM_A {
        match self.bits {
            false => CM_A::_0,
            true => CM_A::_1,
        }
    }
    ///Asynchronous mode or simple I2C mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CM_A::_0
    }
    ///Clock synchronous mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CM_A::_1
    }
}
///Field `CM` writer - Communications Mode
pub type CM_W<'a, REG> = crate::BitWriter<'a, REG, CM_A>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Asynchronous mode or simple I2C mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::_0)
    }
    ///Clock synchronous mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(self.bits & 3)
    }
    ///Bit 2 - Multi-Processor Mode(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn mp(&self) -> MP_R {
        MP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Bit Length(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Parity Mode (Valid only when the PE bit is 1)
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Parity Enable(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Character Length(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn chr(&self) -> CHR_R {
        CHR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Communications Mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<SMR_SPEC> {
        CKS_W::new(self, 0)
    }
    ///Bit 2 - Multi-Processor Mode(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn mp(&mut self) -> MP_W<SMR_SPEC> {
        MP_W::new(self, 2)
    }
    ///Bit 3 - Stop Bit Length(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<SMR_SPEC> {
        STOP_W::new(self, 3)
    }
    ///Bit 4 - Parity Mode (Valid only when the PE bit is 1)
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<SMR_SPEC> {
        PM_W::new(self, 4)
    }
    ///Bit 5 - Parity Enable(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<SMR_SPEC> {
        PE_W::new(self, 5)
    }
    ///Bit 6 - Character Length(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn chr(&mut self) -> CHR_W<SMR_SPEC> {
        CHR_W::new(self, 6)
    }
    ///Bit 7 - Communications Mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<SMR_SPEC> {
        CM_W::new(self, 7)
    }
}
/**Serial Mode Register (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMR_SPEC;
impl crate::RegisterSpec for SMR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`smr::R`](R) reader structure
impl crate::Readable for SMR_SPEC {}
///`write(|w| ..)` method takes [`smr::W`](W) writer structure
impl crate::Writable for SMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMR to value 0
impl crate::Resettable for SMR_SPEC {}
