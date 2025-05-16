///Register `GR%s_FLMRD` reader
pub type R = crate::R<GR_FLMRD_SPEC>;
///Register `GR%s_FLMRD` writer
pub type W = crate::W<GR_FLMRD_SPEC>;
/**Graphics data (frame buffer data) read enable.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RENB_A {
    ///1: Enables reading.
    _1 = 1,
    ///0: Disables reading.
    _0 = 0,
}
impl From<RENB_A> for bool {
    #[inline(always)]
    fn from(variant: RENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RENB` reader - Graphics data (frame buffer data) read enable.
pub type RENB_R = crate::BitReader<RENB_A>;
impl RENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RENB_A {
        match self.bits {
            true => RENB_A::_1,
            false => RENB_A::_0,
        }
    }
    ///Enables reading.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RENB_A::_1
    }
    ///Disables reading.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RENB_A::_0
    }
}
///Field `RENB` writer - Graphics data (frame buffer data) read enable.
pub type RENB_W<'a, REG> = crate::BitWriter<'a, REG, RENB_A>;
impl<'a, REG> RENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables reading.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RENB_A::_1)
    }
    ///Disables reading.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RENB_A::_0)
    }
}
impl R {
    ///Bit 0 - Graphics data (frame buffer data) read enable.
    #[inline(always)]
    pub fn renb(&self) -> RENB_R {
        RENB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Graphics data (frame buffer data) read enable.
    #[inline(always)]
    pub fn renb(&mut self) -> RENB_W<GR_FLMRD_SPEC> {
        RENB_W::new(self, 0)
    }
}
/**Graphics %s Frame Buffer Read Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_flmrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flmrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_FLMRD_SPEC;
impl crate::RegisterSpec for GR_FLMRD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_flmrd::R`](R) reader structure
impl crate::Readable for GR_FLMRD_SPEC {}
///`write(|w| ..)` method takes [`gr_flmrd::W`](W) writer structure
impl crate::Writable for GR_FLMRD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_FLMRD to value 0
impl crate::Resettable for GR_FLMRD_SPEC {}
