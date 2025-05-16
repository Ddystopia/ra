///Register `COLKEY` writer
pub type W = crate::W<COLKEY_SPEC>;
///Field `COLKEYB` writer - Blue channel of color key
pub type COLKEYB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLKEYG` writer - Green channel of color key
pub type COLKEYG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLKEYR` writer - Red channel of color key
pub type COLKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Blue channel of color key
    #[inline(always)]
    pub fn colkeyb(&mut self) -> COLKEYB_W<COLKEY_SPEC> {
        COLKEYB_W::new(self, 0)
    }
    ///Bits 8:15 - Green channel of color key
    #[inline(always)]
    pub fn colkeyg(&mut self) -> COLKEYG_W<COLKEY_SPEC> {
        COLKEYG_W::new(self, 8)
    }
    ///Bits 16:23 - Red channel of color key
    #[inline(always)]
    pub fn colkeyr(&mut self) -> COLKEYR_W<COLKEY_SPEC> {
        COLKEYR_W::new(self, 16)
    }
}
/**Color Key Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`colkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COLKEY_SPEC;
impl crate::RegisterSpec for COLKEY_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`colkey::W`](W) writer structure
impl crate::Writable for COLKEY_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COLKEY to value 0
impl crate::Resettable for COLKEY_SPEC {}
