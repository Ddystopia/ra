///Register `MB%s_TS` reader
pub type R = crate::R<MB_TS_SPEC>;
///Register `MB%s_TS` writer
pub type W = crate::W<MB_TS_SPEC>;
///Field `TSL` reader - Time Stamp Higher ByteBits TSL\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
pub type TSL_R = crate::FieldReader;
///Field `TSL` writer - Time Stamp Higher ByteBits TSL\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
pub type TSL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TSH` reader - Time Stamp Lower ByteBits TSH\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
pub type TSH_R = crate::FieldReader;
///Field `TSH` writer - Time Stamp Lower ByteBits TSH\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
pub type TSH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Time Stamp Higher ByteBits TSL\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
    #[inline(always)]
    pub fn tsl(&self) -> TSL_R {
        TSL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Time Stamp Lower ByteBits TSH\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
    #[inline(always)]
    pub fn tsh(&self) -> TSH_R {
        TSH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Time Stamp Higher ByteBits TSL\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
    #[inline(always)]
    pub fn tsl(&mut self) -> TSL_W<MB_TS_SPEC> {
        TSL_W::new(self, 0)
    }
    ///Bits 8:15 - Time Stamp Lower ByteBits TSH\[7:0\] store the counter value of the time stamp when received messages are stored in the mailbox.
    #[inline(always)]
    pub fn tsh(&mut self) -> TSH_W<MB_TS_SPEC> {
        TSH_W::new(self, 8)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_ts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_ts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_TS_SPEC;
impl crate::RegisterSpec for MB_TS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mb_ts::R`](R) reader structure
impl crate::Readable for MB_TS_SPEC {}
///`write(|w| ..)` method takes [`mb_ts::W`](W) writer structure
impl crate::Writable for MB_TS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_TS to value 0
impl crate::Resettable for MB_TS_SPEC {}
