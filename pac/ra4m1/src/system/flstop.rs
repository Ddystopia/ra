///Register `FLSTOP` reader
pub type R = crate::R<FLSTOP_SPEC>;
///Register `FLSTOP` writer
pub type W = crate::W<FLSTOP_SPEC>;
/**Selecting ON/OFF of the Flash Memory Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSTOP_A {
    ///0: Code flash and data flash memory operates
    _0 = 0,
    ///1: Code flash and data flash memory stops.
    _1 = 1,
}
impl From<FLSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: FLSTOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLSTOP` reader - Selecting ON/OFF of the Flash Memory Operation
pub type FLSTOP_R = crate::BitReader<FLSTOP_A>;
impl FLSTOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLSTOP_A {
        match self.bits {
            false => FLSTOP_A::_0,
            true => FLSTOP_A::_1,
        }
    }
    ///Code flash and data flash memory operates
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLSTOP_A::_0
    }
    ///Code flash and data flash memory stops.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLSTOP_A::_1
    }
}
///Field `FLSTOP` writer - Selecting ON/OFF of the Flash Memory Operation
pub type FLSTOP_W<'a, REG> = crate::BitWriter<'a, REG, FLSTOP_A>;
impl<'a, REG> FLSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Code flash and data flash memory operates
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLSTOP_A::_0)
    }
    ///Code flash and data flash memory stops.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLSTOP_A::_1)
    }
}
/**Flash Memory Operation Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSTPF_A {
    ///0: Transition completed
    _0 = 0,
    ///1: During transition (from the flash-stop-status to flash-operating-status or vice versa)
    _1 = 1,
}
impl From<FLSTPF_A> for bool {
    #[inline(always)]
    fn from(variant: FLSTPF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLSTPF` reader - Flash Memory Operation Status Flag
pub type FLSTPF_R = crate::BitReader<FLSTPF_A>;
impl FLSTPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLSTPF_A {
        match self.bits {
            false => FLSTPF_A::_0,
            true => FLSTPF_A::_1,
        }
    }
    ///Transition completed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLSTPF_A::_0
    }
    ///During transition (from the flash-stop-status to flash-operating-status or vice versa)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLSTPF_A::_1
    }
}
impl R {
    ///Bit 0 - Selecting ON/OFF of the Flash Memory Operation
    #[inline(always)]
    pub fn flstop(&self) -> FLSTOP_R {
        FLSTOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Flash Memory Operation Status Flag
    #[inline(always)]
    pub fn flstpf(&self) -> FLSTPF_R {
        FLSTPF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Selecting ON/OFF of the Flash Memory Operation
    #[inline(always)]
    pub fn flstop(&mut self) -> FLSTOP_W<'_, FLSTOP_SPEC> {
        FLSTOP_W::new(self, 0)
    }
}
/**Flash Operation Control Register

You can [`read`](crate::Reg::read) this register and get [`flstop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flstop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FLSTOP_SPEC;
impl crate::RegisterSpec for FLSTOP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`flstop::R`](R) reader structure
impl crate::Readable for FLSTOP_SPEC {}
///`write(|w| ..)` method takes [`flstop::W`](W) writer structure
impl crate::Writable for FLSTOP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLSTOP to value 0
impl crate::Resettable for FLSTOP_SPEC {}
