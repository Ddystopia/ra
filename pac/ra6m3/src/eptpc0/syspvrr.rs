///Register `SYSPVRR` reader
pub type R = crate::R<SYSPVRR_SPEC>;
///Register `SYSPVRR` writer
pub type W = crate::W<SYSPVRR_SPEC>;
///Field `VER` reader - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2).
pub type VER_R = crate::FieldReader;
///Field `VER` writer - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2).
pub type VER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRSP` reader - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588).
pub type TRSP_R = crate::FieldReader;
///Field `TRSP` writer - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588).
pub type TRSP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2).
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588).
    #[inline(always)]
    pub fn trsp(&self) -> TRSP_R {
        TRSP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2).
    #[inline(always)]
    pub fn ver(&mut self) -> VER_W<SYSPVRR_SPEC> {
        VER_W::new(self, 0)
    }
    ///Bits 4:7 - transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588).
    #[inline(always)]
    pub fn trsp(&mut self) -> TRSP_W<SYSPVRR_SPEC> {
        TRSP_W::new(self, 4)
    }
}
/**SYNFP Specification Version Setting Register

You can [`read`](crate::Reg::read) this register and get [`syspvrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspvrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSPVRR_SPEC;
impl crate::RegisterSpec for SYSPVRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syspvrr::R`](R) reader structure
impl crate::Readable for SYSPVRR_SPEC {}
///`write(|w| ..)` method takes [`syspvrr::W`](W) writer structure
impl crate::Writable for SYSPVRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSPVRR to value 0x02
impl crate::Resettable for SYSPVRR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
