///Register `GTDBD` reader
pub type R = crate::R<GTDBD_SPEC>;
///Register `GTDBD` writer
pub type W = crate::W<GTDBD_SPEC>;
///Field `GTDBD` reader - Dead Time Buffer Register D
pub type GTDBD_R = crate::FieldReader<u32>;
///Field `GTDBD` writer - Dead Time Buffer Register D
pub type GTDBD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Dead Time Buffer Register D
    #[inline(always)]
    pub fn gtdbd(&self) -> GTDBD_R {
        GTDBD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Dead Time Buffer Register D
    #[inline(always)]
    pub fn gtdbd(&mut self) -> GTDBD_W<GTDBD_SPEC> {
        GTDBD_W::new(self, 0)
    }
}
/**General PWM Timer Dead Time Buffer Register D

You can [`read`](crate::Reg::read) this register and get [`gtdbd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdbd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDBD_SPEC;
impl crate::RegisterSpec for GTDBD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdbd::R`](R) reader structure
impl crate::Readable for GTDBD_SPEC {}
///`write(|w| ..)` method takes [`gtdbd::W`](W) writer structure
impl crate::Writable for GTDBD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDBD to value 0xffff_ffff
impl crate::Resettable for GTDBD_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
