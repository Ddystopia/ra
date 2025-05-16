///Register `SYRFL2R` reader
pub type R = crate::R<SYRFL2R_SPEC>;
///Register `SYRFL2R` writer
pub type W = crate::W<SYRFL2R_SPEC>;
/**Management Message Processing Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAN0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<MAN0_A> for bool {
    #[inline(always)]
    fn from(variant: MAN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MAN0` reader - Management Message Processing Setting
pub type MAN0_R = crate::BitReader<MAN0_A>;
impl MAN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MAN0_A {
        match self.bits {
            false => MAN0_A::_0,
            true => MAN0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAN0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAN0_A::_1
    }
}
///Field `MAN0` writer - Management Message Processing Setting
pub type MAN0_W<'a, REG> = crate::BitWriter<'a, REG, MAN0_A>;
impl<'a, REG> MAN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MAN0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MAN0_A::_1)
    }
}
/**Signaling Message Processing Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIG0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<SIG0_A> for bool {
    #[inline(always)]
    fn from(variant: SIG0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SIG0` reader - Signaling Message Processing Setting
pub type SIG0_R = crate::BitReader<SIG0_A>;
impl SIG0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SIG0_A {
        match self.bits {
            false => SIG0_A::_0,
            true => SIG0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIG0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIG0_A::_1
    }
}
///Field `SIG0` writer - Signaling Message Processing Setting
pub type SIG0_W<'a, REG> = crate::BitWriter<'a, REG, SIG0_A>;
impl<'a, REG> SIG0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG0_A::_1)
    }
}
/**Illegal Message Processing Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILL0_A {
    ///0: Messages are not transferred to the PTPEDMAC.
    _0 = 0,
    ///1: Messages are transferred to the PTPEDMAC.
    _1 = 1,
}
impl From<ILL0_A> for bool {
    #[inline(always)]
    fn from(variant: ILL0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ILL0` reader - Illegal Message Processing Setting
pub type ILL0_R = crate::BitReader<ILL0_A>;
impl ILL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ILL0_A {
        match self.bits {
            false => ILL0_A::_0,
            true => ILL0_A::_1,
        }
    }
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILL0_A::_0
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILL0_A::_1
    }
}
///Field `ILL0` writer - Illegal Message Processing Setting
pub type ILL0_W<'a, REG> = crate::BitWriter<'a, REG, ILL0_A>;
impl<'a, REG> ILL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Messages are not transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ILL0_A::_0)
    }
    ///Messages are transferred to the PTPEDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ILL0_A::_1)
    }
}
impl R {
    ///Bit 0 - Management Message Processing Setting
    #[inline(always)]
    pub fn man0(&self) -> MAN0_R {
        MAN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Signaling Message Processing Setting
    #[inline(always)]
    pub fn sig0(&self) -> SIG0_R {
        SIG0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 28 - Illegal Message Processing Setting
    #[inline(always)]
    pub fn ill0(&self) -> ILL0_R {
        ILL0_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Management Message Processing Setting
    #[inline(always)]
    pub fn man0(&mut self) -> MAN0_W<SYRFL2R_SPEC> {
        MAN0_W::new(self, 0)
    }
    ///Bit 4 - Signaling Message Processing Setting
    #[inline(always)]
    pub fn sig0(&mut self) -> SIG0_W<SYRFL2R_SPEC> {
        SIG0_W::new(self, 4)
    }
    ///Bit 28 - Illegal Message Processing Setting
    #[inline(always)]
    pub fn ill0(&mut self) -> ILL0_W<SYRFL2R_SPEC> {
        ILL0_W::new(self, 28)
    }
}
/**SYNFP Reception Filter Register 2

You can [`read`](crate::Reg::read) this register and get [`syrfl2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syrfl2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYRFL2R_SPEC;
impl crate::RegisterSpec for SYRFL2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syrfl2r::R`](R) reader structure
impl crate::Readable for SYRFL2R_SPEC {}
///`write(|w| ..)` method takes [`syrfl2r::W`](W) writer structure
impl crate::Writable for SYRFL2R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYRFL2R to value 0
impl crate::Resettable for SYRFL2R_SPEC {}
