///Register `SD_DMAEN` reader
pub type R = crate::R<SD_DMAEN_SPEC>;
///Register `SD_DMAEN` writer
pub type W = crate::W<SD_DMAEN_SPEC>;
/**SD_BUF Read/Write DMA Transfer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    ///0: The SD_BUF read/write DMA transfer is disabled.
    _0 = 0,
    ///1: The SD_BUF read/write DMA transfer is enabled.
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - SD_BUF Read/Write DMA Transfer
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    ///The SD_BUF read/write DMA transfer is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    ///The SD_BUF read/write DMA transfer is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
///Field `DMAEN` writer - SD_BUF Read/Write DMA Transfer
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN_A>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SD_BUF read/write DMA transfer is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::_0)
    }
    ///The SD_BUF read/write DMA transfer is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::_1)
    }
}
impl R {
    ///Bit 1 - SD_BUF Read/Write DMA Transfer
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - SD_BUF Read/Write DMA Transfer
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<SD_DMAEN_SPEC> {
        DMAEN_W::new(self, 1)
    }
}
/**DMA Mode Enable Register

You can [`read`](crate::Reg::read) this register and get [`sd_dmaen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dmaen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_DMAEN_SPEC;
impl crate::RegisterSpec for SD_DMAEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_dmaen::R`](R) reader structure
impl crate::Readable for SD_DMAEN_SPEC {}
///`write(|w| ..)` method takes [`sd_dmaen::W`](W) writer structure
impl crate::Writable for SD_DMAEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_DMAEN to value 0x1010
impl crate::Resettable for SD_DMAEN_SPEC {
    const RESET_VALUE: u32 = 0x1010;
}
