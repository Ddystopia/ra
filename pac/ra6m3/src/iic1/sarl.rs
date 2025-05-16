///Register `SARL%s` reader
pub type R = crate::R<SARL_SPEC>;
///Register `SARL%s` writer
pub type W = crate::W<SARL_SPEC>;
///Field `SVA` reader - A slave address is set.7-Bit Address = SVA\[7:1\] 10-Bit Address = { SVA9,SVA8,SVA\[7:0\] }
pub type SVA_R = crate::FieldReader;
///Field `SVA` writer - A slave address is set.7-Bit Address = SVA\[7:1\] 10-Bit Address = { SVA9,SVA8,SVA\[7:0\] }
pub type SVA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - A slave address is set.7-Bit Address = SVA\[7:1\] 10-Bit Address = { SVA9,SVA8,SVA\[7:0\] }
    #[inline(always)]
    pub fn sva(&self) -> SVA_R {
        SVA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - A slave address is set.7-Bit Address = SVA\[7:1\] 10-Bit Address = { SVA9,SVA8,SVA\[7:0\] }
    #[inline(always)]
    pub fn sva(&mut self) -> SVA_W<SARL_SPEC> {
        SVA_W::new(self, 0)
    }
}
/**Slave Address Register L%s

You can [`read`](crate::Reg::read) this register and get [`sarl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sarl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SARL_SPEC;
impl crate::RegisterSpec for SARL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sarl::R`](R) reader structure
impl crate::Readable for SARL_SPEC {}
///`write(|w| ..)` method takes [`sarl::W`](W) writer structure
impl crate::Writable for SARL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SARL%s to value 0
impl crate::Resettable for SARL_SPEC {}
