///Register `RDAYAR` reader
pub type R = crate::R<RDAYAR_SPEC>;
///Register `RDAYAR` writer
pub type W = crate::W<RDAYAR_SPEC>;
///Field `DATE1` reader - 1 Day Value for the ones place of days
pub type DATE1_R = crate::FieldReader;
///Field `DATE1` writer - 1 Day Value for the ones place of days
pub type DATE1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATE10` reader - 10 Days Value for the tens place of days
pub type DATE10_R = crate::FieldReader;
///Field `DATE10` writer - 10 Days Value for the tens place of days
pub type DATE10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Compare enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: The register value is not compared with the RDAYCNT counter value.
    _0 = 0,
    ///1: The register value is compared with the RDAYCNT counter value.
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
    ///The register value is not compared with the RDAYCNT counter value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///The register value is compared with the RDAYCNT counter value.
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
    ///The register value is not compared with the RDAYCNT counter value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///The register value is compared with the RDAYCNT counter value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    ///Bits 0:3 - 1 Day Value for the ones place of days
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10 Days Value for the tens place of days
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new((self.bits >> 4) & 3)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - 1 Day Value for the ones place of days
    #[inline(always)]
    pub fn date1(&mut self) -> DATE1_W<RDAYAR_SPEC> {
        DATE1_W::new(self, 0)
    }
    ///Bits 4:5 - 10 Days Value for the tens place of days
    #[inline(always)]
    pub fn date10(&mut self) -> DATE10_W<RDAYAR_SPEC> {
        DATE10_W::new(self, 4)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<RDAYAR_SPEC> {
        ENB_W::new(self, 7)
    }
}
/**Date Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rdayar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdayar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDAYAR_SPEC;
impl crate::RegisterSpec for RDAYAR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rdayar::R`](R) reader structure
impl crate::Readable for RDAYAR_SPEC {}
///`write(|w| ..)` method takes [`rdayar::W`](W) writer structure
impl crate::Writable for RDAYAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDAYAR to value 0
impl crate::Resettable for RDAYAR_SPEC {}
