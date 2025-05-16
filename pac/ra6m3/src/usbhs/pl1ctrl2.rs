///Register `PL1CTRL2` reader
pub type R = crate::R<PL1CTRL2_SPEC>;
///Register `PL1CTRL2` writer
pub type W = crate::W<PL1CTRL2_SPEC>;
/**HIRD Value Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIRDMON_A {
    ///0: The HIRD field value of the LPM token received last is reflected.
    _0 = 0,
    ///1: The HIRD field value of the LPM token received last is reflected.
    _1 = 1,
}
impl From<HIRDMON_A> for u8 {
    #[inline(always)]
    fn from(variant: HIRDMON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIRDMON_A {
    type Ux = u8;
}
impl crate::IsEnum for HIRDMON_A {}
///Field `HIRDMON` reader - HIRD Value Monitor
pub type HIRDMON_R = crate::FieldReader<HIRDMON_A>;
impl HIRDMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<HIRDMON_A> {
        match self.bits {
            0 => Some(HIRDMON_A::_0),
            1 => Some(HIRDMON_A::_1),
            _ => None,
        }
    }
    ///The HIRD field value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HIRDMON_A::_0
    }
    ///The HIRD field value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HIRDMON_A::_1
    }
}
///Field `HIRDMON` writer - HIRD Value Monitor
pub type HIRDMON_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HIRDMON_A>;
impl<'a, REG> HIRDMON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The HIRD field value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HIRDMON_A::_0)
    }
    ///The HIRD field value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HIRDMON_A::_1)
    }
}
/**RWE Value Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWEMON_A {
    ///0: The RWE bit value of the LPM token received last is reflected.
    _0 = 0,
    ///1: The RWE bit value of the LPM token received last is reflected.
    _1 = 1,
}
impl From<RWEMON_A> for bool {
    #[inline(always)]
    fn from(variant: RWEMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RWEMON` reader - RWE Value Monitor
pub type RWEMON_R = crate::BitReader<RWEMON_A>;
impl RWEMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWEMON_A {
        match self.bits {
            false => RWEMON_A::_0,
            true => RWEMON_A::_1,
        }
    }
    ///The RWE bit value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWEMON_A::_0
    }
    ///The RWE bit value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWEMON_A::_1
    }
}
///Field `RWEMON` writer - RWE Value Monitor
pub type RWEMON_W<'a, REG> = crate::BitWriter<'a, REG, RWEMON_A>;
impl<'a, REG> RWEMON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RWE bit value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RWEMON_A::_0)
    }
    ///The RWE bit value of the LPM token received last is reflected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RWEMON_A::_1)
    }
}
impl R {
    ///Bits 8:11 - HIRD Value Monitor
    #[inline(always)]
    pub fn hirdmon(&self) -> HIRDMON_R {
        HIRDMON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - RWE Value Monitor
    #[inline(always)]
    pub fn rwemon(&self) -> RWEMON_R {
        RWEMON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bits 8:11 - HIRD Value Monitor
    #[inline(always)]
    pub fn hirdmon(&mut self) -> HIRDMON_W<PL1CTRL2_SPEC> {
        HIRDMON_W::new(self, 8)
    }
    ///Bit 12 - RWE Value Monitor
    #[inline(always)]
    pub fn rwemon(&mut self) -> RWEMON_W<PL1CTRL2_SPEC> {
        RWEMON_W::new(self, 12)
    }
}
/**Function L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PL1CTRL2_SPEC;
impl crate::RegisterSpec for PL1CTRL2_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pl1ctrl2::R`](R) reader structure
impl crate::Readable for PL1CTRL2_SPEC {}
///`write(|w| ..)` method takes [`pl1ctrl2::W`](W) writer structure
impl crate::Writable for PL1CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PL1CTRL2 to value 0
impl crate::Resettable for PL1CTRL2_SPEC {}
