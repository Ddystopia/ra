///Register `VBTWEGR` reader
pub type R = crate::R<VBTWEGR_SPEC>;
///Register `VBTWEGR` writer
pub type W = crate::W<VBTWEGR_SPEC>;
/**VBATWIO0 Wakeup Trigger Source Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0EG_A {
    ///0: Wakeup trigger is generated at a falling edge
    _0 = 0,
    ///1: Wakeup trigger is generated at a rising edge.
    _1 = 1,
}
impl From<VCH0EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH0EG` reader - VBATWIO0 Wakeup Trigger Source Edge Select
pub type VCH0EG_R = crate::BitReader<VCH0EG_A>;
impl VCH0EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH0EG_A {
        match self.bits {
            false => VCH0EG_A::_0,
            true => VCH0EG_A::_1,
        }
    }
    ///Wakeup trigger is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0EG_A::_0
    }
    ///Wakeup trigger is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0EG_A::_1
    }
}
///Field `VCH0EG` writer - VBATWIO0 Wakeup Trigger Source Edge Select
pub type VCH0EG_W<'a, REG> = crate::BitWriter<'a, REG, VCH0EG_A>;
impl<'a, REG> VCH0EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wakeup trigger is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0EG_A::_0)
    }
    ///Wakeup trigger is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0EG_A::_1)
    }
}
/**VBATWIO1 Wakeup Trigger Source Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1EG_A {
    ///0: Wakeup trigger is generated at a falling edge
    _0 = 0,
    ///1: Wakeup trigger is generated at a rising edge.
    _1 = 1,
}
impl From<VCH1EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH1EG` reader - VBATWIO1 Wakeup Trigger Source Edge Select
pub type VCH1EG_R = crate::BitReader<VCH1EG_A>;
impl VCH1EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH1EG_A {
        match self.bits {
            false => VCH1EG_A::_0,
            true => VCH1EG_A::_1,
        }
    }
    ///Wakeup trigger is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1EG_A::_0
    }
    ///Wakeup trigger is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1EG_A::_1
    }
}
///Field `VCH1EG` writer - VBATWIO1 Wakeup Trigger Source Edge Select
pub type VCH1EG_W<'a, REG> = crate::BitWriter<'a, REG, VCH1EG_A>;
impl<'a, REG> VCH1EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wakeup trigger is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1EG_A::_0)
    }
    ///Wakeup trigger is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1EG_A::_1)
    }
}
/**VBATWIO2 Wakeup Trigger Source Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2EG_A {
    ///0: Wakeup trigger is generated at a falling edge
    _0 = 0,
    ///1: Wakeup trigger is generated at a rising edge.
    _1 = 1,
}
impl From<VCH2EG_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH2EG` reader - VBATWIO2 Wakeup Trigger Source Edge Select
pub type VCH2EG_R = crate::BitReader<VCH2EG_A>;
impl VCH2EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH2EG_A {
        match self.bits {
            false => VCH2EG_A::_0,
            true => VCH2EG_A::_1,
        }
    }
    ///Wakeup trigger is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2EG_A::_0
    }
    ///Wakeup trigger is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2EG_A::_1
    }
}
///Field `VCH2EG` writer - VBATWIO2 Wakeup Trigger Source Edge Select
pub type VCH2EG_W<'a, REG> = crate::BitWriter<'a, REG, VCH2EG_A>;
impl<'a, REG> VCH2EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wakeup trigger is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2EG_A::_0)
    }
    ///Wakeup trigger is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2EG_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select
    #[inline(always)]
    pub fn vch0eg(&self) -> VCH0EG_R {
        VCH0EG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select
    #[inline(always)]
    pub fn vch1eg(&self) -> VCH1EG_R {
        VCH1EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select
    #[inline(always)]
    pub fn vch2eg(&self) -> VCH2EG_R {
        VCH2EG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATWIO0 Wakeup Trigger Source Edge Select
    #[inline(always)]
    pub fn vch0eg(&mut self) -> VCH0EG_W<VBTWEGR_SPEC> {
        VCH0EG_W::new(self, 0)
    }
    ///Bit 1 - VBATWIO1 Wakeup Trigger Source Edge Select
    #[inline(always)]
    pub fn vch1eg(&mut self) -> VCH1EG_W<VBTWEGR_SPEC> {
        VCH1EG_W::new(self, 1)
    }
    ///Bit 2 - VBATWIO2 Wakeup Trigger Source Edge Select
    #[inline(always)]
    pub fn vch2eg(&mut self) -> VCH2EG_W<VBTWEGR_SPEC> {
        VCH2EG_W::new(self, 2)
    }
}
/**VBATT Wakeup Trigger source Edge Register

You can [`read`](crate::Reg::read) this register and get [`vbtwegr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwegr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTWEGR_SPEC;
impl crate::RegisterSpec for VBTWEGR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtwegr::R`](R) reader structure
impl crate::Readable for VBTWEGR_SPEC {}
///`write(|w| ..)` method takes [`vbtwegr::W`](W) writer structure
impl crate::Writable for VBTWEGR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTWEGR to value 0
impl crate::Resettable for VBTWEGR_SPEC {}
