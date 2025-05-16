///Register `OUT_VLATCH` reader
pub type R = crate::R<OUT_VLATCH_SPEC>;
///Register `OUT_VLATCH` writer
pub type W = crate::W<OUT_VLATCH_SPEC>;
/**Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).

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
/**Field `VEN` reader - Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).

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
///Field `VEN` writer - Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
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
    ///Bit 0 - Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).
    #[inline(always)]
    pub fn ven(&mut self) -> VEN_W<OUT_VLATCH_SPEC> {
        VEN_W::new(self, 0)
    }
}
/**Output Control Block Register Update Control Register

You can [`read`](crate::Reg::read) this register and get [`out_vlatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_vlatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_VLATCH_SPEC;
impl crate::RegisterSpec for OUT_VLATCH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_vlatch::R`](R) reader structure
impl crate::Readable for OUT_VLATCH_SPEC {}
///`write(|w| ..)` method takes [`out_vlatch::W`](W) writer structure
impl crate::Writable for OUT_VLATCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
///`reset()` method sets OUT_VLATCH to value 0
impl crate::Resettable for OUT_VLATCH_SPEC {}
