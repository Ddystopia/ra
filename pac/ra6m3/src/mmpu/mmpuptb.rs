///Register `MMPUPTB` reader
pub type R = crate::R<MMPUPTB_SPEC>;
///Register `MMPUPTB` writer
pub type W = crate::W<MMPUPTB_SPEC>;
/**Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTECT_A {
    ///0: All Bus Master MPU Group B register writing is possible.
    _0 = 0,
    ///1: All Bus Master MPU Group B register writing is protected. Read is possible.
    _1 = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PROTECT` reader - Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)
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
    ///All Bus Master MPU Group B register writing is possible.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROTECT_A::_0
    }
    ///All Bus Master MPU Group B register writing is protected. Read is possible.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROTECT_A::_1
    }
}
///Field `PROTECT` writer - Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)
pub type PROTECT_W<'a, REG> = crate::BitWriter<'a, REG, PROTECT_A>;
impl<'a, REG> PROTECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All Bus Master MPU Group B register writing is possible.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PROTECT_A::_0)
    }
    ///All Bus Master MPU Group B register writing is protected. Read is possible.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PROTECT_A::_1)
    }
}
/**Write Keyword The data written to these bits are not stored.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    ///165: Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5.
    _0X_A5 = 165,
    ///0: Writing to the PROTECT bit is invalid.
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
    ///Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5.
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::_0X_A5)
    }
    ///Writing to the PROTECT bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)
    #[inline(always)]
    pub fn protect(&mut self) -> PROTECT_W<MMPUPTB_SPEC> {
        PROTECT_W::new(self, 0)
    }
    ///Bits 8:15 - Write Keyword The data written to these bits are not stored.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<MMPUPTB_SPEC> {
        KEY_W::new(self, 8)
    }
}
/**Group B Protection of Register

You can [`read`](crate::Reg::read) this register and get [`mmpuptb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuptb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUPTB_SPEC;
impl crate::RegisterSpec for MMPUPTB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mmpuptb::R`](R) reader structure
impl crate::Readable for MMPUPTB_SPEC {}
///`write(|w| ..)` method takes [`mmpuptb::W`](W) writer structure
impl crate::Writable for MMPUPTB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUPTB to value 0
impl crate::Resettable for MMPUPTB_SPEC {}
