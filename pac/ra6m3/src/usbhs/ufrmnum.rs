///Register `UFRMNUM` reader
pub type R = crate::R<UFRMNUM_SPEC>;
///Register `UFRMNUM` writer
pub type W = crate::W<UFRMNUM_SPEC>;
///Field `UFRNM` reader - MicroframeIndicate the microframe number.
pub type UFRNM_R = crate::FieldReader;
/**Device State Change

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVCHG_A {
    ///0: Disables the writing to the USBADDR.STSRECOV0\[2:0\] bits and USBADDR.USBADDR\[6:0\].
    _0 = 0,
    ///1: Enables the writing to the USBADDR.STSRECOV0\[2:0\] bits and USBADDR.USBADDR\[6:0\].
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
    ///Disables the writing to the USBADDR.STSRECOV0\[2:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVCHG_A::_0
    }
    ///Enables the writing to the USBADDR.STSRECOV0\[2:0\] bits and USBADDR.USBADDR\[6:0\].
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
    ///Disables the writing to the USBADDR.STSRECOV0\[2:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVCHG_A::_0)
    }
    ///Enables the writing to the USBADDR.STSRECOV0\[2:0\] bits and USBADDR.USBADDR\[6:0\].
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVCHG_A::_1)
    }
}
impl R {
    ///Bits 0:2 - MicroframeIndicate the microframe number.
    #[inline(always)]
    pub fn ufrnm(&self) -> UFRNM_R {
        UFRNM_R::new((self.bits & 7) as u8)
    }
    ///Bit 15 - Device State Change
    #[inline(always)]
    pub fn dvchg(&self) -> DVCHG_R {
        DVCHG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - Device State Change
    #[inline(always)]
    pub fn dvchg(&mut self) -> DVCHG_W<UFRMNUM_SPEC> {
        DVCHG_W::new(self, 15)
    }
}
/**uFrame Number Register

You can [`read`](crate::Reg::read) this register and get [`ufrmnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrmnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UFRMNUM_SPEC;
impl crate::RegisterSpec for UFRMNUM_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ufrmnum::R`](R) reader structure
impl crate::Readable for UFRMNUM_SPEC {}
///`write(|w| ..)` method takes [`ufrmnum::W`](W) writer structure
impl crate::Writable for UFRMNUM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UFRMNUM to value 0
impl crate::Resettable for UFRMNUM_SPEC {}
