///Register `MMPUCTL%s` reader
pub type R = crate::R<MMPUCTL_SPEC>;
///Register `MMPUCTL%s` writer
pub type W = crate::W<MMPUCTL_SPEC>;
/**Master Group enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    ///0: Master Group is disabled. Permission of all regions.
    _0 = 0,
    ///1: Master Group is enabled. Protection of all regions.
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - Master Group enable
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    ///Master Group is disabled. Permission of all regions.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    ///Master Group is enabled. Protection of all regions.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
///Field `ENABLE` writer - Master Group enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master Group is disabled. Permission of all regions.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    ///Master Group is enabled. Protection of all regions.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_1)
    }
}
/**Operation after detection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    ///0: Non-maskable interrupt.
    _0 = 0,
    ///1: Internal reset.
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OAD` reader - Operation after detection
pub type OAD_R = crate::BitReader<OAD_A>;
impl OAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    ///Non-maskable interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    ///Internal reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
///Field `OAD` writer - Operation after detection
pub type OAD_W<'a, REG> = crate::BitWriter<'a, REG, OAD_A>;
impl<'a, REG> OAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-maskable interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_0)
    }
    ///Internal reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_1)
    }
}
/**Write Keyword The data written to these bits are not stored.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    ///165: Writing to the OAD and ENABLE bit is valid, when the KEY bits are written 0xA5.
    _0X_A5 = 165,
    ///0: Writing to the OAD and ENABLE bit is invalid.
    OTHERS = 0,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_A {
    type Ux = u8;
}
impl crate::IsEnum for KEY_A {}
///Field `KEY` writer - Write Keyword The data written to these bits are not stored.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_A, crate::Safe>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Writing to the OAD and ENABLE bit is valid, when the KEY bits are written 0xA5.
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::_0X_A5)
    }
    ///Writing to the OAD and ENABLE bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Master Group enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation after detection
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Master Group enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<MMPUCTL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - Operation after detection
    #[inline(always)]
    pub fn oad(&mut self) -> OAD_W<MMPUCTL_SPEC> {
        OAD_W::new(self, 1)
    }
    ///Bits 8:15 - Write Keyword The data written to these bits are not stored.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<MMPUCTL_SPEC> {
        KEY_W::new(self, 8)
    }
}
/**Bus Master MPU Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUCTL_SPEC;
impl crate::RegisterSpec for MMPUCTL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mmpuctl::R`](R) reader structure
impl crate::Readable for MMPUCTL_SPEC {}
///`write(|w| ..)` method takes [`mmpuctl::W`](W) writer structure
impl crate::Writable for MMPUCTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUCTL%s to value 0
impl crate::Resettable for MMPUCTL_SPEC {}
