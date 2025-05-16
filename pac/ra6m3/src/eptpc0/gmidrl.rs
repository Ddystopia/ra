///Register `GMIDRL` reader
pub type R = crate::R<GMIDRL_SPEC>;
///Register `GMIDRL` writer
pub type W = crate::W<GMIDRL_SPEC>;
///Field `GMIDRL` reader - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
pub type GMIDRL_R = crate::FieldReader<u32>;
///Field `GMIDRL` writer - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
pub type GMIDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
    #[inline(always)]
    pub fn gmidrl(&self) -> GMIDRL_R {
        GMIDRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
    #[inline(always)]
    pub fn gmidrl(&mut self) -> GMIDRL_W<GMIDRL_SPEC> {
        GMIDRL_W::new(self, 0)
    }
}
/**grandmasterIdentity Field Setting Registers

You can [`read`](crate::Reg::read) this register and get [`gmidrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmidrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GMIDRL_SPEC;
impl crate::RegisterSpec for GMIDRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gmidrl::R`](R) reader structure
impl crate::Readable for GMIDRL_SPEC {}
///`write(|w| ..)` method takes [`gmidrl::W`](W) writer structure
impl crate::Writable for GMIDRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GMIDRL to value 0
impl crate::Resettable for GMIDRL_SPEC {}
