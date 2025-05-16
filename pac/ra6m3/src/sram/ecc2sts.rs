///Register `ECC2STS` reader
pub type R = crate::R<ECC2STS_SPEC>;
///Register `ECC2STS` writer
pub type W = crate::W<ECC2STS_SPEC>;
/**ECC 2-Bit Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECC2ERR_A {
    ///0: No 2-bit ECC error occurred
    _0 = 0,
    ///1: 2-bit ECC error occurred
    _1 = 1,
}
impl From<ECC2ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ECC2ERR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ECC2ERR` reader - ECC 2-Bit Error Status

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ECC2ERR_R = crate::BitReader<ECC2ERR_A>;
impl ECC2ERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECC2ERR_A {
        match self.bits {
            false => ECC2ERR_A::_0,
            true => ECC2ERR_A::_1,
        }
    }
    ///No 2-bit ECC error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECC2ERR_A::_0
    }
    ///2-bit ECC error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECC2ERR_A::_1
    }
}
///Field `ECC2ERR` writer - ECC 2-Bit Error Status
pub type ECC2ERR_W<'a, REG> = crate::BitWriter0C<'a, REG, ECC2ERR_A>;
impl<'a, REG> ECC2ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No 2-bit ECC error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECC2ERR_A::_0)
    }
    ///2-bit ECC error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECC2ERR_A::_1)
    }
}
impl R {
    ///Bit 0 - ECC 2-Bit Error Status
    #[inline(always)]
    pub fn ecc2err(&self) -> ECC2ERR_R {
        ECC2ERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ECC 2-Bit Error Status
    #[inline(always)]
    pub fn ecc2err(&mut self) -> ECC2ERR_W<ECC2STS_SPEC> {
        ECC2ERR_W::new(self, 0)
    }
}
/**ECCRAM 2-Bit Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ecc2sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc2sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECC2STS_SPEC;
impl crate::RegisterSpec for ECC2STS_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ecc2sts::R`](R) reader structure
impl crate::Readable for ECC2STS_SPEC {}
///`write(|w| ..)` method takes [`ecc2sts::W`](W) writer structure
impl crate::Writable for ECC2STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
///`reset()` method sets ECC2STS to value 0
impl crate::Resettable for ECC2STS_SPEC {}
