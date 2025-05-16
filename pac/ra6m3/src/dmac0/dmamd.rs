///Register `DMAMD` reader
pub type R = crate::R<DMAMD_SPEC>;
///Register `DMAMD` writer
pub type W = crate::W<DMAMD_SPEC>;
///Field `DARA` reader - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings.
pub type DARA_R = crate::FieldReader;
///Field `DARA` writer - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings.
pub type DARA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Destination Address Update Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DM_A {
    ///0: Fixed address
    _00 = 0,
    ///1: Offset addition
    _01 = 1,
    ///2: Incremented address
    _10 = 2,
    ///3: Decremented address.
    _11 = 3,
}
impl From<DM_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DM_A {
    type Ux = u8;
}
impl crate::IsEnum for DM_A {}
///Field `DM` reader - Destination Address Update Mode
pub type DM_R = crate::FieldReader<DM_A>;
impl DM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DM_A {
        match self.bits {
            0 => DM_A::_00,
            1 => DM_A::_01,
            2 => DM_A::_10,
            3 => DM_A::_11,
            _ => unreachable!(),
        }
    }
    ///Fixed address
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DM_A::_00
    }
    ///Offset addition
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DM_A::_01
    }
    ///Incremented address
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DM_A::_10
    }
    ///Decremented address.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DM_A::_11
    }
}
///Field `DM` writer - Destination Address Update Mode
pub type DM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DM_A, crate::Safe>;
impl<'a, REG> DM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Fixed address
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::_00)
    }
    ///Offset addition
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::_01)
    }
    ///Incremented address
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::_10)
    }
    ///Decremented address.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::_11)
    }
}
///Field `SARA` reader - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings.
pub type SARA_R = crate::FieldReader;
///Field `SARA` writer - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings.
pub type SARA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Source Address Update Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    ///0: Fixed address
    _00 = 0,
    ///1: Offset addition
    _01 = 1,
    ///2: Incremented address
    _10 = 2,
    ///3: Decremented address.
    _11 = 3,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SM_A {
    type Ux = u8;
}
impl crate::IsEnum for SM_A {}
///Field `SM` reader - Source Address Update Mode
pub type SM_R = crate::FieldReader<SM_A>;
impl SM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::_00,
            1 => SM_A::_01,
            2 => SM_A::_10,
            3 => SM_A::_11,
            _ => unreachable!(),
        }
    }
    ///Fixed address
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SM_A::_00
    }
    ///Offset addition
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SM_A::_01
    }
    ///Incremented address
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SM_A::_10
    }
    ///Decremented address.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SM_A::_11
    }
}
///Field `SM` writer - Source Address Update Mode
pub type SM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SM_A, crate::Safe>;
impl<'a, REG> SM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Fixed address
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::_00)
    }
    ///Offset addition
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::_01)
    }
    ///Incremented address
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::_10)
    }
    ///Decremented address.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::_11)
    }
}
impl R {
    ///Bits 0:4 - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings.
    #[inline(always)]
    pub fn dara(&self) -> DARA_R {
        DARA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - Destination Address Update Mode
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings.
    #[inline(always)]
    pub fn sara(&self) -> SARA_R {
        SARA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 14:15 - Source Address Update Mode
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:4 - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings.
    #[inline(always)]
    pub fn dara(&mut self) -> DARA_W<DMAMD_SPEC> {
        DARA_W::new(self, 0)
    }
    ///Bits 6:7 - Destination Address Update Mode
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<DMAMD_SPEC> {
        DM_W::new(self, 6)
    }
    ///Bits 8:12 - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings.
    #[inline(always)]
    pub fn sara(&mut self) -> SARA_W<DMAMD_SPEC> {
        SARA_W::new(self, 8)
    }
    ///Bits 14:15 - Source Address Update Mode
    #[inline(always)]
    pub fn sm(&mut self) -> SM_W<DMAMD_SPEC> {
        SM_W::new(self, 14)
    }
}
/**DMA Address Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmamd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMAMD_SPEC;
impl crate::RegisterSpec for DMAMD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dmamd::R`](R) reader structure
impl crate::Readable for DMAMD_SPEC {}
///`write(|w| ..)` method takes [`dmamd::W`](W) writer structure
impl crate::Writable for DMAMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMD to value 0
impl crate::Resettable for DMAMD_SPEC {}
