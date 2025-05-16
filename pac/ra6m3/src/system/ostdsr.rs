///Register `OSTDSR` reader
pub type R = crate::R<OSTDSR_SPEC>;
///Register `OSTDSR` writer
pub type W = crate::W<OSTDSR_SPEC>;
/**Oscillation Stop Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDF_A {
    ///0: Main clock oscillation stop not detected
    _0 = 0,
    ///1: Main clock oscillation stop detected
    _1 = 1,
}
impl From<OSTDF_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OSTDF` reader - Oscillation Stop Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OSTDF_R = crate::BitReader<OSTDF_A>;
impl OSTDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTDF_A {
        match self.bits {
            false => OSTDF_A::_0,
            true => OSTDF_A::_1,
        }
    }
    ///Main clock oscillation stop not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDF_A::_0
    }
    ///Main clock oscillation stop detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDF_A::_1
    }
}
///Field `OSTDF` writer - Oscillation Stop Detection Flag
pub type OSTDF_W<'a, REG> = crate::BitWriter0C<'a, REG, OSTDF_A>;
impl<'a, REG> OSTDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main clock oscillation stop not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDF_A::_0)
    }
    ///Main clock oscillation stop detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDF_A::_1)
    }
}
impl R {
    ///Bit 0 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostdf(&self) -> OSTDF_R {
        OSTDF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostdf(&mut self) -> OSTDF_W<OSTDSR_SPEC> {
        OSTDF_W::new(self, 0)
    }
}
/**Oscillation Stop Detection Status Register

You can [`read`](crate::Reg::read) this register and get [`ostdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OSTDSR_SPEC;
impl crate::RegisterSpec for OSTDSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ostdsr::R`](R) reader structure
impl crate::Readable for OSTDSR_SPEC {}
///`write(|w| ..)` method takes [`ostdsr::W`](W) writer structure
impl crate::Writable for OSTDSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
///`reset()` method sets OSTDSR to value 0
impl crate::Resettable for OSTDSR_SPEC {}
