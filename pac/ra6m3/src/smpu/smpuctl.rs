///Register `SMPUCTL` reader
pub type R = crate::R<SMPUCTL_SPEC>;
///Register `SMPUCTL` writer
pub type W = crate::W<SMPUCTL_SPEC>;
/**Master Group enable

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
///Field `OAD` reader - Master Group enable
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
///Field `OAD` writer - Master Group enable
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
/**Protection of register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTECT_A {
    ///0: All Bus Slave register writing is possible.
    _0 = 0,
    ///1: All Bus Slave register writing is protected. Read is possible.
    _1 = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PROTECT` reader - Protection of register
pub type PROTECT_R = crate::BitReader<PROTECT_A>;
impl PROTECT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PROTECT_A {
        match self.bits {
            false => PROTECT_A::_0,
            true => PROTECT_A::_1,
        }
    }
    ///All Bus Slave register writing is possible.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROTECT_A::_0
    }
    ///All Bus Slave register writing is protected. Read is possible.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROTECT_A::_1
    }
}
///Field `PROTECT` writer - Protection of register
pub type PROTECT_W<'a, REG> = crate::BitWriter<'a, REG, PROTECT_A>;
impl<'a, REG> PROTECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All Bus Slave register writing is possible.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PROTECT_A::_0)
    }
    ///All Bus Slave register writing is protected. Read is possible.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PROTECT_A::_1)
    }
}
/**Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    ///165: Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5.
    _0X_A5 = 165,
    ///0: Writing to the PROTECT and OAD bit is invalid.
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
///Field `KEY` writer - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_A, crate::Safe>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5.
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::_0X_A5)
    }
    ///Writing to the PROTECT and OAD bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Master Group enable
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Protection of register
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Master Group enable
    #[inline(always)]
    pub fn oad(&mut self) -> OAD_W<SMPUCTL_SPEC> {
        OAD_W::new(self, 0)
    }
    ///Bit 1 - Protection of register
    #[inline(always)]
    pub fn protect(&mut self) -> PROTECT_W<SMPUCTL_SPEC> {
        PROTECT_W::new(self, 1)
    }
    ///Bits 8:15 - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<SMPUCTL_SPEC> {
        KEY_W::new(self, 8)
    }
}
/**Slave MPU Control Register

You can [`read`](crate::Reg::read) this register and get [`smpuctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpuctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMPUCTL_SPEC;
impl crate::RegisterSpec for SMPUCTL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`smpuctl::R`](R) reader structure
impl crate::Readable for SMPUCTL_SPEC {}
///`write(|w| ..)` method takes [`smpuctl::W`](W) writer structure
impl crate::Writable for SMPUCTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPUCTL to value 0
impl crate::Resettable for SMPUCTL_SPEC {}
