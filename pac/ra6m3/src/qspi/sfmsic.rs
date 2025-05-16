///Register `SFMSIC` reader
pub type R = crate::R<SFMSIC_SPEC>;
///Register `SFMSIC` writer
pub type W = crate::W<SFMSIC_SPEC>;
///Field `SFMCIC` reader - Serial ROM instruction code to substitute
pub type SFMCIC_R = crate::FieldReader;
///Field `SFMCIC` writer - Serial ROM instruction code to substitute
pub type SFMCIC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Serial ROM instruction code to substitute
    #[inline(always)]
    pub fn sfmcic(&self) -> SFMCIC_R {
        SFMCIC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Serial ROM instruction code to substitute
    #[inline(always)]
    pub fn sfmcic(&mut self) -> SFMCIC_W<SFMSIC_SPEC> {
        SFMCIC_W::new(self, 0)
    }
}
/**Instruction Code Register

You can [`read`](crate::Reg::read) this register and get [`sfmsic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSIC_SPEC;
impl crate::RegisterSpec for SFMSIC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmsic::R`](R) reader structure
impl crate::Readable for SFMSIC_SPEC {}
///`write(|w| ..)` method takes [`sfmsic::W`](W) writer structure
impl crate::Writable for SFMSIC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSIC to value 0
impl crate::Resettable for SFMSIC_SPEC {}
