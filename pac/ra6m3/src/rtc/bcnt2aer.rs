///Register `BCNT2AER` reader
pub type R = crate::R<BCNT2AER_SPEC>;
///Register `BCNT2AER` writer
pub type W = crate::W<BCNT2AER_SPEC>;
///Field `ENB` reader - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16.
pub type ENB_R = crate::FieldReader;
///Field `ENB` writer - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16.
pub type ENB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16.
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16.
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<BCNT2AER_SPEC> {
        ENB_W::new(self, 0)
    }
}
/**Binary Counter 2 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt2aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT2AER_SPEC;
impl crate::RegisterSpec for BCNT2AER_SPEC {
    type Ux = u16;
}
///`read()` method returns [`bcnt2aer::R`](R) reader structure
impl crate::Readable for BCNT2AER_SPEC {}
///`write(|w| ..)` method takes [`bcnt2aer::W`](W) writer structure
impl crate::Writable for BCNT2AER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT2AER to value 0
impl crate::Resettable for BCNT2AER_SPEC {}
