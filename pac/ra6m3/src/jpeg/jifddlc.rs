///Register `JIFDDLC` reader
pub type R = crate::R<JIFDDLC_SPEC>;
///Register `JIFDDLC` writer
pub type W = crate::W<JIFDDLC_SPEC>;
///Field `LINES` reader - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units.
pub type LINES_R = crate::FieldReader<u16>;
///Field `LINES` writer - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units.
pub type LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units.
    #[inline(always)]
    pub fn lines(&self) -> LINES_R {
        LINES_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units.
    #[inline(always)]
    pub fn lines(&mut self) -> LINES_W<JIFDDLC_SPEC> {
        LINES_W::new(self, 0)
    }
}
/**JPEG Interface Decompression Destination Line Count Register

You can [`read`](crate::Reg::read) this register and get [`jifddlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifddlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDDLC_SPEC;
impl crate::RegisterSpec for JIFDDLC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifddlc::R`](R) reader structure
impl crate::Readable for JIFDDLC_SPEC {}
///`write(|w| ..)` method takes [`jifddlc::W`](W) writer structure
impl crate::Writable for JIFDDLC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDDLC to value 0xfff8_fff8
impl crate::Resettable for JIFDDLC_SPEC {
    const RESET_VALUE: u32 = 0xfff8_fff8;
}
