///Register `OUT_BRIGHT1` reader
pub type R = crate::R<OUT_BRIGHT1_SPEC>;
///Register `OUT_BRIGHT1` writer
pub type W = crate::W<OUT_BRIGHT1_SPEC>;
///Field `BRTG` reader - Brightness (DC) adjustment of G signalUnsigned; 10 bits; +512 with offset; integer
pub type BRTG_R = crate::FieldReader<u16>;
///Field `BRTG` writer - Brightness (DC) adjustment of G signalUnsigned; 10 bits; +512 with offset; integer
pub type BRTG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Brightness (DC) adjustment of G signalUnsigned; 10 bits; +512 with offset; integer
    #[inline(always)]
    pub fn brtg(&self) -> BRTG_R {
        BRTG_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Brightness (DC) adjustment of G signalUnsigned; 10 bits; +512 with offset; integer
    #[inline(always)]
    pub fn brtg(&mut self) -> BRTG_W<OUT_BRIGHT1_SPEC> {
        BRTG_W::new(self, 0)
    }
}
/**Output Control Block Brightness Correction Register 1

You can [`read`](crate::Reg::read) this register and get [`out_bright1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_bright1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_BRIGHT1_SPEC;
impl crate::RegisterSpec for OUT_BRIGHT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_bright1::R`](R) reader structure
impl crate::Readable for OUT_BRIGHT1_SPEC {}
///`write(|w| ..)` method takes [`out_bright1::W`](W) writer structure
impl crate::Writable for OUT_BRIGHT1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUT_BRIGHT1 to value 0
impl crate::Resettable for OUT_BRIGHT1_SPEC {}
