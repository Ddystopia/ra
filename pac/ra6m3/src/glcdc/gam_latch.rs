///Register `GAM%s_LATCH` reader
pub type R = crate::R<GAM_LATCH_SPEC>;
///Register `GAM%s_LATCH` writer
pub type W = crate::W<GAM_LATCH_SPEC>;
/**Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEN_A {
    ///1: Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    _1 = 1,
    ///0: Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    _0 = 0,
}
impl From<VEN_A> for bool {
    #[inline(always)]
    fn from(variant: VEN_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VEN` reader - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VEN_R = crate::BitReader<VEN_A>;
impl VEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VEN_A {
        match self.bits {
            true => VEN_A::_1,
            false => VEN_A::_0,
        }
    }
    ///Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VEN_A::_1
    }
    ///Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VEN_A::_0
    }
}
///Field `VEN` writer - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
pub type VEN_W<'a, REG> = crate::BitWriter0C<'a, REG, VEN_A>;
impl<'a, REG> VEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VEN_A::_1)
    }
    ///Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VEN_A::_0)
    }
}
impl R {
    ///Bit 0 - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn ven(&mut self) -> VEN_W<GAM_LATCH_SPEC> {
        VEN_W::new(self, 0)
    }
}
/**Gamma %s Register Update Control Register

You can [`read`](crate::Reg::read) this register and get [`gam_latch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_latch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAM_LATCH_SPEC;
impl crate::RegisterSpec for GAM_LATCH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gam_latch::R`](R) reader structure
impl crate::Readable for GAM_LATCH_SPEC {}
///`write(|w| ..)` method takes [`gam_latch::W`](W) writer structure
impl crate::Writable for GAM_LATCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
///`reset()` method sets GAM%s_LATCH to value 0
impl crate::Resettable for GAM_LATCH_SPEC {}
