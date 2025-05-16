///Register `PRCR` reader
pub type R = crate::R<PRCR_SPEC>;
///Register `PRCR` writer
pub type W = crate::W<PRCR_SPEC>;
/**Enables writing to the registers related to the clock generation circuit.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC0_A {
    ///0: Writes protected.
    _0 = 0,
    ///1: Writes not protected.
    _1 = 1,
}
impl From<PRC0_A> for bool {
    #[inline(always)]
    fn from(variant: PRC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRC0` reader - Enables writing to the registers related to the clock generation circuit.
pub type PRC0_R = crate::BitReader<PRC0_A>;
impl PRC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRC0_A {
        match self.bits {
            false => PRC0_A::_0,
            true => PRC0_A::_1,
        }
    }
    ///Writes protected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC0_A::_0
    }
    ///Writes not protected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC0_A::_1
    }
}
///Field `PRC0` writer - Enables writing to the registers related to the clock generation circuit.
pub type PRC0_W<'a, REG> = crate::BitWriter<'a, REG, PRC0_A>;
impl<'a, REG> PRC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writes protected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRC0_A::_0)
    }
    ///Writes not protected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRC0_A::_1)
    }
}
/**Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC1_A {
    ///0: Writes protected.
    _0 = 0,
    ///1: Writes not protected.
    _1 = 1,
}
impl From<PRC1_A> for bool {
    #[inline(always)]
    fn from(variant: PRC1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRC1` reader - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function.
pub type PRC1_R = crate::BitReader<PRC1_A>;
impl PRC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRC1_A {
        match self.bits {
            false => PRC1_A::_0,
            true => PRC1_A::_1,
        }
    }
    ///Writes protected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC1_A::_0
    }
    ///Writes not protected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC1_A::_1
    }
}
///Field `PRC1` writer - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function.
pub type PRC1_W<'a, REG> = crate::BitWriter<'a, REG, PRC1_A>;
impl<'a, REG> PRC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writes protected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRC1_A::_0)
    }
    ///Writes not protected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRC1_A::_1)
    }
}
/**Enables writing to the registers related to the LVD.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC3_A {
    ///0: Writes protected.
    _0 = 0,
    ///1: Writes not protected.
    _1 = 1,
}
impl From<PRC3_A> for bool {
    #[inline(always)]
    fn from(variant: PRC3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRC3` reader - Enables writing to the registers related to the LVD.
pub type PRC3_R = crate::BitReader<PRC3_A>;
impl PRC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRC3_A {
        match self.bits {
            false => PRC3_A::_0,
            true => PRC3_A::_1,
        }
    }
    ///Writes protected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC3_A::_0
    }
    ///Writes not protected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC3_A::_1
    }
}
///Field `PRC3` writer - Enables writing to the registers related to the LVD.
pub type PRC3_W<'a, REG> = crate::BitWriter<'a, REG, PRC3_A>;
impl<'a, REG> PRC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writes protected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRC3_A::_0)
    }
    ///Writes not protected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRC3_A::_1)
    }
}
/**PRKEY Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRKEY_A {
    ///90: Enables writing to the PRCR register.
    _0X5A = 90,
    ///0: Disables writing to the PRCR register.
    OTHERS = 0,
}
impl From<PRKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRKEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRKEY_A {
    type Ux = u8;
}
impl crate::IsEnum for PRKEY_A {}
///Field `PRKEY` writer - PRKEY Key Code
pub type PRKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PRKEY_A, crate::Safe>;
impl<'a, REG> PRKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Enables writing to the PRCR register.
    #[inline(always)]
    pub fn _0x5a(self) -> &'a mut crate::W<REG> {
        self.variant(PRKEY_A::_0X5A)
    }
    ///Disables writing to the PRCR register.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PRKEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Enables writing to the registers related to the clock generation circuit.
    #[inline(always)]
    pub fn prc0(&self) -> PRC0_R {
        PRC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function.
    #[inline(always)]
    pub fn prc1(&self) -> PRC1_R {
        PRC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Enables writing to the registers related to the LVD.
    #[inline(always)]
    pub fn prc3(&self) -> PRC3_R {
        PRC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enables writing to the registers related to the clock generation circuit.
    #[inline(always)]
    pub fn prc0(&mut self) -> PRC0_W<PRCR_SPEC> {
        PRC0_W::new(self, 0)
    }
    ///Bit 1 - Enables writing to the registers related to the operating modes, the low power consumption modes and the battery backup function.
    #[inline(always)]
    pub fn prc1(&mut self) -> PRC1_W<PRCR_SPEC> {
        PRC1_W::new(self, 1)
    }
    ///Bit 3 - Enables writing to the registers related to the LVD.
    #[inline(always)]
    pub fn prc3(&mut self) -> PRC3_W<PRCR_SPEC> {
        PRC3_W::new(self, 3)
    }
    ///Bits 8:15 - PRKEY Key Code
    #[inline(always)]
    pub fn prkey(&mut self) -> PRKEY_W<PRCR_SPEC> {
        PRKEY_W::new(self, 8)
    }
}
/**Protect Register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRCR_SPEC;
impl crate::RegisterSpec for PRCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`prcr::R`](R) reader structure
impl crate::Readable for PRCR_SPEC {}
///`write(|w| ..)` method takes [`prcr::W`](W) writer structure
impl crate::Writable for PRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRCR to value 0
impl crate::Resettable for PRCR_SPEC {}
