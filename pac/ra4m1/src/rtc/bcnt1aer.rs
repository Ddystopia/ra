///Register `BCNT1AER` reader
pub type R = crate::R<BCNT1AER_SPEC>;
///Register `BCNT1AER` writer
pub type W = crate::W<BCNT1AER_SPEC>;
///Field `ENB` reader - The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8.
pub type ENB_R = crate::FieldReader;
///Field `ENB` writer - The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8.
pub type ENB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8.
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8.
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<BCNT1AER_SPEC> {
        ENB_W::new(self, 0)
    }
}
/**Binary Counter 1 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt1aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT1AER_SPEC;
impl crate::RegisterSpec for BCNT1AER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt1aer::R`](R) reader structure
impl crate::Readable for BCNT1AER_SPEC {}
///`write(|w| ..)` method takes [`bcnt1aer::W`](W) writer structure
impl crate::Writable for BCNT1AER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT1AER to value 0
impl crate::Resettable for BCNT1AER_SPEC {}
