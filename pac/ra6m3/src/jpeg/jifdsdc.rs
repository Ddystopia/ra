///Register `JIFDSDC` reader
pub type R = crate::R<JIFDSDC_SPEC>;
///Register `JIFDSDC` writer
pub type W = crate::W<JIFDSDC_SPEC>;
///Field `JDATAS` reader - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0.
pub type JDATAS_R = crate::FieldReader<u16>;
///Field `JDATAS` writer - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0.
pub type JDATAS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn jdatas(&self) -> JDATAS_R {
        JDATAS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn jdatas(&mut self) -> JDATAS_W<JIFDSDC_SPEC> {
        JDATAS_W::new(self, 0)
    }
}
/**JPEG Interface Decompression Source Data Count Register

You can [`read`](crate::Reg::read) this register and get [`jifdsdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdsdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDSDC_SPEC;
impl crate::RegisterSpec for JIFDSDC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifdsdc::R`](R) reader structure
impl crate::Readable for JIFDSDC_SPEC {}
///`write(|w| ..)` method takes [`jifdsdc::W`](W) writer structure
impl crate::Writable for JIFDSDC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDSDC to value 0xfff8_fff8
impl crate::Resettable for JIFDSDC_SPEC {
    const RESET_VALUE: u32 = 0xfff8_fff8;
}
