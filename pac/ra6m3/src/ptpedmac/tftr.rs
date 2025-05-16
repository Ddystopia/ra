///Register `TFTR` reader
pub type R = crate::R<TFTR_SPEC>;
///Register `TFTR` writer
pub type W = crate::W<TFTR_SPEC>;
/**Transmit FIFO Threshold00Dh to 200h: The threshold is the set value multiplied by 4. Example: 00Dh: 52 bytes 040h: 256 bytes 100h: 1024 bytes 200h: 2048 bytes

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TFT_A {
    ///0: Store and forward mode
    _0X000 = 0,
    ///1: The threshold is the set value multiplied by 4. (001h to 00Ch and 201h to 7FFh: Setting prohibited)
    OTHERS = 1,
}
impl From<TFT_A> for u16 {
    #[inline(always)]
    fn from(variant: TFT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFT_A {
    type Ux = u16;
}
impl crate::IsEnum for TFT_A {}
///Field `TFT` reader - Transmit FIFO Threshold00Dh to 200h: The threshold is the set value multiplied by 4. Example: 00Dh: 52 bytes 040h: 256 bytes 100h: 1024 bytes 200h: 2048 bytes
pub type TFT_R = crate::FieldReader<TFT_A>;
impl TFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFT_A {
        match self.bits {
            0 => TFT_A::_0X000,
            _ => TFT_A::OTHERS,
        }
    }
    ///Store and forward mode
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == TFT_A::_0X000
    }
    ///The threshold is the set value multiplied by 4. (001h to 00Ch and 201h to 7FFh: Setting prohibited)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TFT_A::OTHERS)
    }
}
///Field `TFT` writer - Transmit FIFO Threshold00Dh to 200h: The threshold is the set value multiplied by 4. Example: 00Dh: 52 bytes 040h: 256 bytes 100h: 1024 bytes 200h: 2048 bytes
pub type TFT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, TFT_A, crate::Safe>;
impl<'a, REG> TFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Store and forward mode
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(TFT_A::_0X000)
    }
    ///The threshold is the set value multiplied by 4. (001h to 00Ch and 201h to 7FFh: Setting prohibited)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TFT_A::OTHERS)
    }
}
impl R {
    ///Bits 0:10 - Transmit FIFO Threshold00Dh to 200h: The threshold is the set value multiplied by 4. Example: 00Dh: 52 bytes 040h: 256 bytes 100h: 1024 bytes 200h: 2048 bytes
    #[inline(always)]
    pub fn tft(&self) -> TFT_R {
        TFT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Transmit FIFO Threshold00Dh to 200h: The threshold is the set value multiplied by 4. Example: 00Dh: 52 bytes 040h: 256 bytes 100h: 1024 bytes 200h: 2048 bytes
    #[inline(always)]
    pub fn tft(&mut self) -> TFT_W<TFTR_SPEC> {
        TFT_W::new(self, 0)
    }
}
/**Transmit FIFO Threshold Register

You can [`read`](crate::Reg::read) this register and get [`tftr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TFTR_SPEC;
impl crate::RegisterSpec for TFTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tftr::R`](R) reader structure
impl crate::Readable for TFTR_SPEC {}
///`write(|w| ..)` method takes [`tftr::W`](W) writer structure
impl crate::Writable for TFTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TFTR to value 0
impl crate::Resettable for TFTR_SPEC {}
