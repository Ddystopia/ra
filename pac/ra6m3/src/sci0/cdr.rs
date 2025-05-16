///Register `CDR` reader
pub type R = crate::R<CDR_SPEC>;
///Register `CDR` writer
pub type W = crate::W<CDR_SPEC>;
///Field `CMPD` reader - Compare Match DataCompare data pattern for address match wake-up function
pub type CMPD_R = crate::FieldReader<u16>;
///Field `CMPD` writer - Compare Match DataCompare data pattern for address match wake-up function
pub type CMPD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Compare Match DataCompare data pattern for address match wake-up function
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(self.bits & 0x01ff)
    }
}
impl W {
    ///Bits 0:8 - Compare Match DataCompare data pattern for address match wake-up function
    #[inline(always)]
    pub fn cmpd(&mut self) -> CMPD_W<CDR_SPEC> {
        CMPD_W::new(self, 0)
    }
}
/**Compare Match Data Register

You can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`cdr::R`](R) reader structure
impl crate::Readable for CDR_SPEC {}
///`write(|w| ..)` method takes [`cdr::W`](W) writer structure
impl crate::Writable for CDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDR_SPEC {}
