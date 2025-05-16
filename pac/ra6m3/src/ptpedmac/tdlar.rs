///Register `TDLAR` reader
pub type R = crate::R<TDLAR_SPEC>;
///Register `TDLAR` writer
pub type W = crate::W<TDLAR_SPEC>;
///Field `TDLAR` reader - The start address of the transmit descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
pub type TDLAR_R = crate::FieldReader<u32>;
///Field `TDLAR` writer - The start address of the transmit descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
pub type TDLAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The start address of the transmit descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
    #[inline(always)]
    pub fn tdlar(&self) -> TDLAR_R {
        TDLAR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - The start address of the transmit descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
    #[inline(always)]
    pub fn tdlar(&mut self) -> TDLAR_W<TDLAR_SPEC> {
        TDLAR_W::new(self, 0)
    }
}
/**Transmit Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`tdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TDLAR_SPEC;
impl crate::RegisterSpec for TDLAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tdlar::R`](R) reader structure
impl crate::Readable for TDLAR_SPEC {}
///`write(|w| ..)` method takes [`tdlar::W`](W) writer structure
impl crate::Writable for TDLAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDLAR to value 0
impl crate::Resettable for TDLAR_SPEC {}
