///Register `PGUDPR` reader
pub type R = crate::R<PGUDPR_SPEC>;
///Register `PGUDPR` writer
pub type W = crate::W<PGUDPR_SPEC>;
///Field `GEUPT` reader - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages.
pub type GEUPT_R = crate::FieldReader<u16>;
///Field `GEUPT` writer - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages.
pub type GEUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages.
    #[inline(always)]
    pub fn geupt(&self) -> GEUPT_R {
        GEUPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages.
    #[inline(always)]
    pub fn geupt(&mut self) -> GEUPT_W<PGUDPR_SPEC> {
        GEUPT_W::new(self, 0)
    }
}
/**PTP general Message UDP Destination Port Number Setting Register

You can [`read`](crate::Reg::read) this register and get [`pgudpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgudpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PGUDPR_SPEC;
impl crate::RegisterSpec for PGUDPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pgudpr::R`](R) reader structure
impl crate::Readable for PGUDPR_SPEC {}
///`write(|w| ..)` method takes [`pgudpr::W`](W) writer structure
impl crate::Writable for PGUDPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PGUDPR to value 0x0140
impl crate::Resettable for PGUDPR_SPEC {
    const RESET_VALUE: u32 = 0x0140;
}
