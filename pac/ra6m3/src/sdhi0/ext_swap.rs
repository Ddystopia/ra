///Register `EXT_SWAP` reader
pub type R = crate::R<EXT_SWAP_SPEC>;
///Register `EXT_SWAP` writer
pub type W = crate::W<EXT_SWAP_SPEC>;
/**SD_BUF0 Swap Write

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWSWP_A {
    ///0: The current data are written without swapping.
    _0 = 0,
    ///1: Swapping of the positions of the higher- and lower-order bytes of data for writing proceeds.
    _1 = 1,
}
impl From<BWSWP_A> for bool {
    #[inline(always)]
    fn from(variant: BWSWP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BWSWP` reader - SD_BUF0 Swap Write
pub type BWSWP_R = crate::BitReader<BWSWP_A>;
impl BWSWP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BWSWP_A {
        match self.bits {
            false => BWSWP_A::_0,
            true => BWSWP_A::_1,
        }
    }
    ///The current data are written without swapping.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWSWP_A::_0
    }
    ///Swapping of the positions of the higher- and lower-order bytes of data for writing proceeds.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWSWP_A::_1
    }
}
///Field `BWSWP` writer - SD_BUF0 Swap Write
pub type BWSWP_W<'a, REG> = crate::BitWriter<'a, REG, BWSWP_A>;
impl<'a, REG> BWSWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The current data are written without swapping.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BWSWP_A::_0)
    }
    ///Swapping of the positions of the higher- and lower-order bytes of data for writing proceeds.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BWSWP_A::_1)
    }
}
/**SD_BUF0 Swap Read

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSWP_A {
    ///0: The current data are read without swapping.
    _0 = 0,
    ///1: Swapping of the positions of the higher- and lower-order bytes of data for reading proceeds.
    _1 = 1,
}
impl From<BRSWP_A> for bool {
    #[inline(always)]
    fn from(variant: BRSWP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRSWP` reader - SD_BUF0 Swap Read
pub type BRSWP_R = crate::BitReader<BRSWP_A>;
impl BRSWP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRSWP_A {
        match self.bits {
            false => BRSWP_A::_0,
            true => BRSWP_A::_1,
        }
    }
    ///The current data are read without swapping.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRSWP_A::_0
    }
    ///Swapping of the positions of the higher- and lower-order bytes of data for reading proceeds.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRSWP_A::_1
    }
}
///Field `BRSWP` writer - SD_BUF0 Swap Read
pub type BRSWP_W<'a, REG> = crate::BitWriter<'a, REG, BRSWP_A>;
impl<'a, REG> BRSWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The current data are read without swapping.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRSWP_A::_0)
    }
    ///Swapping of the positions of the higher- and lower-order bytes of data for reading proceeds.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRSWP_A::_1)
    }
}
impl R {
    ///Bit 6 - SD_BUF0 Swap Write
    #[inline(always)]
    pub fn bwswp(&self) -> BWSWP_R {
        BWSWP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SD_BUF0 Swap Read
    #[inline(always)]
    pub fn brswp(&self) -> BRSWP_R {
        BRSWP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - SD_BUF0 Swap Write
    #[inline(always)]
    pub fn bwswp(&mut self) -> BWSWP_W<EXT_SWAP_SPEC> {
        BWSWP_W::new(self, 6)
    }
    ///Bit 7 - SD_BUF0 Swap Read
    #[inline(always)]
    pub fn brswp(&mut self) -> BRSWP_W<EXT_SWAP_SPEC> {
        BRSWP_W::new(self, 7)
    }
}
/**Swap Control Register

You can [`read`](crate::Reg::read) this register and get [`ext_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_SWAP_SPEC;
impl crate::RegisterSpec for EXT_SWAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_swap::R`](R) reader structure
impl crate::Readable for EXT_SWAP_SPEC {}
///`write(|w| ..)` method takes [`ext_swap::W`](W) writer structure
impl crate::Writable for EXT_SWAP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXT_SWAP to value 0
impl crate::Resettable for EXT_SWAP_SPEC {}
