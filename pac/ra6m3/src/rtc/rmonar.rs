///Register `RMONAR` reader
pub type R = crate::R<RMONAR_SPEC>;
///Register `RMONAR` writer
pub type W = crate::W<RMONAR_SPEC>;
///Field `MON1` reader - 1 Month Value for the ones place of months
pub type MON1_R = crate::FieldReader;
///Field `MON1` writer - 1 Month Value for the ones place of months
pub type MON1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MON10` reader - 10 Months Value for the tens place of months
pub type MON10_R = crate::BitReader;
///Field `MON10` writer - 10 Months Value for the tens place of months
pub type MON10_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Compare enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: The register value is not compared with the RMONCNT counter value.
    _0 = 0,
    ///1: The register value is compared with the RMONCNT counter value.
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
    ///The register value is not compared with the RMONCNT counter value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///The register value is compared with the RMONCNT counter value.
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
    ///The register value is not compared with the RMONCNT counter value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///The register value is compared with the RMONCNT counter value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    ///Bits 0:3 - 1 Month Value for the ones place of months
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    ///Bit 4 - 10 Months Value for the tens place of months
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - 1 Month Value for the ones place of months
    #[inline(always)]
    pub fn mon1(&mut self) -> MON1_W<RMONAR_SPEC> {
        MON1_W::new(self, 0)
    }
    ///Bit 4 - 10 Months Value for the tens place of months
    #[inline(always)]
    pub fn mon10(&mut self) -> MON10_W<RMONAR_SPEC> {
        MON10_W::new(self, 4)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<RMONAR_SPEC> {
        ENB_W::new(self, 7)
    }
}
/**Month Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rmonar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmonar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMONAR_SPEC;
impl crate::RegisterSpec for RMONAR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rmonar::R`](R) reader structure
impl crate::Readable for RMONAR_SPEC {}
///`write(|w| ..)` method takes [`rmonar::W`](W) writer structure
impl crate::Writable for RMONAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMONAR to value 0
impl crate::Resettable for RMONAR_SPEC {}
