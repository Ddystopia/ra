///Register `CS%sCR` reader
pub type R = crate::R<CSCR_SPEC>;
///Register `CS%sCR` writer
pub type W = crate::W<CSCR_SPEC>;
/**Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXENB_A {
    ///0: Disable operation
    _0 = 0,
    ///1: Enable operation
    _1 = 1,
}
impl From<EXENB_A> for bool {
    #[inline(always)]
    fn from(variant: EXENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXENB` reader - Operation Enable
pub type EXENB_R = crate::BitReader<EXENB_A>;
impl EXENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXENB_A {
        match self.bits {
            false => EXENB_A::_0,
            true => EXENB_A::_1,
        }
    }
    ///Disable operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXENB_A::_0
    }
    ///Enable operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXENB_A::_1
    }
}
///Field `EXENB` writer - Operation Enable
pub type EXENB_W<'a, REG> = crate::BitWriter<'a, REG, EXENB_A>;
impl<'a, REG> EXENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXENB_A::_0)
    }
    ///Enable operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXENB_A::_1)
    }
}
/**External Bus Width Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSIZE_A {
    ///0: A 16-bit bus space
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: An 8-bit bus space
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<BSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BSIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for BSIZE_A {}
///Field `BSIZE` reader - External Bus Width Select
pub type BSIZE_R = crate::FieldReader<BSIZE_A>;
impl BSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSIZE_A {
        match self.bits {
            0 => BSIZE_A::_00,
            1 => BSIZE_A::_01,
            2 => BSIZE_A::_10,
            3 => BSIZE_A::_11,
            _ => unreachable!(),
        }
    }
    ///A 16-bit bus space
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BSIZE_A::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BSIZE_A::_01
    }
    ///An 8-bit bus space
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BSIZE_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BSIZE_A::_11
    }
}
///Field `BSIZE` writer - External Bus Width Select
pub type BSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BSIZE_A, crate::Safe>;
impl<'a, REG> BSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A 16-bit bus space
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BSIZE_A::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BSIZE_A::_01)
    }
    ///An 8-bit bus space
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BSIZE_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BSIZE_A::_11)
    }
}
/**Endian Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMODE_A {
    ///0: Little Endian
    _0 = 0,
    ///1: Big Endian
    _1 = 1,
}
impl From<EMODE_A> for bool {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EMODE` reader - Endian Mode
pub type EMODE_R = crate::BitReader<EMODE_A>;
impl EMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EMODE_A {
        match self.bits {
            false => EMODE_A::_0,
            true => EMODE_A::_1,
        }
    }
    ///Little Endian
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMODE_A::_0
    }
    ///Big Endian
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMODE_A::_1
    }
}
///Field `EMODE` writer - Endian Mode
pub type EMODE_W<'a, REG> = crate::BitWriter<'a, REG, EMODE_A>;
impl<'a, REG> EMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little Endian
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EMODE_A::_0)
    }
    ///Big Endian
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EMODE_A::_1)
    }
}
/**Address/Data Multiplexed I/O Interface Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPXEN_A {
    ///0: Separate bus interface is selected for area n
    _0 = 0,
    ///1: Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)
    _1 = 1,
}
impl From<MPXEN_A> for bool {
    #[inline(always)]
    fn from(variant: MPXEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPXEN` reader - Address/Data Multiplexed I/O Interface Select
pub type MPXEN_R = crate::BitReader<MPXEN_A>;
impl MPXEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPXEN_A {
        match self.bits {
            false => MPXEN_A::_0,
            true => MPXEN_A::_1,
        }
    }
    ///Separate bus interface is selected for area n
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPXEN_A::_0
    }
    ///Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPXEN_A::_1
    }
}
///Field `MPXEN` writer - Address/Data Multiplexed I/O Interface Select
pub type MPXEN_W<'a, REG> = crate::BitWriter<'a, REG, MPXEN_A>;
impl<'a, REG> MPXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Separate bus interface is selected for area n
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPXEN_A::_0)
    }
    ///Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPXEN_A::_1)
    }
}
impl R {
    ///Bit 0 - Operation Enable
    #[inline(always)]
    pub fn exenb(&self) -> EXENB_R {
        EXENB_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - External Bus Width Select
    #[inline(always)]
    pub fn bsize(&self) -> BSIZE_R {
        BSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - Endian Mode
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Address/Data Multiplexed I/O Interface Select
    #[inline(always)]
    pub fn mpxen(&self) -> MPXEN_R {
        MPXEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operation Enable
    #[inline(always)]
    pub fn exenb(&mut self) -> EXENB_W<CSCR_SPEC> {
        EXENB_W::new(self, 0)
    }
    ///Bits 4:5 - External Bus Width Select
    #[inline(always)]
    pub fn bsize(&mut self) -> BSIZE_W<CSCR_SPEC> {
        BSIZE_W::new(self, 4)
    }
    ///Bit 8 - Endian Mode
    #[inline(always)]
    pub fn emode(&mut self) -> EMODE_W<CSCR_SPEC> {
        EMODE_W::new(self, 8)
    }
    ///Bit 12 - Address/Data Multiplexed I/O Interface Select
    #[inline(always)]
    pub fn mpxen(&mut self) -> MPXEN_W<CSCR_SPEC> {
        MPXEN_W::new(self, 12)
    }
}
/**CS%s Control Register

You can [`read`](crate::Reg::read) this register and get [`cscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSCR_SPEC;
impl crate::RegisterSpec for CSCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`cscr::R`](R) reader structure
impl crate::Readable for CSCR_SPEC {}
///`write(|w| ..)` method takes [`cscr::W`](W) writer structure
impl crate::Writable for CSCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sCR to value 0
impl crate::Resettable for CSCR_SPEC {}
