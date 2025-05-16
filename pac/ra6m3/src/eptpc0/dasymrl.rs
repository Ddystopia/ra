///Register `DASYMRL` reader
pub type R = crate::R<DASYMRL_SPEC>;
///Register `DASYMRL` writer
pub type W = crate::W<DASYMRL_SPEC>;
///Field `DASYMRL` reader - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value.
pub type DASYMRL_R = crate::FieldReader<u32>;
///Field `DASYMRL` writer - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value.
pub type DASYMRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value.
    #[inline(always)]
    pub fn dasymrl(&self) -> DASYMRL_R {
        DASYMRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the asymmetric delay value.
    #[inline(always)]
    pub fn dasymrl(&mut self) -> DASYMRL_W<DASYMRL_SPEC> {
        DASYMRL_W::new(self, 0)
    }
}
/**Asymmetric Delay Setting Registers

You can [`read`](crate::Reg::read) this register and get [`dasymrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dasymrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DASYMRL_SPEC;
impl crate::RegisterSpec for DASYMRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dasymrl::R`](R) reader structure
impl crate::Readable for DASYMRL_SPEC {}
///`write(|w| ..)` method takes [`dasymrl::W`](W) writer structure
impl crate::Writable for DASYMRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DASYMRL to value 0
impl crate::Resettable for DASYMRL_SPEC {}
