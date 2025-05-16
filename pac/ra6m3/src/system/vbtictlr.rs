///Register `VBTICTLR` reader
pub type R = crate::R<VBTICTLR_SPEC>;
///Register `VBTICTLR` writer
pub type W = crate::W<VBTICTLR_SPEC>;
/**RTCIC0 Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0INEN_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<VCH0INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0INEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH0INEN` reader - RTCIC0 Input Enable
pub type VCH0INEN_R = crate::BitReader<VCH0INEN_A>;
impl VCH0INEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH0INEN_A {
        match self.bits {
            false => VCH0INEN_A::_0,
            true => VCH0INEN_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0INEN_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0INEN_A::_1
    }
}
///Field `VCH0INEN` writer - RTCIC0 Input Enable
pub type VCH0INEN_W<'a, REG> = crate::BitWriter<'a, REG, VCH0INEN_A>;
impl<'a, REG> VCH0INEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0INEN_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0INEN_A::_1)
    }
}
/**RTCIC1 Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1INEN_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<VCH1INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1INEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH1INEN` reader - RTCIC1 Input Enable
pub type VCH1INEN_R = crate::BitReader<VCH1INEN_A>;
impl VCH1INEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH1INEN_A {
        match self.bits {
            false => VCH1INEN_A::_0,
            true => VCH1INEN_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1INEN_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1INEN_A::_1
    }
}
///Field `VCH1INEN` writer - RTCIC1 Input Enable
pub type VCH1INEN_W<'a, REG> = crate::BitWriter<'a, REG, VCH1INEN_A>;
impl<'a, REG> VCH1INEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1INEN_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1INEN_A::_1)
    }
}
/**RTCIC2 Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2INEN_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<VCH2INEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2INEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH2INEN` reader - RTCIC2 Input Enable
pub type VCH2INEN_R = crate::BitReader<VCH2INEN_A>;
impl VCH2INEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH2INEN_A {
        match self.bits {
            false => VCH2INEN_A::_0,
            true => VCH2INEN_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2INEN_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2INEN_A::_1
    }
}
///Field `VCH2INEN` writer - RTCIC2 Input Enable
pub type VCH2INEN_W<'a, REG> = crate::BitWriter<'a, REG, VCH2INEN_A>;
impl<'a, REG> VCH2INEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2INEN_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2INEN_A::_1)
    }
}
impl R {
    ///Bit 0 - RTCIC0 Input Enable
    #[inline(always)]
    pub fn vch0inen(&self) -> VCH0INEN_R {
        VCH0INEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTCIC1 Input Enable
    #[inline(always)]
    pub fn vch1inen(&self) -> VCH1INEN_R {
        VCH1INEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTCIC2 Input Enable
    #[inline(always)]
    pub fn vch2inen(&self) -> VCH2INEN_R {
        VCH2INEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RTCIC0 Input Enable
    #[inline(always)]
    pub fn vch0inen(&mut self) -> VCH0INEN_W<VBTICTLR_SPEC> {
        VCH0INEN_W::new(self, 0)
    }
    ///Bit 1 - RTCIC1 Input Enable
    #[inline(always)]
    pub fn vch1inen(&mut self) -> VCH1INEN_W<VBTICTLR_SPEC> {
        VCH1INEN_W::new(self, 1)
    }
    ///Bit 2 - RTCIC2 Input Enable
    #[inline(always)]
    pub fn vch2inen(&mut self) -> VCH2INEN_W<VBTICTLR_SPEC> {
        VCH2INEN_W::new(self, 2)
    }
}
/**VBATT Input Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtictlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtictlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTICTLR_SPEC;
impl crate::RegisterSpec for VBTICTLR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtictlr::R`](R) reader structure
impl crate::Readable for VBTICTLR_SPEC {}
///`write(|w| ..)` method takes [`vbtictlr::W`](W) writer structure
impl crate::Writable for VBTICTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTICTLR to value 0
impl crate::Resettable for VBTICTLR_SPEC {}
