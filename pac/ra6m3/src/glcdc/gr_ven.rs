///Register `GR%s_VEN` reader
pub type R = crate::R<GR_VEN_SPEC>;
///Register `GR%s_VEN` writer
pub type W = crate::W<GR_VEN_SPEC>;
/**Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVEN_A {
    ///1: Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    _1 = 1,
    ///0: Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    _0 = 0,
}
impl From<PVEN_A> for bool {
    #[inline(always)]
    fn from(variant: PVEN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PVEN` reader - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PVEN_R = crate::BitReader<PVEN_A>;
impl PVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVEN_A {
        match self.bits {
            true => PVEN_A::_1,
            false => PVEN_A::_0,
        }
    }
    ///Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PVEN_A::_1
    }
    ///Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PVEN_A::_0
    }
}
///Field `PVEN` writer - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
pub type PVEN_W<'a, REG> = crate::BitWriter0C<'a, REG, PVEN_A>;
impl<'a, REG> PVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PVEN_A::_1)
    }
    ///Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PVEN_A::_0)
    }
}
impl R {
    ///Bit 0 - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn pven(&self) -> PVEN_R {
        PVEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn pven(&mut self) -> PVEN_W<GR_VEN_SPEC> {
        PVEN_W::new(self, 0)
    }
}
/**Graphics %s Register Update Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_ven::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ven::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_VEN_SPEC;
impl crate::RegisterSpec for GR_VEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ven::R`](R) reader structure
impl crate::Readable for GR_VEN_SPEC {}
///`write(|w| ..)` method takes [`gr_ven::W`](W) writer structure
impl crate::Writable for GR_VEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
///`reset()` method sets GR%s_VEN to value 0
impl crate::Resettable for GR_VEN_SPEC {}
