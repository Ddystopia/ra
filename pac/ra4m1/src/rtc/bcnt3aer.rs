///Register `BCNT3AER` reader
pub type R = crate::R<BCNT3AER_SPEC>;
///Register `BCNT3AER` writer
pub type W = crate::W<BCNT3AER_SPEC>;
///Field `ENB` reader - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24.
pub type ENB_R = crate::FieldReader;
///Field `ENB` writer - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24.
pub type ENB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24.
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24.
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<BCNT3AER_SPEC> {
        ENB_W::new(self, 0)
    }
}
/**Binary Counter 3 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt3aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT3AER_SPEC;
impl crate::RegisterSpec for BCNT3AER_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt3aer::R`](R) reader structure
impl crate::Readable for BCNT3AER_SPEC {}
///`write(|w| ..)` method takes [`bcnt3aer::W`](W) writer structure
impl crate::Writable for BCNT3AER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCNT3AER to value 0
impl crate::Resettable for BCNT3AER_SPEC {}
