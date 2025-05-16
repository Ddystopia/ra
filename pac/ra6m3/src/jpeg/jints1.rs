///Register `JINTS1` reader
pub type R = crate::R<JINTS1_SPEC>;
///Register `JINTS1` writer
pub type W = crate::W<JINTS1_SPEC>;
/**Field `DOUTLF` reader - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DOUTLF_R = crate::BitReader;
///Field `DOUTLF` writer - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1.
pub type DOUTLF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `JINF` reader - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type JINF_R = crate::BitReader;
///Field `JINF` writer - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1.
pub type JINF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `DBTF` reader - This bit is set to 1 when the last output image data is written in decompression.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DBTF_R = crate::BitReader;
///Field `DBTF` writer - This bit is set to 1 when the last output image data is written in decompression.
pub type DBTF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `DINLF` reader - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DINLF_R = crate::BitReader;
///Field `DINLF` writer - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1.
pub type DINLF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `CBTF` reader - This bit is set to 1 when the last output coded data is written in compression.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CBTF_R = crate::BitReader;
///Field `CBTF` writer - This bit is set to 1 when the last output coded data is written in compression.
pub type CBTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1.
    #[inline(always)]
    pub fn doutlf(&self) -> DOUTLF_R {
        DOUTLF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1.
    #[inline(always)]
    pub fn jinf(&self) -> JINF_R {
        JINF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - This bit is set to 1 when the last output image data is written in decompression.
    #[inline(always)]
    pub fn dbtf(&self) -> DBTF_R {
        DBTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1.
    #[inline(always)]
    pub fn dinlf(&self) -> DINLF_R {
        DINLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - This bit is set to 1 when the last output coded data is written in compression.
    #[inline(always)]
    pub fn cbtf(&self) -> CBTF_R {
        CBTF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1.
    #[inline(always)]
    pub fn doutlf(&mut self) -> DOUTLF_W<JINTS1_SPEC> {
        DOUTLF_W::new(self, 0)
    }
    ///Bit 1 - This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression. This bit is valid only when the JINC bit in JIFDCNT is set to 1.
    #[inline(always)]
    pub fn jinf(&mut self) -> JINF_W<JINTS1_SPEC> {
        JINF_W::new(self, 1)
    }
    ///Bit 2 - This bit is set to 1 when the last output image data is written in decompression.
    #[inline(always)]
    pub fn dbtf(&mut self) -> DBTF_W<JINTS1_SPEC> {
        DBTF_W::new(self, 2)
    }
    ///Bit 5 - This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression. This bit is valid only when the DINLC bit in JIFECNT is set to 1.
    #[inline(always)]
    pub fn dinlf(&mut self) -> DINLF_W<JINTS1_SPEC> {
        DINLF_W::new(self, 5)
    }
    ///Bit 6 - This bit is set to 1 when the last output coded data is written in compression.
    #[inline(always)]
    pub fn cbtf(&mut self) -> CBTF_W<JINTS1_SPEC> {
        CBTF_W::new(self, 6)
    }
}
/**JPEG Interrupt Status Register 1

You can [`read`](crate::Reg::read) this register and get [`jints1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jints1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JINTS1_SPEC;
impl crate::RegisterSpec for JINTS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jints1::R`](R) reader structure
impl crate::Readable for JINTS1_SPEC {}
///`write(|w| ..)` method takes [`jints1::W`](W) writer structure
impl crate::Writable for JINTS1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JINTS1 to value 0
impl crate::Resettable for JINTS1_SPEC {}
