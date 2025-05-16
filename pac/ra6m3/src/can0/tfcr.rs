///Register `TFCR` reader
pub type R = crate::R<TFCR_SPEC>;
///Register `TFCR` writer
pub type W = crate::W<TFCR_SPEC>;
/**Transmit FIFO Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    ///0: Transmit FIFO disabled
    _0 = 0,
    ///1: Transmit FIFO enabled
    _1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFE` reader - Transmit FIFO Enable
pub type TFE_R = crate::BitReader<TFE_A>;
impl TFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::_0,
            true => TFE_A::_1,
        }
    }
    ///Transmit FIFO disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFE_A::_0
    }
    ///Transmit FIFO enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFE_A::_1
    }
}
///Field `TFE` writer - Transmit FIFO Enable
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG, TFE_A>;
impl<'a, REG> TFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit FIFO disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFE_A::_0)
    }
    ///Transmit FIFO enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFE_A::_1)
    }
}
/**Transmit FIFO Unsent Message Number Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFUST_A {
    ///0: No unsent message
    _000 = 0,
    ///1: 1 unsent message
    _001 = 1,
    ///2: 2 unsent messages
    _010 = 2,
    ///3: 3 unsent messages
    _011 = 3,
    ///4: 4 unsent messages
    _100 = 4,
    ///5: Setting prohibited
    OTHERS = 5,
}
impl From<TFUST_A> for u8 {
    #[inline(always)]
    fn from(variant: TFUST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFUST_A {
    type Ux = u8;
}
impl crate::IsEnum for TFUST_A {}
///Field `TFUST` reader - Transmit FIFO Unsent Message Number Status
pub type TFUST_R = crate::FieldReader<TFUST_A>;
impl TFUST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFUST_A {
        match self.bits {
            0 => TFUST_A::_000,
            1 => TFUST_A::_001,
            2 => TFUST_A::_010,
            3 => TFUST_A::_011,
            4 => TFUST_A::_100,
            _ => TFUST_A::OTHERS,
        }
    }
    ///No unsent message
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TFUST_A::_000
    }
    ///1 unsent message
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TFUST_A::_001
    }
    ///2 unsent messages
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TFUST_A::_010
    }
    ///3 unsent messages
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TFUST_A::_011
    }
    ///4 unsent messages
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TFUST_A::_100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TFUST_A::OTHERS)
    }
}
/**Transmit FIFO Full Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFST_A {
    ///0: Transmit FIFO is not full
    _0 = 0,
    ///1: Transmit FIFO is full (4 unsent messages)
    _1 = 1,
}
impl From<TFFST_A> for bool {
    #[inline(always)]
    fn from(variant: TFFST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFFST` reader - Transmit FIFO Full Status
pub type TFFST_R = crate::BitReader<TFFST_A>;
impl TFFST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFFST_A {
        match self.bits {
            false => TFFST_A::_0,
            true => TFFST_A::_1,
        }
    }
    ///Transmit FIFO is not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFFST_A::_0
    }
    ///Transmit FIFO is full (4 unsent messages)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFFST_A::_1
    }
}
/**Transmit FIFO Empty Status

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFEST_A {
    ///0: Unsent message in transmit FIFO
    _0 = 0,
    ///1: No unsent message in transmit FIFO
    _1 = 1,
}
impl From<TFEST_A> for bool {
    #[inline(always)]
    fn from(variant: TFEST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFEST` reader - Transmit FIFO Empty Status
pub type TFEST_R = crate::BitReader<TFEST_A>;
impl TFEST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFEST_A {
        match self.bits {
            false => TFEST_A::_0,
            true => TFEST_A::_1,
        }
    }
    ///Unsent message in transmit FIFO
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFEST_A::_0
    }
    ///No unsent message in transmit FIFO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFEST_A::_1
    }
}
impl R {
    ///Bit 0 - Transmit FIFO Enable
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Transmit FIFO Unsent Message Number Status
    #[inline(always)]
    pub fn tfust(&self) -> TFUST_R {
        TFUST_R::new((self.bits >> 1) & 7)
    }
    ///Bit 6 - Transmit FIFO Full Status
    #[inline(always)]
    pub fn tffst(&self) -> TFFST_R {
        TFFST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit FIFO Empty Status
    #[inline(always)]
    pub fn tfest(&self) -> TFEST_R {
        TFEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit FIFO Enable
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<TFCR_SPEC> {
        TFE_W::new(self, 0)
    }
}
/**Transmit FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`tfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TFCR_SPEC;
impl crate::RegisterSpec for TFCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tfcr::R`](R) reader structure
impl crate::Readable for TFCR_SPEC {}
///`write(|w| ..)` method takes [`tfcr::W`](W) writer structure
impl crate::Writable for TFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TFCR to value 0x80
impl crate::Resettable for TFCR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
