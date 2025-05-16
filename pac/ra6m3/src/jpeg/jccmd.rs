///Register `JCCMD` writer
pub type W = crate::W<JCCMD_SPEC>;
/**JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSRT_A {
    ///0: No effect.
    _0 = 0,
    ///1: Start JPEG core processing
    _1 = 1,
}
impl From<JSRT_A> for bool {
    #[inline(always)]
    fn from(variant: JSRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `JSRT` writer - JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation.
pub type JSRT_W<'a, REG> = crate::BitWriter<'a, REG, JSRT_A>;
impl<'a, REG> JSRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(JSRT_A::_0)
    }
    ///Start JPEG core processing
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(JSRT_A::_1)
    }
}
/**JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JRST_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the process-stopped state caused by requests to read the image size and pixel format(enabled by the INT3 bit in JINTE0).
    _1 = 1,
}
impl From<JRST_A> for bool {
    #[inline(always)]
    fn from(variant: JRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `JRST` writer - JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1.
pub type JRST_W<'a, REG> = crate::BitWriter<'a, REG, JRST_A>;
impl<'a, REG> JRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(JRST_A::_0)
    }
    ///Clear the process-stopped state caused by requests to read the image size and pixel format(enabled by the INT3 bit in JINTE0).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(JRST_A::_1)
    }
}
/**Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEND_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear all bits in JINTE0.
    _1 = 1,
}
impl From<JEND_A> for bool {
    #[inline(always)]
    fn from(variant: JEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `JEND` writer - Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1
pub type JEND_W<'a, REG> = crate::BitWriter<'a, REG, JEND_A>;
impl<'a, REG> JEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(JEND_A::_0)
    }
    ///Clear all bits in JINTE0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(JEND_A::_1)
    }
}
/**Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRST_A {
    ///0: No effect.
    _0 = 0,
    ///1: Resets the JCDTCU, JCDTCM, JCDTCD, JCDERR and JCRST registers.
    _1 = 1,
}
impl From<BRST_A> for bool {
    #[inline(always)]
    fn from(variant: BRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRST` writer - Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued.
pub type BRST_W<'a, REG> = crate::BitWriter<'a, REG, BRST_A>;
impl<'a, REG> BRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRST_A::_0)
    }
    ///Resets the JCDTCU, JCDTCM, JCDTCD, JCDERR and JCRST registers.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRST_A::_1)
    }
}
impl W {
    ///Bit 0 - JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation.
    #[inline(always)]
    pub fn jsrt(&mut self) -> JSRT_W<JCCMD_SPEC> {
        JSRT_W::new(self, 0)
    }
    ///Bit 1 - JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1.
    #[inline(always)]
    pub fn jrst(&mut self) -> JRST_W<JCCMD_SPEC> {
        JRST_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1
    #[inline(always)]
    pub fn jend(&mut self) -> JEND_W<JCCMD_SPEC> {
        JEND_W::new(self, 2)
    }
    ///Bit 7 - Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued.
    #[inline(always)]
    pub fn brst(&mut self) -> BRST_W<JCCMD_SPEC> {
        BRST_W::new(self, 7)
    }
}
/**JPEG Code Command Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jccmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCCMD_SPEC;
impl crate::RegisterSpec for JCCMD_SPEC {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`jccmd::W`](W) writer structure
impl crate::Writable for JCCMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCCMD to value 0
impl crate::Resettable for JCCMD_SPEC {}
