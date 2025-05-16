///Register `WDTCR` reader
pub type R = crate::R<WDTCR_SPEC>;
///Register `WDTCR` writer
pub type W = crate::W<WDTCR_SPEC>;
/**Timeout Period Selection

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOPS_A {
    ///0: 1,024 cycles (03FFh)
    _00 = 0,
    ///1: 4,096 cycles (0FFFh)
    _01 = 1,
    ///2: 8,192 cycles (1FFFh)
    _10 = 2,
    ///3: 16,384 cycles (3FFFh)
    _11 = 3,
}
impl From<TOPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOPS_A {
    type Ux = u8;
}
impl crate::IsEnum for TOPS_A {}
///Field `TOPS` reader - Timeout Period Selection
pub type TOPS_R = crate::FieldReader<TOPS_A>;
impl TOPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOPS_A {
        match self.bits {
            0 => TOPS_A::_00,
            1 => TOPS_A::_01,
            2 => TOPS_A::_10,
            3 => TOPS_A::_11,
            _ => unreachable!(),
        }
    }
    ///1,024 cycles (03FFh)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOPS_A::_00
    }
    ///4,096 cycles (0FFFh)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOPS_A::_01
    }
    ///8,192 cycles (1FFFh)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOPS_A::_10
    }
    ///16,384 cycles (3FFFh)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOPS_A::_11
    }
}
///Field `TOPS` writer - Timeout Period Selection
pub type TOPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TOPS_A, crate::Safe>;
impl<'a, REG> TOPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1,024 cycles (03FFh)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TOPS_A::_00)
    }
    ///4,096 cycles (0FFFh)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TOPS_A::_01)
    }
    ///8,192 cycles (1FFFh)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TOPS_A::_10)
    }
    ///16,384 cycles (3FFFh)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TOPS_A::_11)
    }
}
/**Clock Division Ratio Selection

Value on reset: 15*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    ///1: PCLK/4
    _0001 = 1,
    ///4: PCLK/64
    _0100 = 4,
    ///15: PCLK/128
    _1111 = 15,
    ///6: PCLK/512
    _0110 = 6,
    ///7: PCLK/2048
    _0111 = 7,
    ///8: PCLK/8192
    _1000 = 8,
    ///0: setting prohibited
    OTHERS = 0,
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
///Field `CKS` reader - Clock Division Ratio Selection
pub type CKS_R = crate::FieldReader<CKS_A>;
impl CKS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            1 => CKS_A::_0001,
            4 => CKS_A::_0100,
            15 => CKS_A::_1111,
            6 => CKS_A::_0110,
            7 => CKS_A::_0111,
            8 => CKS_A::_1000,
            _ => CKS_A::OTHERS,
        }
    }
    ///PCLK/4
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CKS_A::_0001
    }
    ///PCLK/64
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CKS_A::_0100
    }
    ///PCLK/128
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CKS_A::_1111
    }
    ///PCLK/512
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CKS_A::_0110
    }
    ///PCLK/2048
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CKS_A::_0111
    }
    ///PCLK/8192
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CKS_A::_1000
    }
    ///setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CKS_A::OTHERS)
    }
}
///Field `CKS` writer - Clock Division Ratio Selection
pub type CKS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CKS_A, crate::Safe>;
impl<'a, REG> CKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK/4
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_0001)
    }
    ///PCLK/64
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_0100)
    }
    ///PCLK/128
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_1111)
    }
    ///PCLK/512
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_0110)
    }
    ///PCLK/2048
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_0111)
    }
    ///PCLK/8192
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_1000)
    }
    ///setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::OTHERS)
    }
}
/**Window End Position Selection

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPES_A {
    ///0: 75 percent
    _00 = 0,
    ///1: 50 percent
    _01 = 1,
    ///2: 25 percent
    _10 = 2,
    ///3: 0 percent (window end position is not specified)
    _11 = 3,
}
impl From<RPES_A> for u8 {
    #[inline(always)]
    fn from(variant: RPES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPES_A {
    type Ux = u8;
}
impl crate::IsEnum for RPES_A {}
///Field `RPES` reader - Window End Position Selection
pub type RPES_R = crate::FieldReader<RPES_A>;
impl RPES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPES_A {
        match self.bits {
            0 => RPES_A::_00,
            1 => RPES_A::_01,
            2 => RPES_A::_10,
            3 => RPES_A::_11,
            _ => unreachable!(),
        }
    }
    ///75 percent
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RPES_A::_00
    }
    ///50 percent
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RPES_A::_01
    }
    ///25 percent
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RPES_A::_10
    }
    ///0 percent (window end position is not specified)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RPES_A::_11
    }
}
///Field `RPES` writer - Window End Position Selection
pub type RPES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RPES_A, crate::Safe>;
impl<'a, REG> RPES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///75 percent
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(RPES_A::_00)
    }
    ///50 percent
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(RPES_A::_01)
    }
    ///25 percent
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(RPES_A::_10)
    }
    ///0 percent (window end position is not specified)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(RPES_A::_11)
    }
}
/**Window Start Position Selection

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPSS_A {
    ///0: 25 percent
    _00 = 0,
    ///1: 50 percent
    _01 = 1,
    ///2: 75 percent
    _10 = 2,
    ///3: 100 percent (window start position is not specified)
    _11 = 3,
}
impl From<RPSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RPSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPSS_A {
    type Ux = u8;
}
impl crate::IsEnum for RPSS_A {}
///Field `RPSS` reader - Window Start Position Selection
pub type RPSS_R = crate::FieldReader<RPSS_A>;
impl RPSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPSS_A {
        match self.bits {
            0 => RPSS_A::_00,
            1 => RPSS_A::_01,
            2 => RPSS_A::_10,
            3 => RPSS_A::_11,
            _ => unreachable!(),
        }
    }
    ///25 percent
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RPSS_A::_00
    }
    ///50 percent
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RPSS_A::_01
    }
    ///75 percent
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RPSS_A::_10
    }
    ///100 percent (window start position is not specified)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RPSS_A::_11
    }
}
///Field `RPSS` writer - Window Start Position Selection
pub type RPSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RPSS_A, crate::Safe>;
impl<'a, REG> RPSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///25 percent
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(RPSS_A::_00)
    }
    ///50 percent
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(RPSS_A::_01)
    }
    ///75 percent
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(RPSS_A::_10)
    }
    ///100 percent (window start position is not specified)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(RPSS_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Timeout Period Selection
    #[inline(always)]
    pub fn tops(&self) -> TOPS_R {
        TOPS_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Clock Division Ratio Selection
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Window End Position Selection
    #[inline(always)]
    pub fn rpes(&self) -> RPES_R {
        RPES_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Window Start Position Selection
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Timeout Period Selection
    #[inline(always)]
    pub fn tops(&mut self) -> TOPS_W<WDTCR_SPEC> {
        TOPS_W::new(self, 0)
    }
    ///Bits 4:7 - Clock Division Ratio Selection
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<WDTCR_SPEC> {
        CKS_W::new(self, 4)
    }
    ///Bits 8:9 - Window End Position Selection
    #[inline(always)]
    pub fn rpes(&mut self) -> RPES_W<WDTCR_SPEC> {
        RPES_W::new(self, 8)
    }
    ///Bits 12:13 - Window Start Position Selection
    #[inline(always)]
    pub fn rpss(&mut self) -> RPSS_W<WDTCR_SPEC> {
        RPSS_W::new(self, 12)
    }
}
/**WDT Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDTCR_SPEC;
impl crate::RegisterSpec for WDTCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`wdtcr::R`](R) reader structure
impl crate::Readable for WDTCR_SPEC {}
///`write(|w| ..)` method takes [`wdtcr::W`](W) writer structure
impl crate::Writable for WDTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTCR to value 0x33f3
impl crate::Resettable for WDTCR_SPEC {
    const RESET_VALUE: u16 = 0x33f3;
}
