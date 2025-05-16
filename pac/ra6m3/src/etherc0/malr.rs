///Register `MALR` reader
pub type R = crate::R<MALR_SPEC>;
///Register `MALR` writer
pub type W = crate::W<MALR_SPEC>;
///Field `MALR` reader - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address.
pub type MALR_R = crate::FieldReader<u16>;
///Field `MALR` writer - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address.
pub type MALR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address.
    #[inline(always)]
    pub fn malr(&self) -> MALR_R {
        MALR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - MAC Address Lower Bit RegisterThe MALR register sets the lower 16 bits of the 48-bit MAC address.
    #[inline(always)]
    pub fn malr(&mut self) -> MALR_W<MALR_SPEC> {
        MALR_W::new(self, 0)
    }
}
/**MAC Address Lower Bit Register

You can [`read`](crate::Reg::read) this register and get [`malr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`malr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MALR_SPEC;
impl crate::RegisterSpec for MALR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`malr::R`](R) reader structure
impl crate::Readable for MALR_SPEC {}
///`write(|w| ..)` method takes [`malr::W`](W) writer structure
impl crate::Writable for MALR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MALR to value 0
impl crate::Resettable for MALR_SPEC {}
