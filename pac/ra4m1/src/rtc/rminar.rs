///Register `RMINAR` reader
pub type R = crate::R<RMINAR_SPEC>;
///Register `RMINAR` writer
pub type W = crate::W<RMINAR_SPEC>;
///Field `MIN1` reader - 1-Minute Count Value for the ones place of minutes
pub type MIN1_R = crate::FieldReader;
///Field `MIN1` writer - 1-Minute Count Value for the ones place of minutes
pub type MIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MIN10` reader - 10-Minute Count Value for the tens place of minutes
pub type MIN10_R = crate::FieldReader;
///Field `MIN10` writer - 10-Minute Count Value for the tens place of minutes
pub type MIN10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Compare enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: The register value is not compared with the RMINCNT counter value.
    _0 = 0,
    ///1: The register value is compared with the RMINCNT counter value.
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
    ///The register value is not compared with the RMINCNT counter value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///The register value is compared with the RMINCNT counter value.
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
    ///The register value is not compared with the RMINCNT counter value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///The register value is compared with the RMINCNT counter value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    ///Bits 0:3 - 1-Minute Count Value for the ones place of minutes
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Minute Count Value for the tens place of minutes
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - 1-Minute Count Value for the ones place of minutes
    #[inline(always)]
    pub fn min1(&mut self) -> MIN1_W<RMINAR_SPEC> {
        MIN1_W::new(self, 0)
    }
    ///Bits 4:6 - 10-Minute Count Value for the tens place of minutes
    #[inline(always)]
    pub fn min10(&mut self) -> MIN10_W<RMINAR_SPEC> {
        MIN10_W::new(self, 4)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<RMINAR_SPEC> {
        ENB_W::new(self, 7)
    }
}
/**Minute Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rminar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rminar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMINAR_SPEC;
impl crate::RegisterSpec for RMINAR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rminar::R`](R) reader structure
impl crate::Readable for RMINAR_SPEC {}
///`write(|w| ..)` method takes [`rminar::W`](W) writer structure
impl crate::Writable for RMINAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMINAR to value 0
impl crate::Resettable for RMINAR_SPEC {}
