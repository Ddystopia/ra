///Register `ICBRL` reader
pub type R = crate::R<ICBRL_SPEC>;
///Register `ICBRL` writer
pub type W = crate::W<ICBRL_SPEC>;
///Field `BRL` reader - Bit Rate Low-Level Period(Low-level period of SCL clock)
pub type BRL_R = crate::FieldReader;
///Field `BRL` writer - Bit Rate Low-Level Period(Low-level period of SCL clock)
pub type BRL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Bit Rate Low-Level Period(Low-level period of SCL clock)
    #[inline(always)]
    pub fn brl(&self) -> BRL_R {
        BRL_R::new(self.bits & 0x1f)
    }
}
impl W {
    ///Bits 0:4 - Bit Rate Low-Level Period(Low-level period of SCL clock)
    #[inline(always)]
    pub fn brl(&mut self) -> BRL_W<ICBRL_SPEC> {
        BRL_W::new(self, 0)
    }
}
/**I2C Bus Bit Rate Low-Level Register

You can [`read`](crate::Reg::read) this register and get [`icbrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICBRL_SPEC;
impl crate::RegisterSpec for ICBRL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icbrl::R`](R) reader structure
impl crate::Readable for ICBRL_SPEC {}
///`write(|w| ..)` method takes [`icbrl::W`](W) writer structure
impl crate::Writable for ICBRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICBRL to value 0xff
impl crate::Resettable for ICBRL_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
