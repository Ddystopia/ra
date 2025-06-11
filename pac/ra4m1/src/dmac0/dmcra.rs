///Register `DMCRA` reader
pub type R = crate::R<DMCRA_SPEC>;
///Register `DMCRA` writer
pub type W = crate::W<DMCRA_SPEC>;
///Field `DMCRAL` reader - Lower bits of transfer count
pub type DMCRAL_R = crate::FieldReader<u16>;
///Field `DMCRAL` writer - Lower bits of transfer count
pub type DMCRAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DMCRAH` reader - Upper bits of transfer count
pub type DMCRAH_R = crate::FieldReader<u16>;
///Field `DMCRAH` writer - Upper bits of transfer count
pub type DMCRAH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:15 - Lower bits of transfer count
    #[inline(always)]
    pub fn dmcral(&self) -> DMCRAL_R {
        DMCRAL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:25 - Upper bits of transfer count
    #[inline(always)]
    pub fn dmcrah(&self) -> DMCRAH_R {
        DMCRAH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Lower bits of transfer count
    #[inline(always)]
    pub fn dmcral(&mut self) -> DMCRAL_W<DMCRA_SPEC> {
        DMCRAL_W::new(self, 0)
    }
    ///Bits 16:25 - Upper bits of transfer count
    #[inline(always)]
    pub fn dmcrah(&mut self) -> DMCRAH_W<DMCRA_SPEC> {
        DMCRAH_W::new(self, 16)
    }
}
/**DMA Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMCRA_SPEC;
impl crate::RegisterSpec for DMCRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dmcra::R`](R) reader structure
impl crate::Readable for DMCRA_SPEC {}
///`write(|w| ..)` method takes [`dmcra::W`](W) writer structure
impl crate::Writable for DMCRA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMCRA to value 0
impl crate::Resettable for DMCRA_SPEC {}
