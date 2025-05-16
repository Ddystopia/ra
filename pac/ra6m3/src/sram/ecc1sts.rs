///Register `ECC1STS` reader
pub type R = crate::R<ECC1STS_SPEC>;
///Register `ECC1STS` writer
pub type W = crate::W<ECC1STS_SPEC>;
/**ECC 1-Bit Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECC1ERR_A {
    ///0: No 1-bit ECC error occurred
    _0 = 0,
    ///1: 1-bit ECC error occurred
    _1 = 1,
}
impl From<ECC1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ECC1ERR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ECC1ERR` reader - ECC 1-Bit Error Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ECC1ERR_R = crate::BitReader<ECC1ERR_A>;
impl ECC1ERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECC1ERR_A {
        match self.bits {
            false => ECC1ERR_A::_0,
            true => ECC1ERR_A::_1,
        }
    }
    ///No 1-bit ECC error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECC1ERR_A::_0
    }
    ///1-bit ECC error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECC1ERR_A::_1
    }
}
///Field `ECC1ERR` writer - ECC 1-Bit Error Status
pub type ECC1ERR_W<'a, REG> = crate::BitWriter0C<'a, REG, ECC1ERR_A>;
impl<'a, REG> ECC1ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No 1-bit ECC error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECC1ERR_A::_0)
    }
    ///1-bit ECC error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECC1ERR_A::_1)
    }
}
impl R {
    ///Bit 0 - ECC 1-Bit Error Status
    #[inline(always)]
    pub fn ecc1err(&self) -> ECC1ERR_R {
        ECC1ERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ECC 1-Bit Error Status
    #[inline(always)]
    pub fn ecc1err(&mut self) -> ECC1ERR_W<ECC1STS_SPEC> {
        ECC1ERR_W::new(self, 0)
    }
}
/**ECCRAM 1-Bit Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ecc1sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECC1STS_SPEC;
impl crate::RegisterSpec for ECC1STS_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ecc1sts::R`](R) reader structure
impl crate::Readable for ECC1STS_SPEC {}
///`write(|w| ..)` method takes [`ecc1sts::W`](W) writer structure
impl crate::Writable for ECC1STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
///`reset()` method sets ECC1STS to value 0
impl crate::Resettable for ECC1STS_SPEC {}
