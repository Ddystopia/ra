///Register `JIFESLC` reader
pub type R = crate::R<JIFESLC_SPEC>;
///Register `JIFESLC` writer
pub type W = crate::W<JIFESLC_SPEC>;
///Field `LINES` reader - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0.
pub type LINES_R = crate::FieldReader<u16>;
///Field `LINES` writer - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0.
pub type LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn lines(&self) -> LINES_R {
        LINES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn lines(&mut self) -> LINES_W<JIFESLC_SPEC> {
        LINES_W::new(self, 0)
    }
}
/**JPEG Interface Compression Source Line Count Register

You can [`read`](crate::Reg::read) this register and get [`jifeslc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifeslc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFESLC_SPEC;
impl crate::RegisterSpec for JIFESLC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifeslc::R`](R) reader structure
impl crate::Readable for JIFESLC_SPEC {}
///`write(|w| ..)` method takes [`jifeslc::W`](W) writer structure
impl crate::Writable for JIFESLC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFESLC to value 0xfff8_fff8
impl crate::Resettable for JIFESLC_SPEC {
    const RESET_VALUE: u32 = 0xfff8_fff8;
}
