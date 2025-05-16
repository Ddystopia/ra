///Register `VBTBKR[%s]` reader
pub type R = crate::R<VBTBKR_SPEC>;
///Register `VBTBKR[%s]` writer
pub type W = crate::W<VBTBKR_SPEC>;
///Field `VBTBKR` reader - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT.The value of this register is retained even when VCC is not powered but VBATT is powered.VBTBKR is initialized by VBATT selected voltage power-on-reset.
pub type VBTBKR_R = crate::FieldReader;
///Field `VBTBKR` writer - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT.The value of this register is retained even when VCC is not powered but VBATT is powered.VBTBKR is initialized by VBATT selected voltage power-on-reset.
pub type VBTBKR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT.The value of this register is retained even when VCC is not powered but VBATT is powered.VBTBKR is initialized by VBATT selected voltage power-on-reset.
    #[inline(always)]
    pub fn vbtbkr(&self) -> VBTBKR_R {
        VBTBKR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT.The value of this register is retained even when VCC is not powered but VBATT is powered.VBTBKR is initialized by VBATT selected voltage power-on-reset.
    #[inline(always)]
    pub fn vbtbkr(&mut self) -> VBTBKR_W<VBTBKR_SPEC> {
        VBTBKR_W::new(self, 0)
    }
}
/**VBATT Backup Register \[%s\]

You can [`read`](crate::Reg::read) this register and get [`vbtbkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtbkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTBKR_SPEC;
impl crate::RegisterSpec for VBTBKR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtbkr::R`](R) reader structure
impl crate::Readable for VBTBKR_SPEC {}
///`write(|w| ..)` method takes [`vbtbkr::W`](W) writer structure
impl crate::Writable for VBTBKR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTBKR[%s] to value 0
impl crate::Resettable for VBTBKR_SPEC {}
