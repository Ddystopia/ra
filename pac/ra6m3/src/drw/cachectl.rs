///Register `CACHECTL` writer
pub type W = crate::W<CACHECTL_SPEC>;
/**Framebuffer cache enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENABLEFX_A {
    ///0: disable the framebuffer cache
    _0 = 0,
    ///1: enable the framebuffer cache
    _1 = 1,
}
impl From<CENABLEFX_A> for bool {
    #[inline(always)]
    fn from(variant: CENABLEFX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CENABLEFX` writer - Framebuffer cache enable
pub type CENABLEFX_W<'a, REG> = crate::BitWriter<'a, REG, CENABLEFX_A>;
impl<'a, REG> CENABLEFX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disable the framebuffer cache
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CENABLEFX_A::_0)
    }
    ///enable the framebuffer cache
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CENABLEFX_A::_1)
    }
}
/**Flush framebuffer cache

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFLUSHFX_A {
    ///0: do not flush the framebuffer cache
    _0 = 0,
    ///1: flush the framebuffer cache
    _1 = 1,
}
impl From<CFLUSHFX_A> for bool {
    #[inline(always)]
    fn from(variant: CFLUSHFX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CFLUSHFX` writer - Flush framebuffer cache
pub type CFLUSHFX_W<'a, REG> = crate::BitWriter<'a, REG, CFLUSHFX_A>;
impl<'a, REG> CFLUSHFX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///do not flush the framebuffer cache
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CFLUSHFX_A::_0)
    }
    ///flush the framebuffer cache
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CFLUSHFX_A::_1)
    }
}
/**Texture cache enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENABLETX_A {
    ///0: disable the texture cache
    _0 = 0,
    ///1: enable the texture cache
    _1 = 1,
}
impl From<CENABLETX_A> for bool {
    #[inline(always)]
    fn from(variant: CENABLETX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CENABLETX` writer - Texture cache enable
pub type CENABLETX_W<'a, REG> = crate::BitWriter<'a, REG, CENABLETX_A>;
impl<'a, REG> CENABLETX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disable the texture cache
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CENABLETX_A::_0)
    }
    ///enable the texture cache
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CENABLETX_A::_1)
    }
}
/**Flush texture cache

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFLUSHTX_A {
    ///0: do not flush the texture cache
    _0 = 0,
    ///1: flush the texture cache
    _1 = 1,
}
impl From<CFLUSHTX_A> for bool {
    #[inline(always)]
    fn from(variant: CFLUSHTX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CFLUSHTX` writer - Flush texture cache
pub type CFLUSHTX_W<'a, REG> = crate::BitWriter<'a, REG, CFLUSHTX_A>;
impl<'a, REG> CFLUSHTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///do not flush the texture cache
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CFLUSHTX_A::_0)
    }
    ///flush the texture cache
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CFLUSHTX_A::_1)
    }
}
impl W {
    ///Bit 0 - Framebuffer cache enable
    #[inline(always)]
    pub fn cenablefx(&mut self) -> CENABLEFX_W<CACHECTL_SPEC> {
        CENABLEFX_W::new(self, 0)
    }
    ///Bit 1 - Flush framebuffer cache
    #[inline(always)]
    pub fn cflushfx(&mut self) -> CFLUSHFX_W<CACHECTL_SPEC> {
        CFLUSHFX_W::new(self, 1)
    }
    ///Bit 2 - Texture cache enable
    #[inline(always)]
    pub fn cenabletx(&mut self) -> CENABLETX_W<CACHECTL_SPEC> {
        CENABLETX_W::new(self, 2)
    }
    ///Bit 3 - Flush texture cache
    #[inline(always)]
    pub fn cflushtx(&mut self) -> CFLUSHTX_W<CACHECTL_SPEC> {
        CFLUSHTX_W::new(self, 3)
    }
}
/**Cache Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachectl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHECTL_SPEC;
impl crate::RegisterSpec for CACHECTL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cachectl::W`](W) writer structure
impl crate::Writable for CACHECTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CACHECTL to value 0
impl crate::Resettable for CACHECTL_SPEC {}
