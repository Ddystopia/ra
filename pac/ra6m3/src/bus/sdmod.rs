///Register `SDMOD` reader
pub type R = crate::R<SDMOD_SPEC>;
///Register `SDMOD` writer
pub type W = crate::W<SDMOD_SPEC>;
///Field `MR` reader - Mode Register SettingWriting to these bits: Mode register set command is issued.
pub type MR_R = crate::FieldReader<u16>;
///Field `MR` writer - Mode Register SettingWriting to these bits: Mode register set command is issued.
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Mode Register SettingWriting to these bits: Mode register set command is issued.
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(self.bits & 0x7fff)
    }
}
impl W {
    ///Bits 0:14 - Mode Register SettingWriting to these bits: Mode register set command is issued.
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W<SDMOD_SPEC> {
        MR_W::new(self, 0)
    }
}
/**SDRAM Mode Register

You can [`read`](crate::Reg::read) this register and get [`sdmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDMOD_SPEC;
impl crate::RegisterSpec for SDMOD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`sdmod::R`](R) reader structure
impl crate::Readable for SDMOD_SPEC {}
///`write(|w| ..)` method takes [`sdmod::W`](W) writer structure
impl crate::Writable for SDMOD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDMOD to value 0
impl crate::Resettable for SDMOD_SPEC {}
