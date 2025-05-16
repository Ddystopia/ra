///Register `SYLLCCTLR` reader
pub type R = crate::R<SYLLCCTLR_SPEC>;
///Register `SYLLCCTLR` writer
pub type W = crate::W<SYLLCCTLR_SPEC>;
///Field `CTL` reader - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames.
pub type CTL_R = crate::FieldReader;
///Field `CTL` writer - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames.
pub type CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames.
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames.
    #[inline(always)]
    pub fn ctl(&mut self) -> CTL_W<SYLLCCTLR_SPEC> {
        CTL_W::new(self, 0)
    }
}
/**SYNFP LLC-CTL Value Register

You can [`read`](crate::Reg::read) this register and get [`syllcctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syllcctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYLLCCTLR_SPEC;
impl crate::RegisterSpec for SYLLCCTLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syllcctlr::R`](R) reader structure
impl crate::Readable for SYLLCCTLR_SPEC {}
///`write(|w| ..)` method takes [`syllcctlr::W`](W) writer structure
impl crate::Writable for SYLLCCTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYLLCCTLR to value 0x03
impl crate::Resettable for SYLLCCTLR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
