///Register `RWKAR` reader
pub type R = crate::R<RWKAR_SPEC>;
///Register `RWKAR` writer
pub type W = crate::W<RWKAR_SPEC>;
/**Day-of-Week Counting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAYW_A {
    ///0: Sunday
    _000 = 0,
    ///1: Monday
    _001 = 1,
    ///2: Tuesday
    _010 = 2,
    ///3: Wednesday
    _011 = 3,
    ///4: Thursday
    _100 = 4,
    ///5: Friday
    _101 = 5,
    ///6: Saturday
    _110 = 6,
    ///7: Setting Prohibited
    _111 = 7,
}
impl From<DAYW_A> for u8 {
    #[inline(always)]
    fn from(variant: DAYW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAYW_A {
    type Ux = u8;
}
impl crate::IsEnum for DAYW_A {}
///Field `DAYW` reader - Day-of-Week Counting
pub type DAYW_R = crate::FieldReader<DAYW_A>;
impl DAYW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAYW_A {
        match self.bits {
            0 => DAYW_A::_000,
            1 => DAYW_A::_001,
            2 => DAYW_A::_010,
            3 => DAYW_A::_011,
            4 => DAYW_A::_100,
            5 => DAYW_A::_101,
            6 => DAYW_A::_110,
            7 => DAYW_A::_111,
            _ => unreachable!(),
        }
    }
    ///Sunday
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DAYW_A::_000
    }
    ///Monday
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DAYW_A::_001
    }
    ///Tuesday
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DAYW_A::_010
    }
    ///Wednesday
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DAYW_A::_011
    }
    ///Thursday
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DAYW_A::_100
    }
    ///Friday
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DAYW_A::_101
    }
    ///Saturday
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DAYW_A::_110
    }
    ///Setting Prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DAYW_A::_111
    }
}
///Field `DAYW` writer - Day-of-Week Counting
pub type DAYW_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DAYW_A, crate::Safe>;
impl<'a, REG> DAYW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sunday
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_000)
    }
    ///Monday
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_001)
    }
    ///Tuesday
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_010)
    }
    ///Wednesday
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_011)
    }
    ///Thursday
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_100)
    }
    ///Friday
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_101)
    }
    ///Saturday
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_110)
    }
    ///Setting Prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(DAYW_A::_111)
    }
}
/**Compare enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    ///0: The register value is not compared with the RWKCNT counter value.
    _0 = 0,
    ///1: The register value is compared with the RWKCNT counter value.
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
    ///The register value is not compared with the RWKCNT counter value.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    ///The register value is compared with the RWKCNT counter value.
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
    ///The register value is not compared with the RWKCNT counter value.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    ///The register value is compared with the RWKCNT counter value.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    ///Bits 0:2 - Day-of-Week Counting
    #[inline(always)]
    pub fn dayw(&self) -> DAYW_R {
        DAYW_R::new(self.bits & 7)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Day-of-Week Counting
    #[inline(always)]
    pub fn dayw(&mut self) -> DAYW_W<RWKAR_SPEC> {
        DAYW_W::new(self, 0)
    }
    ///Bit 7 - Compare enable
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<RWKAR_SPEC> {
        ENB_W::new(self, 7)
    }
}
/**Day-of-Week Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rwkar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RWKAR_SPEC;
impl crate::RegisterSpec for RWKAR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rwkar::R`](R) reader structure
impl crate::Readable for RWKAR_SPEC {}
///`write(|w| ..)` method takes [`rwkar::W`](W) writer structure
impl crate::Writable for RWKAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RWKAR to value 0
impl crate::Resettable for RWKAR_SPEC {}
