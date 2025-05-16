///Register `PDTTLR` reader
pub type R = crate::R<PDTTLR_SPEC>;
///Register `PDTTLR` writer
pub type W = crate::W<PDTTLR_SPEC>;
///Field `PDTL` reader - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages.
pub type PDTL_R = crate::FieldReader;
///Field `PDTL` writer - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages.
pub type PDTL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages.
    #[inline(always)]
    pub fn pdtl(&self) -> PDTL_R {
        PDTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages.
    #[inline(always)]
    pub fn pdtl(&mut self) -> PDTL_W<PDTTLR_SPEC> {
        PDTL_W::new(self, 0)
    }
}
/**PTP-pdelay Message TTL Setting Register

You can [`read`](crate::Reg::read) this register and get [`pdttlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdttlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PDTTLR_SPEC;
impl crate::RegisterSpec for PDTTLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pdttlr::R`](R) reader structure
impl crate::Readable for PDTTLR_SPEC {}
///`write(|w| ..)` method takes [`pdttlr::W`](W) writer structure
impl crate::Writable for PDTTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDTTLR to value 0x01
impl crate::Resettable for PDTTLR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
