///Register `MAHR` reader
pub type R = crate::R<MAHR_SPEC>;
///Register `MAHR` writer
pub type W = crate::W<MAHR_SPEC>;
///Field `MAHR` reader - MAC Address Upper Bit RegisterThe MAHR register sets the upper 32 bits (b47 to b16) of the 48-bit MAC address.
pub type MAHR_R = crate::FieldReader<u32>;
///Field `MAHR` writer - MAC Address Upper Bit RegisterThe MAHR register sets the upper 32 bits (b47 to b16) of the 48-bit MAC address.
pub type MAHR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC Address Upper Bit RegisterThe MAHR register sets the upper 32 bits (b47 to b16) of the 48-bit MAC address.
    #[inline(always)]
    pub fn mahr(&self) -> MAHR_R {
        MAHR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - MAC Address Upper Bit RegisterThe MAHR register sets the upper 32 bits (b47 to b16) of the 48-bit MAC address.
    #[inline(always)]
    pub fn mahr(&mut self) -> MAHR_W<MAHR_SPEC> {
        MAHR_W::new(self, 0)
    }
}
/**MAC Address Upper Bit Register

You can [`read`](crate::Reg::read) this register and get [`mahr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mahr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MAHR_SPEC;
impl crate::RegisterSpec for MAHR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mahr::R`](R) reader structure
impl crate::Readable for MAHR_SPEC {}
///`write(|w| ..)` method takes [`mahr::W`](W) writer structure
impl crate::Writable for MAHR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAHR to value 0
impl crate::Resettable for MAHR_SPEC {}
