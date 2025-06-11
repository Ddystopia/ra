///Register `SSISCR` reader
pub type R = crate::R<SSISCR_SPEC>;
///Register `SSISCR` writer
pub type W = crate::W<SSISCR_SPEC>;
/**RDF Setting Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDFS_A {
    ///0: SSIFRDR has one stage or more data size
    _000 = 0,
    ///1: SSIFRDR has two stages or more data size (snip)
    _001 = 1,
    ///6: SSIFRDR has seven stages or more data size
    _110 = 6,
    ///7: SSIFRDR has eight stages or more data size.
    _111 = 7,
}
impl From<RDFS_A> for u8 {
    #[inline(always)]
    fn from(variant: RDFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDFS_A {
    type Ux = u8;
}
impl crate::IsEnum for RDFS_A {}
///Field `RDFS` reader - RDF Setting Condition Select
pub type RDFS_R = crate::FieldReader<RDFS_A>;
impl RDFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDFS_A> {
        match self.bits {
            0 => Some(RDFS_A::_000),
            1 => Some(RDFS_A::_001),
            6 => Some(RDFS_A::_110),
            7 => Some(RDFS_A::_111),
            _ => None,
        }
    }
    ///SSIFRDR has one stage or more data size
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RDFS_A::_000
    }
    ///SSIFRDR has two stages or more data size (snip)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RDFS_A::_001
    }
    ///SSIFRDR has seven stages or more data size
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RDFS_A::_110
    }
    ///SSIFRDR has eight stages or more data size.
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RDFS_A::_111
    }
}
///Field `RDFS` writer - RDF Setting Condition Select
pub type RDFS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RDFS_A>;
impl<'a, REG> RDFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SSIFRDR has one stage or more data size
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(RDFS_A::_000)
    }
    ///SSIFRDR has two stages or more data size (snip)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(RDFS_A::_001)
    }
    ///SSIFRDR has seven stages or more data size
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(RDFS_A::_110)
    }
    ///SSIFRDR has eight stages or more data size.
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(RDFS_A::_111)
    }
}
/**TDE Setting Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TDES_A {
    ///0: SSIFTDR has one stage or more free space
    _000 = 0,
    ///1: SSIFTDR has two stages or more free space (snip)
    _001 = 1,
    ///6: SSIFTDR has seven stages or more free space
    _110 = 6,
    ///7: SSIFTDR has eight stages or more free space.
    _111 = 7,
}
impl From<TDES_A> for u8 {
    #[inline(always)]
    fn from(variant: TDES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TDES_A {
    type Ux = u8;
}
impl crate::IsEnum for TDES_A {}
///Field `TDES` reader - TDE Setting Condition Select
pub type TDES_R = crate::FieldReader<TDES_A>;
impl TDES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TDES_A> {
        match self.bits {
            0 => Some(TDES_A::_000),
            1 => Some(TDES_A::_001),
            6 => Some(TDES_A::_110),
            7 => Some(TDES_A::_111),
            _ => None,
        }
    }
    ///SSIFTDR has one stage or more free space
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TDES_A::_000
    }
    ///SSIFTDR has two stages or more free space (snip)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TDES_A::_001
    }
    ///SSIFTDR has seven stages or more free space
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TDES_A::_110
    }
    ///SSIFTDR has eight stages or more free space.
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == TDES_A::_111
    }
}
///Field `TDES` writer - TDE Setting Condition Select
pub type TDES_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TDES_A>;
impl<'a, REG> TDES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SSIFTDR has one stage or more free space
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(TDES_A::_000)
    }
    ///SSIFTDR has two stages or more free space (snip)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(TDES_A::_001)
    }
    ///SSIFTDR has seven stages or more free space
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(TDES_A::_110)
    }
    ///SSIFTDR has eight stages or more free space.
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(TDES_A::_111)
    }
}
impl R {
    ///Bits 0:2 - RDF Setting Condition Select
    #[inline(always)]
    pub fn rdfs(&self) -> RDFS_R {
        RDFS_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - TDE Setting Condition Select
    #[inline(always)]
    pub fn tdes(&self) -> TDES_R {
        TDES_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - RDF Setting Condition Select
    #[inline(always)]
    pub fn rdfs(&mut self) -> RDFS_W<SSISCR_SPEC> {
        RDFS_W::new(self, 0)
    }
    ///Bits 8:10 - TDE Setting Condition Select
    #[inline(always)]
    pub fn tdes(&mut self) -> TDES_W<SSISCR_SPEC> {
        TDES_W::new(self, 8)
    }
}
/**Status Control Register

You can [`read`](crate::Reg::read) this register and get [`ssiscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSISCR_SPEC;
impl crate::RegisterSpec for SSISCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssiscr::R`](R) reader structure
impl crate::Readable for SSISCR_SPEC {}
///`write(|w| ..)` method takes [`ssiscr::W`](W) writer structure
impl crate::Writable for SSISCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSISCR to value 0
impl crate::Resettable for SSISCR_SPEC {}
