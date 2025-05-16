///Register `RSECAR` reader
pub type R = crate::R<RSECAR_SPEC>;
///Register `RSECAR` writer
pub type W = crate::W<RSECAR_SPEC>;
///Field `SEC1` reader - 1-Second Value for the ones place of seconds
pub type SEC1_R = crate::FieldReader;
///Field `SEC1` writer - 1-Second Value for the ones place of seconds
pub type SEC1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEC10` reader - 10-Seconds Value for the tens place of seconds
pub type SEC10_R = crate::FieldReader;
///Field `SEC10` writer - 10-Seconds Value for the tens place of seconds
pub type SEC10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Compare enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: The register value is not compared with the RSECCNT counter value.
    _0 = 0,
    ///1: The register value is compared with the RSECCNT counter value.
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
    ///The register value is not compared with the RSECCNT counter value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///The register value is compared with the RSECCNT counter value.
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
    ///The register value is not compared with the RSECCNT counter value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///The register value is compared with the RSECCNT counter value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    ///Bits 0:3 - 1-Second Value for the ones place of seconds
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Seconds Value for the tens place of seconds
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - 1-Second Value for the ones place of seconds
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<RSECAR_SPEC> {
        SEC1_W::new(self, 0)
    }
    ///Bits 4:6 - 10-Seconds Value for the tens place of seconds
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W<RSECAR_SPEC> {
        SEC10_W::new(self, 4)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<RSECAR_SPEC> {
        ENB_W::new(self, 7)
    }
}
/**Second Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rsecar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsecar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSECAR_SPEC;
impl crate::RegisterSpec for RSECAR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rsecar::R`](R) reader structure
impl crate::Readable for RSECAR_SPEC {}
///`write(|w| ..)` method takes [`rsecar::W`](W) writer structure
impl crate::Writable for RSECAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSECAR to value 0
impl crate::Resettable for RSECAR_SPEC {}
