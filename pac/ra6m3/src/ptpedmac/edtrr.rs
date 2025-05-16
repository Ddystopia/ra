///Register `EDTRR` reader
pub type R = crate::R<EDTRR_SPEC>;
///Register `EDTRR` writer
pub type W = crate::W<EDTRR_SPEC>;
/**Transmit Request (The read value is 0.)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR_A {
    ///0: no effect.
    _0 = 0,
    ///1: When 1 is written, the EDMAC reads the corresponding descriptor and transmits frames where the TD0.TACT bit is 1. The TR bit becomes 0 after all the valid frames are transmitted.
    _1 = 1,
}
impl From<TR_A> for bool {
    #[inline(always)]
    fn from(variant: TR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TR` reader - Transmit Request (The read value is 0.)
pub type TR_R = crate::BitReader<TR_A>;
impl TR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TR_A {
        match self.bits {
            false => TR_A::_0,
            true => TR_A::_1,
        }
    }
    ///no effect.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TR_A::_0
    }
    ///When 1 is written, the EDMAC reads the corresponding descriptor and transmits frames where the TD0.TACT bit is 1. The TR bit becomes 0 after all the valid frames are transmitted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TR_A::_1
    }
}
///Field `TR` writer - Transmit Request (The read value is 0.)
pub type TR_W<'a, REG> = crate::BitWriter<'a, REG, TR_A>;
impl<'a, REG> TR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TR_A::_0)
    }
    ///When 1 is written, the EDMAC reads the corresponding descriptor and transmits frames where the TD0.TACT bit is 1. The TR bit becomes 0 after all the valid frames are transmitted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TR_A::_1)
    }
}
impl R {
    ///Bit 0 - Transmit Request (The read value is 0.)
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit Request (The read value is 0.)
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W<EDTRR_SPEC> {
        TR_W::new(self, 0)
    }
}
/**EDMAC Transmit Request Register

You can [`read`](crate::Reg::read) this register and get [`edtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EDTRR_SPEC;
impl crate::RegisterSpec for EDTRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`edtrr::R`](R) reader structure
impl crate::Readable for EDTRR_SPEC {}
///`write(|w| ..)` method takes [`edtrr::W`](W) writer structure
impl crate::Writable for EDTRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDTRR to value 0
impl crate::Resettable for EDTRR_SPEC {}
