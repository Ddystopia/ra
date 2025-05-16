///Register `TEXMASK` writer
pub type W = crate::W<TEXMASK_SPEC>;
///Field `TEXUMASK` writer - U maskSet TEXUMASK\[10:0\] = texture_width -1In texture wrapping mode (CONTROL2.TEXTURECLAMPX = 0): texture_width must be a power of 2.In texture clamping mode (CONTROL2.TEXTURECLAMPX = 1):all widths up to 2048 are allowed.
pub type TEXUMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `TEXVMASK` writer - V maskSet TEXVMASK\[20:0\] = TEXPITCH * (texture_height - 1).In texture wrapping mode (CONTROL2.TEXTURECLAMPY = 0): texture_height must be a power of 2In texture clamping mode (CONTROL2.TEXTURECLAMPY = 1):all heights up to 1024 are allowed.
pub type TEXVMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl W {
    ///Bits 0:10 - U maskSet TEXUMASK\[10:0\] = texture_width -1In texture wrapping mode (CONTROL2.TEXTURECLAMPX = 0): texture_width must be a power of 2.In texture clamping mode (CONTROL2.TEXTURECLAMPX = 1):all widths up to 2048 are allowed.
    #[inline(always)]
    pub fn texumask(&mut self) -> TEXUMASK_W<TEXMASK_SPEC> {
        TEXUMASK_W::new(self, 0)
    }
    ///Bits 11:31 - V maskSet TEXVMASK\[20:0\] = TEXPITCH * (texture_height - 1).In texture wrapping mode (CONTROL2.TEXTURECLAMPY = 0): texture_height must be a power of 2In texture clamping mode (CONTROL2.TEXTURECLAMPY = 1):all heights up to 1024 are allowed.
    #[inline(always)]
    pub fn texvmask(&mut self) -> TEXVMASK_W<TEXMASK_SPEC> {
        TEXVMASK_W::new(self, 11)
    }
}
/**Texture Size or Texture Address Mask Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texmask::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXMASK_SPEC;
impl crate::RegisterSpec for TEXMASK_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`texmask::W`](W) writer structure
impl crate::Writable for TEXMASK_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEXMASK to value 0
impl crate::Resettable for TEXMASK_SPEC {}
