///Register `DMTMD` reader
pub type R = crate::R<DMTMD_SPEC>;
///Register `DMTMD` writer
pub type W = crate::W<DMTMD_SPEC>;
/**Transfer Request Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCTG_A {
    ///0: Software
    _00 = 0,
    ///1: Interrupts from peripheral modules or external interrupt input pins
    _01 = 1,
    ///2: Setting prohibited
    OTHERS = 2,
}
impl From<DCTG_A> for u8 {
    #[inline(always)]
    fn from(variant: DCTG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCTG_A {
    type Ux = u8;
}
impl crate::IsEnum for DCTG_A {}
///Field `DCTG` reader - Transfer Request Source Select
pub type DCTG_R = crate::FieldReader<DCTG_A>;
impl DCTG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCTG_A {
        match self.bits {
            0 => DCTG_A::_00,
            1 => DCTG_A::_01,
            _ => DCTG_A::OTHERS,
        }
    }
    ///Software
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DCTG_A::_00
    }
    ///Interrupts from peripheral modules or external interrupt input pins
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DCTG_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DCTG_A::OTHERS)
    }
}
///Field `DCTG` writer - Transfer Request Source Select
pub type DCTG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DCTG_A, crate::Safe>;
impl<'a, REG> DCTG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Software
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::_00)
    }
    ///Interrupts from peripheral modules or external interrupt input pins
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DCTG_A::OTHERS)
    }
}
/**Transfer Data Size Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SZ_A {
    ///0: 8 bits
    _00 = 0,
    ///1: 16 bits
    _01 = 1,
    ///2: 32 bits
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SZ_A {
    type Ux = u8;
}
impl crate::IsEnum for SZ_A {}
///Field `SZ` reader - Transfer Data Size Select
pub type SZ_R = crate::FieldReader<SZ_A>;
impl SZ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SZ_A {
        match self.bits {
            0 => SZ_A::_00,
            1 => SZ_A::_01,
            2 => SZ_A::_10,
            3 => SZ_A::_11,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SZ_A::_00
    }
    ///16 bits
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SZ_A::_01
    }
    ///32 bits
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SZ_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SZ_A::_11
    }
}
///Field `SZ` writer - Transfer Data Size Select
pub type SZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SZ_A, crate::Safe>;
impl<'a, REG> SZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_00)
    }
    ///16 bits
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_01)
    }
    ///32 bits
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SZ_A::_11)
    }
}
/**Repeat Area Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTS_A {
    ///0: Specify destination as the repeat area or block area
    _00 = 0,
    ///1: Specify source as the repeat area or block area
    _01 = 1,
    ///2: Do not specify repeat area or block area
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<DTS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTS_A {
    type Ux = u8;
}
impl crate::IsEnum for DTS_A {}
///Field `DTS` reader - Repeat Area Select
pub type DTS_R = crate::FieldReader<DTS_A>;
impl DTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTS_A {
        match self.bits {
            0 => DTS_A::_00,
            1 => DTS_A::_01,
            2 => DTS_A::_10,
            3 => DTS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Specify destination as the repeat area or block area
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DTS_A::_00
    }
    ///Specify source as the repeat area or block area
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DTS_A::_01
    }
    ///Do not specify repeat area or block area
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTS_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DTS_A::_11
    }
}
///Field `DTS` writer - Repeat Area Select
pub type DTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DTS_A, crate::Safe>;
impl<'a, REG> DTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Specify destination as the repeat area or block area
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_00)
    }
    ///Specify source as the repeat area or block area
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_01)
    }
    ///Do not specify repeat area or block area
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DTS_A::_11)
    }
}
/**Transfer Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    ///0: Normal transfer
    _00 = 0,
    ///1: Repeat transfer
    _01 = 1,
    ///2: Block transfer
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MD_A {
    type Ux = u8;
}
impl crate::IsEnum for MD_A {}
///Field `MD` reader - Transfer Mode Select
pub type MD_R = crate::FieldReader<MD_A>;
impl MD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MD_A {
        match self.bits {
            0 => MD_A::_00,
            1 => MD_A::_01,
            2 => MD_A::_10,
            3 => MD_A::_11,
            _ => unreachable!(),
        }
    }
    ///Normal transfer
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MD_A::_00
    }
    ///Repeat transfer
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MD_A::_01
    }
    ///Block transfer
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MD_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MD_A::_11
    }
}
///Field `MD` writer - Transfer Mode Select
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MD_A, crate::Safe>;
impl<'a, REG> MD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal transfer
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_00)
    }
    ///Repeat transfer
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_01)
    }
    ///Block transfer
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Transfer Request Source Select
    #[inline(always)]
    pub fn dctg(&self) -> DCTG_R {
        DCTG_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - Transfer Data Size Select
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Repeat Area Select
    #[inline(always)]
    pub fn dts(&self) -> DTS_R {
        DTS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Transfer Mode Select
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Transfer Request Source Select
    #[inline(always)]
    pub fn dctg(&mut self) -> DCTG_W<DMTMD_SPEC> {
        DCTG_W::new(self, 0)
    }
    ///Bits 8:9 - Transfer Data Size Select
    #[inline(always)]
    pub fn sz(&mut self) -> SZ_W<DMTMD_SPEC> {
        SZ_W::new(self, 8)
    }
    ///Bits 12:13 - Repeat Area Select
    #[inline(always)]
    pub fn dts(&mut self) -> DTS_W<DMTMD_SPEC> {
        DTS_W::new(self, 12)
    }
    ///Bits 14:15 - Transfer Mode Select
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<DMTMD_SPEC> {
        MD_W::new(self, 14)
    }
}
/**DMA Transfer Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmtmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmtmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMTMD_SPEC;
impl crate::RegisterSpec for DMTMD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dmtmd::R`](R) reader structure
impl crate::Readable for DMTMD_SPEC {}
///`write(|w| ..)` method takes [`dmtmd::W`](W) writer structure
impl crate::Writable for DMTMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMTMD to value 0
impl crate::Resettable for DMTMD_SPEC {}
