///Register `OUT_BRIGHT2` reader
pub type R = crate::R<OUT_BRIGHT2_SPEC>;
///Register `OUT_BRIGHT2` writer
pub type W = crate::W<OUT_BRIGHT2_SPEC>;
///Field `BRTR` reader - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer
pub type BRTR_R = crate::FieldReader<u16>;
///Field `BRTR` writer - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer
pub type BRTR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `BRTB` reader - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer
pub type BRTB_R = crate::FieldReader<u16>;
///Field `BRTB` writer - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer
pub type BRTB_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer
    #[inline(always)]
    pub fn brtr(&self) -> BRTR_R {
        BRTR_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer
    #[inline(always)]
    pub fn brtb(&self) -> BRTB_R {
        BRTB_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer
    #[inline(always)]
    pub fn brtr(&mut self) -> BRTR_W<OUT_BRIGHT2_SPEC> {
        BRTR_W::new(self, 0)
    }
    ///Bits 16:25 - Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer
    #[inline(always)]
    pub fn brtb(&mut self) -> BRTB_W<OUT_BRIGHT2_SPEC> {
        BRTB_W::new(self, 16)
    }
}
/**Output Control Block Brightness Correction Register 2

You can [`read`](crate::Reg::read) this register and get [`out_bright2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_bright2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_BRIGHT2_SPEC;
impl crate::RegisterSpec for OUT_BRIGHT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_bright2::R`](R) reader structure
impl crate::Readable for OUT_BRIGHT2_SPEC {}
///`write(|w| ..)` method takes [`out_bright2::W`](W) writer structure
impl crate::Writable for OUT_BRIGHT2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUT_BRIGHT2 to value 0
impl crate::Resettable for OUT_BRIGHT2_SPEC {}
