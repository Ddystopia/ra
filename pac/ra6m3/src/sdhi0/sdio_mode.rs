///Register `SDIO_MODE` reader
pub type R = crate::R<SDIO_MODE_SPEC>;
///Register `SDIO_MODE` writer
pub type W = crate::W<SDIO_MODE_SPEC>;
/**SDIO Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN_A {
    ///1: Enables the SD host interface to receive SDIO interrupt from the SDIO card
    _1 = 1,
    ///0: Disables the SD host interface to receive SDIO interrupt from the SDIO card
    _0 = 0,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INTEN` reader - SDIO Mode
pub type INTEN_R = crate::BitReader<INTEN_A>;
impl INTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTEN_A {
        match self.bits {
            true => INTEN_A::_1,
            false => INTEN_A::_0,
        }
    }
    ///Enables the SD host interface to receive SDIO interrupt from the SDIO card
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTEN_A::_1
    }
    ///Disables the SD host interface to receive SDIO interrupt from the SDIO card
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTEN_A::_0
    }
}
///Field `INTEN` writer - SDIO Mode
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG, INTEN_A>;
impl<'a, REG> INTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables the SD host interface to receive SDIO interrupt from the SDIO card
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INTEN_A::_1)
    }
    ///Disables the SD host interface to receive SDIO interrupt from the SDIO card
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INTEN_A::_0)
    }
}
/**Read Wait Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWREQ_A {
    ///0: Allow SD/MMC to exit read wait state
    _0 = 0,
    ///1: Request for SD/MMC to enter read wait state.
    _1 = 1,
}
impl From<RWREQ_A> for bool {
    #[inline(always)]
    fn from(variant: RWREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RWREQ` reader - Read Wait Request
pub type RWREQ_R = crate::BitReader<RWREQ_A>;
impl RWREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWREQ_A {
        match self.bits {
            false => RWREQ_A::_0,
            true => RWREQ_A::_1,
        }
    }
    ///Allow SD/MMC to exit read wait state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWREQ_A::_0
    }
    ///Request for SD/MMC to enter read wait state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWREQ_A::_1
    }
}
///Field `RWREQ` writer - Read Wait Request
pub type RWREQ_W<'a, REG> = crate::BitWriter<'a, REG, RWREQ_A>;
impl<'a, REG> RWREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Allow SD/MMC to exit read wait state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RWREQ_A::_0)
    }
    ///Request for SD/MMC to enter read wait state.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RWREQ_A::_1)
    }
}
///Field `IOABT` reader - SDIO AbortNOTE: See manual
pub type IOABT_R = crate::BitReader;
///Field `IOABT` writer - SDIO AbortNOTE: See manual
pub type IOABT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C52PUB` reader - SDIO None AbortNOTE: See manual
pub type C52PUB_R = crate::BitReader;
///Field `C52PUB` writer - SDIO None AbortNOTE: See manual
pub type C52PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SDIO Mode
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Read Wait Request
    #[inline(always)]
    pub fn rwreq(&self) -> RWREQ_R {
        RWREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - SDIO AbortNOTE: See manual
    #[inline(always)]
    pub fn ioabt(&self) -> IOABT_R {
        IOABT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SDIO None AbortNOTE: See manual
    #[inline(always)]
    pub fn c52pub(&self) -> C52PUB_R {
        C52PUB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SDIO Mode
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W<SDIO_MODE_SPEC> {
        INTEN_W::new(self, 0)
    }
    ///Bit 2 - Read Wait Request
    #[inline(always)]
    pub fn rwreq(&mut self) -> RWREQ_W<SDIO_MODE_SPEC> {
        RWREQ_W::new(self, 2)
    }
    ///Bit 8 - SDIO AbortNOTE: See manual
    #[inline(always)]
    pub fn ioabt(&mut self) -> IOABT_W<SDIO_MODE_SPEC> {
        IOABT_W::new(self, 8)
    }
    ///Bit 9 - SDIO None AbortNOTE: See manual
    #[inline(always)]
    pub fn c52pub(&mut self) -> C52PUB_W<SDIO_MODE_SPEC> {
        C52PUB_W::new(self, 9)
    }
}
/**SDIO Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sdio_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_MODE_SPEC;
impl crate::RegisterSpec for SDIO_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_mode::R`](R) reader structure
impl crate::Readable for SDIO_MODE_SPEC {}
///`write(|w| ..)` method takes [`sdio_mode::W`](W) writer structure
impl crate::Writable for SDIO_MODE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIO_MODE to value 0
impl crate::Resettable for SDIO_MODE_SPEC {}
