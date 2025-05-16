///Register `DASYMRU` reader
pub type R = crate::R<DASYMRU_SPEC>;
///Register `DASYMRU` writer
pub type W = crate::W<DASYMRU_SPEC>;
///Field `DASYMRU` reader - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value.
pub type DASYMRU_R = crate::FieldReader<u16>;
///Field `DASYMRU` writer - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value.
pub type DASYMRU_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value.
    #[inline(always)]
    pub fn dasymru(&self) -> DASYMRU_R {
        DASYMRU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the asymmetric delay value.
    #[inline(always)]
    pub fn dasymru(&mut self) -> DASYMRU_W<DASYMRU_SPEC> {
        DASYMRU_W::new(self, 0)
    }
}
/**Asymmetric Delay Setting Registers

You can [`read`](crate::Reg::read) this register and get [`dasymru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dasymru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DASYMRU_SPEC;
impl crate::RegisterSpec for DASYMRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dasymru::R`](R) reader structure
impl crate::Readable for DASYMRU_SPEC {}
///`write(|w| ..)` method takes [`dasymru::W`](W) writer structure
impl crate::Writable for DASYMRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DASYMRU to value 0
impl crate::Resettable for DASYMRU_SPEC {}
