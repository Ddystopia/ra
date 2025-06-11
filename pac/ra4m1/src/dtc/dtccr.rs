///Register `DTCCR` reader
pub type R = crate::R<DTCCR_SPEC>;
///Register `DTCCR` writer
pub type W = crate::W<DTCCR_SPEC>;
/**DTC Transfer Information Read Skip Enable.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS_A {
    ///0: Do not skip transfer information read
    _0 = 0,
    ///1: Skip transfer information read when vector numbers match
    _1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RRS` reader - DTC Transfer Information Read Skip Enable.
pub type RRS_R = crate::BitReader<RRS_A>;
impl RRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::_0,
            true => RRS_A::_1,
        }
    }
    ///Do not skip transfer information read
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRS_A::_0
    }
    ///Skip transfer information read when vector numbers match
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRS_A::_1
    }
}
///Field `RRS` writer - DTC Transfer Information Read Skip Enable.
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG, RRS_A>;
impl<'a, REG> RRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not skip transfer information read
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RRS_A::_0)
    }
    ///Skip transfer information read when vector numbers match
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RRS_A::_1)
    }
}
impl R {
    ///Bit 4 - DTC Transfer Information Read Skip Enable.
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - DTC Transfer Information Read Skip Enable.
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<DTCCR_SPEC> {
        RRS_W::new(self, 4)
    }
}
/**DTC Control Register

You can [`read`](crate::Reg::read) this register and get [`dtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DTCCR_SPEC;
impl crate::RegisterSpec for DTCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dtccr::R`](R) reader structure
impl crate::Readable for DTCCR_SPEC {}
///`write(|w| ..)` method takes [`dtccr::W`](W) writer structure
impl crate::Writable for DTCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTCCR to value 0x08
impl crate::Resettable for DTCCR_SPEC {
    const RESET_VALUE: u8 = 0x08;
}
