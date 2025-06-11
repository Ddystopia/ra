///Register `CSSR` reader
pub type R = crate::R<CSSR_SPEC>;
///Register `CSSR` writer
pub type W = crate::W<CSSR_SPEC>;
///Field `CSSR` reader - When the value for the channel search is input, the channel number is output to MSSR.
pub type CSSR_R = crate::FieldReader;
///Field `CSSR` writer - When the value for the channel search is input, the channel number is output to MSSR.
pub type CSSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR.
    #[inline(always)]
    pub fn cssr(&self) -> CSSR_R {
        CSSR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR.
    #[inline(always)]
    pub fn cssr(&mut self) -> CSSR_W<CSSR_SPEC> {
        CSSR_W::new(self, 0)
    }
}
/**Channel Search Support Register

You can [`read`](crate::Reg::read) this register and get [`cssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSSR_SPEC;
impl crate::RegisterSpec for CSSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cssr::R`](R) reader structure
impl crate::Readable for CSSR_SPEC {}
///`write(|w| ..)` method takes [`cssr::W`](W) writer structure
impl crate::Writable for CSSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSSR to value 0
impl crate::Resettable for CSSR_SPEC {}
