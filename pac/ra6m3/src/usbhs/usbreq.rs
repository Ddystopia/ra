///Register `USBREQ` reader
pub type R = crate::R<USBREQ_SPEC>;
///Register `USBREQ` writer
pub type W = crate::W<USBREQ_SPEC>;
///Field `BMREQUESTTYPE` reader - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write
pub type BMREQUESTTYPE_R = crate::FieldReader;
///Field `BMREQUESTTYPE` writer - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write
pub type BMREQUESTTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BREQUEST` reader - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write
pub type BREQUEST_R = crate::FieldReader;
///Field `BREQUEST` writer - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write
pub type BREQUEST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - USB request bmRequestType value Finction controller selected : read-only Host controller selected : read-write
    #[inline(always)]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W<USBREQ_SPEC> {
        BMREQUESTTYPE_W::new(self, 0)
    }
    ///Bits 8:15 - USB request bRequest value Finction controller selected : read-only Host controller selected : read-write
    #[inline(always)]
    pub fn brequest(&mut self) -> BREQUEST_W<USBREQ_SPEC> {
        BREQUEST_W::new(self, 8)
    }
}
/**USB Request Type Register

You can [`read`](crate::Reg::read) this register and get [`usbreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBREQ_SPEC;
impl crate::RegisterSpec for USBREQ_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbreq::R`](R) reader structure
impl crate::Readable for USBREQ_SPEC {}
///`write(|w| ..)` method takes [`usbreq::W`](W) writer structure
impl crate::Writable for USBREQ_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBREQ to value 0
impl crate::Resettable for USBREQ_SPEC {}
