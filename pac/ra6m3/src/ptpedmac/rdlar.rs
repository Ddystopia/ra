///Register `RDLAR` reader
pub type R = crate::R<RDLAR_SPEC>;
///Register `RDLAR` writer
pub type W = crate::W<RDLAR_SPEC>;
///Field `RDLAR` reader - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
pub type RDLAR_R = crate::FieldReader<u32>;
///Field `RDLAR` writer - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
pub type RDLAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
    #[inline(always)]
    pub fn rdlar(&self) -> RDLAR_R {
        RDLAR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\[1:0\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b
    #[inline(always)]
    pub fn rdlar(&mut self) -> RDLAR_W<RDLAR_SPEC> {
        RDLAR_W::new(self, 0)
    }
}
/**Receive Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`rdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDLAR_SPEC;
impl crate::RegisterSpec for RDLAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdlar::R`](R) reader structure
impl crate::Readable for RDLAR_SPEC {}
///`write(|w| ..)` method takes [`rdlar::W`](W) writer structure
impl crate::Writable for RDLAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDLAR to value 0
impl crate::Resettable for RDLAR_SPEC {}
