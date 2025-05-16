///Register `SFMSDC` reader
pub type R = crate::R<SFMSDC_SPEC>;
///Register `SFMSDC` writer
pub type W = crate::W<SFMSDC_SPEC>;
/**Selection of the number of dummy cycles of Fast Read instructions

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMDN_A {
    ///0: Default dummy cycles of each instruction.
    _0000 = 0,
    ///1: ( SFMDN + 2 ) x SCK
    OTHERS = 1,
}
impl From<SFMDN_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMDN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMDN_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMDN_A {}
///Field `SFMDN` reader - Selection of the number of dummy cycles of Fast Read instructions
pub type SFMDN_R = crate::FieldReader<SFMDN_A>;
impl SFMDN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMDN_A {
        match self.bits {
            0 => SFMDN_A::_0000,
            _ => SFMDN_A::OTHERS,
        }
    }
    ///Default dummy cycles of each instruction.
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SFMDN_A::_0000
    }
    ///( SFMDN + 2 ) x SCK
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), SFMDN_A::OTHERS)
    }
}
///Field `SFMDN` writer - Selection of the number of dummy cycles of Fast Read instructions
pub type SFMDN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SFMDN_A, crate::Safe>;
impl<'a, REG> SFMDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Default dummy cycles of each instruction.
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDN_A::_0000)
    }
    ///( SFMDN + 2 ) x SCK
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(SFMDN_A::OTHERS)
    }
}
/**XIP mode status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMXST_A {
    ///0: Normal (non-XIP) mode is operating
    _0 = 0,
    ///1: XIP mode is operating
    _1 = 1,
}
impl From<SFMXST_A> for bool {
    #[inline(always)]
    fn from(variant: SFMXST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMXST` reader - XIP mode status
pub type SFMXST_R = crate::BitReader<SFMXST_A>;
impl SFMXST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMXST_A {
        match self.bits {
            false => SFMXST_A::_0,
            true => SFMXST_A::_1,
        }
    }
    ///Normal (non-XIP) mode is operating
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXST_A::_0
    }
    ///XIP mode is operating
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXST_A::_1
    }
}
/**XIP mode permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMXEN_A {
    ///0: XIP mode is prohibited
    _0 = 0,
    ///1: XIP mode is permitted
    _1 = 1,
}
impl From<SFMXEN_A> for bool {
    #[inline(always)]
    fn from(variant: SFMXEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMXEN` reader - XIP mode permission
pub type SFMXEN_R = crate::BitReader<SFMXEN_A>;
impl SFMXEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMXEN_A {
        match self.bits {
            false => SFMXEN_A::_0,
            true => SFMXEN_A::_1,
        }
    }
    ///XIP mode is prohibited
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXEN_A::_0
    }
    ///XIP mode is permitted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXEN_A::_1
    }
}
///Field `SFMXEN` writer - XIP mode permission
pub type SFMXEN_W<'a, REG> = crate::BitWriter<'a, REG, SFMXEN_A>;
impl<'a, REG> SFMXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///XIP mode is prohibited
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMXEN_A::_0)
    }
    ///XIP mode is permitted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMXEN_A::_1)
    }
}
/**Mode data for serial ROM. (Control XIP mode)

Value on reset: 255*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMXD_A {
    ///0: XIP mode is prohibited
    _0 = 0,
    ///1: XIP mode is permitted
    _1 = 1,
}
impl From<SFMXD_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMXD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMXD_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMXD_A {}
///Field `SFMXD` reader - Mode data for serial ROM. (Control XIP mode)
pub type SFMXD_R = crate::FieldReader<SFMXD_A>;
impl SFMXD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SFMXD_A> {
        match self.bits {
            0 => Some(SFMXD_A::_0),
            1 => Some(SFMXD_A::_1),
            _ => None,
        }
    }
    ///XIP mode is prohibited
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXD_A::_0
    }
    ///XIP mode is permitted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXD_A::_1
    }
}
///Field `SFMXD` writer - Mode data for serial ROM. (Control XIP mode)
pub type SFMXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SFMXD_A>;
impl<'a, REG> SFMXD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///XIP mode is prohibited
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMXD_A::_0)
    }
    ///XIP mode is permitted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMXD_A::_1)
    }
}
impl R {
    ///Bits 0:3 - Selection of the number of dummy cycles of Fast Read instructions
    #[inline(always)]
    pub fn sfmdn(&self) -> SFMDN_R {
        SFMDN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 6 - XIP mode status
    #[inline(always)]
    pub fn sfmxst(&self) -> SFMXST_R {
        SFMXST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - XIP mode permission
    #[inline(always)]
    pub fn sfmxen(&self) -> SFMXEN_R {
        SFMXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Mode data for serial ROM. (Control XIP mode)
    #[inline(always)]
    pub fn sfmxd(&self) -> SFMXD_R {
        SFMXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:3 - Selection of the number of dummy cycles of Fast Read instructions
    #[inline(always)]
    pub fn sfmdn(&mut self) -> SFMDN_W<SFMSDC_SPEC> {
        SFMDN_W::new(self, 0)
    }
    ///Bit 7 - XIP mode permission
    #[inline(always)]
    pub fn sfmxen(&mut self) -> SFMXEN_W<SFMSDC_SPEC> {
        SFMXEN_W::new(self, 7)
    }
    ///Bits 8:15 - Mode data for serial ROM. (Control XIP mode)
    #[inline(always)]
    pub fn sfmxd(&mut self) -> SFMXD_W<SFMSDC_SPEC> {
        SFMXD_W::new(self, 8)
    }
}
/**Dummy Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSDC_SPEC;
impl crate::RegisterSpec for SFMSDC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmsdc::R`](R) reader structure
impl crate::Readable for SFMSDC_SPEC {}
///`write(|w| ..)` method takes [`sfmsdc::W`](W) writer structure
impl crate::Writable for SFMSDC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSDC to value 0xff00
impl crate::Resettable for SFMSDC_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
