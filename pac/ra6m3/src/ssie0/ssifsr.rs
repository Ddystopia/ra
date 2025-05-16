///Register `SSIFSR` reader
pub type R = crate::R<SSIFSR_SPEC>;
///Register `SSIFSR` writer
pub type W = crate::W<SSIFSR_SPEC>;
/**Receive Data Full Flag NOTE: Since the SSIFRDR register is a 32-byte FIFO register, the maximum number of data bytes that can be read from it while the RDF flag is 1 is indicated in the RDC\[3:0\] flags. If reading data from the SSIFRDR register is continued after all the data is read, undefined values will be read.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    ///0: Number of received data bytes in SSIFRDR is less than the set receive trigger number.
    _0 = 0,
    ///1: Number of received data bytes in SSIFRDR is equal to or greater than the set receive trigger number.
    _1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `RDF` reader - Receive Data Full Flag NOTE: Since the SSIFRDR register is a 32-byte FIFO register, the maximum number of data bytes that can be read from it while the RDF flag is 1 is indicated in the RDC\[3:0\] flags. If reading data from the SSIFRDR register is continued after all the data is read, undefined values will be read.

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
    ///Number of received data bytes in SSIFRDR is less than the set receive trigger number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDF_A::_0
    }
    ///Number of received data bytes in SSIFRDR is equal to or greater than the set receive trigger number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDF_A::_1
    }
}
///Field `RDF` writer - Receive Data Full Flag NOTE: Since the SSIFRDR register is a 32-byte FIFO register, the maximum number of data bytes that can be read from it while the RDF flag is 1 is indicated in the RDC\[3:0\] flags. If reading data from the SSIFRDR register is continued after all the data is read, undefined values will be read.
pub type RDF_W<'a, REG> = crate::BitWriter0C<'a, REG, RDF_A>;
impl<'a, REG> RDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of received data bytes in SSIFRDR is less than the set receive trigger number.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_0)
    }
    ///Number of received data bytes in SSIFRDR is equal to or greater than the set receive trigger number.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDF_A::_1)
    }
}
///Field `RDC` reader - Receive Data Indicate Flag(Indicates the number of data units stored in SSIFRDR)
pub type RDC_R = crate::FieldReader;
/**Transmit Data Empty Flag NOTE: Since the SSIFTDR register is a 32-byte FIFO register, the maximum number of bytes that can be written to it while the TDE flag is 1 is 8 - TDC\[3:0\]. If writing data to the SSIFTDR register is continued after all the data is written, writing will be invalid and an overflow occurs.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    ///0: Number of data bytes for transmission in SSIFTDR is greater than the set transmit trigger number.
    _0 = 0,
    ///1: Number of data bytes for transmission in SSIFTDR is equal to or less than the set transmit trigger number.
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `TDE` reader - Transmit Data Empty Flag NOTE: Since the SSIFTDR register is a 32-byte FIFO register, the maximum number of bytes that can be written to it while the TDE flag is 1 is 8 - TDC\[3:0\]. If writing data to the SSIFTDR register is continued after all the data is written, writing will be invalid and an overflow occurs.

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
    ///Number of data bytes for transmission in SSIFTDR is greater than the set transmit trigger number.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    ///Number of data bytes for transmission in SSIFTDR is equal to or less than the set transmit trigger number.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
///Field `TDE` writer - Transmit Data Empty Flag NOTE: Since the SSIFTDR register is a 32-byte FIFO register, the maximum number of bytes that can be written to it while the TDE flag is 1 is 8 - TDC\[3:0\]. If writing data to the SSIFTDR register is continued after all the data is written, writing will be invalid and an overflow occurs.
pub type TDE_W<'a, REG> = crate::BitWriter0C<'a, REG, TDE_A>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data bytes for transmission in SSIFTDR is greater than the set transmit trigger number.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_0)
    }
    ///Number of data bytes for transmission in SSIFTDR is equal to or less than the set transmit trigger number.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::_1)
    }
}
///Field `TDC` reader - Transmit Data Indicate Flag(Indicates the number of data units stored in SSIFTDR)
pub type TDC_R = crate::FieldReader;
impl R {
    ///Bit 0 - Receive Data Full Flag NOTE: Since the SSIFRDR register is a 32-byte FIFO register, the maximum number of data bytes that can be read from it while the RDF flag is 1 is indicated in the RDC\[3:0\] flags. If reading data from the SSIFRDR register is continued after all the data is read, undefined values will be read.
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - Receive Data Indicate Flag(Indicates the number of data units stored in SSIFRDR)
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - Transmit Data Empty Flag NOTE: Since the SSIFTDR register is a 32-byte FIFO register, the maximum number of bytes that can be written to it while the TDE flag is 1 is 8 - TDC\[3:0\]. If writing data to the SSIFTDR register is continued after all the data is written, writing will be invalid and an overflow occurs.
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 24:27 - Transmit Data Indicate Flag(Indicates the number of data units stored in SSIFTDR)
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Receive Data Full Flag NOTE: Since the SSIFRDR register is a 32-byte FIFO register, the maximum number of data bytes that can be read from it while the RDF flag is 1 is indicated in the RDC\[3:0\] flags. If reading data from the SSIFRDR register is continued after all the data is read, undefined values will be read.
    #[inline(always)]
    pub fn rdf(&mut self) -> RDF_W<SSIFSR_SPEC> {
        RDF_W::new(self, 0)
    }
    ///Bit 16 - Transmit Data Empty Flag NOTE: Since the SSIFTDR register is a 32-byte FIFO register, the maximum number of bytes that can be written to it while the TDE flag is 1 is 8 - TDC\[3:0\]. If writing data to the SSIFTDR register is continued after all the data is written, writing will be invalid and an overflow occurs.
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<SSIFSR_SPEC> {
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
