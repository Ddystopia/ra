///Register `AGTMR2` reader
pub type R = crate::R<AGTMR2_SPEC>;
///Register `AGTMR2` writer
pub type W = crate::W<AGTMR2_SPEC>;
/**fsub/LOCO count source clock frequency division ratio select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    ///0: 1/1
    _000 = 0,
    ///1: 1/2
    _001 = 1,
    ///2: 1/4
    _010 = 2,
    ///3: 1/8
    _011 = 3,
    ///4: 1/16
    _100 = 4,
    ///5: 1/32
    _101 = 5,
    ///6: 1/64
    _110 = 6,
    ///7: 1/128
    _111 = 7,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKS_A {
    type Ux = u8;
}
impl crate::IsEnum for CKS_A {}
///Field `CKS` reader - fsub/LOCO count source clock frequency division ratio select
pub type CKS_R = crate::FieldReader<CKS_A>;
impl CKS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_000,
            1 => CKS_A::_001,
            2 => CKS_A::_010,
            3 => CKS_A::_011,
            4 => CKS_A::_100,
            5 => CKS_A::_101,
            6 => CKS_A::_110,
            7 => CKS_A::_111,
            _ => unreachable!(),
        }
    }
    ///1/1
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CKS_A::_000
    }
    ///1/2
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CKS_A::_001
    }
    ///1/4
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CKS_A::_010
    }
    ///1/8
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CKS_A::_011
    }
    ///1/16
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CKS_A::_100
    }
    ///1/32
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CKS_A::_101
    }
    ///1/64
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CKS_A::_110
    }
    ///1/128
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CKS_A::_111
    }
}
///Field `CKS` writer - fsub/LOCO count source clock frequency division ratio select
pub type CKS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CKS_A, crate::Safe>;
impl<'a, REG> CKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/1
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_000)
    }
    ///1/2
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_001)
    }
    ///1/4
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_010)
    }
    ///1/8
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_011)
    }
    ///1/16
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_100)
    }
    ///1/32
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_101)
    }
    ///1/64
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_110)
    }
    ///1/128
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_111)
    }
}
/**AGT Low Power Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_A {
    ///0: Normal mode
    _0 = 0,
    ///1: Low Power mode
    _1 = 1,
}
impl From<LPM_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPM` reader - AGT Low Power Mode
pub type LPM_R = crate::BitReader<LPM_A>;
impl LPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPM_A {
        match self.bits {
            false => LPM_A::_0,
            true => LPM_A::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPM_A::_0
    }
    ///Low Power mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPM_A::_1
    }
}
///Field `LPM` writer - AGT Low Power Mode
pub type LPM_W<'a, REG> = crate::BitWriter<'a, REG, LPM_A>;
impl<'a, REG> LPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LPM_A::_0)
    }
    ///Low Power mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LPM_A::_1)
    }
}
impl R {
    ///Bits 0:2 - fsub/LOCO count source clock frequency division ratio select
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(self.bits & 7)
    }
    ///Bit 7 - AGT Low Power Mode
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - fsub/LOCO count source clock frequency division ratio select
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<AGTMR2_SPEC> {
        CKS_W::new(self, 0)
    }
    ///Bit 7 - AGT Low Power Mode
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W<AGTMR2_SPEC> {
        LPM_W::new(self, 7)
    }
}
/**AGT Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`agtmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTMR2_SPEC;
impl crate::RegisterSpec for AGTMR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`agtmr2::R`](R) reader structure
impl crate::Readable for AGTMR2_SPEC {}
///`write(|w| ..)` method takes [`agtmr2::W`](W) writer structure
impl crate::Writable for AGTMR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTMR2 to value 0
impl crate::Resettable for AGTMR2_SPEC {}
