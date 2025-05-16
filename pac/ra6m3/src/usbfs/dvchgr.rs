///Register `DVCHGR` reader
pub type R = crate::R<DVCHGR_SPEC>;
///Register `DVCHGR` writer
pub type W = crate::W<DVCHGR_SPEC>;
/**Device State Change

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVCHG_A {
    ///0: Disables the writing to the USBADDR.STSRECOV\[3:0\] bits and USBADDR.USBADDR\[6:0\].
    _0 = 0,
    ///1: Enables the writing to the USBADDR.STSRECOV\[3:0\] bits and USBADDR.USBADDR\[6:0\].
    _1 = 1,
}
impl From<DVCHG_A> for bool {
    #[inline(always)]
    fn from(variant: DVCHG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DVCHG` reader - Device State Change
pub type DVCHG_R = crate::BitReader<DVCHG_A>;
impl DVCHG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVCHG_A {
        match self.bits {
            false => DVCHG_A::_0,
            true => DVCHG_A::_1,
        }
    }
    ///Disables the writing to the USBADDR.STSRECOV\[3:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVCHG_A::_0
    }
    ///Enables the writing to the USBADDR.STSRECOV\[3:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVCHG_A::_1
    }
}
///Field `DVCHG` writer - Device State Change
pub type DVCHG_W<'a, REG> = crate::BitWriter<'a, REG, DVCHG_A>;
impl<'a, REG> DVCHG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the writing to the USBADDR.STSRECOV\[3:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVCHG_A::_0)
    }
    ///Enables the writing to the USBADDR.STSRECOV\[3:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVCHG_A::_1)
    }
}
impl R {
    ///Bit 15 - Device State Change
    #[inline(always)]
    pub fn dvchg(&self) -> DVCHG_R {
        DVCHG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - Device State Change
    #[inline(always)]
    pub fn dvchg(&mut self) -> DVCHG_W<DVCHGR_SPEC> {
        DVCHG_W::new(self, 15)
    }
}
/**Device State Change Register

You can [`read`](crate::Reg::read) this register and get [`dvchgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvchgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DVCHGR_SPEC;
impl crate::RegisterSpec for DVCHGR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dvchgr::R`](R) reader structure
impl crate::Readable for DVCHGR_SPEC {}
///`write(|w| ..)` method takes [`dvchgr::W`](W) writer structure
impl crate::Writable for DVCHGR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVCHGR to value 0
impl crate::Resettable for DVCHGR_SPEC {}
