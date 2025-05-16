///Register `SDIO_INFO1` reader
pub type R = crate::R<SDIO_INFO1_SPEC>;
///Register `SDIO_INFO1` writer
pub type W = crate::W<SDIO_INFO1_SPEC>;
/**SDIO Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOIRQ_A {
    ///0: SDIO interrupt not accepted
    _0 = 0,
    ///1: SDIO interrupt accepted
    _1 = 1,
}
impl From<IOIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IOIRQ_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `IOIRQ` reader - SDIO Interrupt Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type IOIRQ_R = crate::BitReader<IOIRQ_A>;
impl IOIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOIRQ_A {
        match self.bits {
            false => IOIRQ_A::_0,
            true => IOIRQ_A::_1,
        }
    }
    ///SDIO interrupt not accepted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOIRQ_A::_0
    }
    ///SDIO interrupt accepted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOIRQ_A::_1
    }
}
///Field `IOIRQ` writer - SDIO Interrupt Status
pub type IOIRQ_W<'a, REG> = crate::BitWriter0C<'a, REG, IOIRQ_A>;
impl<'a, REG> IOIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDIO interrupt not accepted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOIRQ_A::_0)
    }
    ///SDIO interrupt accepted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOIRQ_A::_1)
    }
}
/**Field `EXPUB52` reader - EXPUB52 Status FlagNOTE: See manual

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type EXPUB52_R = crate::BitReader;
///Field `EXPUB52` writer - EXPUB52 Status FlagNOTE: See manual
pub type EXPUB52_W<'a, REG> = crate::BitWriter0C<'a, REG>;
/**Field `EXWT` reader - EXWT Status FlagNOTE: See manual

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type EXWT_R = crate::BitReader;
///Field `EXWT` writer - EXWT Status FlagNOTE: See manual
pub type EXWT_W<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    ///Bit 0 - SDIO Interrupt Status
    #[inline(always)]
    pub fn ioirq(&self) -> IOIRQ_R {
        IOIRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 14 - EXPUB52 Status FlagNOTE: See manual
    #[inline(always)]
    pub fn expub52(&self) -> EXPUB52_R {
        EXPUB52_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EXWT Status FlagNOTE: See manual
    #[inline(always)]
    pub fn exwt(&self) -> EXWT_R {
        EXWT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SDIO Interrupt Status
    #[inline(always)]
    pub fn ioirq(&mut self) -> IOIRQ_W<SDIO_INFO1_SPEC> {
        IOIRQ_W::new(self, 0)
    }
    ///Bit 14 - EXPUB52 Status FlagNOTE: See manual
    #[inline(always)]
    pub fn expub52(&mut self) -> EXPUB52_W<SDIO_INFO1_SPEC> {
        EXPUB52_W::new(self, 14)
    }
    ///Bit 15 - EXWT Status FlagNOTE: See manual
    #[inline(always)]
    pub fn exwt(&mut self) -> EXWT_W<SDIO_INFO1_SPEC> {
        EXWT_W::new(self, 15)
    }
}
/**SDIO Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`sdio_info1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_INFO1_SPEC;
impl crate::RegisterSpec for SDIO_INFO1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_info1::R`](R) reader structure
impl crate::Readable for SDIO_INFO1_SPEC {}
///`write(|w| ..)` method takes [`sdio_info1::W`](W) writer structure
impl crate::Writable for SDIO_INFO1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0xc001;
}
///`reset()` method sets SDIO_INFO1 to value 0
impl crate::Resettable for SDIO_INFO1_SPEC {}
