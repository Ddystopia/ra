///Register `ICBRH` reader
pub type R = crate::R<ICBRH_SPEC>;
///Register `ICBRH` writer
pub type W = crate::W<ICBRH_SPEC>;
///Field `BRH` reader - Bit Rate High-Level Period(High-level period of SCL clock)
pub type BRH_R = crate::FieldReader;
///Field `BRH` writer - Bit Rate High-Level Period(High-level period of SCL clock)
pub type BRH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Bit Rate High-Level Period(High-level period of SCL clock)
    #[inline(always)]
    pub fn brh(&self) -> BRH_R {
        BRH_R::new(self.bits & 0x1f)
    }
}
impl W {
    ///Bits 0:4 - Bit Rate High-Level Period(High-level period of SCL clock)
    #[inline(always)]
    pub fn brh(&mut self) -> BRH_W<ICBRH_SPEC> {
        BRH_W::new(self, 0)
    }
}
/**I2C Bus Bit Rate High-Level Register

You can [`read`](crate::Reg::read) this register and get [`icbrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICBRH_SPEC;
impl crate::RegisterSpec for ICBRH_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icbrh::R`](R) reader structure
impl crate::Readable for ICBRH_SPEC {}
///`write(|w| ..)` method takes [`icbrh::W`](W) writer structure
impl crate::Writable for ICBRH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICBRH to value 0xff
impl crate::Resettable for ICBRH_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
