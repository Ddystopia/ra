///Register `FDR` reader
pub type R = crate::R<FDR_SPEC>;
///Register `FDR` writer
pub type W = crate::W<FDR_SPEC>;
/**Transmit FIFO Depth

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFD_A {
    ///7: 2048 bytes
    _00111 = 7,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
}
impl From<RFD_A> for u8 {
    #[inline(always)]
    fn from(variant: RFD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RFD_A {
    type Ux = u8;
}
impl crate::IsEnum for RFD_A {}
///Field `RFD` reader - Transmit FIFO Depth
pub type RFD_R = crate::FieldReader<RFD_A>;
impl RFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFD_A {
        match self.bits {
            7 => RFD_A::_00111,
            _ => RFD_A::OTHERS,
        }
    }
    ///2048 bytes
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == RFD_A::_00111
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RFD_A::OTHERS)
    }
}
///Field `RFD` writer - Transmit FIFO Depth
pub type RFD_W<'a, REG> = crate::FieldWriter<'a, REG, 5, RFD_A, crate::Safe>;
impl<'a, REG> RFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2048 bytes
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(RFD_A::_00111)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RFD_A::OTHERS)
    }
}
/**Receive FIFO Depth

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFD_A {
    ///15: 4096 bytes
    _01111 = 15,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
}
impl From<TFD_A> for u8 {
    #[inline(always)]
    fn from(variant: TFD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFD_A {
    type Ux = u8;
}
impl crate::IsEnum for TFD_A {}
///Field `TFD` reader - Receive FIFO Depth
pub type TFD_R = crate::FieldReader<TFD_A>;
impl TFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFD_A {
        match self.bits {
            15 => TFD_A::_01111,
            _ => TFD_A::OTHERS,
        }
    }
    ///4096 bytes
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == TFD_A::_01111
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TFD_A::OTHERS)
    }
}
///Field `TFD` writer - Receive FIFO Depth
pub type TFD_W<'a, REG> = crate::FieldWriter<'a, REG, 5, TFD_A, crate::Safe>;
impl<'a, REG> TFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4096 bytes
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(TFD_A::_01111)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TFD_A::OTHERS)
    }
}
impl R {
    ///Bits 0:4 - Transmit FIFO Depth
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Receive FIFO Depth
    #[inline(always)]
    pub fn tfd(&self) -> TFD_R {
        TFD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Transmit FIFO Depth
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W<FDR_SPEC> {
        RFD_W::new(self, 0)
    }
    ///Bits 8:12 - Receive FIFO Depth
    #[inline(always)]
    pub fn tfd(&mut self) -> TFD_W<FDR_SPEC> {
        TFD_W::new(self, 8)
    }
}
/**Transmit FIFO Threshold Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fdr::R`](R) reader structure
impl crate::Readable for FDR_SPEC {}
///`write(|w| ..)` method takes [`fdr::W`](W) writer structure
impl crate::Writable for FDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDR to value 0
impl crate::Resettable for FDR_SPEC {}
