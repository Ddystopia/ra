///Register `BRR` reader
pub type R = crate::R<BRR_SPEC>;
///Register `BRR` writer
pub type W = crate::W<BRR_SPEC>;
///Field `BRR` reader - BRR is an 8-bit register that adjusts the bit rate.
pub type BRR_R = crate::FieldReader;
///Field `BRR` writer - BRR is an 8-bit register that adjusts the bit rate.
pub type BRR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - BRR is an 8-bit register that adjusts the bit rate.
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - BRR is an 8-bit register that adjusts the bit rate.
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W<BRR_SPEC> {
        BRR_W::new(self, 0)
    }
}
/**Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRR_SPEC {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0xff
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
