///Register `MMPUCTLA` reader
pub type R = crate::R<MMPUCTLA_SPEC>;
///Register `MMPUCTLA` writer
pub type W = crate::W<MMPUCTLA_SPEC>;
/**Master Group enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    ///0: Master Group A disabled
    _0 = 0,
    ///1: Master Group A enabled.
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
    ///Master Group A disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    ///Master Group A enabled.
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
    ///Master Group A disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::_0)
    }
    ///Master Group A enabled.
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
///Field `KEY` writer - Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn enable(&mut self) -> ENABLE_W<MMPUCTLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - Operation after detection
    #[inline(always)]
    pub fn oad(&mut self) -> OAD_W<MMPUCTLA_SPEC> {
        OAD_W::new(self, 1)
    }
    ///Bits 8:15 - Key Code These bits are used to enable or disable writing of the OAD and ENABLE bit.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<MMPUCTLA_SPEC> {
        KEY_W::new(self, 8)
    }
}
/**Bus Master MPU Control Register A

You can [`read`](crate::Reg::read) this register and get [`mmpuctla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuctla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMPUCTLA_SPEC;
impl crate::RegisterSpec for MMPUCTLA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mmpuctla::R`](R) reader structure
impl crate::Readable for MMPUCTLA_SPEC {}
///`write(|w| ..)` method takes [`mmpuctla::W`](W) writer structure
impl crate::Writable for MMPUCTLA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUCTLA to value 0
impl crate::Resettable for MMPUCTLA_SPEC {}
