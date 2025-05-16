///Register `PIPEPERI` reader
pub type R = crate::R<PIPEPERI_SPEC>;
///Register `PIPEPERI` writer
pub type W = crate::W<PIPEPERI_SPEC>;
///Field `IITV` reader - Interval Error Detection IntervalThese bits specify the transfer interval timing for the selected pipe as n-th power of 2 of the frame timing.
pub type IITV_R = crate::FieldReader;
///Field `IITV` writer - Interval Error Detection IntervalThese bits specify the transfer interval timing for the selected pipe as n-th power of 2 of the frame timing.
pub type IITV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Isochronous IN Buffer Flush

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFIS_A {
    ///0: The buffer is not flushed.
    _0 = 0,
    ///1: The buffer is flushed.
    _1 = 1,
}
impl From<IFIS_A> for bool {
    #[inline(always)]
    fn from(variant: IFIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IFIS` reader - Isochronous IN Buffer Flush
pub type IFIS_R = crate::BitReader<IFIS_A>;
impl IFIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IFIS_A {
        match self.bits {
            false => IFIS_A::_0,
            true => IFIS_A::_1,
        }
    }
    ///The buffer is not flushed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IFIS_A::_0
    }
    ///The buffer is flushed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IFIS_A::_1
    }
}
///Field `IFIS` writer - Isochronous IN Buffer Flush
pub type IFIS_W<'a, REG> = crate::BitWriter<'a, REG, IFIS_A>;
impl<'a, REG> IFIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The buffer is not flushed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IFIS_A::_0)
    }
    ///The buffer is flushed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IFIS_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Interval Error Detection IntervalThese bits specify the transfer interval timing for the selected pipe as n-th power of 2 of the frame timing.
    #[inline(always)]
    pub fn iitv(&self) -> IITV_R {
        IITV_R::new((self.bits & 7) as u8)
    }
    ///Bit 12 - Isochronous IN Buffer Flush
    #[inline(always)]
    pub fn ifis(&self) -> IFIS_R {
        IFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Interval Error Detection IntervalThese bits specify the transfer interval timing for the selected pipe as n-th power of 2 of the frame timing.
    #[inline(always)]
    pub fn iitv(&mut self) -> IITV_W<PIPEPERI_SPEC> {
        IITV_W::new(self, 0)
    }
    ///Bit 12 - Isochronous IN Buffer Flush
    #[inline(always)]
    pub fn ifis(&mut self) -> IFIS_W<PIPEPERI_SPEC> {
        IFIS_W::new(self, 12)
    }
}
/**Pipe Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`pipeperi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipeperi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPEPERI_SPEC;
impl crate::RegisterSpec for PIPEPERI_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipeperi::R`](R) reader structure
impl crate::Readable for PIPEPERI_SPEC {}
///`write(|w| ..)` method takes [`pipeperi::W`](W) writer structure
impl crate::Writable for PIPEPERI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPEPERI to value 0
impl crate::Resettable for PIPEPERI_SPEC {}
