///Register `SSIFSR` reader
pub type R = crate::R<SSIFSR_SPEC>;
///Register `SSIFSR` writer
pub type W = crate::W<SSIFSR_SPEC>;
/**Receive Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    ///0: The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS
    _0 = 0,
    ///1: The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one.
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RDF` reader - Receive Data Full Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type RDF_R = crate::BitReader<RDF_A>;
impl RDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::_0,
            true => RDF_A::_1,
        }
    }
    ///The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    ///The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
///Field `RDF` writer - Receive Data Full Flag
pub type RDF_W<'a, REG> = crate::BitWriter0C<'a, REG, RDF_A>;
impl<'a, REG> RDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The size of received data in SSIFRDR is not more than the value of SSISCR.RDFS
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_0)
    }
    ///The size of received data in SSIFRDR is not less than the value of SSISCR.RDFS plus one.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_1)
    }
}
///Field `RDC` reader - Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag.
pub type RDC_R = crate::FieldReader;
/**Transmit Data Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    ///0: The free space of SSIFTDR is not more than the value of SSISCR.TDES
    _0 = 0,
    ///1: The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one.
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TDE` reader - Transmit Data Empty Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type TDE_R = crate::BitReader<TDE_A>;
impl TDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    ///The free space of SSIFTDR is not more than the value of SSISCR.TDES
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    ///The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
///Field `TDE` writer - Transmit Data Empty Flag
pub type TDE_W<'a, REG> = crate::BitWriter0C<'a, REG, TDE_A>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The free space of SSIFTDR is not more than the value of SSISCR.TDES
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    ///The free space of SSIFTDR is not less than the value of SSISCR.TDES plus one.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
///Field `TDC` reader - Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag.
pub type TDC_R = crate::FieldReader;
impl R {
    ///Bit 0 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - Number of Receive FIFO Data Indication Flag Number of receive FIFO data indication flag.
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 24:27 - Number of Transmit FIFO Data Indication Flag Number of transmit FIFO data indication flag.
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdf(&mut self) -> RDF_W<'_, SSIFSR_SPEC> {
        RDF_W::new(self, 0)
    }
    ///Bit 16 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<'_, SSIFSR_SPEC> {
        TDE_W::new(self, 16)
    }
}
/**FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`ssifsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSIFSR_SPEC;
impl crate::RegisterSpec for SSIFSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssifsr::R`](R) reader structure
impl crate::Readable for SSIFSR_SPEC {}
///`write(|w| ..)` method takes [`ssifsr::W`](W) writer structure
impl crate::Writable for SSIFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0001;
}
///`reset()` method sets SSIFSR to value 0x0001_0000
impl crate::Resettable for SSIFSR_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
