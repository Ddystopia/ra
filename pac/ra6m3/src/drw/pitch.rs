///Register `PITCH` writer
pub type W = crate::W<PITCH_SPEC>;
///Field `PITCH` writer - pitch of the framebuffer. A negative width can be used to render bottom-up instead of top-down
pub type PITCH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SSD` writer - Spanstore delay
pub type SSD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    ///Bits 0:15 - pitch of the framebuffer. A negative width can be used to render bottom-up instead of top-down
    #[inline(always)]
    pub fn pitch(&mut self) -> PITCH_W<PITCH_SPEC> {
        PITCH_W::new(self, 0)
    }
    ///Bits 16:31 - Spanstore delay
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W<PITCH_SPEC> {
        SSD_W::new(self, 16)
    }
}
/**Framebuffer Pitch And Spanstore Delay Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pitch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PITCH_SPEC;
impl crate::RegisterSpec for PITCH_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pitch::W`](W) writer structure
impl crate::Writable for PITCH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PITCH to value 0
impl crate::Resettable for PITCH_SPEC {}
