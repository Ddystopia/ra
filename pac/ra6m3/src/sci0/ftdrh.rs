///Register `FTDRH` writer
pub type W = crate::W<FTDRH_SPEC>;
///Field `TDATH` writer - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
pub type TDATH_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Multi-processor transfer bit flag(Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBT_A {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
    _1 = 1,
}
impl From<MPBT_A> for bool {
    #[inline(always)]
    fn from(variant: MPBT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MPBT` writer - Multi-processor transfer bit flag(Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)
pub type MPBT_W<'a, REG> = crate::BitWriter<'a, REG, MPBT_A>;
impl<'a, REG> MPBT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data transmission cycles
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPBT_A::_0)
    }
    ///ID transmission cycles
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPBT_A::_1)
    }
}
impl W {
    ///Bit 0 - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
    #[inline(always)]
    pub fn tdath(&mut self) -> TDATH_W<FTDRH_SPEC> {
        TDATH_W::new(self, 0)
    }
    ///Bit 1 - Multi-processor transfer bit flag(Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)
    #[inline(always)]
    pub fn mpbt(&mut self) -> MPBT_W<FTDRH_SPEC> {
        MPBT_W::new(self, 1)
    }
}
/**Transmit FIFO Data Register H

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FTDRH_SPEC;
impl crate::RegisterSpec for FTDRH_SPEC {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`ftdrh::W`](W) writer structure
impl crate::Writable for FTDRH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTDRH to value 0xff
impl crate::Resettable for FTDRH_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
