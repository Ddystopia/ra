///Register `ELSEGR%s` reader
pub type R = crate::R<ELSEGR_SPEC>;
///Register `ELSEGR%s` writer
pub type W = crate::W<ELSEGR_SPEC>;
/**Software Event Generation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEG_A {
    ///0: Normal operation
    _0 = 0,
    ///1: Generate a software event
    _1 = 1,
}
impl From<SEG_A> for bool {
    #[inline(always)]
    fn from(variant: SEG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEG` writer - Software Event Generation
pub type SEG_W<'a, REG> = crate::BitWriter<'a, REG, SEG_A>;
impl<'a, REG> SEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEG_A::_0)
    }
    ///Generate a software event
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEG_A::_1)
    }
}
/**SEG Bit Write Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_A {
    ///0: Disable writes to SEG bit
    _0 = 0,
    ///1: Enable writes to SEG bit
    _1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WE` reader - SEG Bit Write Enable
pub type WE_R = crate::BitReader<WE_A>;
impl WE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::_0,
            true => WE_A::_1,
        }
    }
    ///Disable writes to SEG bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WE_A::_0
    }
    ///Enable writes to SEG bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WE_A::_1
    }
}
///Field `WE` writer - SEG Bit Write Enable
pub type WE_W<'a, REG> = crate::BitWriter<'a, REG, WE_A>;
impl<'a, REG> WE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to SEG bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WE_A::_0)
    }
    ///Enable writes to SEG bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WE_A::_1)
    }
}
/**ELSEGR Register Write Disable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WI_A {
    ///0: Enable writes to ELSEGR register
    _0 = 0,
    ///1: Disable writes to ELSEGR register.
    _1 = 1,
}
impl From<WI_A> for bool {
    #[inline(always)]
    fn from(variant: WI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WI` writer - ELSEGR Register Write Disable
pub type WI_W<'a, REG> = crate::BitWriter<'a, REG, WI_A>;
impl<'a, REG> WI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable writes to ELSEGR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WI_A::_0)
    }
    ///Disable writes to ELSEGR register.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WI_A::_1)
    }
}
impl R {
    ///Bit 6 - SEG Bit Write Enable
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software Event Generation
    #[inline(always)]
    pub fn seg(&mut self) -> SEG_W<ELSEGR_SPEC> {
        SEG_W::new(self, 0)
    }
    ///Bit 6 - SEG Bit Write Enable
    #[inline(always)]
    pub fn we(&mut self) -> WE_W<ELSEGR_SPEC> {
        WE_W::new(self, 6)
    }
    ///Bit 7 - ELSEGR Register Write Disable
    #[inline(always)]
    pub fn wi(&mut self) -> WI_W<ELSEGR_SPEC> {
        WI_W::new(self, 7)
    }
}
/**Event Link Software Event Generation Register %s

You can [`read`](crate::Reg::read) this register and get [`elsegr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ELSEGR_SPEC;
impl crate::RegisterSpec for ELSEGR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`elsegr::R`](R) reader structure
impl crate::Readable for ELSEGR_SPEC {}
///`write(|w| ..)` method takes [`elsegr::W`](W) writer structure
impl crate::Writable for ELSEGR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELSEGR%s to value 0x80
impl crate::Resettable for ELSEGR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
