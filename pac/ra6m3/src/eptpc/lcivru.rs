///Register `LCIVRU` reader
pub type R = crate::R<LCIVRU_SPEC>;
///Register `LCIVRU` writer
pub type W = crate::W<LCIVRU_SPEC>;
///Field `LCIVRU` reader - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter.
pub type LCIVRU_R = crate::FieldReader<u16>;
///Field `LCIVRU` writer - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter.
pub type LCIVRU_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter.
    #[inline(always)]
    pub fn lcivru(&self) -> LCIVRU_R {
        LCIVRU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter.
    #[inline(always)]
    pub fn lcivru(&mut self) -> LCIVRU_W<LCIVRU_SPEC> {
        LCIVRU_W::new(self, 0)
    }
}
/**Local Time Counter Initial Value Registers

You can [`read`](crate::Reg::read) this register and get [`lcivru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCIVRU_SPEC;
impl crate::RegisterSpec for LCIVRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lcivru::R`](R) reader structure
impl crate::Readable for LCIVRU_SPEC {}
///`write(|w| ..)` method takes [`lcivru::W`](W) writer structure
impl crate::Writable for LCIVRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCIVRU to value 0
impl crate::Resettable for LCIVRU_SPEC {}
