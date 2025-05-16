///Register `JINTS0` reader
pub type R = crate::R<JINTS0_SPEC>;
///Register `JINTS0` writer
pub type W = crate::W<JINTS0_SPEC>;
/**Field `INS3` reader - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type INS3_R = crate::BitReader;
///Field `INS3` writer - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD.
pub type INS3_W<'a, REG> = crate::BitWriter0C<'a, REG>;
/**Field `INS5` reader - This bit is set to 1 when a compressed data error occurs.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type INS5_R = crate::BitReader;
///Field `INS5` writer - This bit is set to 1 when a compressed data error occurs.
pub type INS5_W<'a, REG> = crate::BitWriter0C<'a, REG>;
/**Field `INS6` reader - This bit is set to 1 when this module completes compression process normally.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type INS6_R = crate::BitReader;
///Field `INS6` writer - This bit is set to 1 when this module completes compression process normally.
pub type INS6_W<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    ///Bit 3 - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD.
    #[inline(always)]
    pub fn ins3(&self) -> INS3_R {
        INS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - This bit is set to 1 when a compressed data error occurs.
    #[inline(always)]
    pub fn ins5(&self) -> INS5_R {
        INS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - This bit is set to 1 when this module completes compression process normally.
    #[inline(always)]
    pub fn ins6(&self) -> INS6_R {
        INS6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD.
    #[inline(always)]
    pub fn ins3(&mut self) -> INS3_W<JINTS0_SPEC> {
        INS3_W::new(self, 3)
    }
    ///Bit 5 - This bit is set to 1 when a compressed data error occurs.
    #[inline(always)]
    pub fn ins5(&mut self) -> INS5_W<JINTS0_SPEC> {
        INS5_W::new(self, 5)
    }
    ///Bit 6 - This bit is set to 1 when this module completes compression process normally.
    #[inline(always)]
    pub fn ins6(&mut self) -> INS6_W<JINTS0_SPEC> {
        INS6_W::new(self, 6)
    }
}
/**JPEG Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`jints0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jints0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JINTS0_SPEC;
impl crate::RegisterSpec for JINTS0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jints0::R`](R) reader structure
impl crate::Readable for JINTS0_SPEC {}
///`write(|w| ..)` method takes [`jints0::W`](W) writer structure
impl crate::Writable for JINTS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x68;
}
///`reset()` method sets JINTS0 to value 0
impl crate::Resettable for JINTS0_SPEC {}
