///Register `FTDRHL` writer
pub type W = crate::W<FTDRHL_SPEC>;
///Field `TDAT` writer - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
pub type TDAT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
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
    ///Bits 0:8 - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
    #[inline(always)]
    pub fn tdat(&mut self) -> TDAT_W<FTDRHL_SPEC> {
        TDAT_W::new(self, 0)
    }
    ///Bit 9 - Multi-processor transfer bit flag(Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)
    #[inline(always)]
    pub fn mpbt(&mut self) -> MPBT_W<FTDRHL_SPEC> {
        MPBT_W::new(self, 9)
    }
}
/**Transmit FIFO Data Register HL

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrhl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FTDRHL_SPEC;
impl crate::RegisterSpec for FTDRHL_SPEC {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`ftdrhl::W`](W) writer structure
impl crate::Writable for FTDRHL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTDRHL to value 0xffff
impl crate::Resettable for FTDRHL_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
