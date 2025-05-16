///Register `ECC1STSEN` reader
pub type R = crate::R<ECC1STSEN_SPEC>;
///Register `ECC1STSEN` writer
pub type W = crate::W<ECC1STSEN_SPEC>;
/**ECC 1-Bit Error Information Update Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1STSEN_A {
    ///0: Disables updating of the 1-bit ECC error information.
    _0 = 0,
    ///1: Enables updating of the 1-bit ECC error information.
    _1 = 1,
}
impl From<E1STSEN_A> for bool {
    #[inline(always)]
    fn from(variant: E1STSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `E1STSEN` reader - ECC 1-Bit Error Information Update Enable
pub type E1STSEN_R = crate::BitReader<E1STSEN_A>;
impl E1STSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> E1STSEN_A {
        match self.bits {
            false => E1STSEN_A::_0,
            true => E1STSEN_A::_1,
        }
    }
    ///Disables updating of the 1-bit ECC error information.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == E1STSEN_A::_0
    }
    ///Enables updating of the 1-bit ECC error information.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == E1STSEN_A::_1
    }
}
///Field `E1STSEN` writer - ECC 1-Bit Error Information Update Enable
pub type E1STSEN_W<'a, REG> = crate::BitWriter<'a, REG, E1STSEN_A>;
impl<'a, REG> E1STSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables updating of the 1-bit ECC error information.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(E1STSEN_A::_0)
    }
    ///Enables updating of the 1-bit ECC error information.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(E1STSEN_A::_1)
    }
}
impl R {
    ///Bit 0 - ECC 1-Bit Error Information Update Enable
    #[inline(always)]
    pub fn e1stsen(&self) -> E1STSEN_R {
        E1STSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ECC 1-Bit Error Information Update Enable
    #[inline(always)]
    pub fn e1stsen(&mut self) -> E1STSEN_W<ECC1STSEN_SPEC> {
        E1STSEN_W::new(self, 0)
    }
}
/**ECCRAM 1-Bit Error Information Update Enable Register

You can [`read`](crate::Reg::read) this register and get [`ecc1stsen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1stsen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECC1STSEN_SPEC;
impl crate::RegisterSpec for ECC1STSEN_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ecc1stsen::R`](R) reader structure
impl crate::Readable for ECC1STSEN_SPEC {}
///`write(|w| ..)` method takes [`ecc1stsen::W`](W) writer structure
impl crate::Writable for ECC1STSEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECC1STSEN to value 0
impl crate::Resettable for ECC1STSEN_SPEC {}
