///Register `TDRHL` reader
pub type R = crate::R<TDRHL_SPEC>;
///Register `TDRHL` writer
pub type W = crate::W<TDRHL_SPEC>;
///Field `TDRHL` reader - TDRHL is a 16-bit register that stores transmit data.
pub type TDRHL_R = crate::FieldReader<u16>;
///Field `TDRHL` writer - TDRHL is a 16-bit register that stores transmit data.
pub type TDRHL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - TDRHL is a 16-bit register that stores transmit data.
    #[inline(always)]
    pub fn tdrhl(&self) -> TDRHL_R {
        TDRHL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - TDRHL is a 16-bit register that stores transmit data.
    #[inline(always)]
    pub fn tdrhl(&mut self) -> TDRHL_W<TDRHL_SPEC> {
        TDRHL_W::new(self, 0)
    }
}
/**Transmit 9-bit Data Register

You can [`read`](crate::Reg::read) this register and get [`tdrhl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TDRHL_SPEC;
impl crate::RegisterSpec for TDRHL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`tdrhl::R`](R) reader structure
impl crate::Readable for TDRHL_SPEC {}
///`write(|w| ..)` method takes [`tdrhl::W`](W) writer structure
impl crate::Writable for TDRHL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDRHL to value 0xffff
impl crate::Resettable for TDRHL_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
