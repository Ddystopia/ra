///Register `FDR` reader
pub type R = crate::R<FDR_SPEC>;
///Field `R` reader - Receive FIFO Data CountIndicate the quantity of receive data stored in FRDRH and FRDRL(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)
pub type R_R = crate::FieldReader;
///Field `T` reader - Transmit FIFO Data CountIndicate the quantity of non-transmit data stored in FTDRH and FTDRL(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)
pub type T_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - Receive FIFO Data CountIndicate the quantity of receive data stored in FRDRH and FRDRL(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Transmit FIFO Data CountIndicate the quantity of non-transmit data stored in FTDRH and FTDRL(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
/**FIFO Data Count Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`fdr::R`](R) reader structure
impl crate::Readable for FDR_SPEC {}
///`reset()` method sets FDR to value 0
impl crate::Resettable for FDR_SPEC {}
