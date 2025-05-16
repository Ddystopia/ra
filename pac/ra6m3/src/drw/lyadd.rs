///Register `L%sYADD` writer
pub type W = crate::W<LYADD_SPEC>;
///Field `LYADD` writer - Y-axis increment
pub type LYADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Y-axis increment
    #[inline(always)]
    pub fn lyadd(&mut self) -> LYADD_W<LYADD_SPEC> {
        LYADD_W::new(self, 0)
    }
}
/**Limiter %s Y-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lyadd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LYADD_SPEC;
impl crate::RegisterSpec for LYADD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lyadd::W`](W) writer structure
impl crate::Writable for LYADD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L%sYADD to value 0
impl crate::Resettable for LYADD_SPEC {}
