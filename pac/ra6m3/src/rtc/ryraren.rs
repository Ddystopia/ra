///Register `RYRAREN` reader
pub type R = crate::R<RYRAREN_SPEC>;
///Register `RYRAREN` writer
pub type W = crate::W<RYRAREN_SPEC>;
/**Compare enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: The register value is not compared with the RYRCNT counter value.
    _0 = 0,
    ///1: The register value is compared with the RYRCNT counter value.
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENB` reader - Compare enable
pub type ENB_R = crate::BitReader<ENB_A>;
impl ENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    ///The register value is not compared with the RYRCNT counter value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///The register value is compared with the RYRCNT counter value.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
///Field `ENB` writer - Compare enable
pub type ENB_W<'a, REG> = crate::BitWriter<'a, REG, ENB_A>;
impl<'a, REG> ENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The register value is not compared with the RYRCNT counter value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///The register value is compared with the RYRCNT counter value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<RYRAREN_SPEC> {
        ENB_W::new(self, 7)
    }
}
/**Year Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`ryraren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryraren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RYRAREN_SPEC;
impl crate::RegisterSpec for RYRAREN_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ryraren::R`](R) reader structure
impl crate::Readable for RYRAREN_SPEC {}
///`write(|w| ..)` method takes [`ryraren::W`](W) writer structure
impl crate::Writable for RYRAREN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RYRAREN to value 0
impl crate::Resettable for RYRAREN_SPEC {}
