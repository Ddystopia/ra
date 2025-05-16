///Register `PIPECFG` reader
pub type R = crate::R<PIPECFG_SPEC>;
///Register `PIPECFG` writer
pub type W = crate::W<PIPECFG_SPEC>;
/**Endpoint Number

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPNUM_A {
    ///0: the selected pipe is not used
    _000 = 0,
    ///1: specify the endpoint number for the selected pipe.
    OTHERS = 1,
}
impl From<EPNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: EPNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPNUM_A {
    type Ux = u8;
}
impl crate::IsEnum for EPNUM_A {}
///Field `EPNUM` reader - Endpoint Number
pub type EPNUM_R = crate::FieldReader<EPNUM_A>;
impl EPNUM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EPNUM_A {
        match self.bits {
            0 => EPNUM_A::_000,
            _ => EPNUM_A::OTHERS,
        }
    }
    ///the selected pipe is not used
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == EPNUM_A::_000
    }
    ///specify the endpoint number for the selected pipe.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), EPNUM_A::OTHERS)
    }
}
///Field `EPNUM` writer - Endpoint Number
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EPNUM_A, crate::Safe>;
impl<'a, REG> EPNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///the selected pipe is not used
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(EPNUM_A::_000)
    }
    ///specify the endpoint number for the selected pipe.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(EPNUM_A::OTHERS)
    }
}
/**Transfer Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    ///0: Receiving direction
    _0 = 0,
    ///1: Transmitting direction
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Transfer Direction
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    ///Receiving direction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIR_A::_0
    }
    ///Transmitting direction
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIR_A::_1
    }
}
///Field `DIR` writer - Transfer Direction
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR_A>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiving direction
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::_0)
    }
    ///Transmitting direction
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::_1)
    }
}
/**Pipe Disabled at End of Transfer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTNAK_A {
    ///0: The pipe is continued at the end of transfer.
    _0 = 0,
    ///1: The pipe is disabled at the end of transfer.
    _1 = 1,
}
impl From<SHTNAK_A> for bool {
    #[inline(always)]
    fn from(variant: SHTNAK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHTNAK` reader - Pipe Disabled at End of Transfer
pub type SHTNAK_R = crate::BitReader<SHTNAK_A>;
impl SHTNAK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHTNAK_A {
        match self.bits {
            false => SHTNAK_A::_0,
            true => SHTNAK_A::_1,
        }
    }
    ///The pipe is continued at the end of transfer.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTNAK_A::_0
    }
    ///The pipe is disabled at the end of transfer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTNAK_A::_1
    }
}
///Field `SHTNAK` writer - Pipe Disabled at End of Transfer
pub type SHTNAK_W<'a, REG> = crate::BitWriter<'a, REG, SHTNAK_A>;
impl<'a, REG> SHTNAK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The pipe is continued at the end of transfer.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SHTNAK_A::_0)
    }
    ///The pipe is disabled at the end of transfer.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SHTNAK_A::_1)
    }
}
/**Continuous Transfer Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMD_A {
    ///0: Discontinuous transfer mode
    _0 = 0,
    ///1: Continuous transfer mode
    _1 = 1,
}
impl From<CNTMD_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CNTMD` reader - Continuous Transfer Mode
pub type CNTMD_R = crate::BitReader<CNTMD_A>;
impl CNTMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNTMD_A {
        match self.bits {
            false => CNTMD_A::_0,
            true => CNTMD_A::_1,
        }
    }
    ///Discontinuous transfer mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMD_A::_0
    }
    ///Continuous transfer mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMD_A::_1
    }
}
///Field `CNTMD` writer - Continuous Transfer Mode
pub type CNTMD_W<'a, REG> = crate::BitWriter<'a, REG, CNTMD_A>;
impl<'a, REG> CNTMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous transfer mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CNTMD_A::_0)
    }
    ///Continuous transfer mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTMD_A::_1)
    }
}
/**Double Buffer Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLB_A {
    ///0: Single buffer
    _0 = 0,
    ///1: Double buffer
    _1 = 1,
}
impl From<DBLB_A> for bool {
    #[inline(always)]
    fn from(variant: DBLB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBLB` reader - Double Buffer Mode
pub type DBLB_R = crate::BitReader<DBLB_A>;
impl DBLB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBLB_A {
        match self.bits {
            false => DBLB_A::_0,
            true => DBLB_A::_1,
        }
    }
    ///Single buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBLB_A::_0
    }
    ///Double buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBLB_A::_1
    }
}
///Field `DBLB` writer - Double Buffer Mode
pub type DBLB_W<'a, REG> = crate::BitWriter<'a, REG, DBLB_A>;
impl<'a, REG> DBLB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBLB_A::_0)
    }
    ///Double buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBLB_A::_1)
    }
}
/**BRDY Interrupt Operation Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFRE_A {
    ///0: BRDY interrupt upon transmitting or receiving data
    _0 = 0,
    ///1: BRDY interrupt upon completion of reading data
    _1 = 1,
}
impl From<BFRE_A> for bool {
    #[inline(always)]
    fn from(variant: BFRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BFRE` reader - BRDY Interrupt Operation Specification
pub type BFRE_R = crate::BitReader<BFRE_A>;
impl BFRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BFRE_A {
        match self.bits {
            false => BFRE_A::_0,
            true => BFRE_A::_1,
        }
    }
    ///BRDY interrupt upon transmitting or receiving data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFRE_A::_0
    }
    ///BRDY interrupt upon completion of reading data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFRE_A::_1
    }
}
///Field `BFRE` writer - BRDY Interrupt Operation Specification
pub type BFRE_W<'a, REG> = crate::BitWriter<'a, REG, BFRE_A>;
impl<'a, REG> BFRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BRDY interrupt upon transmitting or receiving data
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BFRE_A::_0)
    }
    ///BRDY interrupt upon completion of reading data
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BFRE_A::_1)
    }
}
/**Transfer Type

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    ///0: Pipe not used
    _00 = 0,
    ///1: Bulk transfer
    _01 = 1,
    ///2: Interrupt transfer
    _10 = 2,
    ///3: Isochronous transfer
    _11 = 3,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for TYPE_A {}
///Field `TYPE` reader - Transfer Type
pub type TYPE_R = crate::FieldReader<TYPE_A>;
impl TYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TYPE_A {
        match self.bits {
            0 => TYPE_A::_00,
            1 => TYPE_A::_01,
            2 => TYPE_A::_10,
            3 => TYPE_A::_11,
            _ => unreachable!(),
        }
    }
    ///Pipe not used
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TYPE_A::_00
    }
    ///Bulk transfer
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TYPE_A::_01
    }
    ///Interrupt transfer
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TYPE_A::_10
    }
    ///Isochronous transfer
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TYPE_A::_11
    }
}
///Field `TYPE` writer - Transfer Type
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TYPE_A, crate::Safe>;
impl<'a, REG> TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Pipe not used
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_00)
    }
    ///Bulk transfer
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_01)
    }
    ///Interrupt transfer
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_10)
    }
    ///Isochronous transfer
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TYPE_A::_11)
    }
}
impl R {
    ///Bits 0:3 - Endpoint Number
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&self) -> SHTNAK_R {
        SHTNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Continuous Transfer Mode
    #[inline(always)]
    pub fn cntmd(&self) -> CNTMD_R {
        CNTMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Double Buffer Mode
    #[inline(always)]
    pub fn dblb(&self) -> DBLB_R {
        DBLB_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRDY Interrupt Operation Specification
    #[inline(always)]
    pub fn bfre(&self) -> BFRE_R {
        BFRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 14:15 - Transfer Type
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:3 - Endpoint Number
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W<PIPECFG_SPEC> {
        EPNUM_W::new(self, 0)
    }
    ///Bit 4 - Transfer Direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<PIPECFG_SPEC> {
        DIR_W::new(self, 4)
    }
    ///Bit 7 - Pipe Disabled at End of Transfer
    #[inline(always)]
    pub fn shtnak(&mut self) -> SHTNAK_W<PIPECFG_SPEC> {
        SHTNAK_W::new(self, 7)
    }
    ///Bit 8 - Continuous Transfer Mode
    #[inline(always)]
    pub fn cntmd(&mut self) -> CNTMD_W<PIPECFG_SPEC> {
        CNTMD_W::new(self, 8)
    }
    ///Bit 9 - Double Buffer Mode
    #[inline(always)]
    pub fn dblb(&mut self) -> DBLB_W<PIPECFG_SPEC> {
        DBLB_W::new(self, 9)
    }
    ///Bit 10 - BRDY Interrupt Operation Specification
    #[inline(always)]
    pub fn bfre(&mut self) -> BFRE_W<PIPECFG_SPEC> {
        BFRE_W::new(self, 10)
    }
    ///Bits 14:15 - Transfer Type
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<PIPECFG_SPEC> {
        TYPE_W::new(self, 14)
    }
}
/**Pipe Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pipecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPECFG_SPEC;
impl crate::RegisterSpec for PIPECFG_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipecfg::R`](R) reader structure
impl crate::Readable for PIPECFG_SPEC {}
///`write(|w| ..)` method takes [`pipecfg::W`](W) writer structure
impl crate::Writable for PIPECFG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPECFG to value 0
impl crate::Resettable for PIPECFG_SPEC {}
