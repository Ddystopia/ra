///Register `LCIVRL` reader
pub type R = crate::R<LCIVRL_SPEC>;
///Register `LCIVRL` writer
pub type W = crate::W<LCIVRL_SPEC>;
///Field `LCIVRL` reader - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds.
pub type LCIVRL_R = crate::FieldReader<u32>;
///Field `LCIVRL` writer - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds.
pub type LCIVRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds.
    #[inline(always)]
    pub fn lcivrl(&self) -> LCIVRL_R {
        LCIVRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds.
    #[inline(always)]
    pub fn lcivrl(&mut self) -> LCIVRL_W<LCIVRL_SPEC> {
        LCIVRL_W::new(self, 0)
    }
}
/**Local Time Counter Initial Value Registers

You can [`read`](crate::Reg::read) this register and get [`lcivrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCIVRL_SPEC;
impl crate::RegisterSpec for LCIVRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lcivrl::R`](R) reader structure
impl crate::Readable for LCIVRL_SPEC {}
///`write(|w| ..)` method takes [`lcivrl::W`](W) writer structure
impl crate::Writable for LCIVRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCIVRL to value 0
impl crate::Resettable for LCIVRL_SPEC {}
